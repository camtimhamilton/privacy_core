//! Генератор подмен для сессии.
//!
//! # Что он обеспечивает
//!
//! | Требование | Как выполнено |
//! |---|---|
//! | Форматная валидность | Для числовых реквизитов вызывается [`ValidRuGenerator`], значения проходят контрольные суммы |
//! | Отсутствие коллизий | Внутренние множества занятых значений и назначенных подмен |
//! | Детерминированность | Один `seed` даёт одну последовательность |
//! | Обратимость | Одно значение даёт одну подмену на всю сессию |
//!
//! # Почему генератор помнит назначения сам
//!
//! Ядро тоже помнит карту соответствий. Дублирование сделано намеренно:
//! генератор обязан гарантировать **биекцию** независимо от того, как его
//! вызывает ядро. Если он будет полагаться на внешнее состояние, то при
//! первом же прямом вызове (например, в тестах или в бенчмарке) выдаст
//! двум разным значениям одну подмену — и это нарушение инварианта I5,
//! которое заметить будет тяжело.

use super::mask::{mask_value, MaskStyle};
use super::rng::Rng;
use super::valid::{RegistryCheck, ValidRuGenerator};
use super::SubstitutionGenerator;
use crate::entity::EntityTypeId;
use crate::mapping::SubstitutionKind;
use std::collections::{BTreeMap, BTreeSet};

/// Настройки генератора.
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Вид подмены по умолчанию.
    pub kind: SubstitutionKind,
    /// Стиль маски для режима маскирования.
    pub mask_style: MaskStyle,
    /// Генерировать ли фейки, проходящие контрольные суммы.
    ///
    /// Выключение полезно для замеров: так видно, сколько даёт валидность.
    pub valid_fakes: bool,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            kind: SubstitutionKind::Placeholder,
            mask_style: MaskStyle::KeepPrefix { keep: 4 },
            valid_fakes: true,
        }
    }
}

/// Генератор подмен для одной сессии.
pub struct SessionGenerator {
    config: GeneratorConfig,
    rng: Rng,
    valid: ValidRuGenerator,
    /// Счётчики по типам: для маркеров вида `[ИНН_7]`.
    counters: BTreeMap<String, u64>,
    /// Уже назначенные подмены: значение → подмена.
    assigned: BTreeMap<String, String>,
    /// Обратный индекс: подмена → значение. Нужен для защиты биекции.
    reverse: BTreeSet<String>,
}

impl SessionGenerator {
    /// Создаёт генератор с семенем и настройками.
    pub fn new(seed: u64, config: GeneratorConfig) -> Self {
        Self {
            config,
            rng: Rng::from_seed(seed ^ 0x5EED_1234),
            valid: ValidRuGenerator::from_seed(seed),
            counters: BTreeMap::new(),
            assigned: BTreeMap::new(),
            reverse: BTreeSet::new(),
        }
    }

    /// Создаёт генератор с настройками по умолчанию.
    pub fn from_seed(seed: u64) -> Self {
        Self::new(seed, GeneratorConfig::default())
    }

    /// Сколько подмен назначено.
    pub fn assigned_count(&self) -> usize {
        self.assigned.len()
    }

    /// Возвращает уже назначенную подмену, если она есть.
    pub fn existing(&self, key: &str) -> Option<&str> {
        self.assigned.get(key).map(String::as_str)
    }

    /// Создаёт подмену заданного вида.
    ///
    /// Вынесено отдельно, чтобы ядро могло запросить конкретный вид
    /// подмены по методу политики, а не полагаться на настройку по умолчанию.
    pub fn make(
        &mut self,
        entity_type: &EntityTypeId,
        key: &str,
        kind: SubstitutionKind,
    ) -> String {
        if let Some(existing) = self.assigned.get(key) {
            return existing.clone();
        }

        // До десяти попыток: за это время коллизия практически невозможна,
        // а бесконечный цикл при ошибке в логике был бы хуже.
        for _ in 0..10 {
            let candidate = match kind {
                SubstitutionKind::Placeholder => self.placeholder(entity_type),
                SubstitutionKind::CategoryLabel => self.category_label(entity_type),
                SubstitutionKind::SimilarFake => self.similar_fake(entity_type, key),
                SubstitutionKind::Masked => self.masked(key),
            };
            if self.reverse.contains(&candidate) {
                continue;
            }
            self.assigned.insert(key.to_string(), candidate.clone());
            self.reverse.insert(candidate.clone());
            return candidate;
        }

        // Подмены исчерпались: возвращаем маркер с уникальным номером,
        // который точно не занят.
        let fallback = format!("[{}_{}]", entity_type.as_str(), self.assigned.len());
        self.assigned.insert(key.to_string(), fallback.clone());
        self.reverse.insert(fallback.clone());
        fallback
    }

