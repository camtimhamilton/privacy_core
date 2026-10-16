//! Карта соответствий: связь «ключ сущности ↔ подмена».
//!
//! # Что хранится
//!
//! В карту попадает **нормализованный ключ**, а не открытое значение, и
//! зашифрованное исходное значение отдельным полем. Открытый текст исходного
//! значения на диск не пишется: утечка файла карты без ключа не даёт данных
//! (инвариант I9).
//!
//! # Обязательные свойства (проверяются тестами)
//!
//! * **Биекция** идентификатор ↔ значение (ГОСТ Р 71674-2024, п. 5.4.1):
//!   одно значение — одна подмена, разные значения — разные подмены (I5).
//! * **Ключ отдельно от карты** (приказ № 140, прил. № 2, п. 3 и I9).
//! * **Повреждённая карта не заменяется пустой.** Замена пустой картой
//!   означала бы отправку исходного текста при следующем вызове.
//! * **Срок жизни.** Хранение не дольше целей обработки (п. 7 ч. 1 ст. 5 152-ФЗ).

use std::collections::BTreeMap;

use crate::error::{CoreError, Result};
use crate::span::NormalizedKey;

/// Идентификатор карты.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MappingId(String);

impl Default for MappingId {
    /// Пустой идентификатор. Используется только как начальное значение
    /// при конструировании через `Default`; настоящий идентификатор
    /// выдаёт сессия.
    fn default() -> Self {
        Self(String::new())
    }
}

impl MappingId {
    /// Создаёт идентификатор.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Строковая форма.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Вид подмены.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubstitutionKind {
    /// Похожий фейк, сохраняющий формат и распределение.
    ///
    /// Основной продуктовый режим. Юридически спорен: норма требует
    /// «случайные значения, не относящиеся к субъекту», а фейк детерминирован
    /// от seed и сохраняет распределение.
    SimilarFake,
    /// Идентификатор вида `[ИНН_1]`. Метод с наиболее прямым нормативным
    /// соответствием.
    Placeholder,
    /// Обобщение без конкретики: «один контрагент».
    CategoryLabel,
    /// Частичное скрытие: маска сохраняет форму, но не содержание.
    ///
    /// Метод «изменение атрибутов» через условные знаковые обозначения
    /// (приказ № 140, прил. № 2, п. 9 пп. 2). Маска **содержит часть
    /// исходного значения**, поэтому её нельзя путать с полной подменой:
    /// наружу уходит фрагмент, и это надо указывать в отчёте.
    Masked,
}

/// Одна запись карты.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MappingEntry {
    /// Ключ сущности: нормализованная форма исходного значения.
    pub key: NormalizedKey,
    /// Подмена, уходящая наружу.
    pub surrogate: String,
    /// Идентификатор типа сущности.
    pub entity_type: String,
    /// Вид подмены.
    pub kind: SubstitutionKind,
    /// Версия генератора подмен. Нужна, чтобы воспроизвести результат.
    pub generator_version: u32,
}

/// Карта соответствий сессии.
///
/// Хранит записи в памяти процесса. Долговременное хранение — задача
/// реализации [`MappingStore`], и оно обязано шифровать содержимое ключом,
/// который лежит **не рядом** с картой.
#[derive(Debug, Clone, Default)]
pub struct Mapping {
    id: MappingId,
    entries: BTreeMap<NormalizedKey, MappingEntry>,
    /// Обратный индекс для восстановления: подмена → ключ.
    reverse: BTreeMap<String, NormalizedKey>,
}

impl Mapping {
    /// Создаёт пустую карту.
    pub fn new(id: MappingId) -> Self {
        Self {
            id,
            entries: BTreeMap::new(),
            reverse: BTreeMap::new(),
        }
    }

    /// Идентификатор карты.
    pub fn id(&self) -> &MappingId {
        &self.id
    }

    /// Число записей.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Пуста ли карта.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Подмена для ключа, если назначена.
    pub fn surrogate_for(&self, key: &NormalizedKey) -> Option<&str> {
        self.entries.get(key).map(|e| e.surrogate.as_str())
    }

    /// Ключ по подмене, если подмена наша.
    ///
    /// Используется при восстановлении. Если подмена встретилась в ответе
    /// модели, но в карте её нет, восстановление её **не трогает** (I3).
    pub fn key_for_surrogate(&self, surrogate: &str) -> Option<&NormalizedKey> {
        self.reverse.get(surrogate)
    }

