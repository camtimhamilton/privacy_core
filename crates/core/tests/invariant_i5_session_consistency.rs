//! Инвариант I5: согласованность подмен внутри сессии.
//!
//! # Что проверяется
//!
//! * Одно исходное значение даёт **одну** подмену на всю сессию.
//! * Разные значения дают **разные** подмены.
//! * Карта биективна (ГОСТ Р 71674-2024, п. 5.4.1).
//!
//! # Почему это не косметика
//!
//! Если два разных Ивана Петрова получат одну подмену, модель начнёт их
//! путать, а восстановление станет неоднозначным. Если один человек получит
//! две разные подмены, модель решит, что их двое, и ответ будет бессмысленным.

mod common;

use common::{FixedDetector, TestGenerator};
use privacy_core::{CoreError, Policy, Session};

/// Позиции спанов считаются по байтам, поэтому в тестах их нельзя
/// задавать «по буквам»: кириллица занимает по два байта на символ.
/// Ниже позиции вычисляются поиском подстроки, а не угадываются.
fn span_of(text: &str, needle: &str) -> (usize, usize) {
    let start = text.find(needle).expect("подстрока должна присутствовать");
    (start, start + needle.len())
}

#[test]
fn same_value_gets_same_surrogate_within_session() {
    let text = "Иван текст текст текст Иван";
    // Оба вхождения одного слова должны дать одну запись в карте.
    let (s1, e1) = span_of(text, "Иван");
    let second_start = text.rfind("Иван").expect("второе вхождение");
    let detector = FixedDetector::new(vec![
        (s1, e1, "ru.fio"),
        (second_start, second_start + "Иван".len(), "ru.fio"),
    ]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize(text).expect("обезличивание");
    assert_eq!(out.report.methods.len(), 2, "оба вхождения обработаны");
    assert_eq!(
        session.mapping().len(),
        1,
        "одно значение — одна запись в карте"
    );
}

#[test]
fn different_values_get_different_surrogates() {
    let text = "Иван Петр";
    let (s1, e1) = span_of(text, "Иван");
    let (s2, e2) = span_of(text, "Петр");
    let detector = FixedDetector::new(vec![(s1, e1, "ru.fio"), (s2, e2, "ru.fio")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    session.anonymize(text).expect("обезличивание");
    assert_eq!(
        session.mapping().len(),
        2,
        "разные значения — две записи в карте"
    );
}

/// Прямая проверка биекции через карту: повторная вставка другой подмены
/// для того же ключа обязана быть отвергнута.
#[test]
fn mapping_rejects_collisions() {
    use privacy_core::mapping::{Mapping, MappingEntry, MappingId, SubstitutionKind};
    use privacy_core::NormalizedKey;

    let mut mapping = Mapping::new(MappingId::new("t"));
    mapping
        .insert(MappingEntry {
            key: NormalizedKey::from_normalized("иван петров"),
            surrogate: "[ФИО-1]".into(),
            entity_type: "ru.fio".into(),
            kind: SubstitutionKind::SimilarFake,
            generator_version: 1,
        })
        .expect("первая вставка");

    // Тот же ключ, другая подмена — нарушение I5.
    let err = mapping.insert(MappingEntry {
        key: NormalizedKey::from_normalized("иван петров"),
        surrogate: "[ФИО-2]".into(),
        entity_type: "ru.fio".into(),
        kind: SubstitutionKind::SimilarFake,
        generator_version: 1,
    });
    assert!(matches!(
        err,
        Err(CoreError::InvariantViolation { invariant: "I5", .. })
    ));
}

/// Нормализация: «ИНН 000-000-000-0» и «0000000000» — одно значение.
#[test]
#[ignore = "фаза 2: нормализация по типу в детекторах ещё не подключена"]
fn normalized_variants_share_one_surrogate() {
    let text = "ИНН 000-000-000-0, снова 0000000000";
    let (s1, e1) = span_of(text, "000-000-000-0");
    let (s2, e2) = span_of(text, "0000000000");
    let detector = FixedDetector::new(vec![(s1, e1, "ru.inn_legal"), (s2, e2, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    session.anonymize(text).expect("обезличивание");
    assert_eq!(
        session.mapping().len(),
        1,
        "разные написания одного значения — одна запись"
    );
}