    /// Маркер вида `[ru.inn_legal_1]`.
    ///
    /// Самый прямой по нормативному соответствию вид: метод введения
    /// идентификаторов (приказ № 140, приложение № 2, п. 2–3).
    fn placeholder(&mut self, entity_type: &EntityTypeId) -> String {
        let counter = self
            .counters
            .entry(entity_type.to_string())
            .or_insert(0);
        *counter += 1;
        format!("[{}_{}]", entity_type.as_str(), counter)
    }

    /// Обобщение без конкретики.
    ///
    /// Метод изменения атрибутов через обобщение (п. 9 пп. 1): значение
    /// заменяется на категорию, а не на другое значение.
    fn category_label(&mut self, entity_type: &EntityTypeId) -> String {
        let label = category_for(entity_type.as_str());
        let counter = self.counters.entry(format!("label:{label}")).or_insert(0);
        *counter += 1;
        if *counter == 1 {
            label.to_string()
        } else {
            format!("{label} №{counter}")
        }
    }

    /// Похожий фейк, проходящий контрольную сумму.
    fn similar_fake(&mut self, entity_type: &EntityTypeId, key: &str) -> String {
        match entity_type.as_str() {
            "ru.inn_legal" if self.config.valid_fakes => {
                self.valid.inn_legal(&UnknownCheck)
            }
            "ru.inn_person" if self.config.valid_fakes => {
                self.valid.inn_person(&UnknownCheck)
            }
            "ru.snils" if self.config.valid_fakes => self.valid.snils(&UnknownCheck),
            "ru.ogrn" if self.config.valid_fakes => self.valid.ogrn(&UnknownCheck),
            "ru.ogrnip" if self.config.valid_fakes => self.valid.ogrnip(&UnknownCheck),
            _ => {
                // Для типов без контрольной суммы формат сохраняем,
                // но содержимое берём случайное.
                self.random_like(key)
            }
        }
    }

    /// Частичное скрытие значения.
    ///
    /// Маска сохраняет длину и разделители, поэтому текст остаётся
    /// читаемым, а содержание скрыто. Наружу при этом уходит **часть
    /// исходного значения** — это осознанный компромисс метода, и он
    /// отличается от полной подмены.
    fn masked(&mut self, key: &str) -> String {
        mask_value(key, self.config.mask_style)
    }

    /// Случайное значение той же формы, что исходное.
    ///
    /// Цифры заменяются цифрами, буквы — буквами, разделители сохраняются.
    /// Это сохраняет читаемость и не требует знать тип реквизита.
    fn random_like(&mut self, key: &str) -> String {
        const CYR: &[char] = &[
            'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'к', 'л', 'м', 'н', 'о', 'п', 'р', 'с',
            'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'э', 'ю', 'я',
        ];
        key.chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    char::from(b'0' + self.rng.digit())
                } else if c.is_alphabetic() {
                    *self.rng.pick(CYR)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl SubstitutionGenerator for SessionGenerator {
    fn name(&self) -> &'static str {
        "session-generator"
    }

    fn version(&self) -> u32 {
        1
    }

    fn generate(&mut self, request: &super::SubstitutionRequest) -> String {
        self.make(&request.entity_type, &request.key, request.kind)
    }
}

/// Человекочитаемая категория по типу сущности.
fn category_for(entity_type: &str) -> &'static str {
    match entity_type {
        "ru.inn_legal" | "ru.inn_person" => "один идентификатор",
        "ru.snils" => "один номер",
        "ru.ogrn" | "ru.ogrnip" => "одна организация",
        "ru.account" => "один счёт",
        "ru.phone" => "один телефон",
        "ru.email" => "одна почта",
        "ru.fio" => "один человек",
        "ru.address" => "один адрес",
        _ => "одно значение",
    }
}

/// Проверка по реестру, которая ничего не знает.
///
/// Используется по умолчанию: реестра у нас нет, и делать вид, что
/// проверка выполнена, нельзя.
struct UnknownCheck;

impl RegistryCheck for UnknownCheck {
    fn contains(&self, _value: &str) -> Option<bool> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::checksum;

    fn entity(name: &str) -> EntityTypeId {
        EntityTypeId::new(name)
    }

    #[test]
    fn same_key_gets_same_substitution() {
        let mut gen = SessionGenerator::from_seed(1);
        let a = gen.make(&entity("ru.inn_legal"), "7707083893", SubstitutionKind::Placeholder);
        let b = gen.make(&entity("ru.inn_legal"), "7707083893", SubstitutionKind::Placeholder);
        assert_eq!(a, b, "одно значение — одна подмена");
        assert_eq!(gen.assigned_count(), 1);
    }

