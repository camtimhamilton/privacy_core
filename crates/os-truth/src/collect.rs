//! Сбор идентификаторов: что именно берём и откуда.
//!
//! # Принцип: только явные источники
//!
//! Каждый источник перечислен в коде поимённо. Ничего не сканируется
//! «на всякий случай». Сознательно **не** собираются:
//!
//! * список процессов и история команд — это данные, которые надо защищать
//!   сильнее, чем то, что мы защищаем;
//! * содержимое буфера обмена;
//! * содержимое домашнего каталога целиком;
//! * любые секреты вида токенов и ключей: они не персональные данные,
//!   и для них существуют отдельные инструменты.
//!
//! # Источники
//!
//! | Вид | Откуда | Почему надёжен |
//! |---|---|---|
//! | `HomeDir` | переменная `HOME` | Точное значение, не догадка |
//! | `UserName` | `USER`, `LOGNAME`, либо владелец домашнего каталога | Так система называет пользователя |
//! | `HostName` | системный вызов, только чтение | Имя устройства в сети |
//! | `GitEmail`, `GitName` | `~/.gitconfig` | То, что пользователь сам подставляет в коммиты |
//! | `ProjectDir` | имена каталогов первого уровня в домашнем | Так называются проекты |
//! | `Email` | из git, из переменных почты | Личный адрес |
//! | `SshKeyComment` | комментарий в `~/.ssh/*.pub` | Часто содержит `user@host` |
//! | `EnvValue` | переменные с явно идентифицирующими именами | Заранее заданный список имён |

use crate::{CollectError, IdentitySet, IdentityValue, IdentityKind};
use std::fs;
use std::path::Path;

/// Собирает идентификаторы из окружения.
///
/// Источники, которые недоступны, пропускаются: их отсутствие не ошибка.
/// Возвращает набор и список пропущенных источников — второй нужен, чтобы
/// в диагностике было видно, что именно не собралось, **без значений**.
pub fn collect_identity() -> (IdentitySet, Vec<CollectError>) {
    let mut set = IdentitySet::new();
    let mut skipped = Vec::new();

    if let Some(home) = crate::home_dir() {
        set.push(IdentityValue::new(
            IdentityKind::HomeDir,
            home.to_string_lossy().to_string(),
        ));

        // Имя пользователя: сначала переменные, потом имя домашнего каталога.
        let user = std::env::var("USER")
            .ok()
            .or_else(|| std::env::var("LOGNAME").ok())
            .or_else(|| {
                home.file_name()
                    .map(|name| name.to_string_lossy().to_string())
            });
        match user {
            Some(user) if !user.is_empty() => {
                set.push(IdentityValue::new(IdentityKind::UserName, user));
            }
            _ => skipped.push(CollectError::Unavailable { source: "user-name" }),
        }

        collect_git_config(&home, &mut set, &mut skipped);
        collect_project_dirs(&home, &mut set, &mut skipped);
        collect_ssh_comments(&home, &mut set);
    } else {
        skipped.push(CollectError::Unavailable { source: "home-dir" });
    }

    collect_hostname(&mut set, &mut skipped);
    collect_env(&mut set);

    (set, skipped)
}

/// Имя устройства. Только чтение, без сети.
fn collect_hostname(set: &mut IdentitySet, skipped: &mut Vec<CollectError>) {
    match hostname_from_system() {
        Some(name) if !name.is_empty() => {
            set.push(IdentityValue::new(IdentityKind::HostName, name));
        }
        _ => skipped.push(CollectError::Unavailable { source: "hostname" }),
    }
}

