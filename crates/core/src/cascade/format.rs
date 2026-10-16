//! Форматный каскад: поиск реквизитов по структуре и контрольной сумме.
//!
//! # Что делает каскад
//!
//! Находит группы цифр нужной длины, проверяет контрольную сумму и, если
//! правило это требует, ищет подтверждающую метку рядом. Никаких моделей,
//! никаких зависимостей: только арифметика и словари меток.
//!
//! # Ключевое решение: два уровня уверенности
//!
//! Там, где вероятность ложного прохождения случайных цифр около 10%
//! (ИНН юрлица, ОГРН, ОГРНИП), правило помечено как требующее контекста.
//! Такая находка **не повышается** до уверенности «сумма плюс контекст»,
//! если метки рядом нет. Это не осторожность, а следствие замера из фазы 0.
//!
//! # Осмысленность отдельно от суммы
//!
//! Строка из нулей проходит контрольную сумму. Поэтому, кроме суммы,
//! проверяется осмысленность: например, ИНН не может начинаться с нуля,
//! а ОГРН физического лица и юридического различаются диапазоном.
//! Без этой проверки тестовая строка `0000000000` находилась бы в тексте,
//! а вместе с ней — любой мусор из нулей.

use crate::cascade::checksum;
use crate::confidence::Confidence;
use crate::detect::{Detected, Detector};
use crate::entity::{Cascade, EntityTypeId};
use crate::error::Result;
use crate::span::Span;

/// Правило форматного каскада.
#[derive(Debug, Clone, Copy)]
pub struct FormatRule {
    /// Идентификатор типа сущности, например `ru.inn_legal`.
    pub entity_type: &'static str,
    /// Сколько цифр в реквизите.
    pub digits: usize,
    /// Функция проверки контрольной суммы.
    pub validator: fn(&[u8]) -> bool,
    /// Требуется ли подтверждающая метка рядом.
    ///
    /// `true` для типов с вероятностью ложного прохождения около 10%.
    pub requires_context: bool,
    /// Проверка осмысленности: отсекает структурно невозможные значения.
    pub plausible: fn(&[u8]) -> bool,
}

/// Все правила каскада.
///
/// Порядок важен: более длинные реквизиты проверяются первыми, потому что
/// ИНН физлица (12 цифр) и ИНН юрлица (10 цифр) могут пересекаться в
/// длинных числовых последовательностях.
pub const RULES: &[FormatRule] = &[
    FormatRule {
        entity_type: "ru.ogrnip",
        digits: 15,
        validator: checksum::is_valid_ogrnip,
        requires_context: true,
        plausible: plausible_ogrnip,
    },
    FormatRule {
        entity_type: "ru.ogrn",
        digits: 13,
        validator: checksum::is_valid_ogrn,
        requires_context: true,
        plausible: plausible_ogrn,
    },
    FormatRule {
        entity_type: "ru.inn_person",
        digits: 12,
        validator: checksum::is_valid_inn_person,
        requires_context: false,
        plausible: plausible_nonzero_start,
    },
    FormatRule {
        entity_type: "ru.snils",
        digits: 11,
        validator: checksum::is_valid_snils,
        requires_context: false,
        plausible: plausible_snils,
    },
    FormatRule {
        entity_type: "ru.inn_legal",
        digits: 10,
        validator: checksum::is_valid_inn_legal,
        requires_context: true,
        plausible: plausible_nonzero_start,
    },
];

/// Значение не может начинаться с нуля.
///
/// Отсекает строки вроде `0000000000`, которые проходят контрольную сумму,
/// но не могут быть настоящим реквизитом.
fn plausible_nonzero_start(digits: &[u8]) -> bool {
    digits.first().is_some_and(|d| *d != 0)
}

/// ОГРН и ОГРНИП: первая цифра — признак записи, обычно 1, 2, 3 или 5.
///
/// Ноль в начале невозможен.
fn plausible_ogrn(digits: &[u8]) -> bool {
    matches!(digits.first(), Some(1 | 2 | 3 | 5))
}

/// ОГРНИП: первая цифра — признак записи индивидуального предпринимателя.
///
/// По документу это 3 или 4, и структура отличается от ОГРН юридического
/// лица, поэтому проверка отдельная.
fn plausible_ogrnip(digits: &[u8]) -> bool {
    matches!(digits.first(), Some(3 | 4))
}

/// СНИЛС: номер не может быть меньше `001-001-998`.
///
/// Дополнительно отсекаются номера из одних нулей.
fn plausible_snils(digits: &[u8]) -> bool {
    let nine: u32 = digits
        .iter()
        .take(9)
        .fold(0u32, |acc, d| acc * 10 + u32::from(*d));
    nine >= 1_001_998
}

/// Форматный каскад.
#[derive(Debug, Default, Clone, Copy)]
pub struct FormatCascade {
    /// Искать ли подтверждающие метки для правил, которые их требуют.
    ///
    /// Выключение полезно для замеров: так видно, сколько находок даёт
    /// одна арифметика.
    pub use_context: bool,
}

impl FormatCascade {
    /// Каскад с поиском меток.
    pub fn new() -> Self {
        Self { use_context: true }
    }

    /// Каскад без поиска меток: только структура и контрольная сумма.
    pub fn without_context() -> Self {
        Self { use_context: false }
    }
}

