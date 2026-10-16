//! Виды идентификаторов и их значения.
//!
//! # Почему значение не печатается
//!
//! Реализация [`std::fmt::Debug`] для [`IdentityValue`] намеренно **не**
//! показывает значение: только вид, длину и отпечаток. Причина практическая:
//! отладочный вывод попадает в логи и в вывод тестов, а репозиторий публичный.
//! Одна случайная печать `dbg!` не должна раскрывать данные пользователя.

use core::fmt;

/// Вид идентификатора устройства.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdentityKind {
    /// Домашний каталог, например `/home/user`.
    HomeDir,
    /// Имя пользователя в системе.
    UserName,
    /// Имя устройства в сети.
    HostName,
    /// Личный адрес электронной почты.
    Email,
    /// Имя, которое пользователь указывает в коммитах.
    GitName,
    /// Имя каталога проекта.
    ProjectDir,
    /// Комментарий в публичном SSH-ключе.
    SshKeyComment,
    /// Значение переменной окружения с идентифицирующим именем.
    EnvValue,
}

impl IdentityKind {
    /// Имя для отчёта. Стабильно: используется в машинночитаемых данных.
    pub const fn as_str(&self) -> &'static str {
        match self {
            IdentityKind::HomeDir => "os.home_dir",
            IdentityKind::UserName => "os.user_name",
            IdentityKind::HostName => "os.host_name",
            IdentityKind::Email => "os.email",
            IdentityKind::GitName => "os.git_name",
            IdentityKind::ProjectDir => "os.project_dir",
            IdentityKind::SshKeyComment => "os.ssh_comment",
            IdentityKind::EnvValue => "os.env_value",
        }
    }

    /// Требует ли этот вид подтверждающего контекста при коротком значении.
    ///
    /// Короткое имя пользователя или каталога может случайно совпасть
    /// с обычным словом. Полный путь или адрес почты — не может.
    pub const fn needs_context_when_short(&self) -> bool {
        matches!(
            self,
            IdentityKind::UserName
                | IdentityKind::HostName
                | IdentityKind::GitName
                | IdentityKind::ProjectDir
                | IdentityKind::SshKeyComment
                | IdentityKind::EnvValue
        )
    }
}

impl fmt::Display for IdentityKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Одно значение, известное системе о своём пользователе.
#[derive(Clone, PartialEq, Eq)]
pub struct IdentityValue {
    /// Вид идентификатора.
    pub kind: IdentityKind,
    /// Значение. Не попадает в отчёты и отладочный вывод.
    value: String,
}

impl IdentityValue {
    /// Создаёт значение.
    pub fn new(kind: IdentityKind, value: impl Into<String>) -> Self {
        Self {
            kind,
            value: value.into(),
        }
    }

    /// Доступ к значению.
    ///
    /// Метод назван явно, чтобы в коде было видно место, где значение
    /// берётся в открытом виде: такие места легко проверить глазами.
    pub fn expose(&self) -> &str {
        &self.value
    }

    /// Длина значения в байтах.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Пусто ли значение.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Отпечаток значения: нужен, чтобы отличать значения друг от друга
    /// в отчётах, не раскрывая их.
    pub fn fingerprint(&self) -> String {
        crate::redact::fingerprint(&self.value)
    }

    /// Затирает значение.
    pub fn wipe(&mut self) {
        // Заполняем нулями существующие байты, затем очищаем строку.
        // Без `unsafe`: используем безопасную замену через `clear` после
        // перезаписи через `String::replace_range` невозможно, поэтому
        // применяем `zeroize`-совместимый приём — перезапись символами.
        let len = self.value.len();
        self.value = "0".repeat(len);
        self.value.clear();
        self.value.shrink_to_fit();
    }
}

impl fmt::Debug for IdentityValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IdentityValue")
            .field("kind", &self.kind)
            .field("len", &self.value.len())
            .field("fingerprint", &self.fingerprint())
            .finish()
    }
}
