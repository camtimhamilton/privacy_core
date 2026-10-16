//! Политики: методы подмены, строгость, поведение при сбое.
//!
//! Пять методов взяты из закрытого перечня приложения № 2 приказа
//! Роскомнадзора от 19.06.2025 № 140. Разбор — в `phase-0/05-legal/README.md`.
//!
//! Три метода приказа (перемешивание, декомпозиция, преобразование) требуют
//! массива записей, а не одного текста или диалога. Они объявлены
//! неприменимыми **явно** — см. [`Method::NOT_APPLICABLE`] — потому что иначе
//! проверяющий сочтёт обязательные методы нереализованными.

use crate::error::FailureBehavior;

/// Метод обезличивания.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Method {
    /// п. 2–3: замена на идентификатор («код, номер или иное обозначение»).
    ///
    /// Требует ключа, который хранится **отдельно** и не передаётся третьим
    /// лицам. Это совпадает с инвариантом I9 (ключ не рядом с картой).
    Identifier,
    /// п. 4–9, изменение через удаление атрибутов.
    Delete,
    /// п. 4–9, искажение: добавление препятствующей информации и (или)
    /// случайных значений, включая сдвиг числа.
    Distort,
    /// п. 4–9, изменение через обобщение: округление, среднее, порог, укрупнение.
    Generalize,
    /// п. 4–9, изменение через условные знаковые обозначения, в том числе `*`.
    Mask,
}

impl Method {
    /// Все методы, применимые к одному тексту или диалогу.
    pub const APPLICABLE: [Method; 5] = [
        Method::Identifier,
        Method::Delete,
        Method::Distort,
        Method::Generalize,
        Method::Mask,
    ];

    /// Методы приказа, объявленные неприменимыми к одиночному диалогу.
    ///
    /// Названия сохранены, чтобы в отчёте можно было явно сослаться на пункт.
    pub const NOT_APPLICABLE: [&'static str; 3] = [
        "п. 10 перемешивание — требует массива записей",
        "п. 11 декомпозиция — требует массива записей",
        "п. 12 преобразование — требует массива записей",
    ];

    /// Пункт приказа, которому соответствует метод.
    pub const fn order_clause(&self) -> &'static str {
        match self {
            Method::Identifier => "приказ № 140, прил. № 2, п. 2–3",
            Method::Delete => "приказ № 140, прил. № 2, п. 6–7",
            Method::Distort => "приказ № 140, прил. № 2, п. 8",
            Method::Generalize => "приказ № 140, прил. № 2, п. 9 пп. 1",
            Method::Mask => "приказ № 140, прил. № 2, п. 9 пп. 2",
        }
    }
}

/// Юридический режим работы.
///
/// Деление введено по итогам фазы 0: замена на похожие фейки — основной
/// продуктовый режим, но норма п. 9 пп. 3 требует «случайные значения...
/// не относящихся к субъекту», а фейк детерминирован от seed и сохраняет
/// распределение. Поэтому юридический режим по умолчанию — идентификаторы.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalMode {
    /// Режим нормативного соответствия: метод [`Method::Identifier`].
    CompliantRu140,
    /// Режим максимальной полезности: похожие фейки. Требует обоснования
    /// оператором в локальном акте.
    Utility,
}

/// Режим строгости: сколько каскадов включено.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strictness {
    /// Только форматные правила и контрольные суммы. Без нейросетей.
    Fast,
    /// Форматные правила плюс контекст плюс лёгкий распознаватель.
    Balanced,
    /// Всё, включая крупную модель и второй проход проверки.
    Max,
}

/// Политика обезличивания.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// Юридический режим.
    pub legal_mode: LegalMode,
    /// Режим строгости.
    pub strictness: Strictness,
    /// Основной метод подмены.
    pub method: Method,
    /// Что делать при недоступности каскада.
    pub on_failure: FailureBehavior,
    /// Блокировать ли отправку при обнаружении специальных категорий.
    ///
    /// По ч. 1 ст. 10 152-ФЗ обезличивание таких данных не допускается,
    /// поэтому значение по умолчанию — `true`.
    pub block_special_categories: bool,
}

