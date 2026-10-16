//! Текст и нормализация.
//!
//! Два разных вида нормализации, которые нельзя путать:
//!
//! 1. **Нормализация текста** перед детекцией — приведение юникода, свёртка
//!    похожих символов, разбор чисел словами. Влияет на то, что найдено.
//! 2. **Нормализация ключа сущности** — по ней два написания одного значения
//!    признаются одним и тем же (инвариант I5). Влияет на карту соответствий
//!    и на совместимость сохранённых карт.

use unicode_normalization::UnicodeNormalization;
use zeroize::Zeroize;

/// Тип нормализации ключа. От него зависит, считаются ли два написания одним.
///
/// Это публичный контракт: смена поведения ломает совместимость карт,
/// сохранённых предыдущими версиями, поэтому требует минорной версии.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyNormalization {
    /// Для числовых реквизитов: удаляются все разделители.
    ///
    /// «7707 083 893», «7707-083-893» и «7707083893» дают один ключ.
    /// Применяется к ИНН, СНИЛС, ОГРН, счетам, телефонам, картам.
    DigitsOnly,
    /// Для имён и названий: юникод приводится к NFC, регистр сворачивается,
    /// пробелы сжимаются, буква ё приводится к е.
    ///
    /// Последнее спорно (различие ё/е смысловое), но для сопоставления
    /// «Алёна» и «Алена» как одного человека это правильный выбор по умолчанию.
    FoldCase,
    /// Произвольный текст: только юникод в NFC и обрезка пробелов по краям.
    Opaque,
}

/// Нормализует текст, сохраняя длину в байтах.
///
/// **Важно:** результат обязан иметь ту же длину в байтах, что и вход. Иначе
/// границы спанов, найденных по нормализованному тексту, нельзя перенести
/// обратно в исходный, а это основа всей детекции.
///
/// Возвращает `None`, если нормализация изменила длину. Это не ошибка
/// вызывающей стороны, а сигнал: такой фрагмент нельзя нормализовать
/// посимвольно, и его надо обрабатывать отдельно.
pub fn normalize_in_place(text: &str) -> Option<String> {
    let normalized: String = text.nfc().collect();
    if normalized.len() == text.len() {
        Some(normalized)
    } else {
        None
    }
}

/// Строит ключ сущности из исходного значения.
///
/// Возвращает нормализованную форму, по которой определяется совпадение
/// значений внутри сессии.
pub fn normalize_key(value: &str, mode: KeyNormalization) -> String {
    match mode {
        KeyNormalization::DigitsOnly => value.chars().filter(char::is_ascii_digit).collect(),
        KeyNormalization::FoldCase => {
            let lowered: String = value.nfc().collect::<String>().to_lowercase();
            let folded = lowered.replace('ё', "е");
            folded.split_whitespace().collect::<Vec<_>>().join(" ")
        }
        KeyNormalization::Opaque => value.nfc().collect::<String>().trim().to_string(),
    }
}

/// Текст, подлежащий обработке.
///
/// Стирает содержимое при уничтожении. Это **не** гарантия: копии могли
/// остаться в аллокаторе, в буферах ввода-вывода и в свопе. Мера снижает окно
/// утечки, но не защищает от чтения памяти процесса, и обещать большее нельзя.
#[derive(Clone)]
pub struct Text {
    inner: String,
}

impl Text {
    /// Оборачивает строку.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            inner: value.into(),
        }
    }

    /// Доступ к содержимому.
    pub fn as_str(&self) -> &str {
        &self.inner
    }

    /// Длина в байтах.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Пуст ли текст.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Стирает содержимое немедленно, не дожидаясь уничтожения значения.
    pub fn wipe(&mut self) {
        // zeroize затирает буфер и компилятор не вправе выбросить запись.
        self.inner.zeroize();
        self.inner.shrink_to_fit();
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        self.inner.zeroize();
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_only_strips_separators() {
        assert_eq!(
            normalize_key("7707 083 893", KeyNormalization::DigitsOnly),
            "7707083893"
        );
        assert_eq!(
            normalize_key("7707-083-893", KeyNormalization::DigitsOnly),
            "7707083893"
        );
    }

    #[test]
    fn fold_case_unifies_yo_and_e() {
        assert_eq!(
            normalize_key("Алёна", KeyNormalization::FoldCase),
            normalize_key("Алена", KeyNormalization::FoldCase)
        );
    }

    #[test]
    fn fold_case_squeezes_whitespace() {
        assert_eq!(
            normalize_key("  Иван   Петров ", KeyNormalization::FoldCase),
            "иван петров"
        );
    }

    #[test]
    fn normalize_in_place_keeps_byte_length() {
        let input = "Иван Петров";
        let out = normalize_in_place(input).expect("NFC не должен менять длину здесь");
        assert_eq!(out.len(), input.len());
    }

    #[test]
    fn wipe_clears_content() {
        let mut text = Text::new("Мой ИНН 7707083893");
        assert!(!text.is_empty());
        text.wipe();
        assert!(text.is_empty(), "после wipe текст должен быть пуст");
        assert_eq!(text.len(), 0);
    }
}
