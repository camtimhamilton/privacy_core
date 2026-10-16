//! Типы сущностей и каскады детекции.

use core::fmt;

/// Идентификатор типа сущности, например `ru.inn_legal` или `intl.email`.
///
/// Реестр типов с правилами уверенности и контрольными суммами —
/// в `docs/20-entity-registry.md`. Здесь только представление.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityTypeId(String);

impl EntityTypeId {
    /// Создаёт идентификатор.
    ///
    /// Формат: `юрисдикция.имя`, где имя — латиница в нижнем регистре.
    /// Проверяется на этапе загрузки реестра, а не в горячем пути.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Строковая форма.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Юрисдикция: часть до первой точки. Для `ru.inn_legal` это `ru`.
    pub fn jurisdiction(&self) -> &str {
        self.0.split('.').next().unwrap_or(&self.0)
    }
}

impl fmt::Display for EntityTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Тип сущности вместе с правилом нормализации ключа.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityType {
    /// Идентификатор, например `ru.inn_legal`.
    pub id: EntityTypeId,
    /// Как нормализуется значение для ключа сущности (инвариант I5).
    pub normalization: crate::text::KeyNormalization,
}

/// Каскад, обнаруживший сущность.
///
/// Каскады **не отменяют** друг друга, а только повышают уверенность.
/// Понижение уверенности сущности молча не происходит: оно попадает в отчёт.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cascade {
    /// L0: форматные правила и контрольные суммы. Без внешних зависимостей.
    Format,
    /// L1: контекстные метки и соседство реквизитов. Без внешних зависимостей.
    Context,
    /// L2: статистический распознаватель. Опционально.
    Model,
    /// L3: диалоговый каскад, ссылки и анафора. Опционально.
    Dialog,
}

impl Cascade {
    /// Понятное имя для отчёта.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Cascade::Format => "L0-format",
            Cascade::Context => "L1-context",
            Cascade::Model => "L2-model",
            Cascade::Dialog => "L3-dialog",
        }
    }
}
