//! Тесты детектора по данным системы.
//!
//! # Все значения вымышленные
//!
//! Репозиторий публичный, поэтому ни одно настоящее значение здесь не
//! используется: ни домашний каталог, ни почта, ни имя машины. Наборы для
//! тестов собираются вручную из выдуманных данных.
//!
//! Дополнительно проверяется, что отладочный вывод не раскрывает значение:
//! это защита от случайного `dbg!` в будущем.

use privacy_os_truth::detect::{identity_from_pairs, DetectionPolicy, OsTruthDetector};
use privacy_os_truth::{redact_for_display, IdentityKind};

/// Выдуманное имя пользователя. Восемь символов: порог «длинного»
/// значения начинается отсюда, поэтому контекст для него не требуется.
const FAKE_USER: &str = "testuser";
/// Выдуманное **короткое** имя пользователя: для него контекст обязателен.
const FAKE_SHORT_USER: &str = "tester";
/// Выдуманный хост.
const FAKE_HOST: &str = "workstation";
/// Выдуманная почта.
const FAKE_EMAIL: &str = "someone@example.invalid";
/// Выдуманный домашний каталог.
const FAKE_HOME: &str = "/home/testuser";
/// Выдуманное имя проекта.
const FAKE_PROJECT: &str = "secret-project";

fn fake_identity() -> privacy_os_truth::IdentitySet {
    identity_from_pairs(&[
        (IdentityKind::UserName, FAKE_USER),
        (IdentityKind::HostName, FAKE_HOST),
        (IdentityKind::Email, FAKE_EMAIL),
        (IdentityKind::HomeDir, FAKE_HOME),
        (IdentityKind::ProjectDir, FAKE_PROJECT),
    ])
}

fn detector() -> OsTruthDetector {
    OsTruthDetector::with_defaults(fake_identity())
}

#[test]
fn finds_home_directory_in_text() {
    let text = "файл лежит в /home/testuser/core/nothing.txt";
    let found = detector().detect(text);
    assert!(
        found.iter().any(|d| d.kind == IdentityKind::HomeDir),
        "домашний каталог должен находиться: {found:?}"
    );
}

#[test]
fn finds_email_without_context() {
    let text = "напиши на someone@example.invalid сегодня";
    let found = detector().detect(text);
    let email = found
        .iter()
        .find(|d| d.kind == IdentityKind::Email)
        .expect("почта должна находиться");
    assert!(email.context_confirmed, "почте контекст не требуется");
}

#[test]
fn short_username_requires_context() {
    let short = OsTruthDetector::with_defaults(identity_from_pairs(&[(
        IdentityKind::UserName,
        FAKE_SHORT_USER,
    )]));

    // Без контекста короткое значение не считается находкой: иначе оно
    // совпадёт с обычным словом в тексте.
    let without = short.detect("это tester просто слово");
    assert!(
        !without.iter().any(|d| d.kind == IdentityKind::UserName),
        "короткое имя без контекста не должно находиться"
    );

    // С контекстом формата пользователь@машина — считается.
    let with = short.detect("зайди как tester@workstation по ssh");
    assert!(
        with.iter().any(|d| d.kind == IdentityKind::UserName),
        "имя в формате пользователь@машина должно находиться"
    );

    // И в форме пути тоже.
    let path_form = short.detect("файл в /home/tester/документы");
    assert!(path_form.iter().any(|d| d.kind == IdentityKind::UserName));
}

#[test]
fn finds_user_in_path_form() {
    let found = detector().detect("путь /home/testuser/работа");
    assert!(found.iter().any(|d| d.kind == IdentityKind::UserName));
}

