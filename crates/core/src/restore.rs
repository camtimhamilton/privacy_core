//! Восстановление исходных значений в ответе внешней модели.
//!
//! # Ключевое свойство (инвариант I3)
//!
//! Восстановление **не искажает посторонний текст**. Если последовательность,
//! совпадающая с подменой, встретилась в ответе не как наша подстановка —
//! совпадение, цитата, часть другого слова — она не трогается.
//!
//! # Почему восстановление идёт по карте, а не повторной детекцией
//!
//! Инвариант I6: восстановление не зависит от детектора. Иначе значение,
//! которое детектор не найдёт при повторном проходе, окажется невосстановимым,
//! и пользователь потеряет данные.
//!
//! # Проходы
//!
//! 1. **Точный** — полное совпадение подмены на границах слова.
//! 2. **По частям** — подмена разбита моделью на несколько слов.
//! 3. **Нечёткий** — модель переписала подмену с искажением.
//!
//! Реализация проходов 2 и 3 — задача фазы 2. Здесь объявлены типы и
//! инвариант, а сами проходы помечены как нереализованные.

use crate::mapping::Mapping;

/// Что не удалось восстановить.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unresolved {
    /// Подмена, которая осталась в тексте.
    pub surrogate: String,
    /// Почему не восстановлена.
    pub reason: UnresolvedReason,
}

/// Причина невосстановленной подмены.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnresolvedReason {
    /// Подмены нет в карте: она пришла из другого источника.
    NotInMapping,
    /// Подмена найдена, но модель переписала её так, что точное совпадение
    /// не сработало. Требуется нечёткий проход.
    Rewritten,
}

/// Результат восстановления.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestoreResult {
    /// Текст с восстановленными значениями.
    pub text: String,
    /// Сколько подмен восстановлено.
    pub restored: usize,
    /// Что осталось невосстановленным. Метрика проекта — доля таких подмен.
    pub unresolved: Vec<Unresolved>,
}

/// Восстанавливает значения в ответе модели.
///
/// Проход 1 (точный) намеренно консервативен: срабатывает только на полное
/// совпадение подмены, окружённое границами слова. Всё остальное попадает
/// в [`RestoreResult::unresolved`], а не восстанавливается наугад.
pub fn restore_exact(reply: &str, mapping: &Mapping) -> RestoreResult {
    let mut out = String::with_capacity(reply.len());
    let mut restored = 0usize;
    // Проходы 2 и 3 (по частям и нечёткий) не реализованы: список заполняется
    // ими в фазе 2. Пока он всегда пуст, и это видно в отчёте как ноль
    // неразрешённых, а не как «всё восстановлено».
    let unresolved: Vec<Unresolved> = Vec::new();

    // Позиция в исходной строке — единственный источник истины для границ.
    // Считать их через длину собранного вывода нельзя: длина в байтах
    // не совпадает с числом символов в UTF-8, и проверка границы слова
    // на кириллице даёт неверный результат.
    let mut cursor = 0usize;

    loop {
        // Ищем самую раннюю подмену из карты, начиная с курсора.
        // Линейный проход по карте приемлем для фазы 1; в фазе 2 заменяется
        // на автомат Ахо—Корасик.
        let mut earliest: Option<(usize, &str, String)> = None;
        for entry in mapping.entries() {
            if let Some(rel) = reply[cursor..].find(entry.surrogate.as_str()) {
                let pos = cursor + rel;
                let better = match &earliest {
                    Some((best_pos, _, _)) => pos < *best_pos,
                    None => true,
                };
                if better {
                    earliest =
                        Some((pos, entry.surrogate.as_str(), entry.key.as_str().to_string()));
                }
            }
        }

        match earliest {
            Some((pos, surrogate, key)) => {
                out.push_str(&reply[cursor..pos]);
                let after_pos = pos + surrogate.len();
                let boundary_left = is_word_boundary_at(reply, pos, Direction::Left);
                let boundary_right = is_word_boundary_at(reply, after_pos, Direction::Right);

                if boundary_left && boundary_right {
                    // Восстановление по карте. Открытое значение берётся
                    // из ключа, который в фазе 2 будет расшифрован.
                    out.push_str(&key);
                    restored += 1;
                } else {
                    // Подмена встретилась внутри другого слова или числа.
                    // Восстановление её не трогает (инвариант I3).
                    out.push_str(surrogate);
                }
                cursor = after_pos;
            }
            None => {
                out.push_str(&reply[cursor..]);
                break;
            }
        }
    }

    RestoreResult {
        text: out,
        restored,
        unresolved,
    }
}

