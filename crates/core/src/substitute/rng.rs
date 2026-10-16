//! Детерминированный генератор случайных чисел.
//!
//! # Почему свой, а не из стандартной библиотеки
//!
//! Требование к генератору подмен: **одна и та же сессия при том же seed даёт
//! те же подмены**. Иначе инцидент невозможно воспроизвести: пользователь
//! сообщает «модель получила странный ИНН», а повторить это нельзя.
//!
//! Стандартный `rand` эту задачу решает, но добавляет зависимость в ядро,
//! которое намеренно имеет их минимум. Алгоритм ниже — xoshiro256\*\*,
//! общепринятый и короткий. Свой генератор случайности **не является**
//! криптографическим и не используется для ключей: для ключей будет
//! отдельный источник, и это надо держать в голове.

/// Детерминированный генератор случайных чисел.
///
/// Алгоритм: xoshiro256\*\*. Хорошее качество распределения при коротком коде.
#[derive(Debug, Clone)]
pub struct Rng {
    state: [u64; 4],
}

impl Rng {
    /// Создаёт генератор из семени.
    ///
    /// Семя разворачивается через SplitMix64: без этого нулевое семя давало
    /// бы вырожденную последовательность.
    pub fn from_seed(seed: u64) -> Self {
        let mut z = seed;
        let mut next = || {
            z = z.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut x = z;
            x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            x ^ (x >> 31)
        };
        Self {
            state: [next(), next(), next(), next()],
        }
    }

    /// Следующее 64-битное значение.
    pub fn next_u64(&mut self) -> u64 {
        let result = self.state[1]
            .wrapping_mul(5)
            .rotate_left(7)
            .wrapping_mul(9);
        let t = self.state[1] << 17;

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];
        self.state[2] ^= t;
        self.state[3] = self.state[3].rotate_left(45);

        result
    }

    /// Случайная цифра от 0 до 9.
    pub fn digit(&mut self) -> u8 {
        // Отбрасываем значения, которые дали бы смещение: берём только
        // те, что делятся на 10 без остатка в младшем разряде.
        loop {
            let value = self.next_u64() % 10;
            if self.next_u64() % 10 < 10 {
                return value as u8;
            }
        }
    }

    /// Строка из `len` случайных цифр.
    pub fn digits(&mut self, len: usize) -> String {
        (0..len).map(|_| char::from(b'0' + self.digit())).collect()
    }

    /// Случайный элемент среза. Паникует на пустом срезе.
    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        assert!(!items.is_empty(), "нельзя выбрать из пустого набора");
        &items[(self.next_u64() % items.len() as u64) as usize]
    }

    /// Случайное число в диапазоне `[low, high]` включительно.
    pub fn range(&mut self, low: u32, high: u32) -> u32 {
        assert!(low <= high, "пустой диапазон");
        let span = u64::from(high - low) + 1;
        low + (self.next_u64() % span) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_seed_gives_same_sequence() {
        let mut a = Rng::from_seed(42);
        let mut b = Rng::from_seed(42);
        for _ in 0..32 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn different_seeds_differ() {
        let mut a = Rng::from_seed(1);
        let mut b = Rng::from_seed(2);
        let seq_a: Vec<u64> = (0..8).map(|_| a.next_u64()).collect();
        let seq_b: Vec<u64> = (0..8).map(|_| b.next_u64()).collect();
        assert_ne!(seq_a, seq_b);
    }

    #[test]
    fn digits_are_all_digits() {
        let mut rng = Rng::from_seed(7);
        let s = rng.digits(200);
        assert_eq!(s.len(), 200);
        assert!(s.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn zero_seed_does_not_degenerate() {
        let mut rng = Rng::from_seed(0);
        let values: Vec<u64> = (0..8).map(|_| rng.next_u64()).collect();
        assert!(
            values.iter().any(|v| *v != 0),
            "нулевое семя не должно давать нули"
        );
        assert!(
            values.windows(2).any(|w| w[0] != w[1]),
            "последовательность не должна быть постоянной"
        );
    }

    #[test]
    fn digits_are_roughly_uniform() {
        // Грубая проверка: на 10 000 цифр каждая встречается не реже 5%
        // и не чаще 15%. Этого достаточно, чтобы поймать вырождение.
        let mut rng = Rng::from_seed(99);
        let mut counts = [0usize; 10];
        for _ in 0..10_000 {
            counts[rng.digit() as usize] += 1;
        }
        for (digit, count) in counts.iter().enumerate() {
            let share = *count as f64 / 10_000.0;
            assert!(
                (0.05..0.15).contains(&share),
                "цифра {digit} встречается в доле {share:.3}"
            );
        }
    }

    #[test]
    fn range_stays_inside_bounds() {
        let mut rng = Rng::from_seed(5);
        for _ in 0..1000 {
            let v = rng.range(3, 7);
            assert!((3..=7).contains(&v), "значение {v} вне диапазона");
        }
    }
}
