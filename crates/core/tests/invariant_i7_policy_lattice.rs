//! Инвариант I7: композиция политик сужается, а не расширяется.
//!
//! # Что проверяется
//!
//! Объединение двух политик даёт множество подмен **не меньшее**, чем каждая
//! по отдельности. Нельзя случайно ослабить приватность, добавив политику.
//!
//! # Почему это отдельный инвариант
//!
//! Ослабление через композицию — типичная ошибка конфигурации: «взяли политику
//! отдела, объединили с общей, и блокировка специальных категорий отключилась».
//! Такое нарушение не видно в тестах отдельных политик, только в их сочетаниях.
//!
//! # Почему без внешних зависимостей
//!
//! Property-based тестирование обычно требует `proptest` или `quickcheck`.
//! Ядро намеренно обходится без них: перебор по небольшому пространству
//! политик даёт полное покрытие и не тянет зависимостей в дерево сборки.

use privacy_core::error::FailureBehavior;
use privacy_core::policy::{LegalMode, Method, Policy, Strictness};

/// Все политики, которые можно собрать из доступных значений.
fn all_policies() -> Vec<Policy> {
    let mut out = Vec::new();
    for legal_mode in [LegalMode::CompliantRu140, LegalMode::Utility] {
        for strictness in [Strictness::Fast, Strictness::Balanced, Strictness::Max] {
            for method in Method::APPLICABLE {
                for on_failure in [
                    FailureBehavior::Block,
                    FailureBehavior::DegradeAndWarn,
                    FailureBehavior::PassThrough,
                ] {
                    for block_special_categories in [true, false] {
                        out.push(Policy {
                            legal_mode,
                            strictness,
                            method,
                            on_failure,
                            block_special_categories,
                        });
                    }
                }
            }
        }
    }
    out
}

/// «Строгость» политики как числовая мера: чем больше, тем безопаснее.
fn safety_rank(policy: &Policy) -> (u8, u8, u8, u8, u8) {
    let strictness = match policy.strictness {
        Strictness::Fast => 0,
        Strictness::Balanced => 1,
        Strictness::Max => 2,
    };
    let legal = match policy.legal_mode {
        LegalMode::Utility => 0,
        LegalMode::CompliantRu140 => 1,
    };
    let failure = match policy.on_failure {
        FailureBehavior::PassThrough => 0,
        FailureBehavior::DegradeAndWarn => 1,
        FailureBehavior::Block => 2,
    };
    let method = policy.method as u8;
    let block = u8::from(policy.block_special_categories);
    (strictness, legal, failure, method, block)
}

/// Сужение не может быть слабее ни одного из входов.
#[test]
fn narrow_is_never_weaker_than_either_input() {
    for a in all_policies() {
        for b in all_policies() {
            let merged = a.clone().narrow(b.clone());
            let (ma, _, fa, _, ba) = safety_rank(&merged);
            let (sa, _, ff_a, _, bf_a) = safety_rank(&a);
            let (sb, _, ff_b, _, bf_b) = safety_rank(&b);

            assert!(
                ma >= sa.max(sb),
                "строгость после сужения ({ma}) ниже, чем у входа ({} или {sa})",
                sb
            );
            assert!(
                fa >= ff_a.max(ff_b),
                "поведение при сбое ослабло после сужения"
            );
            assert!(
                ba >= bf_a.max(bf_b),
                "блокировка специальных категорий отключилась после сужения"
            );
        }
    }
}

/// Сужение коммутативно: порядок аргументов не меняет результат.
#[test]
fn narrow_is_commutative() {
    for a in all_policies() {
        for b in all_policies() {
            let ab = a.clone().narrow(b.clone());
            let ba = b.narrow(a.clone());
            assert_eq!(
                ab.on_failure, ba.on_failure,
                "поведение при сбое зависит от порядка аргументов"
            );
            assert_eq!(
                ab.block_special_categories, ba.block_special_categories,
                "блокировка зависит от порядка аргументов"
            );
            assert_eq!(ab.legal_mode, ba.legal_mode, "режим зависит от порядка");
            assert_eq!(ab.strictness, ba.strictness, "строгость зависит от порядка");
        }
    }
}

/// Сужение идемпотентно: политика с собой даёт себя.
#[test]
fn narrow_is_idempotent() {
    for a in all_policies() {
        assert_eq!(
            a.clone().narrow(a.clone()),
            a,
            "сужение политики с собой изменило её"
        );
    }
}

/// Сужение ассоциативно: порядок группировки не важен.
#[test]
fn narrow_is_associative() {
    let policies = all_policies();
    // Полный перебор по всем тройкам был бы 216^3 = 10 млн; берём выборку
    // с шагом, который покрывает все значения каждого поля.
    let sample: Vec<Policy> = policies.iter().step_by(7).cloned().collect();
    for a in &sample {
        for b in &sample {
            for c in &sample {
                let left = a.clone().narrow(b.clone()).narrow(c.clone());
                let right = a.clone().narrow(b.clone().narrow(c.clone()));
                assert_eq!(
                    left.on_failure, right.on_failure,
                    "сужение не ассоциативно по поведению при сбое"
                );
                assert_eq!(
                    left.block_special_categories, right.block_special_categories,
                    "сужение не ассоциативно по блокировке"
                );
                assert_eq!(left.strictness, right.strictness);
                assert_eq!(left.legal_mode, right.legal_mode);
            }
        }
    }
}

/// Никакая комбинация политик не даёт режим «пропустить при сбое».
#[test]
fn no_combination_produces_pass_through_from_block() {
    for a in all_policies() {
        for b in all_policies() {
            if a.on_failure == FailureBehavior::Block
                || b.on_failure == FailureBehavior::Block
            {
                let merged = a.clone().narrow(b.clone());
                assert_eq!(
                    merged.on_failure,
                    FailureBehavior::Block,
                    "блокировка потерялась при сужении"
                );
            }
        }
    }
}