/// Читает имя устройства из файла ядра, чтобы не тянуть зависимости
/// и не выполнять сетевых запросов.
fn hostname_from_system() -> Option<String> {
    if let Ok(name) = fs::read_to_string("/proc/sys/kernel/hostname") {
        let trimmed = name.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    std::env::var("HOSTNAME").ok().filter(|s| !s.is_empty())
}

/// Почта и имя из конфигурации git.
///
/// Разбирается вручную, чтобы не зависеть от внешнего `git`: крейт не должен
/// запускать процессы.
fn collect_git_config(home: &Path, set: &mut IdentitySet, skipped: &mut Vec<CollectError>) {
    let path = home.join(".gitconfig");
    let Ok(content) = fs::read_to_string(&path) else {
        skipped.push(CollectError::Unavailable { source: "gitconfig" });
        return;
    };
    let mut found_email = false;
    let mut found_name = false;
    for line in content.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().to_ascii_lowercase();
        let value = value.trim().to_string();
        if value.is_empty() {
            continue;
        }
        if key == "email" && value.contains('@') {
            set.push(IdentityValue::new(IdentityKind::Email, value));
            found_email = true;
        } else if (key == "name" || key == "signingkey") && key == "name" {
            set.push(IdentityValue::new(IdentityKind::GitName, value));
            found_name = true;
        }
    }
    if !found_email {
        skipped.push(CollectError::Unavailable {
            source: "git-email",
        });
    }
    if !found_name {
        skipped.push(CollectError::Unavailable { source: "git-name" });
    }
}

/// Имена каталогов первого уровня в домашнем каталоге.
///
/// Это то, как пользователь называет свои проекты, и такие имена часто
/// попадают в промпты («посмотри в папке my-secret-project»).
fn collect_project_dirs(home: &Path, set: &mut IdentitySet, skipped: &mut Vec<CollectError>) {
    let Ok(entries) = fs::read_dir(home) else {
        skipped.push(CollectError::Unavailable {
            source: "project-dirs",
        });
        return;
    };
    let mut count = 0usize;
    for entry in entries.flatten() {
        // Ограничение нужно, чтобы не превратить сбор в обход всего диска.
        if count >= 32 {
            break;
        }
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        if !file_type.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        // Служебные каталоги пропускаем: они одинаковы у всех и не
        // идентифицируют пользователя.
        if name.starts_with('.') || IGNORED_DIRS.contains(&name.as_str()) {
            continue;
        }
        set.push(IdentityValue::new(IdentityKind::ProjectDir, name));
        count += 1;
    }
    if count == 0 {
        skipped.push(CollectError::Unavailable {
            source: "project-dirs",
        });
    }
}

/// Каталоги, которые есть у всех и потому ничего не говорят о человеке.
const IGNORED_DIRS: &[&str] = &[
    "Desktop",
    "Documents",
    "Downloads",
    "Music",
    "Pictures",
    "Public",
    "Templates",
    "Videos",
    "bin",
    "tmp",
    "Рабочий стол",
    "Документы",
    "Загрузки",
];

/// Комментарии в публичных SSH-ключах: часто содержат `пользователь@машина`.
fn collect_ssh_comments(home: &Path, set: &mut IdentitySet) {
    let dir = home.join(".ssh");
    let Ok(entries) = fs::read_dir(&dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(".pub") {
            continue;
        }
        let Ok(content) = fs::read_to_string(entry.path()) else {
            continue;
        };
        // Формат: "тип ключа base64 комментарий". Берём только комментарий,
        // сам ключ нам не нужен и мы его не сохраняем.
        if let Some(comment) = content.split_whitespace().nth(2) {
            set.push(IdentityValue::new(
                IdentityKind::SshKeyComment,
                comment.to_string(),
            ));
        }
    }
}

/// Переменные окружения с явно идентифицирующими именами.
///
/// Список закрытый: собирать всё окружение целиком опасно, потому что там
/// лежат токены и ключи.
fn collect_env(set: &mut IdentitySet) {
    const IDENTIFYING_VARS: &[&str] = &[
        "EMAIL",
        "MAIL",
        "GIT_AUTHOR_EMAIL",
        "GIT_COMMITTER_EMAIL",
        "GIT_AUTHOR_NAME",
        "GIT_COMMITTER_NAME",
        "USER_EMAIL",
        "FULL_NAME",
    ];
    for name in IDENTIFYING_VARS {
        let Ok(value) = std::env::var(name) else {
            continue;
        };
        let value = value.trim().to_string();
        if value.is_empty() {
            continue;
        }
        let kind = if value.contains('@') {
            IdentityKind::Email
        } else {
            IdentityKind::EnvValue
        };
        set.push(IdentityValue::new(kind, value));
    }
}
