#!/usr/bin/env python3
"""
Генератор ВАЛИДНЫХ российских реквизитов (прототип) + самопроверка.

ЗАЧЕМ. В отчёте phase-0/03-repos/README.md (раздел 6.3) зафиксировано:
готового открытого генератора валидных российских реквизитов НЕТ.
  - Faker('ru_RU') даёт формат без контрольного ключа (см. faker_ru_checksum_probe.py)
  - mimesis вообще не покрывает ИНН/СНИЛС/БИК/счета

Следствие: подмена, сгенерированная Faker, может не пройти проверку
контрольного ключа на стороне получателя (LLM-инструмент, валидатор, downstream).
Этот скрипт показывает, что раз мы умеем ПРОВЕРЯТЬ контрольные суммы
(см. phase-0/06-method/checksum-verification.md), мы почти бесплатно умеем
и ГЕНЕРИРОВАТЬ валидные значения.

СТАТУС. Чистый Python 3.10+, зависимостей нет. Запускается где угодно.

    python3 gen_ru_checksums.py

ВАЖНО ПРО ЛИЦЕНЗИИ И ЭТИКУ. Все значения синтетические и НЕ принадлежат
реальным лицам. Генератор детерминирован по seed — это требование к
воспроизводимости подмен (одинаковый вход -> одинаковая подмена).

ОГРАНИЧЕНИЕ. Корреспондентский счёт (301...) здесь НЕ генерируется: по
phase-0/06-method/checksum-verification.md его контрольный ключ не проверяется.
См. функцию correspondent_account_note().
"""

from __future__ import annotations

import random
import re
from dataclasses import dataclass

# --- Алгоритмы --------------------------------------------------------------

_INN_W10 = [2, 4, 10, 3, 5, 9, 4, 6, 8]
_INN_W11 = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8]
_INN_W12 = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8]

# Последние 3 цифры БИК + 20 цифр счёта = 23 цифры.
_ACC_WEIGHTS = [7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1]


def _inn_check_digit(digits: str, weights: list[int]) -> int:
    return (sum(int(d) * w for d, w in zip(digits, weights)) % 11) % 10


def inn_check_digit_10(first9: str) -> int:
    """Контрольная цифра ИНН юрлица (10-значный ИНН)."""
    return _inn_check_digit(first9, _INN_W10)


def inn_check_digits_12(first10: str) -> tuple[int, int]:
    """Две контрольные цифры ИНН физлица (12-значный ИНН)."""
    d11 = _inn_check_digit(first10, _INN_W11)
    d12 = _inn_check_digit(first10 + str(d11), _INN_W12)
    return d11, d12


def inn_is_valid(inn: str) -> bool:
    if not inn.isdigit():
        return False
    if len(inn) == 10:
        return inn_check_digit_10(inn[:9]) == int(inn[9])
    if len(inn) == 12:
        d11, d12 = inn_check_digits_12(inn[:10])
        return d11 == int(inn[10]) and d12 == int(inn[11])
    return False


def snils_check_number(first9: str) -> int:
    """Контрольное число СНИЛС (2 цифры, 0..99)."""
    total = sum(int(d) * (9 - i) for i, d in enumerate(first9))
    check = total % 101
    if check == 100:
        check = 0
    return check


def snils_is_valid(snils: str) -> bool:
    digits = re.sub(r"\D", "", snils)
    if len(digits) != 11:
        return False
    return snils_check_number(digits[:9]) == int(digits[9:11])


def account_control_digit(account_without_check: str, bik: str) -> int:
    """Подобрать последнюю цифру счёта так, чтобы контрольный ключ сошёлся.

    `account_without_check` — 19 цифр (счёт без последней), `bik` — 9 цифр.
    Возвращает цифру, при которой сумма произведений 23-значной строки
    (последние 3 цифры БИК + 20 цифр счёта) делится на 10.
    """
    if len(account_without_check) != 19 or len(bik) != 9:
        raise ValueError("нужно 19 цифр счёта и 9 цифр БИК")
    prefix = bik[-3:] + account_without_check  # 22 цифры
    partial = sum(int(c) * w for c, w in zip(prefix, _ACC_WEIGHTS[:22]))
    last_weight = _ACC_WEIGHTS[22]
    for digit in range(10):
        if (partial + digit * last_weight) % 10 == 0:
            return digit
    raise AssertionError("решение обязано существовать")


