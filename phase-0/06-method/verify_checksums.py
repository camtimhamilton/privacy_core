#!/usr/bin/env python3
"""
Фаза 0: сверка контрольных сумм российских реквизитов.

Назначение: проверить алгоритм контрольного ключа банковского счёта (порядок
нумерации счетов, положение Банка России 1417-У) на ЖИВЫХ данных и на примерах
из учебной литературы, и замерить долю ложных прохождений случайных цифр.

Запуск:  python3 phase-0/06-method/verify_checksums.py
Зависимости: только стандартная библиотека.

Статус источников и результатов — в phase-0/06-method/checksum-verification.md
"""
import random

# Весовые коэффициенты: 7,1,3 по кругу. Для 23 разрядов берём первые 23.
WEIGHTS = [7, 1, 3] * 8


def account_key_ok(prefix3: str, account20: str) -> bool:
    """
    Проверка контрольного ключа лицевого счёта.

    Алгоритм (подтверждён на реальных реквизитах, см. отчёт):
      1. Разряды 1-3: условный номер РКЦ = разряды 5-6 БИК, дополненные слева нулём.
         Практически для 9-значного БИК это три последние цифры БИК.
      2. Разряды 4-23: номер лицевого счёта (20 цифр).
      3. Сумма МЛАДШИХ разрядов произведений цифр на веса должна быть кратна 10.
    """
    if len(prefix3) != 3 or len(account20) != 20:
        return False
    if not (prefix3 + account20).isdigit():
        return False
    s = prefix3 + account20
    total = sum((int(c) * WEIGHTS[i]) % 10 for i, c in enumerate(s))
    return total % 10 == 0


def expected_key(prefix3: str, account20: str) -> int:
    """Какой контрольный ключ (9-я цифра счёта) обязан быть при данном префиксе."""
    s = prefix3 + account20[:8] + "0" + account20[9:]
    k = sum((int(c) * WEIGHTS[i]) % 10 for i, c in enumerate(s)) % 10
    return k * 3 % 10


def rkc_prefix(bic9: str) -> str:
    """Условный номер РКЦ из БИК: разряды 5-6, дополненные слева нулём."""
    return bic9[6:9]


# --- Данные для проверки -----------------------------------------------------

# Живые реквизиты: платёжный документ ООО «ОнЛайм» (публичный PDF).
REAL = [
    ("044525225", "40702810638180003655", "расчётный счёт клиента", True),
    ("044525225", "30101810400000000225", "корреспондентский счёт банка", False),
]

# Примеры из учебника (КФУ, «Банковский практикум»). В тексте обоим примерам
# подставлен префикс 005 — это опечатка, см. отчёт.
TEXTBOOK = [
    ("049805000", "30101810800000000746", "пример 1, корсчёт", "005"),
    ("040305000", "40102810100000010001", "пример 2, лицевой счёт", "005"),
]


def main() -> None:
    print("=" * 78)
    print("A. Живые реквизиты (реальный платёжный документ)")
    print("=" * 78)
    for bic, acc, label, expect in REAL:
        got = account_key_ok(rkc_prefix(bic), acc)
        mark = "совпало " if got == expect else "РАСХОЖДЕНИЕ"
        print(f"  [{mark}] {acc} БИК {bic} -> правило {'применимо' if got else 'не применимо':14s} | {label}")

    print()
    print("=" * 78)
    print("B. Учебник: проверка с префиксом из текста (005) и с префиксом из БИК")
    print("=" * 78)
    for bic, acc, label, tb_prefix in TEXTBOOK:
        from_bic = account_key_ok(rkc_prefix(bic), acc)
        from_text = account_key_ok(tb_prefix, acc)
        print(f"  {acc}")
        print(f"    префикс из БИК ({rkc_prefix(bic)}): правило {'ПРИМЕНИМО' if from_bic else 'не применимо'}")
        print(f"    префикс из текста ({tb_prefix}): правило {'применимо' if from_text else 'НЕ применимо'}  <- так напечатано в учебнике")
        print(f"    ожидаемый ключ при префиксе из БИК: {expected_key(rkc_prefix(bic), acc)}, в счёте стоит: {acc[8]}")

    print()
    print("=" * 78)
    print("C. Ложное прохождение: счёт 20 цифр без знания БИК")
    print("=" * 78)
    random.seed(11)
    n = 200_000

    def rate(make_prefix, make_acc, label):
        hits = sum(
            1 for _ in range(n)
            if account_key_ok(make_prefix(), make_acc())
        )
        print(f"  {label:52s} 1 в {n / hits:5.1f}  ({hits / n * 100:6.3f}%)")

    rate(
        lambda: "".join(random.choice("0123456789") for _ in range(3)),
        lambda: "".join(random.choice("0123456789") for _ in range(20)),
        "префикс неизвестен, счёт случайный",
    )
    rate(
        lambda: "".join(random.choice("0123456789") for _ in range(3)),
        lambda: "40702" + "".join(random.choice("0123456789") for _ in range(15)),
        "префикс неизвестен, реалистичное начало счёта",
    )

    print()
    print("  ВЫВОД: даже валидный контрольный ключ не является доказательством без БИК.")
    print("         Для детектора счёт требует контекста (БИК рядом либо метка «счёт»).")


if __name__ == "__main__":
    main()
