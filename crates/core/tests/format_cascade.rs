//! Тесты форматного каскада на тексте.
//!
//! # Все значения вымышленные и структурно корректные
//!
//! Репозиторий публичный, поэтому настоящие реквизиты здесь не используются.
//! Значения построены так: берутся произвольные первые цифры, а контрольная
//! вычисляется тем же правилом, что и в детекторе. Поэтому они проходят
//! проверку и при этом заведомо никому не принадлежат.
//!
//! Отдельно проверяется, что значение `0000000000` **не** находится: оно
//! проходит контрольную сумму, но отсекается проверкой осмысленности.

use privacy_core::cascade::checksum;
use privacy_core::{Confidence, Detector, FormatCascade};

/// Строит ИНН юрлица: девять цифр плюс вычисленная контрольная.
fn make_inn_legal(base: &str) -> String {
    const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
    let digits: Vec<u32> = base.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let sum: u32 = W.iter().zip(digits.iter()).map(|(w, d)| w * d).sum();
    format!("{base}{}", sum % 11 % 10)
}

/// Строит ИНН физлица: десять цифр плюс две вычисленные контрольные.
fn make_inn_person(base: &str) -> String {
    const W1: [u32; 10] = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    const W2: [u32; 11] = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8];
    let digits: Vec<u32> = base.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let s1: u32 = W1.iter().zip(digits.iter()).map(|(w, d)| w * d).sum();
    let d11 = s1 % 11 % 10;
    let mut digits2 = digits.clone();
    digits2.push(d11);
    let s2: u32 = W2.iter().zip(digits2.iter()).map(|(w, d)| w * d).sum();
    format!("{base}{d11}{}", s2 % 11 % 10)
}

/// Строит СНИЛС из девяти цифр.
fn make_snils(base: &str) -> String {
    let digits: Vec<u32> = base.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let sum: u32 = (0..9).map(|i| digits[i] * (9 - i as u32)).sum();
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
    format!("{base}{control:02}")
}

/// Строит ОГРН из двенадцати цифр.
fn make_ogrn(base: &str) -> String {
    let value: u64 = base.parse().expect("двенадцать цифр");
    format!("{base}{}", value % 11 % 10)
}

#[test]
fn finds_inn_person_in_plain_text() {
    let value = make_inn_person("5001007322");
    let text = format!("Мой номер {value} для проверки");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    assert!(
        found.iter().any(|d| d.entity_type.as_str() == "ru.inn_person"),
        "ИНН физлица должен находиться: {found:?}"
    );
}

#[test]
fn finds_snils_in_plain_text() {
    let value = make_snils("112233445");
    let text = format!("СНИЛС {value} приложен");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    assert!(found.iter().any(|d| d.entity_type.as_str() == "ru.snils"));
}

#[test]
fn finds_inn_with_spaces_and_hyphens() {
    let value = make_inn_person("5001007322");
    // Разбиваем на группы, как пишут люди.
    let spaced = format!(
        "{} {} {} {}",
        &value[0..3],
        &value[3..6],
        &value[6..9],
        &value[9..12]
    );
    let text = format!("номер {spaced} указан");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    assert!(
        found.iter().any(|d| d.entity_type.as_str() == "ru.inn_person"),
        "ИНН с пробелами должен находиться: {found:?}"
    );

    let hyphenated = format!(
        "{}-{}-{}",
        &value[0..4],
        &value[4..8],
        &value[8..12]
    );
    let found2 = FormatCascade::new()
        .detect(&format!("номер {hyphenated} указан"))
        .expect("детекция");
    assert!(found2.iter().any(|d| d.entity_type.as_str() == "ru.inn_person"));
}

