//! Командная строка ядра приватности.
//!
//! # Про порядок появления команд
//!
//! `show` — не украшение и не отладочная команда. Это основной артефакт
//! проекта: страница, где рядом видно «что ты написал» и «что реально ушло
//! наружу». Без неё никто не разберётся в проекте за минуту.
//!
//! На фазе 1 команды объявлены, но не реализованы: печатают состояние и
//! выходят с кодом 2. Это осознанно — CLI не должен создавать впечатление
//! работающего ядра, пока детекторов нет.

use std::process::ExitCode;

const USAGE: &str = "\
privacy-core — ядро приватности для текста, уходящего внешним языковым моделям

Использование:
    privacy-core show <текст>      показать, что ушло бы наружу
    privacy-core anonymize         обезличить текст со стандартного ввода
    privacy-core restore           восстановить значения в ответе модели
    privacy-core version           версия ядра и протокола
    privacy-core --help            эта справка

Статус: фаза 1. Ядро собирается, инварианты объявлены, детекторов пока нет.
Команды anonymize и restore вернут код 2 до появления каскадов.
";

const EXIT_NOT_IMPLEMENTED: u8 = 2;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().map(String::as_str).unwrap_or("--help");

    match command {
        "--help" | "-h" | "help" => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
        "version" | "--version" | "-V" => {
            println!("privacy-core {}", privacy_core::VERSION);
            println!("protocol    {}", privacy_core::PROTOCOL_VERSION);
            ExitCode::SUCCESS
        }
        "show" | "anonymize" | "restore" => {
            eprintln!("команда «{command}» ещё не реализована: фаза 1 объявляет контракт, а не поведение");
            eprintln!("инварианты и план — в docs/10-architecture.md");
            ExitCode::from(EXIT_NOT_IMPLEMENTED)
        }
        other => {
            eprintln!("неизвестная команда: {other}");
            eprint!("{USAGE}");
            ExitCode::from(64)
        }
    }
}
