//! Детерминированные идентификаторы устройства.
//!
//! # Зачем это существует
//!
//! Все известные решения пытаются угадать персональные данные по тексту:
//! моделью, правилами, контрольными суммами. У операционной системы есть
//! преимущество, которого нет ни у одного прокси: **она знает правду о себе**.
//! Домашний каталог, имя пользователя, имя машины, почта из конфигурации git —
//! это не догадки, это факты.
//!
//! Значит, целый класс данных можно находить **детерминированно**: без модели,
//! без вероятностей, без миллисекунд. Сравнение идёт по точному вхождению.
//!
//! # Три правила этого крейта
//!
//! 1. **Значения не покидают память.** Наружу (в отчёты, логи, тесты) уходят
//!    только вид идентификатора, хэш значения и место находки. Это требование
//!    инварианта I10 и следствие того, что репозиторий публичный.
//! 2. **Сбор только явный.** Источники перечислены поимённо, ничего не
//!    сканируется «на всякий случай». Список процессов, история команд и
//!    буфер обмена сюда не входят: это данные, которые надо защищать
//!    сильнее, чем то, что мы защищаем.
//! 3. **Короткие значения осторожно.** Имя пользователя из семи букв может
//!    случайно совпасть с обычным словом, поэтому для коротких значений
//!    требуется подтверждающий контекст. Иначе мы получим ложные срабатывания
//!    на каждом втором сообщении.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::BTreeMap;
use std::path::PathBuf;

pub mod collect;
pub mod detect;
pub mod identity;
pub mod redact;

pub use detect::{Detection, DetectionPolicy, OsTruthDetector};
pub use collect::collect_identity;
pub use identity::{IdentityKind, IdentityValue};
pub use redact::{fingerprint, redact_for_display};

/// Набор идентификаторов устройства, собранный из окружения.
///
/// Хранит значения в памяти процесса. Отдельные значения умеют затирать
/// себя, но это снижение окна утечки, а не защита от чтения памяти:
/// копии могли остаться в аллокаторе и в свопе.
#[derive(Debug, Default, Clone)]
pub struct IdentitySet {
    values: Vec<IdentityValue>,
}

impl IdentitySet {
    /// Создаёт пустой набор.
    pub fn new() -> Self {
        Self::default()
    }

    /// Добавляет значение.
    pub fn push(&mut self, value: IdentityValue) {
        self.values.push(value);
    }

    /// Все значения.
    pub fn values(&self) -> &[IdentityValue] {
        &self.values
    }

    /// Сколько значений собрано.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Пуст ли набор.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Сколько значений каждого вида. Полезно для диагностики:
    /// можно показать состав, не раскрывая содержимого.
    pub fn summary(&self) -> BTreeMap<IdentityKind, usize> {
        let mut out = BTreeMap::new();
        for value in &self.values {
            *out.entry(value.kind).or_insert(0) += 1;
        }
        out
    }

    /// Затирает значения.
    pub fn wipe(&mut self) {
        self.values.clear();
        self.values.shrink_to_fit();
    }
}

/// Ошибка сбора идентификаторов.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectError {
    /// Источник недоступен: файла нет, нет прав, переменная не задана.
    ///
    /// Это **не** ошибка: отсутствие источника означает лишь, что по нему
    /// нечего искать, и не должно ломать сбор остального.
    Unavailable {
        /// Имя источника.
        source: &'static str,
    },
}

impl std::fmt::Display for CollectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CollectError::Unavailable { source } => {
                write!(f, "источник недоступен: {source}")
            }
        }
    }
}

impl std::error::Error for CollectError {}

/// Путь к домашнему каталогу без обращения к сети и без побочных эффектов.
pub(crate) fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}
