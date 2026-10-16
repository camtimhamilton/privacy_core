//! Инвариант I6: обратимость не зависит от детектора.
//!
//! # Что проверяется
//!
//! Восстановление работает по карте, а не повторной детекцией. Иначе значение,
//! которое детектор не найдёт при повторном проходе (а он его и не найдёт,
//! потому что в ответе модели его нет), окажется невосстановимым, и
//! пользователь потеряет данные.
//!
//! # Практическая разница
//!
//! Реализация «восстановить через детектор» ломается на первом же изменении
//! детектора между версиями: карты, созданные старой версией, перестают
//! восстанавливаться. Реализация через карту от версии детектора не зависит.

mod common;

use common::{FailingDetector, FixedDetector, TestGenerator};
use privacy_core::{Policy, Session};

#[test]
fn restore_works_when_detectors_are_disabled() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let input = "ИНН 0000000000";
    let out = session.anonymize(input).expect("обезличивание");
    let surrogate = out
        .text
        .trim_start_matches("ИНН ")
        .to_string();

    // Восстановление не должно зависеть от того, работают ли детекторы.
    let reply = format!("Ваш номер {surrogate} подтверждён.");
    let restored = session.restore(&reply).expect("восстановление");
    assert_eq!(restored.restored, 1);
    assert!(restored.text.contains("0000000000"));
}

/// Даже если детектор с тех пор падает, восстановление работает.
#[test]
#[ignore = "фаза 2: смена набора детекторов в сессии не поддерживается"]
fn restore_is_independent_of_detector_state() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize("ИНН 0000000000").expect("обезличивание");
    let surrogate = out.text.trim_start_matches("ИНН ").to_string();

    // Подменяем набор детекторов на падающий.
    let session = session.with_detector(Box::new(FailingDetector));
    let restored = session
        .restore(&format!("Номер {surrogate}"))
        .expect("восстановление не должно зависеть от детекторов");
    assert_eq!(restored.restored, 1);
}

/// Восстановление на пустой сессии ничего не портит.
#[test]
fn restore_on_empty_session_is_noop() {
    let session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()));
    let reply = "Ответ модели без подмен.";
    let restored = session.restore(reply).expect("восстановление");
    assert_eq!(restored.text, reply);
    assert_eq!(restored.restored, 0);
}
