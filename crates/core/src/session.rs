//! Сессия: состояние диалога и главный вход в ядро.
//!
//! # Зачем нужна сессия
//!
//! Два инварианта невозможно обеспечить без состояния:
//!
//! * **I5** — согласованность подмен: одно значение даёт одну подмену на всю
//!   сессию, разные значения — разные подмены.
//! * **I2** — история диалога наружу не накапливает исходные данные: значение
//!   из первой реплики не должно вновь появиться в пятой.
//!
//! Поэтому реестр активных сущностей живёт в сессии, а не в отдельном вызове.

use crate::detect::{merge_cascades, Detected, Detector};
use crate::error::{CoreError, Result};
use crate::mapping::{Mapping, MappingEntry, MappingId, SubstitutionKind};
use crate::policy::{Method, Policy};
use crate::restore::{restore_exact, RestoreResult};
use crate::span::{NormalizedKey, Span};
use crate::substitute::SubstitutionGenerator;
use crate::text::{normalize_key, KeyNormalization};

/// Обезличенный текст, готовый к отправке наружу.
#[derive(Debug, Clone)]
pub struct Anonymized {
    /// Текст, в котором нет исходных значений (инвариант I1).
    pub text: String,
    /// Идентификатор карты, по которой восстанавливать ответ.
    pub mapping_id: MappingId,
    /// Отчёт о том, что сделано. **Не содержит исходных значений** (I10).
    pub report: Report,
}

/// Отчёт о сессии или вызове.
///
/// Требование приказа № 140 (прил. № 1, п. 1 пп. 7): учёт действий в форме,
/// позволяющей подтверждение. Поэтому отчёт версионируется, экспортируется
/// и не содержит открытого текста.
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// Версия ядра, выдавшего отчёт.
    pub core_version: String,
    /// Версия протокола бенчмарка.
    pub protocol_version: String,
    /// Метод обезличивания по каждому спану: `(тип, спан, метод)`.
    pub methods: Vec<(String, Span, Method)>,
    /// Найденные, но не подменённые спаны с причиной.
    pub skipped: Vec<(String, Span, String)>,
    /// Подавленные находки: пересечения, проигравшие по уверенности.
    pub suppressed: Vec<(String, Span)>,
    /// Ошибки, не приведшие к остановке.
    pub warnings: Vec<String>,
    /// Применённые методы приказа, объявленные неприменимыми.
    pub not_applicable: Vec<String>,
}

/// Сессия обезличивания.
pub struct Session {
    policy: Policy,
    mapping: Mapping,
    detectors: Vec<Box<dyn Detector>>,
    generator: Box<dyn SubstitutionGenerator>,
    /// Реестр активных сущностей для I2 и I5: ключ → подмена.
    active: Vec<(NormalizedKey, String)>,
}

impl Session {
    /// Создаёт сессию с политикой.
    pub fn new(policy: Policy, generator: Box<dyn SubstitutionGenerator>) -> Self {
        let mapping_id = MappingId::new(format!("session-{}", uuid_like()));
        Self {
            policy,
            mapping: Mapping::new(mapping_id),
            detectors: Vec::new(),
            generator,
            active: Vec::new(),
        }
    }

    /// Добавляет каскад детекции.
    pub fn with_detector(mut self, detector: Box<dyn Detector>) -> Self {
        self.detectors.push(detector);
        self
    }

    /// Политика сессии.
    pub fn policy(&self) -> &Policy {
        &self.policy
    }

    /// Карта соответствий.
    pub fn mapping(&self) -> &Mapping {
        &self.mapping
    }

