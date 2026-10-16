//! Инвариант I2: история диалога наружу не накапливает исходные данные.
//!
//! # Что проверяется
//!
//! Третья реплика сессии не может содержать исходное значение, даже если оно
//! было названо в первой и больше не упоминалось. Проверять надо **всю
//! историю**, а не одно сообщение: именно накопление даёт утечку в 3 раза
//! на диалогах в 6 ходов (замер CIMemories Dialogue).
//!
//! # Почему это не то же самое, что I1
//!
//! I1 — про один вызов. I2 — про последовательность вызовов одной сессии.
//! Реализация I1 не даёт I2: значение может быть заменено в первой реплике
//! и снова появиться в третьей, если состояние сессии не учитывается.

mod common;

use common::{FixedDetector, TestGenerator};
use privacy_core::{Policy, Session};

/// Значение из первой реплики не появляется в наружном тексте пятой.
#[test]
#[ignore = "фаза 2: состояние сессии между вызовами не реализовано"]
fn value_from_first_turn_does_not_reappear_in_fifth() {
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![])));

    let first = "Мой ИНН 0000000000 и телефон";
    let out1 = session.anonymize(first).expect("первая реплика");
    assert!(!out1.text.contains("0000000000"));

    for turn in 2..=4 {
        let text = format!("Реплика {turn} без данных");
        session.anonymize(&text).expect("промежуточная реплика");
    }

    // В пятой реплике значение упоминается снова, но уже как известное сессии.
    let fifth = "Напомню: ИНН 0000000000";
    let out5 = session.anonymize(fifth).expect("пятая реплика");
    assert!(
        !out5.text.contains("0000000000"),
        "значение из первой реплики не должно утечь в пятой"
    );
}

/// Наружная история не содержит исходных данных ни на одной реплике.
#[test]
#[ignore = "фаза 2: накопление истории не проверяется"]
fn accumulated_outbound_history_is_clean() {
    let turns = [
        "Меня зовут Иван, ИНН 0000000000",
        "Уточню детали",
        "Ещё вопрос",
        "И последнее",
        "Итог",
    ];
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![])));

    let mut outbound = String::new();
    for turn in turns {
        let out = session.anonymize(turn).expect("реплика");
        outbound.push_str(&out.text);
        outbound.push('\n');
    }

    assert!(
        !outbound.contains("0000000000"),
        "ни одна наружная реплика не должна содержать исходное значение"
    );
}

/// Длинный диалог: проверка на 20 репликах, как требует Core-уровень
/// бенчмарка (проверка C2).
#[test]
#[ignore = "фаза 2: 20 реплик не проверялись"]
fn twenty_turn_session_has_no_accumulation() {
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FixedDetector::new(vec![])));

    let mut outbound = String::new();
    for turn in 1..=20 {
        let text = if turn == 1 {
            "Первый вопрос, ИНН 0000000000".to_string()
        } else {
            format!("Реплика номер {turn}")
        };
        let out = session.anonymize(&text).expect("реплика");
        outbound.push_str(&out.text);
    }

    assert!(!outbound.contains("0000000000"));
}