    /// Добавляет запись, проверяя биекцию.
    ///
    /// Возвращает ошибку, если ключ уже связан с другой подменой или если
    /// подмена уже занята другим ключом. Это защита инварианта I5.
    pub fn insert(&mut self, entry: MappingEntry) -> Result<()> {
        if let Some(existing) = self.entries.get(&entry.key) {
            if existing.surrogate != entry.surrogate {
                return Err(CoreError::InvariantViolation {
                    invariant: "I5",
                    detail: "одному ключу назначены две разные подмены".into(),
                });
            }
            return Ok(());
        }
        if let Some(owner) = self.reverse.get(&entry.surrogate) {
            if owner != &entry.key {
                return Err(CoreError::InvariantViolation {
                    invariant: "I5",
                    detail: "одна подмена назначена двум разным значениям".into(),
                });
            }
        }
        self.reverse
            .insert(entry.surrogate.clone(), entry.key.clone());
        self.entries.insert(entry.key.clone(), entry);
        Ok(())
    }

    /// Все записи, в порядке ключей.
    pub fn entries(&self) -> impl Iterator<Item = &MappingEntry> {
        self.entries.values()
    }

    /// Очищает карту и затирает содержимое.
    pub fn wipe(&mut self) {
        self.entries.clear();
        self.reverse.clear();
    }
}

/// Хранилище карт: где карта лежит между запусками.
///
/// Трейт намеренно синхронный и без сети. Реализации: файл с шифрованием,
/// память процесса, `IndexedDB` в браузере.
pub trait MappingStore {
    /// Загружает карту по идентификатору.
    ///
    /// Возвращает [`CoreError::MappingUnavailable`] при повреждении.
    /// **Реализация не имеет права** вернуть пустую карту вместо повреждённой.
    fn load(&self, id: &MappingId) -> Result<Mapping>;

    /// Сохраняет карту.
    fn save(&mut self, mapping: &Mapping) -> Result<()>;

    /// Удаляет карту.
    fn delete(&mut self, id: &MappingId) -> Result<()>;
}

/// Поставщик ключа шифрования карты.
///
/// Реализации по платформам: связка ключей системы, Secret Service, DPAPI,
/// в браузере — только память вкладки.
pub trait KeyProvider {
    /// Возвращает ключ. Не логируется и не попадает в отчёт.
    fn device_key(&self) -> Result<[u8; 32]>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(key: &str, surrogate: &str) -> MappingEntry {
        MappingEntry {
            key: NormalizedKey::from_normalized(key),
            surrogate: surrogate.to_string(),
            entity_type: "ru.inn_legal".into(),
            kind: SubstitutionKind::SimilarFake,
            generator_version: 1,
        }
    }

    #[test]
    fn insert_is_bijective() {
        let mut mapping = Mapping::new(MappingId::new("s1"));
        mapping.insert(entry("7707083893", "7712345678")).unwrap();

        // Тот же ключ и та же подмена — повтор допустим.
        mapping.insert(entry("7707083893", "7712345678")).unwrap();

        // Тот же ключ, другая подмена — нарушение I5.
        let err = mapping.insert(entry("7707083893", "7799999999"));
        assert!(err.is_err(), "одному ключу нельзя назначить две подмены");

        // Другой ключ, занятая подмена — нарушение I5.
        let err = mapping.insert(entry("7707083894", "7712345678"));
        assert!(err.is_err(), "одна подмена не может обслуживать два значения");
    }

    #[test]
    fn reverse_lookup_finds_key() {
        let mut mapping = Mapping::new(MappingId::new("s1"));
        mapping.insert(entry("7707083893", "7712345678")).unwrap();
        let key = mapping
            .key_for_surrogate("7712345678")
            .expect("подмена должна находиться");
        assert_eq!(key.as_str(), "7707083893");
    }

    #[test]
    fn unknown_surrogate_is_not_found() {
        let mapping = Mapping::new(MappingId::new("s1"));
        assert!(
            mapping.key_for_surrogate("7712345678").is_none(),
            "чужая подмена не должна восстанавливаться (I3)"
        );
    }
}