def account_is_valid(account: str, bik: str) -> bool:
    if len(account) != 20 or len(bik) != 9:
        return False
    digits = bik[-3:] + account
    return sum(int(c) * w for c, w in zip(digits, _ACC_WEIGHTS)) % 10 == 0


def bik_is_wellformed(bik: str) -> bool:
    """БИК: 9 цифр, начинается с '04' (РФ). Контрольного разряда нет."""
    return bool(re.fullmatch(r"04\d{7}", bik))


def correspondent_account_note() -> str:
    return (
        "Корреспондентский счёт НЕ генерируется: по "
        "phase-0/06-method/checksum-verification.md его контрольный ключ "
        "не проверяется стандартным способом. В релизе не подменяем."
    )


# --- Генератор --------------------------------------------------------------


@dataclass
class RuIdentifiers:
    inn_ul: str          # ИНН юрлица, 10 цифр
    inn_fl: str          # ИНН физлица, 12 цифр
    snils: str           # СНИЛС, формат XXX-XXX-XXX YY
    bik: str             # БИК, 9 цифр
    account: str         # расчётный счёт, 20 цифр (валидный ключ)
    passport: str        # серия и номер паспорта РФ (без контрольной суммы)


_REGION_CODES = ["01", "02", "03", "04", "05", "06", "07", "08", "09", "10",
                 "11", "12", "14", "15", "16", "17", "18", "19", "20", "21",
                 "22", "23", "24", "25", "26", "27", "28", "29", "30", "31",
                 "32", "33", "34", "35", "36", "37", "38", "39", "40", "41",
                 "42", "43", "44", "45", "46", "47", "48", "49", "50", "51",
                 "52", "53", "54", "55", "56", "57", "58", "59", "60", "61",
                 "62", "63", "64", "65", "66", "67", "68", "69", "70", "71",
                 "72", "73", "74", "75", "76", "77", "78", "79", "80", "81",
                 "82", "83", "84", "85", "86", "87", "88", "89", "90", "91",
                 "92", "93", "99"]


def generate(seed: int = 20261016) -> RuIdentifiers:
    """Детерминированная генерация валидных реквизитов."""
    rng = random.Random(seed)

    # ИНН юрлица: 10 цифр, первая пара — код региона
    first9 = rng.choice(_REGION_CODES) + "".join(str(rng.randrange(10)) for _ in range(7))
    inn_ul = first9 + str(inn_check_digit_10(first9))

    # ИНН физлица: 12 цифр
    first10 = rng.choice(_REGION_CODES) + "".join(str(rng.randrange(10)) for _ in range(8))
    d11, d12 = inn_check_digits_12(first10)
    inn_fl = first10 + str(d11) + str(d12)

    # СНИЛС: 9 значимых цифр + контрольное число
    base9 = "".join(str(rng.randrange(10)) for _ in range(9))
    check = snils_check_number(base9)
    snils_digits = base9 + "%02d" % check
    snils = "%s-%s-%s %s" % (snils_digits[0:3], snils_digits[3:6],
                             snils_digits[6:9], snils_digits[9:11])

    # БИК: 04 + регион + условный номер подразделения + код кредитной организации
    bik = "04" + rng.choice(_REGION_CODES) + "".join(str(rng.randrange(10)) for _ in range(5))

    # Расчётный счёт: 3 (код счёта) + 2 (организация) + 3 (валюта) + 11 случайных
    # + подобранная контрольная цифра
    account_prefix = "407" + "02" + "810" + "".join(str(rng.randrange(10)) for _ in range(11))
    account = account_prefix + str(account_control_digit(account_prefix, bik))

    # Паспорт РФ: серия (4) + номер (6). Контрольной суммы нет.
    passport = "%02d %02d %06d" % (rng.randrange(10, 99), rng.randrange(10, 99),
                                   rng.randrange(100000, 999999))

    return RuIdentifiers(inn_ul=inn_ul, inn_fl=inn_fl, snils=snils, bik=bik,
                         account=account, passport=passport)


