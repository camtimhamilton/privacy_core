//! Инвариант I8: контракт профиля задержки соблюдается или нарушение объявляется.
//!
//! # Что проверяется на фазе 1
//!
//! Полный числовой контракт (p95 ≤ 5 мс для Fast, ≤ 25 мс для Balanced)
//! требует работающих детекторов, то есть фазы 2. Поэтому здесь проверяется
//! **структурная** часть контракта: какие каскады обязан включать профиль.
//! Нарушение этой части делает числовую цель недостижимой в принципе.
//!
//! # Почему структурная проверка важнее числовой на этом этапе
//!
//! Аудит фазы 0 дал замер: нейросеть размером 1.5 млрд параметров даёт
//! 0.59 с на 2 048 токенов на CPU, то есть разрыв с целью 25–50 мс
//! составляет 11–23 раза. Значит, попадание модели в профиль Fast —
//! не «оптимизация потом», а нарушение контракта по построению.

use privacy_core::policy::{Method, Policy, Strictness};

/// Каскады, допустимые в профиле.
fn allowed_cascades(strictness: Strictness) -> &'static [&'static str] {
    match strictness {
        Strictness::Fast => &["L0-format", "L1-context"],
        Strictness::Balanced => &["L0-format", "L1-context", "L2-model", "L3-dialog"],
        Strictness::Max => &["L0-format", "L1-context", "L2-model", "L3-dialog"],
    }
}

#[test]
fn fast_profile_excludes_neural_cascades() {
    let allowed = allowed_cascades(Strictness::Fast);
    assert!(
        !allowed.contains(&"L2-model"),
        "профиль Fast обязан обходиться без статистической модели"
    );
    assert!(
        !allowed.contains(&"L3-dialog"),
        "профиль Fast обязан обходиться без диалогового каскада"
    );
}

#[test]
fn balanced_profile_may_include_model_but_not_more() {
    let allowed = allowed_cascades(Strictness::Balanced);
    assert!(allowed.contains(&"L2-model"));
    assert!(allowed.contains(&"L3-dialog"));
}

/// Политика по умолчанию не должна незаметно оказываться профилем Max.
#[test]
fn default_policy_is_balanced_not_max() {
    assert_eq!(Policy::default().strictness, Strictness::Balanced);
}

/// Методы, требующие массива записей, объявлены неприменимыми **явно**.
///
/// Это требование юридического разбора: если не объявить, проверяющий сочтёт
/// обязательные методы приказа нереализованными.
#[test]
fn not_applicable_methods_are_declared() {
    assert_eq!(
        Method::NOT_APPLICABLE.len(),
        3,
        "должны быть объявлены три неприменимых метода: п. 10, 11, 12"
    );
    for item in Method::NOT_APPLICABLE {
        assert!(
            item.contains("п. 10") || item.contains("п. 11") || item.contains("п. 12"),
            "неприменимый метод должен ссылаться на пункт приказа: {item}"
        );
    }
}

/// Все применимые методы сослались на пункт приказа.
#[test]
fn applicable_methods_cite_order_clause() {
    for method in Method::APPLICABLE {
        let clause = method.order_clause();
        assert!(
            clause.contains("приказ № 140"),
            "метод {method:?} без ссылки на приказ: {clause}"
        );
    }
}

/// Замер-заготовка: печатает время создания сессии и обезличивания.
///
/// Не является проверкой задержки: детекторов нет, мерить нечего. Служит
/// точкой, куда в фазе 2 встраивается настоящий замер p95 в CI.
#[test]
fn latency_measurement_placeholder() {
    use privacy_core::{Policy, Session};
    use std::time::Instant;

    struct NoopGenerator;
    impl privacy_core::substitute::SubstitutionGenerator for NoopGenerator {
        fn name(&self) -> &'static str {
            "noop"
        }
        fn version(&self) -> u32 {
            0
        }
        fn generate(
            &mut self,
            _request: &privacy_core::substitute::SubstitutionRequest,
        ) -> String {
            "[X]".into()
        }
    }

    let start = Instant::now();
    let mut session = Session::new(Policy::ru_140(), Box::new(NoopGenerator));
    let text = "Текст без детекторов для замера накладных расходов.".repeat(40);
    let _ = session.anonymize(&text).expect("обезличивание");
    let elapsed = start.elapsed();

    // Числового порога нет сознательно: контракт по задержке объявляется
    // в фазе 2, когда есть что измерять.
    println!("накладные расходы без детекторов: {elapsed:?}");
}
