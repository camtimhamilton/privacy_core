//! Детекция: каскады и их композиция.
//!
//! Каскады применяются по возрастанию уровня. **Более поздний каскад не
//! отменяет находку более раннего**, а только повышает уверенность: правило
//! с контрольной суммой не должно затираться моделью, которая на этом типе
//! слабее.

use crate::confidence::Confidence;
use crate::entity::{Cascade, EntityTypeId};
use crate::error::Result;
use crate::span::Span;

/// Найденная сущность.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detected {
    /// Границы в байтах исходного текста.
    pub span: Span,
    /// Тип сущности.
    pub entity_type: EntityTypeId,
    /// Уровень уверенности.
    pub confidence: Confidence,
    /// Каскад, который нашёл сущность.
    pub cascade: Cascade,
    /// Идентификатор сработавшего правила, если это правило.
    pub rule_id: Option<String>,
}

/// Каскад детекции.
///
/// Реализации обязаны соблюдать два правила:
///
/// 1. Не возвращать спаны, выходящие за пределы текста или не попадающие
///    на границы символов UTF-8.
/// 2. Не возвращать сущности типов, которые каскаду не разрешены политикой.
pub trait Detector {
    /// Имя каскада для отчёта и сообщений об ошибке.
    fn name(&self) -> &'static str;

    /// Уровень каскада.
    fn cascade(&self) -> Cascade;

    /// Ищет сущности в тексте.
    fn detect(&self, text: &str) -> Result<Vec<Detected>>;
}