impl Detector for FormatCascade {
    fn name(&self) -> &'static str {
        "format-cascade"
    }

    fn cascade(&self) -> Cascade {
        Cascade::Format
    }

    fn detect(&self, text: &str) -> Result<Vec<Detected>> {
        let chars: Vec<char> = text.chars().collect();
        // Байтовое смещение каждого символа: нужно, чтобы вернуть спан
        // в байтах исходного текста.
        let mut offsets: Vec<usize> = Vec::with_capacity(chars.len() + 1);
        let mut acc = 0usize;
        for c in &chars {
            offsets.push(acc);
            acc += c.len_utf8();
        }
        offsets.push(acc);

        let mut found: Vec<Detected> = Vec::new();

        for rule in RULES {
            let mut i = 0usize;
            while i < chars.len() {
                if !chars[i].is_ascii_digit() {
                    i += 1;
                    continue;
                }
                if !boundary_ok(&chars, i as isize - 1, i) {
                    i += 1;
                    continue;
                }

                // Собираем группу цифр с разделителями.
                let start = i;
                let mut digits: Vec<u8> = Vec::with_capacity(rule.digits);
                let mut j = i;
                let mut last_digit_end = i;
                while j < chars.len() && digits.len() < rule.digits {
                    let c = chars[j];
                    if c.is_ascii_digit() {
                        digits.push(c as u8 - b'0');
                        last_digit_end = j + 1;
                        j += 1;
                    } else if digits.len() < rule.digits
                        && is_separator(c)
                        && j + 1 < chars.len()
                        && chars[j + 1].is_ascii_digit()
                    {
                        j += 1;
                    } else {
                        break;
                    }
                }

                // Слишком мало цифр: не продолжаем, иначе зациклимся.
                if digits.len() < rule.digits {
                    i = last_digit_end.max(i + 1);
                    continue;
                }
                // Слишком много: это часть более длинного числа.
                if j < chars.len() && chars[j].is_ascii_digit() {
                    i = j + 1;
                    continue;
                }
                if !boundary_ok(&chars, j as isize, j) {
                    i = j + 1;
                    continue;
                }

                let end = last_digit_end;
                let byte_start = offsets[start];
                let byte_end = offsets[end];

                let valid = (rule.validator)(&digits) && (rule.plausible)(&digits);
                if valid {
                    let context = if rule.requires_context && self.use_context {
                        context_near(text, byte_start, byte_end, rule.entity_type)
                    } else {
                        false
                    };
                    let confidence = if rule.requires_context && !context {
                        // Сумма прошла, контекст не подтвердился: понижаем
                        // уровень, потому что вероятность ложного
                        // прохождения здесь около 10%.
                        Confidence::ChecksumOnly
                    } else if rule.requires_context {
                        Confidence::ChecksumPlusContext
                    } else {
                        // Для ИНН физлица и СНИЛС вероятность ложного
                        // прохождения около 1%, суммы достаточно.
                        Confidence::ChecksumPlusContext
                    };
                    found.push(Detected {
                        span: Span::new(byte_start, byte_end),
                        entity_type: EntityTypeId::new(rule.entity_type),
                        confidence,
                        cascade: Cascade::Format,
                        rule_id: Some(format!("{}.checksum", rule.entity_type)),
                    });
                }
                i = end;
            }
        }

        found.sort_by_key(|d| (d.span.start, d.span.end));
        Ok(found)
    }
}

/// Символ-разделитель внутри реквизита.
fn is_separator(c: char) -> bool {
    matches!(c, ' ' | '\u{00A0}' | '\u{202F}' | '-' | '\u{2013}')
}

/// Проверяет, что позиция — граница: слева и справа не буквы и не знаки,
/// которые сделали бы находку частью большего слова или числа.
///
/// `pos` может быть отрицательным или за пределами: это концы текста.
fn boundary_ok(chars: &[char], pos: isize, _after: usize) -> bool {
    if pos < 0 {
        return true;
    }
    let idx = pos as usize;
    if idx >= chars.len() {
        return true;
    }
    let c = chars[idx];
    !c.is_alphanumeric() && c != '_'
}

/// Ищет подтверждающую метку рядом с реквизитом.
///
/// Окно — 40 символов до и 20 после. Значения подобраны по следам чужого
/// опыта: в `pii-guard` измерены бюджеты расстояния до ключевого слова
/// (для БИК 40 символов), и брать меньше означало бы терять находки.
fn context_near(text: &str, start: usize, end: usize, entity_type: &str) -> bool {
    let labels: &[&str] = match entity_type {
        "ru.inn_legal" => &["инн", "плательщик", "контрагент", "налогоплат"],
        "ru.ogrn" => &["огрн", "основной государственный"],
        "ru.ogrnip" => &["огрнип", "индивидуальн", "предпринимател"],
        _ => &[],
    };
    if labels.is_empty() {
        return false;
    }

    let win_start = floor_char_boundary(text, start.saturating_sub(40));
    let win_end = ceil_char_boundary(text, (end + 20).min(text.len()));
    let window = text[win_start..win_end].to_lowercase();
    labels.iter().any(|label| window.contains(label))
}

/// Округляет позицию вниз до границы символа UTF-8.
fn floor_char_boundary(text: &str, mut pos: usize) -> usize {
    while pos > 0 && !text.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

/// Округляет позицию вверх до границы символа UTF-8.
fn ceil_char_boundary(text: &str, mut pos: usize) -> usize {
    let len = text.len();
    while pos < len && !text.is_char_boundary(pos) {
        pos += 1;
    }
    pos
}
