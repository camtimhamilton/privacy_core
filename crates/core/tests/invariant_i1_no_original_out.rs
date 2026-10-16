//! Инвариант I1: ни одно исходное значение не покидает устройство.
//!
//! # Что проверяется
//!
//! Для любого входа и любого выхода наружу множество исходных значений,
//! присутствующих в выходе, пусто. Проверка идёт по всем нормализациям:
//! регистр, пробелы, дефисы, транслит, юникод-гомоглифы, цифры словами.
//!
//! # Почему тест помечен `#[ignore]`
//!
//! Фаза 1 объявляет контракт, фаза 2 реализует поведение. Тест обязан
//! **падать**, пока поведение не реализовано. Снятие `ignore` — критерий
//! готовности соответствующей части фазы 2.

mod common;

use common::{FixedDetector, TestGenerator};
use privacy_core::{Policy, Session};

/// Простой случай: значение заменено, в наружном тексте его нет.
#[test]
#[ignore = "фаза 2: детекторы и генератор подмен ещё не реализованы"]
fn original_value_is_absent_after_anonymization() {
    let input = "Мой ИНН 0000000000 для проверки";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize(input).expect("обезличивание не должно падать");
    assert!(
        !out.text.contains("0000000000"),
        "исходное значение не должно присутствовать в наружном тексте"
    );
}

/// Значение в другом регистре тоже считается утечкой.
#[test]
#[ignore = "фаза 2: проверка нормализационных вариантов не реализована"]
fn uppercase_variant_is_not_a_leak() {
    let input = "Инн 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize(input).expect("обезличивание не должно падать");
    assert!(!out.text.to_lowercase().contains("0000000000"));
}

/// Значение, разбитое дефисами и пробелами, тоже считается утечкой.
///
/// Это прямая проверка того, что нормализация ключа работает не только
/// для карты, но и для сравнения выхода с исходными значениями.
#[test]
#[ignore = "фаза 2: проверка разделителей не реализована"]
fn separated_variant_is_not_a_leak() {
    let input = "ИНН 000-000-000-0";
    let (s, e) = common::span_of(input, "000-000-000-0");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize(input).expect("обезличивание не должно падать");
    let digits_only: String = out.text.chars().filter(char::is_ascii_digit).collect();
    assert!(
        !digits_only.contains("0000000000"),
        "разбитое разделителями значение не должно утечь"
    );
}

/// Дефект детектора не должен приводить к отправке исходного текста.
#[test]
#[ignore = "фаза 2: обработка дефектов детектора не доведена"]
fn detector_defect_does_not_leak_original() {
    let input = "Мой ИНН 0000000000";
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(common::OutOfBoundsDetector));

    let result = session.anonymize(input);
    assert!(
        result.is_err(),
        "дефектный спан обязан приводить к ошибке, а не к отправке текста"
    );
}
