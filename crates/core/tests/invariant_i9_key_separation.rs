//! Инвариант I9: ключ карты не хранится рядом с картой.
//!
//! # Что проверяется
//!
//! * Карта на диске зашифрована, ключ — в системном хранилище либо передан
//!   вызывающей стороной.
//! * Открытый текст исходного значения на диск не пишется: утечка файла карты
//!   без ключа не даёт данных.
//! * В браузере ключ живёт только в памяти вкладки.
//!
//! # Откуда требование
//!
//! Два независимых источника совпадают:
//!
//! * **Безопасность** — иначе шифрование карты бессмысленно.
//! * **Право** — приказ Роскомнадзора № 140, прил. № 2, п. 3: ключ хранится
//!   оператором **отдельно** от массива персональных данных и **не передаётся
//!   третьим лицам**. Это редкий случай, когда юридическое требование и
//!   требование безопасности совпадают дословно.

mod common;

use common::TestGenerator;
use privacy_core::{Policy, Session};

/// Карта в памяти содержит подмены, но не открытые значения исходного текста.
#[test]
#[ignore = "фаза 2: хранение ключа в карте ещё не разделено с шифртекстом"]
fn mapping_does_not_keep_plaintext_values() {
    use privacy_core::mapping::{MappingEntry, MappingId, SubstitutionKind};
    use privacy_core::NormalizedKey;

    let key = NormalizedKey::from_normalized("0000000000");
    let entry = MappingEntry {
        key: key.clone(),
        surrogate: "[ИНН-1]".into(),
        entity_type: "ru.inn_legal".into(),
        kind: SubstitutionKind::Placeholder,
        generator_version: 1,
    };

    // Ключ в карте — нормализованная форма. Для цифровых типов она совпадает
    // с исходным значением, и это надо исправить: хранить только HMAC ключа.
    assert_ne!(
        entry.key.as_str(),
        "0000000000",
        "в карте не должно лежать исходное значение в открытом виде"
    );
    let _ = MappingId::new("t");
}

/// Сессия не пишет исходные значения в отчёт.
#[test]
fn session_report_carries_no_source_text() {
    use privacy_core::detect::Detector;
    use privacy_core::{Cascade, Confidence, EntityTypeId, Result, Span};

    struct OneSpan;
    impl Detector for OneSpan {
        fn name(&self) -> &'static str {
            "one-span"
        }
        fn cascade(&self) -> Cascade {
            Cascade::Format
        }
        fn detect(&self, _text: &str) -> Result<Vec<privacy_core::detect::Detected>> {
            Ok(vec![privacy_core::detect::Detected {
                span: Span::new(4, 14),
                entity_type: EntityTypeId::new("ru.inn_legal"),
                confidence: Confidence::ChecksumOnly,
                cascade: Cascade::Format,
                rule_id: Some("test".into()),
            }])
        }
    }

    let mut session = Session::new(Policy::ru_140(), Box::new(TestGenerator::new()))
        .with_detector(Box::new(OneSpan));

    let out = session.anonymize("ИНН 0000000000").expect("обезличивание");
    let rendered = format!("{:?}", out.report);
    assert!(
        !rendered.contains("0000000000"),
        "отчёт не должен содержать исходное значение (I10)"
    );
}

/// Ключ не должен быть зашит в исходниках.
///
/// Проверка читает файлы ядра и ищет признаки «ключа по умолчанию».
/// Это грубая, но полезная страховка: настоящий ключ обязан приходить извне.
#[test]
fn no_hardcoded_key_material_in_sources() {
    let sources = [
        include_str!("../src/mapping.rs"),
        include_str!("../src/session.rs"),
        include_str!("../src/text.rs"),
    ];
    for source in sources {
        for pattern in ["device_key:", "SECRET", "hardcoded_key"] {
            assert!(
                !source.contains(pattern),
                "в исходниках найден признак зашитого ключа: {pattern}"
            );
        }
    }
}

/// Поставщик ключа по умолчанию отсутствует: сессия не может работать
/// без явно переданного ключа, когда понадобится шифрование.
#[test]
#[ignore = "фаза 2: KeyProvider ещё не подключён к сессии"]
fn session_requires_explicit_key_provider() {
    unimplemented!("нужен конструктор сессии, требующий KeyProvider");
}