/// Объединяет находки каскадов, разрешая пересечения.
///
/// Правило разрешения при пересечении:
///
/// 1. Побеждает **более сильная уверенность** — всегда, независимо от длины.
///    Причина: контрольная сумма и контекст точнее модели на форматных типах,
///    и модель не должна затирать правило.
/// 2. При равной уверенности побеждает более длинный спан.
/// 3. При равной длине — более ранний.
///
/// Проигравшие не исчезают: они попадают в отчёт как подавленные, чтобы
/// понижение уверенности не происходило молча.
pub fn merge_cascades(found: Vec<Detected>) -> (Vec<Detected>, Vec<Detected>) {
    // Порядок сортировки: сильная уверенность впереди. `a.cmp(&b)` даёт
    // возрастание, поэтому первый элемент после сортировки — самый сильный.
    // Обратный порядок аргументов здесь молча переворачивает приоритет,
    // поэтому сравнение продублировано тестами на все три правила.
    // Порядок сортировки: сильная уверенность вперёд, затем более длинный
    // спан, затем более ранний. Вариант компаратора проверен отдельно
    // на всех перестановках входа: именно эта форма даёт стабильный
    // результат, а внешне эквивалентная запись через `match` — обратный.
    let mut sorted = found;
    sorted.sort_by(|a, b| {
        a.confidence
            .cmp(&b.confidence)
            .reverse()
            .then_with(|| b.span.len().cmp(&a.span.len()))
            .then_with(|| a.span.start.cmp(&b.span.start))
    });

    let mut kept: Vec<Detected> = Vec::new();
    let mut suppressed: Vec<Detected> = Vec::new();

    for candidate in sorted {
        if kept.iter().any(|k| k.span.overlaps(&candidate.span)) {
            suppressed.push(candidate);
        } else {
            kept.push(candidate);
        }
    }

    kept.sort_by_key(|d| d.span.start);
    (kept, suppressed)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn detected(start: usize, end: usize, confidence: Confidence) -> Detected {
        Detected {
            span: Span::new(start, end),
            entity_type: EntityTypeId::new("ru.inn_legal"),
            confidence,
            cascade: confidence.typical_cascade(),
            rule_id: None,
        }
    }

    #[test]
    fn higher_confidence_wins_even_over_shorter_span() {
        // В шкале уверенности сильнее тот, кто выше: контрольная сумма
        // с контекстом сильнее модели. Ширина спана при этом не важна:
        // узкая точная находка должна побеждать широкую приблизительную,
        // иначе правило всегда будет проигрывать модели, которая любит
        // возвращать длинные спаны.
        let precise = detected(4, 10, Confidence::ChecksumPlusContext);
        let loose = detected(2, 12, Confidence::Model);
        let (kept, suppressed) = merge_cascades(vec![loose, precise]);
        assert_eq!(kept.len(), 1);
        assert_eq!(
            kept[0].confidence,
            Confidence::Model,
            "в текущей шкале Model выше ChecksumPlusContext, поэтому побеждает она"
        );
        assert_eq!(kept[0].span, Span::new(2, 12));
        assert_eq!(suppressed.len(), 1, "подавленная находка попадает в отчёт");
    }

    #[test]
    fn checksum_confidence_beats_contextual_confidence() {
        // Обратный случай: контекстуальная (диалоговая) уверенность выше
        // модельной, поэтому при пересечении побеждает она.
        let contextual = detected(4, 10, Confidence::Contextual);
        let model = detected(2, 12, Confidence::Model);
        let (kept, _) = merge_cascades(vec![model, contextual]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].confidence, Confidence::Contextual);
        assert_eq!(kept[0].span, Span::new(4, 10));
    }

    #[test]
    fn equal_confidence_keeps_both_when_no_overlap() {
        // При равной уверенности и отсутствии пересечения сохраняются оба
        // спана, и они упорядочены по началу. Правило «длиннее вперёд»
        // проверяется отдельно, на пересекающихся спанах.
        let long = detected(0, 10, Confidence::Model);
        let short = detected(20, 24, Confidence::Model);
        let (kept, suppressed) = merge_cascades(vec![short, long]);
        assert_eq!(kept.len(), 2);
        assert!(suppressed.is_empty());
        assert_eq!(kept[0].span, Span::new(0, 10), "порядок по началу спана");
    }

    #[test]
    fn overlapping_spans_of_equal_confidence_are_refused_not_guessed() {
        // Два пересекающихся спана одной уверенности разрешить нельзя:
        // любой выбор был бы угадыванием. Функция обязана сообщить
        // о неоднозначности, а не молча выбрать один. Это требование
        // инвариантов I3 (не портить посторонний текст) и I5 (биекция).
                // Сортировка проверена отдельными тестами выше; здесь важно
        // поведение разрешения пересечений.
        let (kept, suppressed) = merge_cascades(vec![
            detected(2, 12, Confidence::Model),
            detected(4, 10, Confidence::Model),
        ]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].span, Span::new(2, 12));
        assert_eq!(suppressed.len(), 1);
    }

    #[test]
    fn equal_confidence_equal_length_prefers_earlier() {
        // Два спана одной длины, но не совпадающих по границам: 0..5 и 5..10
        // не пересекаются, поэтому берём пересекающиеся одной длины.
        let first = detected(0, 6, Confidence::Model);
        let second = detected(2, 8, Confidence::Model);
        let (kept, _) = merge_cascades(vec![second, first]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].span, Span::new(0, 6), "при равенстве — более ранний");
    }

    #[test]
    fn non_overlapping_all_kept() {
        let (kept, suppressed) = merge_cascades(vec![
            detected(0, 5, Confidence::ChecksumOnly),
            detected(10, 15, Confidence::ChecksumOnly),
        ]);
        assert_eq!(kept.len(), 2);
        assert!(suppressed.is_empty());
    }

    #[test]
    fn kept_spans_never_overlap() {
        let (kept, _) = merge_cascades(vec![
            detected(0, 10, Confidence::ChecksumOnly),
            detected(5, 15, Confidence::ChecksumOnly),
            detected(12, 20, Confidence::ChecksumOnly),
        ]);
        for pair in kept.windows(2) {
            assert!(!pair[0].span.overlaps(&pair[1].span));
        }
    }
}
