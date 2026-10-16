//! Уверенность обнаружения.
//!
//! Две вещи, которые нельзя смешивать:
//!
//! * **Уровень** — откуда пришло знание (контрольная сумма, контекст, модель).
//! * **Оценка** — числовая уверенность модели, если она есть.
//!
//! Разделение существует потому, что контрольная сумма не даёт вероятности:
//! она даёт факт «прошло» или «не прошло». Приписывать этому числу 0.9 значит
//! выдумывать точность, которой нет.

use crate::entity::Cascade;

/// Уровень уверенности согласно реестру типов.
///
/// Порядок вариантов задаёт порядок возрастания: сравнение через `Ord`
/// показывает, какой уровень сильнее.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    /// Только контрольная сумма, без подтверждающего контекста.
    ///
    /// Для типов с вероятностью ложного прохождения около 10% (ИНН юрлица,
    /// ОГРН, карты) **подмена запрещена без контекста** — см. реестр типов.
    ChecksumOnly,
    /// Контрольная сумма плюс контекстная метка или соседство реквизитов.
    ChecksumPlusContext,
    /// Статистическая модель.
    Model,
    /// Диалоговый каскад: значение выведено из ссылки или истории.
    Contextual,
}

impl Confidence {
    /// Каким каскадом обычно достигается этот уровень.
    pub const fn typical_cascade(&self) -> Cascade {
        match self {
            Confidence::ChecksumOnly => Cascade::Format,
            Confidence::ChecksumPlusContext => Cascade::Context,
            Confidence::Model => Cascade::Model,
            Confidence::Contextual => Cascade::Dialog,
        }
    }

    /// Имя для отчёта.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Confidence::ChecksumOnly => "checksum_only",
            Confidence::ChecksumPlusContext => "checksum_plus_context",
            Confidence::Model => "model",
            Confidence::Contextual => "contextual",
        }
    }
}