#[test]
fn project_name_requires_directory_context() {
    // Короткое имя каталога: без контекста не считается находкой.
    let short = OsTruthDetector::with_defaults(identity_from_pairs(&[(
        IdentityKind::ProjectDir,
        "proj7",
    )]));
    assert!(
        !short.detect("proj7 это просто слово").iter().any(|d| d.kind == IdentityKind::ProjectDir),
        "короткое имя проекта без контекста не должно находиться"
    );
    assert!(
        short.detect("посмотри в папке proj7").iter().any(|d| d.kind == IdentityKind::ProjectDir),
        "имя проекта с контекстом должно находиться"
    );

    // Длинное имя проекта находится и без контекста: оно достаточно
    // уникально, чтобы не совпасть со словом.
    let long = OsTruthDetector::with_defaults(identity_from_pairs(&[(
        IdentityKind::ProjectDir,
        FAKE_PROJECT,
    )]));
    assert!(
        long.detect("secret-project упомянут тут").iter().any(|d| d.kind == IdentityKind::ProjectDir),
        "длинное имя проекта находится без контекста"
    );
}

#[test]
fn word_boundaries_are_respected() {
    // «testuser» внутри «testuserovka» не должно считаться находкой.
    let found = detector().detect("testuserovka — это деревня");
    assert!(
        !found.iter().any(|d| d.kind == IdentityKind::UserName),
        "часть слова не должна считаться идентификатором"
    );
}

#[test]
fn case_insensitive_matching_works() {
    let found = detector().detect("почта SOMEONE@EXAMPLE.INVALID указана");
    assert!(found.iter().any(|d| d.kind == IdentityKind::Email));
}

#[test]
fn cyrillic_text_with_identifiers_is_handled() {
    // Проверка границ окна контекста на кириллице: они не должны попадать
    // в середину символа и вызывать панику.
    let text = "Здравствуйте! Я пишу с testuser@workstation, помогите с задачей.";
    let found = detector().detect(text);
    assert!(!found.is_empty(), "находки должны быть");
}

#[test]
fn detection_carries_no_value() {
    let found = detector().detect("почта someone@example.invalid");
    let rendered = format!("{found:?}");
    assert!(
        !rendered.contains("someone@example.invalid"),
        "находка не должна содержать значение: {rendered}"
    );
    assert!(rendered.contains("fingerprint"), "должен быть отпечаток");
}

#[test]
fn identity_debug_hides_value() {
    let set = fake_identity();
    let rendered = format!("{set:?}");
    assert!(
        !rendered.contains(FAKE_EMAIL) && !rendered.contains(FAKE_USER),
        "отладочный вывод набора не должен раскрывать значения: {rendered}"
    );
}

#[test]
fn redact_for_display_hides_value() {
    let shown = redact_for_display(FAKE_EMAIL);
    assert!(!shown.contains(FAKE_EMAIL));
}

#[test]
fn detections_are_sorted_by_position() {
    let text = "почта someone@example.invalid и путь /home/testuser/x";
    let found = detector().detect(text);
    for pair in found.windows(2) {
        assert!(pair[0].start <= pair[1].start, "находки должны идти по порядку");
    }
}

#[test]
fn empty_identity_finds_nothing() {
    let detector = OsTruthDetector::with_defaults(identity_from_pairs(&[]));
    assert!(detector.detect("любой текст, /home/testuser, someone@example.invalid").is_empty());
}

#[test]
fn long_username_needs_no_context() {
    // Длинное значение достаточно уникально, чтобы не требовать контекста.
    let set = identity_from_pairs(&[(IdentityKind::UserName, "ivan.petrov.long")]);
    let detector = OsTruthDetector::with_defaults(set);
    let found = detector.detect("тут упомянут ivan.petrov.long без контекста");
    assert_eq!(found.len(), 1);
    assert!(found[0].context_confirmed);
}

#[test]
fn custom_policy_changes_threshold() {
    let set = identity_from_pairs(&[(IdentityKind::UserName, "abcdefg")]);
    // Порог 7: значение длиной 7 уже не считается коротким.
    let strict = OsTruthDetector::new(
        set.clone(),
        DetectionPolicy {
            long_enough: 7,
            case_insensitive: true,
        },
    );
    assert_eq!(strict.detect("тут abcdefg встретилось").len(), 1);

    // Порог 8: то же значение требует контекста и без него не находится.
    let lax = OsTruthDetector::new(
        set,
        DetectionPolicy {
            long_enough: 8,
            case_insensitive: true,
        },
    );
    assert!(lax.detect("тут abcdefg встретилось").is_empty());
}