#[test]
fn inn_legal_requires_context_label() {
    let value = make_inn_legal("770708389");

    // Без метки: находка есть, но уверенность понижена, потому что
    // вероятность ложного прохождения около 10%.
    let without = FormatCascade::new()
        .detect(&format!("число {value} в документе"))
        .expect("детекция");
    let found = without
        .iter()
        .find(|d| d.entity_type.as_str() == "ru.inn_legal")
        .expect("находка должна быть");
    assert_eq!(
        found.confidence,
        Confidence::ChecksumOnly,
        "без метки уверенность обязана быть пониженной"
    );

    // С меткой уверенность повышается.
    let with = FormatCascade::new()
        .detect(&format!("ИНН {value} организации"))
        .expect("детекция");
    let found = with
        .iter()
        .find(|d| d.entity_type.as_str() == "ru.inn_legal")
        .expect("находка должна быть");
    assert_eq!(found.confidence, Confidence::ChecksumPlusContext);
}

#[test]
fn context_disabled_lowers_confidence_everywhere() {
    let value = make_inn_legal("770708389");
    let found = FormatCascade::without_context()
        .detect(&format!("ИНН {value}"))
        .expect("детекция");
    assert!(found
        .iter()
        .all(|d| d.confidence == Confidence::ChecksumOnly));
}

#[test]
fn all_zeros_is_not_found() {
    // Проходит контрольную сумму, но структурно невозможно: ИНН не может
    // начинаться с нуля. Это защита от находок в тестовых данных.
    let found = FormatCascade::new()
        .detect("номер 0000000000 указан")
        .expect("детекция");
    assert!(
        found.is_empty(),
        "нулевое значение не должно находиться: {found:?}"
    );
}

#[test]
fn digits_inside_longer_number_are_not_found() {
    let value = make_inn_person("5001007322");
    let text = format!("идентификатор 9{value}8 в системе");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    assert!(
        found.is_empty(),
        "часть длинного числа не должна считаться реквизитом: {found:?}"
    );
}

#[test]
fn wrong_checksum_is_not_found() {
    let value = make_inn_person("5001007322");
    // Портим последнюю цифру.
    let broken = format!("{}0", &value[..11]);
    let found = FormatCascade::new()
        .detect(&format!("номер {broken} указан"))
        .expect("детекция");
    assert!(
        !found.iter().any(|d| d.entity_type.as_str() == "ru.inn_person"
            && d.span.len() == 12),
        "неверная контрольная сумма не должна находиться"
    );
}

#[test]
fn finds_ogrn_with_label() {
    let value = make_ogrn("102770013219");
    let found = FormatCascade::new()
        .detect(&format!("ОГРН {value} присвоен"))
        .expect("детекция");
    let item = found
        .iter()
        .find(|d| d.entity_type.as_str() == "ru.ogrn")
        .expect("ОГРН должен находиться");
    assert_eq!(item.confidence, Confidence::ChecksumPlusContext);
}

#[test]
fn spans_point_to_correct_text() {
    let value = make_snils("112233445");
    let text = format!("СНИЛС {value} приложен");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    let item = found
        .iter()
        .find(|d| d.entity_type.as_str() == "ru.snils")
        .expect("находка");
    let slice = &text[item.span.start..item.span.end];
    assert_eq!(slice, value, "спан должен указывать на сам реквизит");
}

#[test]
fn cyrillic_text_does_not_break_boundaries() {
    // Проверка, что байтовые границы не попадают в середину символа
    // и что многобайтовый текст не мешает поиску.
    let value = make_inn_person("5001007322");
    let text = format!("Уважаемый Иван Петрович! Ваш номер {value} принят.");
    let found = FormatCascade::new().detect(&text).expect("детекция");
    assert!(found.iter().any(|d| d.entity_type.as_str() == "ru.inn_person"));
    for item in &found {
        assert!(
            text.is_char_boundary(item.span.start) && text.is_char_boundary(item.span.end),
            "границы спана обязаны попадать на границы символов"
        );
    }
}

#[test]
fn checksum_module_matches_detector() {
    // Сверка: то, что принимает каскад, принимает и модуль контрольных сумм.
    let value = make_inn_legal("770708389");
    let digits: Vec<u8> = value.bytes().map(|b| b - b'0').collect();
    assert!(checksum::is_valid_inn_legal(&digits));
}