    #[test]
    fn different_keys_get_different_substitutions() {
        let mut gen = SessionGenerator::from_seed(2);
        let a = gen.make(&entity("ru.inn_legal"), "7707083893", SubstitutionKind::SimilarFake);
        let b = gen.make(&entity("ru.inn_legal"), "7707083894", SubstitutionKind::SimilarFake);
        assert_ne!(a, b, "разные значения — разные подмены");
    }

    #[test]
    fn placeholder_contains_type_and_counter() {
        let mut gen = SessionGenerator::from_seed(3);
        let value = gen.make(
            &entity("ru.inn_legal"),
            "7707083893",
            SubstitutionKind::Placeholder,
        );
        assert!(value.contains("ru.inn_legal"), "маркер должен называть тип");
        assert!(value.starts_with('[') && value.ends_with(']'));
    }

    #[test]
    fn similar_fake_for_inn_passes_checksum() {
        let mut gen = SessionGenerator::from_seed(4);
        for i in 0..100 {
            let key = format!("770708389{i}");
            let fake = gen.make(&entity("ru.inn_legal"), &key, SubstitutionKind::SimilarFake);
            let digits: Vec<u8> = fake.bytes().map(|b| b - b'0').collect();
            assert!(
                checksum::is_valid_inn_legal(&digits),
                "фейк не проходит контрольную сумму: {fake}"
            );
        }
    }

    #[test]
    fn similar_fake_for_snils_passes_checksum() {
        let mut gen = SessionGenerator::from_seed(5);
        for i in 0..100 {
            let key = format!("11223344{i}");
            let fake = gen.make(&entity("ru.snils"), &key, SubstitutionKind::SimilarFake);
            let digits: Vec<u8> = fake.bytes().map(|b| b - b'0').collect();
            assert!(
                checksum::is_valid_snils(&digits),
                "фейк СНИЛС не проходит: {fake}"
            );
        }
    }

    #[test]
    fn fake_is_not_equal_to_original() {
        let mut gen = SessionGenerator::from_seed(6);
        let original = "7707083893";
        let fake = gen.make(&entity("ru.inn_legal"), original, SubstitutionKind::SimilarFake);
        assert_ne!(fake, original, "подмена не должна совпасть с исходным");
    }

    #[test]
    fn deterministic_for_same_seed() {
        let mut a = SessionGenerator::from_seed(42);
        let mut b = SessionGenerator::from_seed(42);
        for i in 0..50 {
            let key = format!("770708389{i}");
            assert_eq!(
                a.make(&entity("ru.inn_legal"), &key, SubstitutionKind::SimilarFake),
                b.make(&entity("ru.inn_legal"), &key, SubstitutionKind::SimilarFake)
            );
        }
    }

    #[test]
    fn different_seeds_give_different_fakes() {
        let mut a = SessionGenerator::from_seed(1);
        let mut b = SessionGenerator::from_seed(2);
        let x = a.make(&entity("ru.inn_legal"), "k1", SubstitutionKind::SimilarFake);
        let y = b.make(&entity("ru.inn_legal"), "k1", SubstitutionKind::SimilarFake);
        assert_ne!(x, y);
    }

    #[test]
    fn category_label_has_no_digits_of_original() {
        let mut gen = SessionGenerator::from_seed(7);
        let label = gen.make(
            &entity("ru.inn_legal"),
            "7707083893",
            SubstitutionKind::CategoryLabel,
        );
        assert!(!label.contains("7707083893"));
        assert!(label.contains("идентификатор") || label.contains("организац"));
    }

    #[test]
    fn no_collisions_across_many_values() {
        let mut gen = SessionGenerator::from_seed(8);
        let mut seen = std::collections::BTreeSet::new();
        for i in 0..500 {
            let key = format!("key{i}");
            let fake = gen.make(&entity("ru.inn_legal"), &key, SubstitutionKind::SimilarFake);
            assert!(seen.insert(fake.clone()), "подмена повторилась: {fake}");
        }
    }

    #[test]
    fn random_like_preserves_shape() {
        let mut gen = SessionGenerator::from_seed(9);
        // Тип без контрольной суммы: форма должна сохраниться.
        let fake = gen.make(&entity("ru.fio"), "Петров", SubstitutionKind::SimilarFake);
        assert_eq!(
            fake.chars().count(),
            "Петров".chars().count(),
            "длина должна сохраниться"
        );
        assert_ne!(fake, "Петров");
    }
}
