//! Детектор по идентификаторам устройства.
//!
//! # Как он работает
//!
//! Никакой модели и никаких вероятностей: поиск **точного вхождения**
//! значения из набора идентификаторов. Поэтому точность по построению
//! максимальная — ложное срабатывание возможно только тогда, когда значение
//! случайно совпало с обычным текстом.
//!
//! # Где живёт риск ложных срабатываний
//!
//! Риск целиком в **коротких значениях**. Имя пользователя из семи букв
//! («ромашка», «admin») встретится в обычном тексте, и мы подменим лишнее.
//! Поэтому для коротких значений требуется подтверждающий контекст:
//!
//! * для имени пользователя и хоста — `@`, слэш, обратный слэш, путь,
//!   либо двойное двоеточие (формат `пользователь@машина`);
//! * для имени проекта — кавычки, слэш, слово «проект», «папка», «репозиторий»;
//! * для адреса почты и полного пути контекст не нужен: они сами по себе
//!   достаточно длинные и структурные.
//!
//! # Почему это лучший детектор в проекте по скорости
//!
//! Поиск подстроки — это линейный проход по тексту без модели, без аллокаций
//! в горячем пути (в текущей версии аллокации есть, они уйдут в фазе 2).
//! Порядок величины — микросекунды на сообщение.

use crate::{IdentityKind, IdentitySet, IdentityValue};

/// Настройки детектора.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionPolicy {
    /// Минимальная длина значения, при которой контекст не требуется.
    ///
    /// Значения короче считаются «рискованными»: для них нужен контекст.
    /// Значение подобрано так, чтобы обычные слова не попадали в порог:
    /// 8 символов отсекает «admin», «user», «test», но пропускает
    /// «ivan.petrov».
    pub long_enough: usize,

    /// Искать ли значение без учёта регистра.
    ///
    /// Для почты и хоста регистр обычно не важен, но для имён проектов
    /// важен. По умолчанию — да, потому что цена пропуска выше цены лишней
    /// находки, а контекстная проверка сдерживает ложные срабатывания.
    pub case_insensitive: bool,
}

impl Default for DetectionPolicy {
    fn default() -> Self {
        Self {
            long_enough: 8,
            case_insensitive: true,
        }
    }
}

/// Одна находка.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Detection {
    /// Вид идентификатора.
    pub kind: IdentityKind,
    /// Границы находки в байтах.
    pub start: usize,
    /// Конец находки.
    pub end: usize,
    /// Отпечаток значения: позволяет сопоставлять находки, не раскрывая их.
    pub fingerprint: String,
    /// Требовался ли подтверждающий контекст и был ли он найден.
    pub context_confirmed: bool,
}

impl Detection {
    /// Пересекается ли находка с другой.
    pub fn overlaps(&self, other: &Detection) -> bool {
        self.start < other.end && other.start < self.end
    }
}

/// Детектор по известным системе идентификаторам.
pub struct OsTruthDetector {
    identity: IdentitySet,
    policy: DetectionPolicy,
}

impl OsTruthDetector {
    /// Создаёт детектор по набору идентификаторов.
    pub fn new(identity: IdentitySet, policy: DetectionPolicy) -> Self {
        Self { identity, policy }
    }

    /// Создаёт детектор со стандартными настройками.
    pub fn with_defaults(identity: IdentitySet) -> Self {
        Self::new(identity, DetectionPolicy::default())
    }

    /// Сколько идентификаторов в наборе. Полезно для диагностики.
    pub fn identity_count(&self) -> usize {
        self.identity.len()
    }

    /// Состав набора **без значений**: вид и количество.
    pub fn identity_summary(&self) -> std::collections::BTreeMap<IdentityKind, usize> {
        self.identity.summary()
    }

    /// Ищет идентификаторы в тексте.
    ///
    /// Возвращает находки, упорядоченные по началу. Пересечения не
    /// разрешаются здесь: это задача ядра (`merge_cascades`), и дублировать
    /// это правило в детекторе нельзя.
    pub fn detect(&self, text: &str) -> Vec<Detection> {
        let mut found: Vec<Detection> = Vec::new();
        let haystack = if self.policy.case_insensitive {
            text.to_lowercase()
        } else {
            text.to_string()
        };

        for value in self.identity.values() {
            let needle = if self.policy.case_insensitive {
                value.expose().to_lowercase()
            } else {
                value.expose().to_string()
            };
            if needle.is_empty() {
                continue;
            }
            let needs_context = value.kind.needs_context_when_short()
                && value.len() < self.policy.long_enough;

            for (start, _) in haystack.match_indices(&needle) {
                let end = start + needle.len();
                // Границы слова: иначе «ivan» найдётся внутри «ivanovka».
                if !self.boundaries_ok(text, start, end) {
                    continue;
                }
                let confirmed = if needs_context {
                    self.context_near(text, start, end, value.kind)
                } else {
                    true
                };
                if needs_context && !confirmed {
                    continue;
                }
                found.push(Detection {
                    kind: value.kind,
                    start,
                    end,
                    fingerprint: value.fingerprint(),
                    context_confirmed: confirmed,
                });
            }
        }

        found.sort_by_key(|d| (d.start, d.end));
        found
    }