impl Policy {
    /// Политика нормативного соответствия для российского режима.
    pub fn ru_140() -> Self {
        Self {
            legal_mode: LegalMode::CompliantRu140,
            strictness: Strictness::Balanced,
            method: Method::Identifier,
            on_failure: FailureBehavior::Block,
            block_special_categories: true,
        }
    }

    /// Политика максимальной полезности: похожие фейки.
    pub fn utility() -> Self {
        Self {
            legal_mode: LegalMode::Utility,
            strictness: Strictness::Balanced,
            method: Method::Identifier,
            on_failure: FailureBehavior::Block,
            block_special_categories: true,
        }
    }

    /// Сужение: объединение политик не должно ослаблять приватность (I7).
    ///
    /// Правило: более строгий режим строгости побеждает, блокировка специальных
    /// категорий включается, если включена хотя бы в одной политике, а
    /// поведение при сбое берётся более безопасное.
    pub fn narrow(self, other: Self) -> Self {
        let strictness = match (self.strictness, other.strictness) {
            (Strictness::Max, _) | (_, Strictness::Max) => Strictness::Max,
            (Strictness::Balanced, _) | (_, Strictness::Balanced) => Strictness::Balanced,
            _ => Strictness::Fast,
        };
        // Юридический режим: нормативное соответствие строже полезности.
        let legal_mode = if self.legal_mode == LegalMode::CompliantRu140
            || other.legal_mode == LegalMode::CompliantRu140
        {
            LegalMode::CompliantRu140
        } else {
            LegalMode::Utility
        };
        // Поведение при сбое: блокировка безопаснее деградации,
        // деградация безопаснее пропуска.
        let on_failure = match (self.on_failure, other.on_failure) {
            (FailureBehavior::Block, _) | (_, FailureBehavior::Block) => FailureBehavior::Block,
            (FailureBehavior::DegradeAndWarn, _) | (_, FailureBehavior::DegradeAndWarn) => {
                FailureBehavior::DegradeAndWarn
            }
            _ => FailureBehavior::PassThrough,
        };
        Self {
            legal_mode,
            strictness,
            method: std::cmp::max(self.method, other.method),
            on_failure,
            block_special_categories: self.block_special_categories
                || other.block_special_categories,
        }
    }
}

impl Default for Policy {
    fn default() -> Self {
        Self::ru_140()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_policy_is_fail_closed() {
        let policy = Policy::default();
        assert_eq!(policy.on_failure, FailureBehavior::Block);
        assert!(policy.on_failure.preserves_fail_closed());
        assert!(policy.block_special_categories);
    }

    #[test]
    fn narrow_prefers_block_on_failure() {
        let mut lax = Policy::ru_140();
        lax.on_failure = FailureBehavior::PassThrough;
        let strict = Policy::ru_140();
        let merged = lax.narrow(strict);
        assert_eq!(merged.on_failure, FailureBehavior::Block);
    }

    #[test]
    fn narrow_is_commutative_on_failure_and_flags() {
        let mut a = Policy::utility();
        a.on_failure = FailureBehavior::DegradeAndWarn;
        a.block_special_categories = false;
        let b = Policy::ru_140();

        let ab = a.clone().narrow(b.clone());
        let ba = b.narrow(a);
        assert_eq!(ab.on_failure, ba.on_failure);
        assert_eq!(ab.block_special_categories, ba.block_special_categories);
        assert_eq!(ab.legal_mode, ba.legal_mode);
    }

    #[test]
    fn narrow_is_idempotent() {
        let policy = Policy::ru_140();
        assert_eq!(policy.clone().narrow(policy.clone()), policy);
    }
}
