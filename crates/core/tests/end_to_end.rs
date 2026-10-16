//! Сквозная проверка: обезличивание от начала до конца.
//!
//! Здесь проверяется связка, а не отдельные части: форматный каскад ищет
//! реквизиты, ядро назначает подмены, наружный текст собирается, ответ
//! восстанавливается.
//!
//! # Что это доказывает
//!
//! До этой проверки все части работали по отдельности, и можно было
//! утверждать что угодно. Здесь видно, что ядро действительно **обезличивает
//! текст**, а не только умеет хранить типы.

use privacy_core::{
    Confidence, Detector, FormatCascade, Policy, Session,
};
use privacy_core::substitute::{SubstitutionGenerator, SubstitutionRequest};

/// Генератор подмен, который выдаёт валидные по формату значения.
///
/// Настоящий генератор похожих фейков появится в фазе 2. Этот нужен, чтобы
/// проверить связку: подмена должна быть не похожа на исходное значение,
/// иначе инвариант I1 не выполнить.
struct MarkerGenerator;

impl SubstitutionGenerator for MarkerGenerator {
    fn name(&self) -> &'static str {
        "marker"
    }

    fn version(&self) -> u32 {
        1
    }

    fn generate(&mut self, request: &SubstitutionRequest) -> String {
        // Не используем исходное значение: только тип и длину ключа.
        format!("[{}:{}]", request.entity_type.as_str(), request.key.len())
    }
}

/// Строит ИНН физлица из десяти произвольных цифр.
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

/// Строит СНИЛС из девяти произвольных цифр.
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

#[test]
fn end_to_end_anonymize_and_restore() {
    let inn = make_inn_person("5001007322");
    let snils = make_snils("112233445");
    let input = format!("Мои данные: ИНН {inn} и СНИЛС {snils}. Проверь их.");

    let mut session = Session::new(Policy::ru_140(), Box::new(MarkerGenerator))
        .with_detector(Box::new(FormatCascade::new()));

    let out = session.anonymize(&input).expect("обезличивание");

    // Инвариант I1: исходных значений в наружном тексте нет.
    assert!(!out.text.contains(&inn), "ИНН утёк: {}", out.text);
    assert!(!out.text.contains(&snils), "СНИЛС утёк: {}", out.text);

    // В отчёте есть типы и методы, но нет значений (I10).
    let report = format!("{:?}", out.report);
    assert!(report.contains("ru.inn_person"), "в отчёте должен быть тип");
    assert!(!report.contains(&inn), "отчёт не должен содержать значение");

    // Оба реквизита должны быть заменены.
    assert_eq!(session.mapping().len(), 2, "две записи в карте");

    // Восстановление ответа модели.
    let reply = format!("Проверил: {}. Второй номер {}.", "[ru.inn_person:12]", "[ru.snils:11]");
    let restored = session.restore(&reply).expect("восстановление");
    assert_eq!(restored.restored, 2, "обе подмены должны восстановиться");
    assert!(restored.text.contains(&inn), "ИНН должен вернуться");
    assert!(restored.text.contains(&snils), "СНИЛС должен вернуться");
}

#[test]
fn same_value_twice_gets_one_substitution() {
    let inn = make_inn_person("5001007322");
    let input = format!("ИНН {inn} указан. Повторю: {inn}.");

    let mut session = Session::new(Policy::ru_140(), Box::new(MarkerGenerator))
        .with_detector(Box::new(FormatCascade::new()));
    let out = session.anonymize(&input).expect("обезличивание");

    assert_eq!(
        session.mapping().len(),
        1,
        "одно значение — одна подмена (инвариант I5)"
    );
    assert!(!out.text.contains(&inn));
}

#[test]
fn text_without_identifiers_passes_through_unchanged() {
    let input = "Объясни разницу между TCP и UDP.";
    let mut session = Session::new(Policy::ru_140(), Box::new(MarkerGenerator))
        .with_detector(Box::new(FormatCascade::new()));
    let out = session.anonymize(input).expect("обезличивание");
    assert_eq!(out.text, input, "текст без данных не должен меняться");
    assert!(session.mapping().is_empty());
}

#[test]
fn confidence_is_reported_for_each_finding() {
    // ИНН юрлица без метки: уверенность понижена. Это видно через детектор
    // напрямую, потому что Session пока не публикует уверенность в отчёте.
    let detector = FormatCascade::new();
    let value = {
        const W: [u32; 9] = [2, 4, 10, 3, 5, 9, 4, 6, 8];
        let digits: Vec<u32> = "770708389".chars().map(|c| c.to_digit(10).unwrap()).collect();
        let sum: u32 = W.iter().zip(digits.iter()).map(|(w, d)| w * d).sum();
        format!("770708389{}", sum % 11 % 10)
    };
    let found = detector.detect(&format!("число {value} тут")).expect("детекция");
    let item = found
        .iter()
        .find(|d| d.entity_type.as_str() == "ru.inn_legal")
        .expect("находка");
    assert_eq!(item.confidence, Confidence::ChecksumOnly);
}

#[test]
fn session_records_order_of_methods() {
    let inn = make_inn_person("5001007322");
    let mut session = Session::new(Policy::ru_140(), Box::new(MarkerGenerator))
        .with_detector(Box::new(FormatCascade::new()));
    let out = session
        .anonymize(&format!("ИНН {inn}"))
        .expect("обезличивание");
    assert_eq!(out.report.methods.len(), 1, "метод записан по спану");
    assert!(!out.report.not_applicable.is_empty(), "неприменимые методы объявлены");
}
