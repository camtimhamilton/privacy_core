//! Общие средства для тестов инвариантов.
//!
//! # Никаких настоящих персональных данных
//!
//! Репозиторий публичный. Все значения в тестах — **вымышленные**, и это
//! проверяется отдельно: тест `no_real_world_identifiers` в
//! `tests/invariant_i10_no_leak_in_report.rs` следит за тем, чтобы в исходниках
//! не появилось ничего похожего на настоящие реквизиты.
//!
//! Дополнительно: вымышленные значения подобраны так, чтобы они **не проходили
//! контрольные суммы** там, где это возможно. Иначе тест может случайно
//! совпасть с реальным реестровым номером — а ИНН и ОГРН публичны, и такое
//! совпадение было бы настоящей проблемой, а не теоретической.

#![allow(dead_code)]

/// Вычисляет байтовые границы подстроки в тексте.
///
/// В тестах **нельзя** задавать спаны «по буквам»: кириллица занимает
/// по два байта на символ, а спан задаётся в байтах. Любая ручная позиция
/// здесь — источник ошибок, поэтому все тесты обязаны вычислять границы
/// этим помощником.
pub fn span_of(text: &str, needle: &str) -> (usize, usize) {
    let start = text
        .find(needle)
        .unwrap_or_else(|| panic!("подстрока {needle:?} не найдена в тексте"));
    (start, start + needle.len())
}

/// Тестовый детектор: находит заранее заданные спаны.
pub struct FixedDetector {
    spans: Vec<(usize, usize, &'static str)>,
}

impl FixedDetector {
    /// Создаёт детектор, возвращающий указанные спаны.
    pub fn new(spans: Vec<(usize, usize, &'static str)>) -> Self {
        Self { spans }
    }
}

impl privacy_core::detect::Detector for FixedDetector {
    fn name(&self) -> &'static str {
        "fixed-test-detector"
    }

    fn cascade(&self) -> privacy_core::Cascade {
        privacy_core::Cascade::Format
    }

    fn detect(&self, _text: &str) -> privacy_core::Result<Vec<privacy_core::detect::Detected>> {
        Ok(self
            .spans
            .iter()
            .map(|(start, end, ty)| privacy_core::detect::Detected {
                span: privacy_core::Span::new(*start, *end),
                entity_type: privacy_core::EntityTypeId::new(*ty),
                confidence: privacy_core::Confidence::ChecksumOnly,
                cascade: privacy_core::Cascade::Format,
                rule_id: Some("test-rule".into()),
            })
            .collect())
    }
}

/// Детектор, который всегда падает. Нужен для проверки fail-closed (I4).
pub struct FailingDetector;

impl privacy_core::detect::Detector for FailingDetector {
    fn name(&self) -> &'static str {
        "failing-test-detector"
    }

    fn cascade(&self) -> privacy_core::Cascade {
        privacy_core::Cascade::Model
    }

    fn detect(&self, _text: &str) -> privacy_core::Result<Vec<privacy_core::detect::Detected>> {
        Err(privacy_core::CoreError::DetectorUnavailable {
            cascade: "L2-model",
            reason: "модель не загружена (тест)".into(),
        })
    }
}

/// Детектор, возвращающий спан за пределами текста. Проверяет I4 и защиту
/// от дефекта детектора.
pub struct OutOfBoundsDetector;

impl privacy_core::detect::Detector for OutOfBoundsDetector {
    fn name(&self) -> &'static str {
        "out-of-bounds-detector"
    }

    fn cascade(&self) -> privacy_core::Cascade {
        privacy_core::Cascade::Format
    }

    fn detect(&self, _text: &str) -> privacy_core::Result<Vec<privacy_core::detect::Detected>> {
        Ok(vec![privacy_core::detect::Detected {
            span: privacy_core::Span::new(0, 10_000),
            entity_type: privacy_core::EntityTypeId::new("ru.inn_legal"),
            confidence: privacy_core::Confidence::ChecksumOnly,
            cascade: privacy_core::Cascade::Format,
            rule_id: None,
        }])
    }
}

/// Детерминированный генератор подмен для тестов.
///
/// Выдаёт предсказуемые значения вида `[ТИП-N]`, чтобы тесты не зависели
/// от случайности.
pub struct TestGenerator {
    counter: usize,
}

impl TestGenerator {
    /// Создаёт генератор.
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl privacy_core::substitute::SubstitutionGenerator for TestGenerator {
    fn name(&self) -> &'static str {
        "test-generator"
    }

    fn version(&self) -> u32 {
        1
    }

    fn generate(
        &mut self,
        request: &privacy_core::substitute::SubstitutionRequest,
    ) -> String {
        self.counter += 1;
        format!("[{}-{}]", request.entity_type.as_str(), self.counter)
    }
}

/// Вымышленное значение, похожее на ИНН, но **не проходящее контрольную сумму**.
///
/// Это сделано намеренно: тест не должен случайно использовать настоящий
/// реестровый номер.
pub const FAKE_INN: &str = "0000000000";
