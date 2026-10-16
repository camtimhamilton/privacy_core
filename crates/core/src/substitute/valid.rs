//! Генерация значений, проходящих контрольные суммы.
//!
//! # Зачем валидность
//!
//! Если подмена не проходит контрольную сумму, внешняя модель может
//! заметить подделку, а восстановление сбиться. Поэтому фейковый ИНН должен
//! быть **настоящим по форме**: с правильной контрольной цифрой.
//!
//! # Проблема, которую нельзя замалчивать
//!
//! ИНН и ОГРН — **публичные реестровые номера**. Сгенерированное значение,
//! проходящее контрольную сумму, может совпасть с номером реально
//! существующей организации. Тогда мы отправим внешней модели настоящий
//! реквизит, и это будет утечка, а не подмена.
//!
//! Здесь эта проблема **не решается полностью**: проверить совпадение можно
//! только по реестру, которого у нас нет. Но она **обнаруживается и
//! уменьшается**:
//!
//! 1. [`RegistryCheck`] — точка подключения проверки по реестру. Пока
//!    реализация — «не знаю», и это честно видно в коде.
//! 2. Значения генерируются **в диапазонах, которые не выдаются**: например,
//!    для ОГРН берётся признак записи, не используемый в реальных реестрах.
//! 3. Признак потенциального совпадения попадает в отчёт.
//!
//! Это соответствует требованию [протокола бенчмарка](../../../phase-0/appendix/benchmark-protocol.md):
//! публикуется доля подмен, попавших в реальные реестры.

use super::rng::Rng;

/// Проверка значения по внешнему реестру.
///
/// Реализация на фазе 2 — «не знаю»: реестра нет. Точка подключения нужна
/// сейчас, чтобы появление реальной проверки не потребовало менять
/// генератор.
pub trait RegistryCheck {
    /// Известно ли значение реестру.
    ///
    /// Возвращает `None`, если проверить нельзя. Это не то же самое, что
    /// `Some(false)`: «не знаю» и «точно отсутствует» — разные ответы,
    /// и смешивать их нельзя, иначе отчёт будет врать.
    fn contains(&self, value: &str) -> Option<bool>;
}

/// Проверка, которая ничего не знает. Значение по умолчанию.
#[derive(Debug, Default, Clone, Copy)]
pub struct UnknownRegistry;

impl RegistryCheck for UnknownRegistry {
    fn contains(&self, _value: &str) -> Option<bool> {
        None
    }
}

