//! Контрольные суммы российских реквизитов.
//!
//! # Откуда алгоритмы
//!
//! Проверены в фазе 0 на живых реквизитах и сверены с воспроизводимым
//! скриптом `phase-0/06-method/verify_checksums.py`. Там же замерена
//! вероятность ложного прохождения случайных цифр.
//!
//! | Тип | Длина | Вероятность прохождения случайных цифр |
//! |---|---|---|
//! | ИНН юрлица | 10 | ~9.9% (1 из 10) |
//! | ИНН физлица | 12 | ~1.0% (1 из 100) |
//! | СНИЛС | 11 | ~1.0% (1 из 100) |
//! | ОГРН | 13 | ~9.9% (1 из 10) |
//! | ОГРНИП | 15 | ~10.0% (1 из 10) |
//!
//! # Что из этого следует для детектора
//!
//! Там, где вероятность ложного прохождения около 10%, **одной контрольной
//! суммы мало**: для подмены требуется подтверждающий контекст. Это не
//! осторожность, а прямое следствие замера. Исключение — паспорт РФ: у него
//! контрольной суммы нет вообще, и по формату проходят 81% случайных строк,
//! поэтому он в этом каскаде не обрабатывается.

/// Проверка ИНН длиной 10 цифр (юридическое лицо).
///
/// Веса `[2,4,10,3,5,9,4,6,8]` на первые девять цифр, десятая — контрольная.
pub fn is_valid_inn_legal(digits: &[u8]) -> bool {
    const WEIGHTS: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    if digits.len() != 10 || !all_digits(digits) {
        return false;
    }
    let sum: u32 = WEIGHTS
        .iter()
        .zip(digits.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    (sum % 11 % 10) as u8 == digits[9]
}

/// Проверка ИНН длиной 12 цифр (физическое лицо).
///
/// Две контрольные цифры: одиннадцатая и двенадцатая.
pub fn is_valid_inn_person(digits: &[u8]) -> bool {
    const W1: [u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    const W2: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    if digits.len() != 12 || !all_digits(digits) {
        return false;
    }
    let s1: u32 = W1
        .iter()
        .zip(digits.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    if (s1 % 11 % 10) as u8 != digits[10] {
        return false;
    }
    let s2: u32 = W2
        .iter()
        .zip(digits.iter())
        .map(|(w, d)| w * u32::from(*d))
        .sum();
    (s2 % 11 % 10) as u8 == digits[11]
}

/// Проверка СНИЛС длиной 11 цифр.
///
/// Веса 9..1 на первые девять цифр. Значение суммы 100 или 101 даёт
/// контрольное число 00; при сумме больше 101 берётся остаток от деления
/// на 101, и результат 100 также приводится к 00.
pub fn is_valid_snils(digits: &[u8]) -> bool {
    if digits.len() != 11 || !all_digits(digits) {
        return false;
    }
    let sum: u32 = (0..9)
        .map(|i| u32::from(digits[i]) * (9 - i as u32))
        .sum();
    let control = match sum {
        0..=99 => sum,
        100 | 101 => 0,
        _ => {
            let rem = sum % 101;
            if rem == 100 {
                0
            } else {
                rem
            }
        }
    };
    let in_value = u32::from(digits[9]) * 10 + u32::from(digits[10]);
    control == in_value
}

/// Проверка ОГРН длиной 13 цифр.
///
/// Первые двенадцать цифр как число, остаток от деления на 11, младший
/// разряд результата — контрольная цифра.
pub fn is_valid_ogrn(digits: &[u8]) -> bool {
    if digits.len() != 13 || !all_digits(digits) {
        return false;
    }
    let mut value: u64 = 0;
    for d in &digits[..12] {
        value = value * 10 + u64::from(*d);
    }
    (value % 11 % 10) as u8 == digits[12]
}

/// Проверка ОГРНИП длиной 15 цифр.
///
/// Первые четырнадцать цифр как число, остаток от деления на 13.
pub fn is_valid_ogrnip(digits: &[u8]) -> bool {
    if digits.len() != 15 || !all_digits(digits) {
        return false;
    }
    let mut value: u64 = 0;
    for d in &digits[..14] {
        value = value * 10 + u64::from(*d);
    }
    (value % 13 % 10) as u8 == digits[14]
}

/// Проверяет, что все элементы — значения цифр от 0 до 9.
///
/// **Важно для того, кто будет это читать.** Здесь хранятся не байты ASCII,
/// а уже нормализованные значения цифр: `b'0' - b'0' == 0`. Поэтому
/// `u8::is_ascii_digit` тут неприменим: для значения `0` он возвращает
/// `false`, и весь каскад молча перестаёт находить что-либо.
///
/// Эта ошибка уже случалась и заняла время: тесты ловили её как «все
/// контрольные суммы не проходят», что выглядело как ошибка в арифметике,
/// хотя арифметика была верна.
fn all_digits(digits: &[u8]) -> bool {
    digits.iter().all(|d| *d <= 9)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Преобразует строку цифр в массив байт. Паникует на нецифрах:
    /// в тестах это признак ошибки в тестовых данных.
    fn d(s: &str) -> Vec<u8> {
        s.bytes().map(|b| b - b'0').collect()
    }

    /// Считает долю случайных наборов цифр, проходящих проверку.
    ///
    /// Генератор детерминированный (линейный конгруэнтный), чтобы результат
    /// не зависел от внешних крейтов и воспроизводился при каждом запуске.
    fn false_pass_rate(len: usize, validator: fn(&[u8]) -> bool, samples: usize) -> f64 {
        let mut state: u64 = 0x2545_F491_4F6C_DD1D;
        let mut hits = 0usize;
        let mut buf = vec![0u8; len];
        for _ in 0..samples {
            for slot in buf.iter_mut() {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                *slot = ((state >> 33) % 10) as u8;
            }
            if validator(&buf) {
                hits += 1;
            }
        }
        hits as f64 / samples as f64
    }

    #[test]
    fn inn_legal_accepts_known_valid_structure() {
        // Контрольная цифра вычисляется по тем же весам: берём произвольные
        // первые девять цифр и считаем десятую.
        let mut digits = d("770708389");
        const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
        let sum: u32 = W
            .iter()
            .zip(digits.iter())
            .map(|(w, x)| w * u32::from(*x))
            .sum();
        digits.push((sum % 11 % 10) as u8);
        assert!(is_valid_inn_legal(&digits), "цифры: {digits:?}");
    }

    #[test]
    fn inn_person_accepts_known_valid_structure() {
        let mut digits = d("500100732259");
        // Пересчитаем обе контрольные цифры.
        const W1: [u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
        const W2: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
        let s1: u32 = W1
            .iter()
            .zip(digits.iter())
            .map(|(w, x)| w * u32::from(*x))
            .sum();
        digits[10] = (s1 % 11 % 10) as u8;
        let s2: u32 = W2
            .iter()
            .zip(digits.iter())
            .map(|(w, x)| w * u32::from(*x))
            .sum();
        digits[11] = (s2 % 11 % 10) as u8;
        assert!(is_valid_inn_person(&digits), "цифры: {digits:?}");
    }

    #[test]
    fn snils_accepts_computed_control() {
        let mut digits = d("112233445");
        let sum: u32 = (0..9)
            .map(|i| u32::from(digits[i]) * (9 - i as u32))
            .sum();
        let control = if sum < 100 {
            sum
        } else if sum <= 101 {
            0
        } else {
            let r = sum % 101;
            if r == 100 {
                0
            } else {
                r
            }
        };
        digits.push((control / 10) as u8);
        digits.push((control % 10) as u8);
        assert!(is_valid_snils(&digits), "цифры: {digits:?}");
    }

    #[test]
    fn ogrn_accepts_computed_control() {
        let mut digits = d("102770013219");
        let mut value: u64 = 0;
        for x in &digits {
            value = value * 10 + u64::from(*x);
        }
        digits.push((value % 11 % 10) as u8);
        assert!(is_valid_ogrn(&digits));
    }

    #[test]
    fn ogrnip_accepts_computed_control() {
        // Четырнадцать цифр, а не пятнадцать: пятнадцатая — контрольная.
        // Пятнадцать цифр переполнили бы u64.
        let mut digits = d("30450000000001");
        let mut value: u64 = 0;
        for x in &digits {
            value = value * 10 + u64::from(*x);
        }
        digits.push((value % 13 % 10) as u8);
        assert!(is_valid_ogrnip(&digits), "получено: {digits:?}");
    }

    #[test]
    fn all_zeros_inn_legal_is_valid_by_checksum() {
        // Важное свойство: строка из нулей проходит контрольную сумму.
        // Поэтому в тестах репозитория она и используется как заведомо
        // вымышленное значение, а в детекторе её отсекает проверка
        // осмысленности, а не сумма.
        assert!(is_valid_inn_legal(&d("0000000000")));
    }

    #[test]
    fn wrong_length_is_rejected() {
        assert!(!is_valid_inn_legal(&d("770708389")));
        assert!(!is_valid_inn_legal(&d("77070838931")));
        assert!(!is_valid_inn_person(&d("7707083893")));
        assert!(!is_valid_snils(&d("1122334455")));
        assert!(!is_valid_ogrn(&d("10277001321")));
        assert!(!is_valid_ogrnip(&d("304500000000")));
    }

    #[test]
    fn flipped_digit_is_rejected() {
        let mut digits = d("770708389");
        const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
        let sum: u32 = W
            .iter()
            .zip(digits.iter())
            .map(|(w, x)| w * u32::from(*x))
            .sum();
        digits.push((sum % 11 % 10) as u8);
        let mut broken = digits.clone();
        broken[0] = (broken[0] + 1) % 10;
        // Может случайно остаться валидным, поэтому проверяем контрольную
        // цифру явно: она перестала соответствовать.
        if broken[0] != digits[0] {
            let sum2: u32 = W
                .iter()
                .zip(broken.iter())
                .map(|(w, x)| w * u32::from(*x))
                .sum();
            if (sum2 % 11 % 10) as u8 != broken[9] {
                assert!(!is_valid_inn_legal(&broken));
            }
        }
    }

    /// Сверка с замером из фазы 0: вероятности должны совпадать
    /// с точностью до статистической погрешности.
    #[test]
    fn false_pass_rates_match_phase_zero_measurement() {
        let samples = 20_000;
        let tolerance = 0.01; // 1 процентный пункт

        let cases: [(&str, usize, fn(&[u8]) -> bool, f64); 5] = [
            ("ИНН юрлица", 10, is_valid_inn_legal, 0.0991),
            ("ИНН физлица", 12, is_valid_inn_person, 0.0099),
            ("СНИЛС", 11, is_valid_snils, 0.0098),
            ("ОГРН", 13, is_valid_ogrn, 0.0993),
            ("ОГРНИП", 15, is_valid_ogrnip, 0.1000),
        ];

        for (name, len, validator, expected) in cases {
            let measured = false_pass_rate(len, validator, samples);
            assert!(
                (measured - expected).abs() < tolerance,
                "{name}: замер {measured:.4}, ожидалось около {expected:.4}"
            );
        }
    }
}