    /// Проверяет, что находка стоит на границах слова.
    fn boundaries_ok(&self, text: &str, start: usize, end: usize) -> bool {
        let left_ok = text[..start]
            .chars()
            .next_back()
            .map(|c| !c.is_alphanumeric() && c != '_' && c != '-')
            .unwrap_or(true);
        let right_ok = text[end..]
            .chars()
            .next()
            .map(|c| !c.is_alphanumeric() && c != '_' && c != '-')
            .unwrap_or(true);
        left_ok && right_ok
    }

    /// Ищет подтверждающий контекст рядом с находкой.
    fn context_near(&self, text: &str, start: usize, end: usize, kind: IdentityKind) -> bool {
        // Окно в 24 символа с каждой стороны: достаточно для «user@host»,
        // «/home/user», «в папке "project"».
        let window_start = start.saturating_sub(24);
        let window_end = (end + 24).min(text.len());
        // Границы окна могут попасть в середину символа UTF-8, поэтому
        // расширяем их до корректных границ.
        let window_start = floor_char_boundary(text, window_start);
        let window_end = ceil_char_boundary(text, window_end);
        let window = &text[window_start..window_end];

        match kind {
            IdentityKind::UserName | IdentityKind::HostName | IdentityKind::SshKeyComment => {
                window.contains('@') || window.contains('/') || window.contains('\\')
            }
            IdentityKind::ProjectDir => {
                window.contains('/')
                    || window.contains('\\')
                    || window.contains('"')
                    || window.contains('«')
                    || window.contains('\'')
                    || {
                        let lower = window.to_lowercase();
                        lower.contains("проект")
                            || lower.contains("папк")
                            || lower.contains("репозитор")
                            || lower.contains("каталог")
                    }
            }
            IdentityKind::GitName | IdentityKind::EnvValue => {
                // Для имени человека и значения переменной контекст — это
                // слово-указатель либо обращение. Полный перечень появится
                // в фазе 2 вместе с реестром меток.
                let lower = window.to_lowercase();
                lower.contains('@')
                    || lower.contains("меня зовут")
                    || lower.contains("моё имя")
                    || lower.contains("мое имя")
                    || lower.contains("фамили")
                    || lower.contains("зовут")
            }
            // Полный путь и адрес почты структурны сами по себе.
            IdentityKind::HomeDir | IdentityKind::Email => true,
        }
    }
}

/// Округляет позицию вниз до границы символа UTF-8.
fn floor_char_boundary(text: &str, mut pos: usize) -> usize {
    while pos > 0 && !text.is_char_boundary(pos) {
        pos -= 1;
    }
    pos
}

/// Округляет позицию вверх до границы символа UTF-8.
fn ceil_char_boundary(text: &str, mut pos: usize) -> usize {
    let len = text.len();
    while pos < len && !text.is_char_boundary(pos) {
        pos += 1;
    }
    pos
}

/// Строит набор идентификаторов из явных значений. Для тестов и для
/// случаев, когда вызывающая сторона сама знает, что подставлять.
pub fn identity_from_pairs(pairs: &[(IdentityKind, &str)]) -> IdentitySet {
    let mut set = IdentitySet::new();
    for (kind, value) in pairs {
        set.push(IdentityValue::new(*kind, *value));
    }
    set
}

/// Реализация моста ядра: позволяет использовать детектор как каскад.
///
/// Тип `Cascade::Format` выбран потому, что источник не использует ни
/// статистику, ни диалог: это точное сравнение строк, и его результат
/// не должен зависеть от загрузки модели.
impl privacy_core::SpanSource for OsTruthDetector {
    fn source_name(&self) -> &'static str {
        "os-truth"
    }

    fn find_spans(&self, text: &str) -> Vec<privacy_core::ExternalSpan> {
        self.detect(text)
            .into_iter()
            .map(|d| privacy_core::ExternalSpan {
                entity_type: d.kind.as_str().to_string(),
                rule_id: format!("os-truth.{}", d.kind.as_str()),
                start: d.start,
                end: d.end,
                context_confirmed: d.context_confirmed,
                fingerprint: d.fingerprint,
            })
            .collect()
    }

    fn fingerprint_of(&self, detection: &privacy_core::ExternalSpan) -> String {
        detection.fingerprint.clone()
    }
}
