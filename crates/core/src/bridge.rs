//! Мост между ядром и внешними источниками находок.
//!
//! # Зачем мост, а не прямая зависимость
//!
//! Идентификаторы устройства живут в отдельном крейте `privacy-os-truth`,
//! потому что они опираются на окружение процесса и файлы пользователя,
//! тогда как ядро обязано быть чистым преобразованием текста. Если ядро
//! начнёт зависеть от сборщика идентификаторов, то:
//!
//! * ядро станет невозможно тестировать без окружения;
//! * появится соблазн читать файлы изнутри детекции;
//! * нарушится запрет из раздела 4.1 рабочего документа на знание ядра
//!   о чём-либо, кроме текста.
//!
//! Поэтому здесь только **тип-переходник**: ядро объявляет трейт
//! [`SpanSource`], а конкретную реализацию для идентификаторов устройства
//! предоставляет крейт `privacy-os-truth`.

use crate::confidence::Confidence;
use crate::detect::{Detected, Detector};
use crate::entity::{Cascade, EntityTypeId};
use crate::error::Result;
use crate::span::Span;

/// Источник готовых находок, у которого нет зависимости на типы ядра.
///
/// Реализуется внешними крейтами: `privacy-os-truth` и любыми будущими
/// источниками (например, реестром сущностей сессии).
pub trait SpanSource {
    /// Имя источника для отчёта.
    fn source_name(&self) -> &'static str;

    /// Ищет находки и отдаёт их в виде `(тип, начало, конец, подтверждён)`.
    ///
    /// Четвёртый элемент — подтверждён ли контекст. Источники, которым
    /// контекст не нужен (полный путь, адрес почты), всегда возвращают `true`.
    ///
    /// Возвращает `Vec`, а не итератор, потому что ядру нужна длина и
    /// возможность проверить границы до использования.
    fn find_spans(&self, text: &str) -> Vec<ExternalSpan>;

    /// Отпечаток значения. Нужен для отчёта, чтобы отличать находки,
    /// не раскрывая значений.
    fn fingerprint_of(&self, detection: &ExternalSpan) -> String;
}

/// Находка внешнего источника.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalSpan {
    /// Тип сущности в терминах ядра, например `os.user_name`.
    pub entity_type: String,
    /// Идентификатор сработавшего правила.
    pub rule_id: String,
    /// Начало в байтах.
    pub start: usize,
    /// Конец в байтах.
    pub end: usize,
    /// Подтверждён ли контекст.
    pub context_confirmed: bool,
    /// Отпечаток значения.
    pub fingerprint: String,
}

/// Адаптер: превращает внешний источник находок в каскад ядра.
///
/// # Уровень уверенности
///
/// Находки источника идентификаторов устройства получают уровень
/// [`Confidence::ChecksumPlusContext`] при подтверждённом контексте и
/// [`Confidence::ChecksumOnly`] без него. Причина: значение в тексте **не
/// угадано, а точно известно** системе, поэтому уверенность выше, чем у
/// модели, но требование контекста для коротких значений сохраняется.
///
/// # Каскад
///
/// Всегда [`Cascade::Format`]: источник не использует ни статистику, ни
/// диалог, и его результат не должен зависеть от того, загружена ли модель.
pub struct ExternalDetector<S: SpanSource> {
    source: S,
}

impl<S: SpanSource> ExternalDetector<S> {
    /// Создаёт адаптер.
    pub fn new(source: S) -> Self {
        Self { source }
    }

    /// Доступ к источнику.
    pub fn source(&self) -> &S {
        &self.source
    }
}

impl<S: SpanSource> Detector for ExternalDetector<S> {
    fn name(&self) -> &'static str {
        self.source.source_name()
    }

    fn cascade(&self) -> Cascade {
        Cascade::Format
    }

    fn detect(&self, text: &str) -> Result<Vec<Detected>> {
        let found = self.source.find_spans(text);
        Ok(found
            .into_iter()
            .map(|item| Detected {
                span: Span::new(item.start, item.end),
                entity_type: EntityTypeId::new(item.entity_type),
                confidence: if item.context_confirmed {
                    Confidence::ChecksumPlusContext
                } else {
                    Confidence::ChecksumOnly
                },
                cascade: Cascade::Format,
                rule_id: Some(item.rule_id),
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeSource {
        spans: Vec<ExternalSpan>,
    }

    impl SpanSource for FakeSource {
        fn source_name(&self) -> &'static str {
            "fake-source"
        }

        fn find_spans(&self, _text: &str) -> Vec<ExternalSpan> {
            self.spans.clone()
        }

        fn fingerprint_of(&self, detection: &ExternalSpan) -> String {
            detection.fingerprint.clone()
        }
    }

    fn span(start: usize, end: usize, confirmed: bool) -> ExternalSpan {
        ExternalSpan {
            entity_type: "os.user_name".into(),
            rule_id: "os-truth.user-name".into(),
            start,
            end,
            context_confirmed: confirmed,
            fingerprint: "00".into(),
        }
    }

    #[test]
    fn confirmed_context_gets_higher_confidence() {
        let adapter = ExternalDetector::new(FakeSource {
            spans: vec![span(0, 5, true), span(10, 15, false)],
        });
        let found = adapter.detect("любой текст").expect("детекция");
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].confidence, Confidence::ChecksumPlusContext);
        assert_eq!(found[1].confidence, Confidence::ChecksumOnly);
    }

    #[test]
    fn adapter_reports_format_cascade() {
        let adapter = ExternalDetector::new(FakeSource { spans: vec![] });
        assert_eq!(adapter.cascade(), Cascade::Format);
        assert_eq!(adapter.name(), "fake-source");
    }

    #[test]
    fn spans_are_passed_through_unchanged() {
        let adapter = ExternalDetector::new(FakeSource {
            spans: vec![span(3, 9, true)],
        });
        let found = adapter.detect("любой текст").expect("детекция");
        assert_eq!(found[0].span, Span::new(3, 9));
        assert_eq!(found[0].entity_type.as_str(), "os.user_name");
        assert_eq!(found[0].rule_id.as_deref(), Some("os-truth.user-name"));
    }
}