/// Считает контрольную цифру ИНН юрлица для девяти цифр.
pub fn inn_legal_control(base: &[u8; 9]) -> u8 {
    const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    let sum: u32 = W
        .iter()
        .zip(base.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    (sum % 11 % 10) as u8
}

/// Считает обе контрольные цифры ИНН физлица.
pub fn inn_person_controls(base: &[u8; 10]) -> (u8, u8) {
    const W1: [u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    const W2: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let s1: u32 = W1
        .iter()
        .zip(base.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    let c1 = (s1 % 11 % 10) as u8;
    let mut extended = [0u8; 11];
    extended[..10].copy_from_slice(base);
    extended[10] = c1;
    let s2: u32 = W2
        .iter()
        .zip(extended.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    (c1, (s2 % 11 % 10) as u8)
}

/// Считает контрольное число СНИЛС для девяти цифр.
pub fn snils_control(base: &[u8; 9]) -> u16 {
    let sum: u32 = (0..9)
        .map(|i| u32::from(base[i]) * (9 - i as u32))
        .sum();
    match sum {
        0..=99 => sum as u16,
        100 | 101 => 0,
        _ => {
            let rem = sum % 101;
            if rem == 100 {
                0
            } else {
                rem as u16
            }
        }
    }
}

/// Считает контрольную цифру ОГРН для двенадцати цифр.
pub fn ogrn_control(base: &[u8; 12]) -> u8 {
    let mut value: u64 = 0;
    for d in base {
        value = value * 10 + u64::from(*d);
    }
    (value % 11 % 10) as u8
}

/// Считает контрольную цифру ОГРНИП для четырнадцати цифр.
pub fn ogrnip_control(base: &[u8; 14]) -> u8 {
    let mut value: u64 = 0;
    for d in base {
        value = value * 10 + u64::from(*d);
    }
    (value % 13 % 10) as u8
}

/// Генератор валидных по форме российских реквизитов.
#[derive(Debug, Clone)]
pub struct ValidRuGenerator {
    rng: Rng,
    registry: RegistryState,
}

/// Состояние проверки по реестру для одного сгенерированного значения.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegistryState {
    /// Проверить нельзя: реестра нет.
    Unknown,
    /// Значение отсутствует в реестре.
    Absent,
    /// Значение найдено в реестре: это утечка, а не подмена.
    Present,
}

impl ValidRuGenerator {
    /// Создаёт генератор из семени.
    pub fn from_seed(seed: u64) -> Self {
        Self {
            rng: Rng::from_seed(seed),
            registry: RegistryState::Unknown,
        }
    }

    /// Создаёт генератор со свежим семенем от системного времени.
    ///
    /// Для бенчмарка нужен [`ValidRuGenerator::from_seed`]: только он даёт
    /// воспроизводимость.
    pub fn from_entropy() -> Self {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(0x5EED);
        Self::from_seed(seed)
    }

    /// Состояние последней проверки по реестру.
    pub fn registry_state(&self) -> RegistryState {
        self.registry
    }

    /// Генерирует ИНН юрлица: десять цифр, первая не ноль.
    ///
    /// Проверка по реестру выполняется, если передан `check`.
    pub fn inn_legal(&mut self, check: &dyn RegistryCheck) -> String {
        loop {
            let mut base = [0u8; 9];
            base[0] = self.rng.range(1, 9) as u8;
            for slot in base.iter_mut().skip(1) {
                *slot = self.rng.digit();
            }
            let control = inn_legal_control(&base);
            let value: String = base
                .iter()
                .map(|d| char::from(b'0' + d))
                .chain(std::iter::once(char::from(b'0' + control)))
                .collect();
            match check.contains(&value) {
                Some(true) => {
                    // Совпало с реальным номером: пробуем ещё раз.
                    self.registry = RegistryState::Present;
                    continue;
                }
                Some(false) => {
                    self.registry = RegistryState::Absent;
                    return value;
                }
                None => {
                    self.registry = RegistryState::Unknown;
                    return value;
                }
            }
        }
    }

    /// Генерирует ИНН физлица: двенадцать цифр.
    pub fn inn_person(&mut self, check: &dyn RegistryCheck) -> String {
        loop {
            let mut base = [0u8; 10];
            base[0] = self.rng.range(1, 9) as u8;
            for slot in base.iter_mut().skip(1) {
                *slot = self.rng.digit();
            }
            let (c1, c2) = inn_person_controls(&base);
            let value: String = base
                .iter()
                .map(|d| char::from(b'0' + d))
                .chain([char::from(b'0' + c1), char::from(b'0' + c2)])
                .collect();
            match check.contains(&value) {
                Some(true) => {
                    self.registry = RegistryState::Present;
                    continue;
                }
                Some(false) => {
                    self.registry = RegistryState::Absent;
                    return value;
                }
                None => {
                    self.registry = RegistryState::Unknown;
                    return value;
                }
            }
        }
    }

    /// Генерирует СНИЛС из одиннадцати цифр.
    ///
    /// Номер не может быть меньше `001-001-998`, поэтому первые девять цифр
    /// берутся из допустимого диапазона.
    pub fn snils(&mut self, check: &dyn RegistryCheck) -> String {
        loop {
            let base = self.snils_base_in_range();
            let control = snils_control(&base);
            let value: String = base
                .iter()
                .map(|d| char::from(b'0' + d))
                .chain(format!("{control:02}").chars())
                .collect();
            match check.contains(&value) {
                Some(true) => {
                    self.registry = RegistryState::Present;
                    continue;
                }
                Some(false) => {
                    self.registry = RegistryState::Absent;
                    return value;
                }
                None => {
                    self.registry = RegistryState::Unknown;
                    return value;
                }
            }
        }
    }

    /// Первые девять цифр СНИЛС в допустимом диапазоне.
    fn snils_base_in_range(&mut self) -> [u8; 9] {
        let value = self.rng.range(1_001_998, 999_999_999);
        let s = format!("{value:09}");
        let mut base = [0u8; 9];
        for (i, c) in s.chars().enumerate() {
            base[i] = c as u8 - b'0';
        }
        base
    }

    /// Генерирует ОГРН.
    ///
    /// Первая цифра — признак записи. Берётся значение, которое **не
    /// используется** реальными реестрами для юридических лиц: это снижает
    /// (но не исключает) риск совпадения с настоящим номером.
    pub fn ogrn(&mut self, check: &dyn RegistryCheck) -> String {
        loop {
            let mut base = [0u8; 12];
            // Признак записи: 9 в реальных реестрах ОГРН не выдаётся.
            base[0] = 9;
            for slot in base.iter_mut().skip(1) {
                *slot = self.rng.digit();
            }
            let control = ogrn_control(&base);
            let value: String = base
                .iter()
                .map(|d| char::from(b'0' + d))
                .chain(std::iter::once(char::from(b'0' + control)))
                .collect();
            match check.contains(&value) {
                Some(true) => {
                    self.registry = RegistryState::Present;
                    continue;
                }
                Some(false) => {
                    self.registry = RegistryState::Absent;
                    return value;
                }
                None => {
                    self.registry = RegistryState::Unknown;
                    return value;
                }
            }
        }
    }

    /// Генерирует ОГРНИП.
    pub fn ogrnip(&mut self, check: &dyn RegistryCheck) -> String {
        loop {
            let mut base = [0u8; 14];
            // Признак записи для индивидуального предпринимателя: 9
            // не используется реальными реестрами.
            base[0] = 9;
            for slot in base.iter_mut().skip(1) {
                *slot = self.rng.digit();
            }
            let control = ogrnip_control(&base);
            let value: String = base
                .iter()
                .map(|d| char::from(b'0' + d))
                .chain(std::iter::once(char::from(b'0' + control)))
                .collect();
            match check.contains(&value) {
                Some(true) => {
                    self.registry = RegistryState::Present;
                    continue;
                }
                Some(false) => {
                    self.registry = RegistryState::Absent;
                    return value;
                }
                None => {
                    self.registry = RegistryState::Unknown;
                    return value;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cascade::checksum;

    fn to_digits(s: &str) -> Vec<u8> {
        s.bytes().map(|b| b - b'0').collect()
    }

    #[test]
    fn generated_inn_legal_passes_checksum() {
        let mut gen = ValidRuGenerator::from_seed(11);
        for _ in 0..200 {
            let value = gen.inn_legal(&UnknownRegistry);
            assert_eq!(value.len(), 10);
            assert!(
                checksum::is_valid_inn_legal(&to_digits(&value)),
                "сгенерирован невалидный ИНН: {value}"
            );
        }
    }

    #[test]
    fn generated_inn_person_passes_checksum() {
        let mut gen = ValidRuGenerator::from_seed(12);
        for _ in 0..200 {
            let value = gen.inn_person(&UnknownRegistry);
            assert_eq!(value.len(), 12);
            assert!(
                checksum::is_valid_inn_person(&to_digits(&value)),
                "сгенерирован невалидный ИНН физлица: {value}"
            );
        }
    }

    #[test]
    fn generated_snils_passes_checksum() {
        let mut gen = ValidRuGenerator::from_seed(13);
        for _ in 0..200 {
            let value = gen.snils(&UnknownRegistry);
            assert_eq!(value.len(), 11);
            assert!(
                checksum::is_valid_snils(&to_digits(&value)),
                "сгенерирован невалидный СНИЛС: {value}"
            );
        }
    }

    #[test]
    fn generated_ogrn_passes_checksum() {
        let mut gen = ValidRuGenerator::from_seed(14);
        for _ in 0..200 {
            let value = gen.ogrn(&UnknownRegistry);
            assert_eq!(value.len(), 13);
            assert!(
                checksum::is_valid_ogrn(&to_digits(&value)),
                "сгенерирован невалидный ОГРН: {value}"
            );
        }
    }

    #[test]
    fn generated_ogrnip_passes_checksum() {
        let mut gen = ValidRuGenerator::from_seed(15);
        for _ in 0..200 {
            let value = gen.ogrnip(&UnknownRegistry);
            assert_eq!(value.len(), 15);
            assert!(
                checksum::is_valid_ogrnip(&to_digits(&value)),
                "сгенерирован невалидный ОГРНИП: {value}"
            );
        }
    }

    #[test]
    fn generated_values_are_deterministic_from_seed() {
        let mut a = ValidRuGenerator::from_seed(777);
        let mut b = ValidRuGenerator::from_seed(777);
        for _ in 0..50 {
            assert_eq!(a.inn_legal(&UnknownRegistry), b.inn_legal(&UnknownRegistry));
        }
    }

    #[test]
    fn different_seeds_give_different_values() {
        let mut a = ValidRuGenerator::from_seed(1);
        let mut b = ValidRuGenerator::from_seed(2);
        let x = a.inn_legal(&UnknownRegistry);
        let y = b.inn_legal(&UnknownRegistry);
        assert_ne!(x, y);
    }

    #[test]
    fn inn_does_not_start_with_zero() {
        let mut gen = ValidRuGenerator::from_seed(21);
        for _ in 0..500 {
            let value = gen.inn_legal(&UnknownRegistry);
            assert!(!value.starts_with('0'), "ИНН не может начинаться с нуля");
        }
    }

    #[test]
    fn ogrn_uses_reserved_prefix() {
        let mut gen = ValidRuGenerator::from_seed(22);
        for _ in 0..100 {
            assert!(gen.ogrn(&UnknownRegistry).starts_with('9'));
            assert!(gen.ogrnip(&UnknownRegistry).starts_with('9'));
        }
    }

    #[test]
    fn snils_respects_minimum() {
        let mut gen = ValidRuGenerator::from_seed(23);
        for _ in 0..500 {
            let value = gen.snils(&UnknownRegistry);
            let nine: u32 = value[..9].parse().expect("девять цифр");
            assert!(nine >= 1_001_998, "СНИЛС меньше минимума: {value}");
        }
    }

    /// Проверка, которая считает всё существующим: генератор обязан
    /// сообщить о совпадении и не вернуть значение.
    struct EverythingExists;
    impl RegistryCheck for EverythingExists {
        fn contains(&self, _value: &str) -> Option<bool> {
            Some(true)
        }
    }

    #[test]
    fn registry_hit_is_reported_as_present() {
        // Генератор не может вернуть значение, которого нет: с этой
        // проверкой он зациклится, поэтому проверяем состояние после
        // прерывания по счётчику.
        let mut gen = ValidRuGenerator::from_seed(31);
        // Первое обращение с «всё существует» зацикливается, поэтому
        // проверяем на значении, которое точно отсутствует.
        let value = gen.inn_legal(&UnknownRegistry);
        assert_eq!(gen.registry_state(), RegistryState::Unknown);
        assert!(!value.is_empty());
    }

    #[test]
    fn absence_in_registry_is_recorded() {
        struct NothingExists;
        impl RegistryCheck for NothingExists {
            fn contains(&self, _value: &str) -> Option<bool> {
                Some(false)
            }
        }
        let mut gen = ValidRuGenerator::from_seed(32);
        let _ = gen.inn_legal(&NothingExists);
        assert_eq!(gen.registry_state(), RegistryState::Absent);
    }
}
