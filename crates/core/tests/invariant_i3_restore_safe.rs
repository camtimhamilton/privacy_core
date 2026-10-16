//! Инвариант I3: восстановление не искажает посторонний текст.
//!
//! # Что проверяется
//!
//! Если последовательность, совпадающая с подменой, встретилась в ответе
//! модели **не как наша подстановка** — совпадение, цитата, часть другого
//! слова — восстановление её не трогает.
//!
//! # Почему это серьёзно
//!
//! Наивная реализация «взять и заменить все вхождения» портит текст: модель
//! может написать «[ИНН_1]» в примере, в объяснении или внутри другого слова.
//! Пользователь получит правку там, где её не должно быть, и не заметит.

mod common;

use common::{FixedDetector, TestGenerator};
use privacy_core::{Policy, Session};

/// Подмена внутри другого слова не восстанавливается.
#[test]
#[ignore = "фаза 2: различение границ слова не доведено"]
fn surrogate_inside_a_longer_word_is_untouched() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![(s, e, "ru.inn_legal")])));

    let out = session.anonymize(input).expect("обезличивание");
    // Строим ответ модели, где подмена приклеена к другому слову.
    let reply = format!("Префикс{}суффикс", out.text.trim_start_matches("ИНН "));
    let restored = session.restore(&reply).expect("восстановление");
    assert_eq!(
        restored.text, reply,
        "подмена внутри слова не должна восстанавливаться"
    );
}

/// Цитата подмены в кавычках восстанавливается корректно, без порчи кавычек.
#[test]
#[ignore = "фаза 2: обработка кавычек не проверена"]
fn quoted_surrogate_restores_without_damaging_quotes() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![(s, e, "ru.inn_legal")])));

    let out = session.anonymize(input).expect("обезличивание");
    let surrogate = out.text.trim_start_matches("ИНН ").to_string();
    let reply = format!("Я записал «{surrogate}» в документ.");
    let restored = session.restore(&reply).expect("восстановление");

    assert!(restored.text.contains('«') && restored.text.contains('»'));
    assert!(
        !restored.text.contains(&surrogate),
        "подмена в кавычках должна восстановиться"
    );
}

/// Подмена встречается дважды — оба вхождения восстанавливаются.
#[test]
#[ignore = "фаза 2: повторное вхождение не проверено"]
fn repeated_surrogate_restores_everywhere() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![(s, e, "ru.inn_legal")])));

    let out = session.anonymize(input).expect("обезличивание");
    let surrogate = out.text.trim_start_matches("ИНН ").to_string();
    let reply = format!("{surrogate} и снова {surrogate}.");
    let restored = session.restore(&reply).expect("восстановление");

    assert_eq!(
        restored.restored, 2,
        "оба вхождения должны быть восстановлены"
    );
}

/// Текст без подмен не меняется ни на байт.
#[test]
fn reply_without_surrogates_is_byte_identical() {
    let session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![])));

    let reply = "Обычный ответ модели без подмен.";
    let restored = session.restore(reply).expect("восстановление");
    assert_eq!(restored.text, reply);
    assert_eq!(restored.restored, 0);
}
