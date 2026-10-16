//! Показывает, что детектор по данным системы находит в тексте.
//!
//! # Правило вывода
//!
//! Программа **никогда не печатает значения** идентификаторов: только вид,
//! границы находки и отпечаток. Это позволяет показывать результат вслух,
//! писать в логи и вставлять в отчёты, не раскрывая данные пользователя.
//!
//! Запуск:
//!
//! ```text
//! os-truth --summary
//! os-truth --text "текст для проверки"
//! echo "текст" | os-truth
//! ```

use privacy_os_truth::detect::{DetectionPolicy, OsTruthDetector};
use privacy_os_truth::{collect_identity, redact_for_display};
use std::io::Read;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (identity, skipped) = collect_identity();

    if args.iter().any(|a| a == "--summary") || args.is_empty() {
        print_summary(&identity, skipped.len());
        return ExitCode::SUCCESS;
    }

    let text = match args.iter().position(|a| a == "--text") {
        Some(idx) => match args.get(idx + 1) {
            Some(t) => t.clone(),
            None => {
                eprintln!("после --text нужен текст");
                return ExitCode::from(64);
            }
        },
        None => {
            // Читаем со стандартного ввода, не полагаясь на внешние крейты.
            let mut buffer = String::new();
            if std::io::stdin().read_to_string(&mut buffer).is_err() {
                eprintln!("не удалось прочитать стандартный ввод");
                return ExitCode::from(64);
            }
            buffer
        }
    };

    let detector = OsTruthDetector::new(identity, DetectionPolicy::default());
    let found = detector.detect(&text);

    println!("текст: {}", redact_for_display(&text));
    println!("идентификаторов в наборе: {}", detector.identity_count());
    println!("находок: {}", found.len());
    println!();
    if found.is_empty() {
        println!("ничего не найдено");
        return ExitCode::SUCCESS;
    }

    println!("{:<18} {:>6} {:>6} {:>18}  {}", "вид", "начало", "конец", "отпечаток", "контекст");
    for d in &found {
        println!(
            "{:<18} {:>6} {:>6} {:>18}  {}",
            d.kind.as_str(),
            d.start,
            d.end,
            d.fingerprint,
            if d.context_confirmed { "подтверждён" } else { "нет" }
        );
    }
    ExitCode::SUCCESS
}

fn print_summary(identity: &privacy_os_truth::IdentitySet, skipped: usize) {
    println!("Собрано идентификаторов: {}", identity.len());
    println!();
    println!("{:<20} {}", "вид", "количество");
    for (kind, count) in identity.summary() {
        println!("{:<20} {}", kind.as_str(), count);
    }
    println!();
    println!("Источников недоступно: {skipped}");
    println!();
    println!("Значения не печатаются: только вид, количество и отпечатки.");
    println!("Проверить текст: os-truth --text \"...\"");
}