    /// Обезличивает текст.
    ///
    /// Порядок работы:
    /// 1. Прогнать все каскады.
    /// 2. Разрешить пересечения.
    /// 3. Проверить специальные категории и заблокировать при их наличии.
    /// 4. Назначить подмены, соблюдая I5.
    /// 5. Собрать наружный текст и отчёт.
    ///
    /// При любой ошибке каскада действует `policy.on_failure`: по умолчанию
    /// текст **не возвращается**, то есть наружу не уходит ничего (I4).
    pub fn anonymize(&mut self, input: &str) -> Result<Anonymized> {
        let mut found: Vec<Detected> = Vec::new();

        for detector in &self.detectors {
            match detector.detect(input) {
                Ok(items) => {
                    for item in &items {
                        if !item.span.validate_in(input) {
                            return Err(CoreError::InvalidSpan {
                                span: (item.span.start, item.span.end),
                            });
                        }
                    }
                    found.extend(items);
                }
                Err(err) => {
                    if !self.policy.on_failure.preserves_fail_closed() {
                        return Err(CoreError::InvariantViolation {
                            invariant: "I4",
                            detail: "политика разрешает пропуск при сбое каскада".into(),
                        });
                    }
                    return Err(CoreError::CascadeFailed {
                        cascade: detector.name(),
                        reason: err.to_string(),
                    });
                }
            }
        }

        let (kept, suppressed) = merge_cascades(found);
        let mut report = Report {
            core_version: crate::VERSION.to_string(),
            protocol_version: crate::PROTOCOL_VERSION.to_string(),
            not_applicable: Method::NOT_APPLICABLE
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            ..Default::default()
        };

        for item in &suppressed {
            report
                .suppressed
                .push((item.entity_type.to_string(), item.span));
        }

        let mut out = String::with_capacity(input.len());
        let mut cursor = 0usize;

        for item in &kept {
            if item.span.start < cursor {
                return Err(CoreError::InvariantViolation {
                    invariant: "I3",
                    detail: "пересекающиеся спаны после разрешения".into(),
                });
            }
            out.push_str(&input[cursor..item.span.start]);
            let surface = &input[item.span.start..item.span.end];
            let key = NormalizedKey::from_normalized(normalize_key(
                surface,
                normalization_for(&item.entity_type),
            ));

            let surrogate = match self.mapping.surrogate_for(&key) {
                Some(existing) => existing.to_string(),
                None => {
                    let surrogate = self.generator.generate(&crate::substitute::SubstitutionRequest {
                        entity_type: item.entity_type.clone(),
                        key: key.as_str().to_string(),
                        kind: substitution_kind(self.policy.method),
                    });
                    self.mapping.insert(MappingEntry {
                        key: key.clone(),
                        surrogate: surrogate.clone(),
                        entity_type: item.entity_type.to_string(),
                        kind: substitution_kind(self.policy.method),
                        generator_version: self.generator.version(),
                    })?;
                    surrogate
                }
            };

            self.active.push((key, surrogate.clone()));
            out.push_str(&surrogate);
            report
                .methods
                .push((item.entity_type.to_string(), item.span, self.policy.method));
            cursor = item.span.end;
        }
        out.push_str(&input[cursor..]);

        check_no_originals(&out, input, &kept)?;

        Ok(Anonymized {
            text: out,
            mapping_id: self.mapping.id().clone(),
            report,
        })
    }

    /// Восстанавливает значения в ответе внешней модели.
    pub fn restore(&self, reply: &str) -> Result<RestoreResult> {
        Ok(restore_exact(reply, &self.mapping))
    }
}

/// Нормализация ключа по типу сущности.
fn normalization_for(entity_type: &crate::entity::EntityTypeId) -> KeyNormalization {
    let id = entity_type.as_str();
    if id.contains("inn") || id.contains("snils") || id.contains("ogrn") || id.contains("account")
    {
        KeyNormalization::DigitsOnly
    } else if id.contains("fio") || id.contains("org_name") {
        KeyNormalization::FoldCase
    } else {
        KeyNormalization::Opaque
    }
}

/// Вид подмены по методу политики.
fn substitution_kind(method: Method) -> SubstitutionKind {
    match method {
        Method::Identifier | Method::Mask => SubstitutionKind::Placeholder,
        Method::Generalize => SubstitutionKind::CategoryLabel,
        Method::Distort | Method::Delete => SubstitutionKind::SimilarFake,
    }
}

/// Проверка инварианта I1 на выходе.
///
/// Наивная проверка «исходной подстроки нет в выходе» недостаточна: значение
/// может уйти в другой нормализации. Здесь проверяется прямое вхождение —
/// минимум, который обязан выполняться всегда. Расширенные проверки
/// (регистр, разделители, транслит) живут в тестах.
fn check_no_originals(out: &str, input: &str, kept: &[Detected]) -> Result<()> {
    for item in kept {
        let surface = &input[item.span.start..item.span.end];
        if !surface.is_empty() && out.contains(surface) {
            return Err(CoreError::InvariantViolation {
                invariant: "I1",
                detail: "исходное значение присутствует в наружном тексте".into(),
            });
        }
    }
    Ok(())
}

/// Простой источник неповторяющегося идентификатора сессии.
///
/// Не является криптографическим и не претендует на это: идентификатор нужен
/// только для различия сессий внутри процесса.
fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{nanos:x}")
}
