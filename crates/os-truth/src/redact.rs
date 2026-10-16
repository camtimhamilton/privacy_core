//! Отпечатки и безопасный показ значений.
//!
//! Зачем отпечаток: в отчёте полезно различать значения между собой
//! («эти две находки — одно и то же значение»), но нельзя показывать само
//! значение. Отпечаток решает обе задачи.
//!
//! Отпечаток **не является** криптографической защитой: по короткому
//! значению его можно перебрать. Он нужен для сопоставления, а не для
//! сокрытия, и это надо понимать.

use sha2::{Digest, Sha256};

/// Отпечаток значения: первые 8 байт SHA-256 в шестнадцатеричном виде.
///
/// Длина выбрана из соображений читаемости в отчётах. Для сопоставления
/// внутри сессии этого достаточно.
pub fn fingerprint(value: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(value.as_bytes());
    let digest = hasher.finalize();
    let mut out = String::with_capacity(16);
    for byte in &digest[..8] {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Показывает значение в усечённом виде для отладки человеком.
///
/// Показывает только длину и отпечаток. Усечённый показ первых символов
/// намеренно **не** делается: даже двух-трёх символов имени пользователя
/// вместе с отпечатком достаточно, чтобы опознать человека в узком кругу.
pub fn redact_for_display(value: &str) -> String {
    format!("<скрыто len={} fp={}>", value.len(), fingerprint(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fingerprint_is_stable() {
        assert_eq!(fingerprint("пример"), fingerprint("пример"));
    }

    #[test]
    fn fingerprint_differs_for_different_values() {
        assert_ne!(fingerprint("пример"), fingerprint("пример2"));
    }

    #[test]
    fn redacted_output_contains_no_original() {
        let secret = "очень-секретное-значение";
        let shown = redact_for_display(secret);
        assert!(!shown.contains(secret));
        assert!(shown.contains("скрыто"));
    }
}
