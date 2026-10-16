//! Ядро приватности: обезличивание текста перед отправкой внешним языковым моделям.
//!
//! # Что это
//!
//! Библиотека получает текст и политику, возвращает обезличенный текст, карту
//! соответствий и отчёт, умеет обратное превращение по карте.
//!
//! # Чего ядро не делает (проверяется в CI)
//!
//! 1. **Не ходит в сеть.** Здесь нет и не должно появиться HTTP-клиентов.
//! 2. **Не требует ML-модели.** Сборка без фич даёт рабочий артефакт, который
//!    проходит базовые проверки. Нейросеть — сменный провайдер за трейтом.
//! 3. **Не знает о провайдерах моделей.** В ядре нет понятий «модель»,
//!    «endpoint», «провайдер». Ядро не знает, куда уходит текст.
//!
//! # Инварианты
//!
//! Спецификация инвариантов I1–I10 — в `docs/10-architecture.md`. Тесты,
//! соответствующие инвариантам, живут в `tests/`, и на момент фазы 1 они
//! помечены `#[ignore]`: тест обязан падать, пока поведение не реализовано.
//!
//! # Пример (целевой вид API, пока не работает)
//!
//! ```ignore
//! use privacy_core::{Policy, Session};
//!
//! let mut session = Session::new(Policy::ru_140());
//! let out = session.anonymize("Мой ИНН 7707083893")?;
//! // out.text не содержит исходного значения (инвариант I1)
//! let restored = session.restore(&model_reply)?;
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod bridge;
pub mod cascade;
pub mod confidence;
pub mod detect;
pub mod entity;
pub mod error;
pub mod mapping;
pub mod policy;
pub mod restore;
pub mod session;
pub mod span;
pub mod substitute;
pub mod text;

pub use bridge::{ExternalDetector, ExternalSpan, SpanSource};
pub use cascade::{FormatCascade, FormatRule};
pub use detect::{Detected, Detector};
pub use confidence::Confidence;
pub use entity::{Cascade, EntityType, EntityTypeId};
pub use error::{CoreError, FailureBehavior, Result};
pub use mapping::{Mapping, MappingId};
pub use policy::{Method, Policy};
pub use session::{Anonymized, Report, Session};
pub use span::{NormalizedKey, Span};
pub use text::Text;

/// Версия ядра. Меняется по семантическому версионированию:
/// сдвиг результатов детекции на регрессионном наборе — минорная версия
/// с публикацией дельты, а не патч.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Версия протокола бенчмарка, с которым совместимо это ядро.
///
/// Бенчмарк проверяет именно её, а не версию решения: иначе сравнение
/// результатов разных версий протокола теряет смысл.
pub const PROTOCOL_VERSION: &str = "0.1";
