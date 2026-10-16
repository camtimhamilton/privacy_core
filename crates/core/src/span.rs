//! Границы спанов и ключ нормализации.
//!
//! Спан задаётся **байтовыми** границами в UTF-8. Причина: срезы строк в Rust
//! работают по байтам, а границы в символах требуют пересчёта и открывают путь
//! к ошибкам на составных эмодзи и комбинирующих знаках.

use core::fmt;

/// Полуинтервал `[start, end)` в байтах исходного текста.
///
/// Инварианты, которые обязан соблюдать любой код, создающий `Span`:
/// границы не выходят за длину текста и попадают на границу символа UTF-8.
/// Проверяются при нормализации через [`Span::validate_in`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    /// Начало, включительно.
    pub start: usize,
    /// Конец, исключительно.
    pub end: usize,
}

impl Span {
    /// Создаёт спан без проверок. Для проверки используйте [`Span::validate_in`].
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Длина в байтах.
    pub const fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Пуст ли спан.
    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Пересекаются ли спаны (включая вложенность и совпадение границ).
    pub const fn overlaps(&self, other: &Span) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Проверяет, что спан корректен для данного текста.
    ///
    /// Отдельный метод, а не проверка в конструкторе: конструктор вызывается
    /// в горячем пути, где лишняя работа недопустима, а проверка нужна один
    /// раз на входе.
    pub fn validate_in(&self, text: &str) -> bool {
        self.start <= self.end
            && self.end <= text.len()
            && text.is_char_boundary(self.start)
            && text.is_char_boundary(self.end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

/// Ключ сущности: нормализованная форма исходного значения.
///
/// По нему определяется, что «ИНН 7707 083 893» и «7707083893» — одно и то же
/// (инвариант I5). Правило нормализации — часть публичного контракта:
/// его изменение ломает совместимость сохранённых карт.
///
/// Внутри хранится не открытое значение, а результат нормализации, и только он
/// попадает в карту. Открытый текст исходного значения на диск не пишется (I9).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NormalizedKey(String);

impl NormalizedKey {
    /// Создаёт ключ из уже нормализованной строки.
    ///
    /// Не принимает произвольный текст намеренно: вызывающая сторона обязана
    /// сначала прогнать значение через [`crate::text::normalize_key`], иначе
    /// инвариант I5 будет нарушен незаметно.
    pub fn from_normalized(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Доступ к нормализованной форме.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for NormalizedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Показываем не значение, а длину: ключ не должен утекать в логи (I10).
        write!(f, "key(len={})", self.0.len())
    }
}