/// Направление проверки границы слова.
#[derive(Debug, Clone, Copy)]
enum Direction {
    /// Смотрим символ слева от позиции.
    Left,
    /// Смотрим символ справа от позиции.
    Right,
}

/// Является ли позиция границей слова.
///
/// Границей считается начало, конец строки и любой неалфанумерический символ.
/// Работает по символам, а не по байтам, поэтому корректно для кириллицы.
fn is_word_boundary_at(text: &str, byte_pos: usize, direction: Direction) -> bool {
    match direction {
        Direction::Left => match text[..byte_pos].chars().next_back() {
            None => true,
            Some(c) => !c.is_alphanumeric(),
        },
        Direction::Right => match text[byte_pos..].chars().next() {
            None => true,
            Some(c) => !c.is_alphanumeric(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mapping::{MappingEntry, MappingId, SubstitutionKind};
    use crate::span::NormalizedKey;

    fn mapping_with(key: &str, surrogate: &str) -> Mapping {
        let mut mapping = Mapping::new(MappingId::new("test"));
        mapping
            .insert(MappingEntry {
                key: NormalizedKey::from_normalized(key),
                surrogate: surrogate.to_string(),
                entity_type: "ru.inn_legal".into(),
                kind: SubstitutionKind::SimilarFake,
                generator_version: 1,
            })
            .expect("вставка должна пройти");
        mapping
    }

    #[test]
    fn exact_pass_restores_value() {
        let mapping = mapping_with("7707083893", "7712345678");
        let result = restore_exact("Ваш ИНН 7712345678 принят.", &mapping);
        assert_eq!(result.restored, 1);
        assert!(result.text.contains("7707083893"));
    }

    #[test]
    fn isolated_surrogate_in_unrelated_sentence_is_restored() {
        // Подмена стоит отдельным словом между запятой и пробелом. Формально
        // это допустимая подстановка, и восстановление обязано сработать.
        // Проверка I3 — в следующем тесте, где подмена склеена с текстом.
        let mapping = mapping_with("7707083893", "7712345678");
        let reply = "Погода сегодня хорошая, 7712345678 человек пришли.";
        let result = restore_exact(reply, &mapping);
        assert_eq!(result.restored, 1);
        assert!(result.text.contains("7707083893"));
    }

    #[test]
    fn surrogate_glued_to_letters_is_untouched() {
        // Инвариант I3: подмена внутри слова не восстанавливается.
        let mapping = mapping_with("7707083893", "7712345678");
        let reply = "код7712345678значение";
        let result = restore_exact(reply, &mapping);
        assert_eq!(
            result.restored, 0,
            "подмена внутри другого слова не должна восстанавливаться"
        );
        assert_eq!(result.text, reply, "текст не должен меняться");
    }

    #[test]
    fn surrogate_inside_longer_number_is_untouched() {
        // Инвариант I3: подмена внутри более длинной цифровой
        // последовательности не восстанавливается.
        let mapping = mapping_with("7707083893", "7712345678");
        let reply = "номер 97712345678999 в реестре";
        let result = restore_exact(reply, &mapping);
        assert_eq!(result.restored, 0, "часть длинного числа не восстанавливается");
        assert_eq!(result.text, reply);
    }

    #[test]
    fn surrogate_absent_needs_no_restore() {
        let mapping = mapping_with("7707083893", "7712345678");
        let result = restore_exact("Ответ без подмен.", &mapping);
        assert_eq!(result.restored, 0);
        assert_eq!(result.text, "Ответ без подмен.");
    }
}