# --- Самопроверка -----------------------------------------------------------


def self_test() -> int:
    failures = 0

    # 1. Известные валидные ИНН (публичные реквизиты юрлиц из открытых реестров,
    #    используются только как тестовые векторы алгоритма).
    known_inn_10 = ["7707083893", "7736050003", "7728168971"]
    for inn in known_inn_10:
        ok = inn_is_valid(inn)
        print("  ИНН-10 %s -> %s" % (inn, "валиден" if ok else "НЕВАЛИДЕН"))
        failures += 0 if ok else 1

    # 2. Известный валидный ИНН физлица (тестовый вектор)
    known_inn_12 = ["500100732259"]
    for inn in known_inn_12:
        ok = inn_is_valid(inn)
        print("  ИНН-12 %s -> %s" % (inn, "валиден" if ok else "НЕВАЛИДЕН"))
        failures += 0 if ok else 1

    # 3. Известный валидный СНИЛС (тестовый вектор)
    for snils in ["112-233-445 95"]:
        ok = snils_is_valid(snils)
        print("  СНИЛС  %s -> %s" % (snils, "валиден" if ok else "НЕВАЛИДЕН"))
        failures += 0 if ok else 1

    # 4. Генерация: всё должно проходить собственные проверки
    for seed in (1, 42, 20261016):
        ids = generate(seed)
        checks = {
            "inn_ul": inn_is_valid(ids.inn_ul),
            "inn_fl": inn_is_valid(ids.inn_fl),
            "snils": snils_is_valid(ids.snils),
            "bik_wellformed": bik_is_wellformed(ids.bik),
            "account": account_is_valid(ids.account, ids.bik),
        }
        bad = [k for k, v in checks.items() if not v]
        print("  seed=%-9d %s" % (seed, "всё валидно" if not bad else "СБОЙ: %s" % bad))
        failures += len(bad)

    # 5. Детерминированность
    if generate(7) == generate(7):
        print("  детерминированность по seed: ok")
    else:
        print("  детерминированность по seed: СБОЙ")
        failures += 1

    # 6. Отрицательный контроль: испорченная цифра обязана ломать проверку
    ids = generate(3)
    broken = ids.inn_fl[:11] + str((int(ids.inn_fl[11]) + 1) % 10)
    if not inn_is_valid(broken):
        print("  отрицательный контроль (испорченный ИНН): ok")
    else:
        print("  отрицательный контроль (испорченный ИНН): СБОЙ")
        failures += 1

    return failures


def main() -> int:
    print("=" * 72)
    print("Самопроверка алгоритмов контрольных сумм")
    print("=" * 72)
    failures = self_test()

    print()
    print("=" * 72)
    print("Пример генерации (seed=20261016)")
    print("=" * 72)
    ids = generate(20261016)
    print("  ИНН юрлица (10):  %s" % ids.inn_ul)
    print("  ИНН физлица (12): %s" % ids.inn_fl)
    print("  СНИЛС:            %s" % ids.snils)
    print("  БИК:              %s" % ids.bik)
    print("  Расчётный счёт:   %s" % ids.account)
    print("  Паспорт РФ:       %s" % ids.passport)
    print()
    print("  %s" % correspondent_account_note())
    print()
    print("  Проверка сгенерированного:")
    print("    ИНН юрлица валиден:   %s" % inn_is_valid(ids.inn_ul))
    print("    ИНН физлица валиден:  %s" % inn_is_valid(ids.inn_fl))
    print("    СНИЛС валиден:        %s" % snils_is_valid(ids.snils))
    print("    счёт валиден по БИК:  %s" % account_is_valid(ids.account, ids.bik))
    print()
    if failures:
        print("ИТОГ: %d сбоев" % failures)
        return 1
    print("ИТОГ: все проверки пройдены")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
