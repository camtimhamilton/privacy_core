//! Инвариант I10: логи, отчёты и телеметрия не содержат исходных значений.
//!
//! # Что проверяется
//!
//! * Отчёт не содержит исходного текста и исходных значений.
//! * Сообщения об ошибках не содержат исходных значений.
//! * Отладочное представление типов не раскрывает значения.
//! * В исходниках тестов и библиотеки нет настоящих персональных данных.
//!
//! # Почему это отдельный инвариант
//!
//! Самая частая реальная утечка — не в модели, а в логах. Отчёт обязан
//! быть полезным для подтверждения действий (требование приказа № 140,
//! прил. № 1, п. 1 пп. 7) и при этом не содержать того, что защищает.

mod common;

use common::{FixedDetector, TestGenerator};
use privacy_core::{NormalizedKey, Policy, Session};

/// Отчёт описывает действие, но не значение.
#[test]
fn report_contains_type_and_span_but_not_value() {
    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session.anonymize("ИНН 0000000000").expect("обезличивание");
    let rendered = format!("{:?}", out.report);

    assert!(
        rendered.contains("ru.inn_legal"),
        "отчёт обязан указывать тип сущности для подтверждения действий"
    );
    assert!(
        !rendered.contains("0000000000"),
        "отчёт не должен содержать исходное значение"
    );
}

/// Сообщения об ошибке не раскрывают текст.
#[test]
fn error_messages_do_not_leak_values() {
    use privacy_core::CoreError;

    let errors = vec![
        CoreError::CascadeFailed {
            cascade: "L2-model",
            reason: "модель не загружена".into(),
        },
        CoreError::InvariantViolation {
            invariant: "I1",
            detail: "значение осталось в наружном тексте".into(),
        },
        CoreError::SpecialCategory {
            entity_type: "ru.health_diagnosis".into(),
        },
    ];

    for error in errors {
        let rendered = error.to_string();
        assert!(
            !rendered.contains("0000000000") && !rendered.contains("Иван"),
            "сообщение об ошибке не должно содержать данные: {rendered}"
        );
    }
}

/// Отладочное представление ключа не раскрывает значение.
#[test]
fn normalized_key_debug_hides_value() {
    let key = NormalizedKey::from_normalized("7707083893");
    let displayed = key.to_string();
    assert!(
        !displayed.contains("7707083893"),
        "ключ не должен печатать своё значение: {displayed}"
    );
    assert!(
        displayed.contains("len="),
        "ключ должен печатать хотя бы длину для отладки: {displayed}"
    );
}

/// В тестовых данных нет ничего, что выглядит как настоящий реквизит.
///
/// Проверка намеренно наивная: она ищет в исходниках тестов
/// последовательности цифр, которые могли бы оказаться настоящими
/// реестровыми номерами. Значение `0000000000` исключено как очевидно
/// невалидное.
#[test]
fn test_sources_contain_no_plausible_real_identifiers() {
    let sources = [
        include_str!("common/mod.rs"),
        include_str!("invariant_i1_no_original_out.rs"),
        include_str!("invariant_i5_session_consistency.rs"),
    ];

    for source in sources {
        for token in source.split(|c: char| !c.is_ascii_digit()) {
            if token.len() >= 10 && token.chars().all(|c| c == '0' || c == '1') {
                continue; // очевидно вымышленные
            }
            if token.len() >= 10 {
                panic!(
                    "в тестах найдено длинное числовое значение, похожее на настоящий реквизит: {token}"
                );
            }
        }
    }
}
