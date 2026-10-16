//! Инвариант I4: любой сбой детекции не приводит к отправке исходного текста.
//!
//! # Что проверяется
//!
//! Восемь сценариев сбоя. В каждом из них наружу не уходит исходный текст:
//! либо вызов возвращает ошибку, либо наружный текст не содержит исходных
//! значений. Промежуточного варианта «текст ушёл как есть» быть не должно.
//!
//! | № | Сценарий | Ожидаемое поведение |
//! |---|---|---|
//! | 1 | Падение каскада L0 | ошибка, отправки нет |
//! | 2 | Модель L2 не загружена | ошибка, отправки нет |
//! | 3 | Таймаут провайдера L2 | ошибка, отправки нет |
//! | 4 | Диалоговый каскад отключён при требующей политике | ошибка, отправки нет |
//! | 5 | Карта повреждена | ошибка, **карта не заменяется пустой** |
//! | 6 | Ключ шифрования недоступен | ошибка, отправки нет |
//! | 7 | Детектор вернул спан за границами текста | ошибка, отправки нет |
//! | 8 | Обнаружены данные специальной категории | блокировка отправки |

mod common;

use common::{FailingDetector, FixedDetector, OutOfBoundsDetector, TestGenerator};
use privacy_core::{CoreError, FailureBehavior, Policy, Session};

#[test]
fn scenario_1_cascade_panic_blocks_sending() {
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FailingDetector));

    let result = session.anonymize("Мой ИНН 0000000000");
    assert!(
        matches!(result, Err(CoreError::CascadeFailed { .. })),
        "сбой каскада обязан возвращать ошибку, а не текст"
    );
}

#[test]
fn scenario_2_model_unavailable_blocks_sending() {
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FailingDetector));

    let result = session.anonymize("Текст без явных данных");
    assert!(result.is_err(), "недоступная модель не должна пропускать текст");
}

#[test]
fn scenario_3_provider_timeout_blocks_sending() {
    // Таймаут моделируется тем же путём, что и недоступность: провайдер
    // возвращает ошибку, и ядро обязано её пробросить, не пропуская текст.
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(FailingDetector));

    assert!(session.anonymize("Что-то важное").is_err());
}

#[test]
#[ignore = "фаза 2: политика, требующая L3, ещё не выражена в коде"]
fn scenario_4_dialog_cascade_disabled_blocks_sending() {
    // Когда политика требует диалоговый каскад, а он отключён, ядро должно
    // вернуть DetectorUnavailable, а не молча работать без него.
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()));
    let result = session.anonymize("Он сказал, что та фирма заплатит");
    assert!(matches!(
        result,
        Err(CoreError::DetectorUnavailable { .. })
    ));
}

#[test]
#[ignore = "фаза 2: хранилище карт не реализовано"]
fn scenario_5_corrupted_mapping_is_not_replaced_with_empty() {
    // Ключевое: при повреждении карты запрещено вернуть пустую карту,
    // иначе следующая же реплика уйдёт без обезличивания.
    unimplemented!("требуется реализация MappingStore с проверкой целостности");
}

#[test]
#[ignore = "фаза 2: поставщик ключа не реализован"]
fn scenario_6_missing_key_blocks_sending() {
    unimplemented!("требуется реализация KeyProvider и его отказ");
}

#[test]
fn scenario_7_out_of_bounds_span_blocks_sending() {
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(OutOfBoundsDetector));

    let result = session.anonymize("Короткий текст");
    assert!(
        matches!(result, Err(CoreError::InvalidSpan { .. })),
        "спан за границами текста обязан приводить к ошибке"
    );
}

#[test]
#[ignore = "фаза 2: реестр специальных категорий не подключён"]
fn scenario_8_special_category_blocks_sending() {
    // По ч. 1 ст. 10 152-ФЗ обезличивание специальных категорий не допускается,
    // поэтому реакция — блокировка, а не подмена. См. phase-0/05-legal/.
    let input = "Диагноз: вымышленный";
    let (s, e) = common::span_of(input, "вымышленный");
    let detector = FixedDetector::new(vec![(s, e, "ru.health_diagnosis")]);
    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let result = session.anonymize("Диагноз: вымышленный");
    assert!(
        matches!(result, Err(CoreError::SpecialCategory { .. })),
        "специальные категории обязаны блокировать отправку"
    );
}

/// Режим «пропустить при сбое» не должен быть поведением по умолчанию.
#[test]
fn default_policy_never_passes_through_on_failure() {
    let policy = Policy::default();
    assert_eq!(policy.on_failure, FailureBehavior::Block);
    assert!(policy.on_failure.preserves_fail_closed());
}

/// Даже при явном выборе «деградировать» текст не уходит без обезличивания.
#[test]
fn degrade_mode_still_anonymizes_what_it_found() {
    let mut policy = Policy::ru_140();
    policy.on_failure = FailureBehavior::DegradeAndWarn;

    let input = "ИНН 0000000000";
    let (s, e) = common::span_of(input, "0000000000");
    let detector = FixedDetector::new(vec![(s, e, "ru.inn_legal")]);
    let mut session = Session::new(policy, Box::new(TestGenerator::new()))
        .with_detector(Box::new(detector));

    let out = session
        .anonymize("ИНН 0000000000")
        .expect("при деградации текст всё равно должен обезличиваться");
    assert!(!out.text.contains("0000000000"));
}

/// Явный режим «пропустить» нарушает I4 и должен быть виден как нарушение.
#[test]
#[ignore = "фаза 2: режим PassThrough должен быть запрещён в продакшн-профиле"]
fn pass_through_mode_is_reported_as_invariant_violation() {
    let mut policy = Policy::ru_140();
    policy.on_failure = FailureBehavior::PassThrough;

    let mut session = Session::new(policy, Box::new(TestGenerator::new()))
        .with_detector(Box::new(FailingDetector));

    let result = session.anonymize("ИНН 0000000000");
    assert!(matches!(
        result,
        Err(CoreError::InvariantViolation { invariant: "I4", .. })
    ));
}
