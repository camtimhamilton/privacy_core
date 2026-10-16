//! Маскирование: частичное скрытие значения.
//!
//! Метод «изменение атрибутов» через условные знаковые обозначения
//! (приказ Роскомнадзора № 140, приложение № 2, п. 9 пп. 2).
//!
//! # Правила
//!
//! Маска сохраняет **структуру** значения, чтобы текст оставался читаемым,
//! и скрывает **содержание**. Для российских реквизитов это важно: маска
//! должна показать, что это ИНН, но не показать, какой именно.

/// Что оставить от значения.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskStyle {
    /// Оставить первые `keep` цифр, остальные заменить.
    ///
    /// Полезно, когда начало несёт смысл: например, первые цифры ИНН
    /// указывают на регион.
    KeepPrefix {
        /// Сколько первых символов оставить.
        keep: usize,
    },
    /// Оставить последние `keep` символов.
    ///
    /// Модель это не помогает, но пользователю видно, о каком именно
    /// документе речь.
    KeepSuffix {
        /// Сколько последних символов оставить.
        keep: usize,
    },
    /// Заменить всё, оставив только тип и длину.
    All,
}

/// Символ маски.
pub const MASK_CHAR: char = '*';

/// Маскирует значение.
///
/// # Примеры
///
/// ```
/// use privacy_core::substitute::mask::{mask_value, MaskStyle};
///
/// let masked = mask_value("7707083893", MaskStyle::KeepPrefix { keep: 4 });
/// assert_eq!(masked, "7707******");
///
/// let masked = mask_value("7707083893", MaskStyle::KeepSuffix { keep: 2 });
/// assert_eq!(masked, "********93");
/// ```
pub fn mask_value(value: &str, style: MaskStyle) -> String {
    let chars: Vec<char> = value.chars().collect();
    let len = chars.len();
    match style {
        MaskStyle::All => std::iter::repeat_n(MASK_CHAR, len).collect(),
        MaskStyle::KeepPrefix { keep } => {
            let keep = keep.min(len);
            chars[..keep]
                .iter()
                .copied()
                .chain(std::iter::repeat_n(MASK_CHAR, len - keep))
                .collect()
        }
        MaskStyle::KeepSuffix { keep } => {
            let keep = keep.min(len);
            std::iter::repeat_n(MASK_CHAR, len - keep)
                .chain(chars[len - keep..].iter().copied())
                .collect()
        }
    }
}

/// Маскирует значение, оставляя разделители на своих местах.
///
/// Так маска сохраняет читаемость: «770-708-389-3» превращается
/// в «770-***-***-*», а не в сплошные звёзды.
pub fn mask_preserving_separators(value: &str, style: MaskStyle) -> String {
    let chars: Vec<char> = value.chars().collect();
    let separators: Vec<bool> = chars.iter().map(|c| !c.is_alphanumeric()).collect();
    let masked = mask_value(value, style);
    masked
        .chars()
        .zip(separators.iter())
        .map(|(m, is_sep)| if *is_sep { ' ' } else { m })
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keep_prefix_masks_the_rest() {
        assert_eq!(
            mask_value("7707083893", MaskStyle::KeepPrefix { keep: 4 }),
            "7707******"
        );
    }

    #[test]
    fn keep_suffix_masks_the_beginning() {
        assert_eq!(
            mask_value("7707083893", MaskStyle::KeepSuffix { keep: 2 }),
            "********93"
        );
    }

    #[test]
    fn all_masks_everything() {
        assert_eq!(mask_value("7707083893", MaskStyle::All), "**********");
    }

    #[test]
    fn keep_longer_than_value_masks_nothing() {
        assert_eq!(
            mask_value("123", MaskStyle::KeepPrefix { keep: 10 }),
            "123"
        );
    }

    #[test]
    fn length_is_preserved() {
        let value = "7707083893";
        for style in [
            MaskStyle::All,
            MaskStyle::KeepPrefix { keep: 3 },
            MaskStyle::KeepSuffix { keep: 3 },
        ] {
            let masked = mask_value(value, style);
            assert_eq!(
                masked.chars().count(),
                value.chars().count(),
                "длина должна сохраняться: {style:?}"
            );
        }
    }

    #[test]
    fn mask_reveals_no_more_than_asked() {
        let value = "7707083893";
        let masked = mask_value(value, MaskStyle::KeepPrefix { keep: 2 });
        // В маске не должно быть цифр, кроме двух разрешённых.
        let digits_in_mask = masked.chars().filter(char::is_ascii_digit).count();
        assert_eq!(digits_in_mask, 2);
    }

    #[test]
    fn works_with_cyrillic_names() {
        let masked = mask_value("Петров", MaskStyle::KeepPrefix { keep: 1 });
        assert_eq!(masked, "П*****");
    }
}
