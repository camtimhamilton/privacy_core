#!/usr/bin/env python3
"""
Проверка гипотезы: валидны ли российские банковские реквизиты, которые
генерирует Faker (локаль ru_RU)?

ЗАЧЕМ. В отчёте phase-0/03-repos/README.md (раздел 6.2) утверждается, что
Faker('ru_RU') даёт ФОРМАТ российских реквизитов, но НЕ КОНТРОЛЬНЫЙ КЛЮЧ.
Утверждение получено чтением исходного кода:

    faker/providers/bank/ru_RU/__init__.py

        def checking_account(self) -> str:
            account = self.random_element(self.checking_account_codes)
            organization = self.random_element(self.organization_codes)
            currency = self.random_element(self.currency_codes)
            return account + organization + currency + self.numerify("#" * 12)

Последние 12 цифр — случайны, значит контрольный разряд (последняя цифра)
не является функцией остальных. Ожидаемая доля прохождения проверки ~10 %.

Этот скрипт проверяет гипотезу ИЗМЕРЕНИЕМ.

СТАТУС. В среде аудита (2026-10-16) запустить не удалось:
  - `pip install faker` -> externally-managed-environment (PEP 668)
  - `pip install --break-system-packages --user faker` -> Errno 13 Permission denied
    на /home/timofey/.local/lib/python3.12

ЗАПУСК на машине, где faker доступен:

    python3 faker_ru_checksum_probe.py            # 3000 итераций
    python3 faker_ru_checksum_probe.py 20000      # больше итераций

ОЖИДАЕМЫЙ РЕЗУЛЬТАТ (если гипотеза верна):
    checking_account: pass rate ~10 %
    correspondent_account: pass rate ~10 %
    ssn (12 цифр) как ИНН: pass rate ~1 %  (две контрольные суммы)
    passport: контрольной суммы нет -> проверять нечего

Если pass rate окажется существенно выше 10 % — гипотеза НЕВЕРНА, и раздел 6.2
отчёта надо переписать.
"""

from __future__ import annotations

import sys

# --- Алгоритмы контрольных сумм (независимая реализация) --------------------

# Веса для 23-значной строки: последние 3 цифры БИК + 20 цифр счёта.
_ACC_WEIGHTS = [7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1, 3, 7, 1]


def account_control_ok(account: str, bik: str) -> bool | None:
    """Проверка расчётного/корреспондентского счёта по контрольному ключу.

    Стандартный приём: к 20 цифрам счёта слева приписываются последние 3 цифры
    БИК, получается 23 цифры, которые умножаются на фиксированные веса.
    Счёт корректен, если сумма произведений делится на 10 без остатка.
    """
    if len(account) != 20 or len(bik) != 9:
        return None
    if not (account.isdigit() and bik.isdigit()):
        return None
    digits = bik[-3:] + account
    total = sum(int(c) * w for c, w in zip(digits, _ACC_WEIGHTS))
    return total % 10 == 0


def snils_control_ok(snils: str) -> bool | None:
    """Проверка СНИЛС (11 цифр: 9 значимых + 2 контрольных)."""
    digits = "".join(c for c in snils if c.isdigit())
    if len(digits) != 11:
        return None
    total = sum(int(d) * (9 - i) for i, d in enumerate(digits[:9]))
    check = total % 101
    if check == 100:
        check = 0
    return check == int(digits[9:11])


_INN_W10 = [2, 4, 10, 3, 5, 9, 4, 6, 8]
_INN_W11 = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8]
_INN_W12 = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8]


def _inn_digit(digits: str, weights: list[int]) -> int:
    return (sum(int(d) * w for d, w in zip(digits, weights)) % 11) % 10


def inn_control_ok(inn: str) -> bool | None:
    """Проверка ИНН: 10 цифр (юрлицо) или 12 цифр (физлицо)."""
    if not inn.isdigit():
        return None
    if len(inn) == 10:
        return _inn_digit(inn[:9], _INN_W10) == int(inn[9])
    if len(inn) == 12:
        if _inn_digit(inn[:10], _INN_W11) != int(inn[10]):
            return False
        return _inn_digit(inn[:11], _INN_W12) == int(inn[11])
    return None


# --- Основная проверка ------------------------------------------------------


def main() -> int:
    iterations = int(sys.argv[1]) if len(sys.argv) > 1 else 3000

    try:
        from faker import Faker
    except ImportError as exc:  # pragma: no cover
        print("Faker не установлен: %s" % exc)
        print("Установите: pip install faker (или через venv)")
        return 2

    fake = Faker("ru_RU")
    Faker.seed(20261016)

    stats = {
        "checking_account": [0, 0],
        "correspondent_account": [0, 0],
        "ssn_as_inn": [0, 0],
        "snils_attr": [0, 0],
    }
    samples: dict[str, list[tuple[str, str, bool | None]]] = {
        "checking_account": [],
        "correspondent_account": [],
    }

    has_snils = hasattr(fake, "snils")
    has_inn = hasattr(fake, "inn")

    for _ in range(iterations):
        bik = fake.bic()

        acc = fake.checking_account()
        res = account_control_ok(acc, bik)
        if res is not None:
            stats["checking_account"][1] += 1
            stats["checking_account"][0] += res
            if len(samples["checking_account"]) < 3:
                samples["checking_account"].append((bik, acc, res))

        corr = fake.correspondent_account()
        res = account_control_ok(corr, bik)
        if res is not None:
            stats["correspondent_account"][1] += 1
            stats["correspondent_account"][0] += res
            if len(samples["correspondent_account"]) < 3:
                samples["correspondent_account"].append((bik, corr, res))

        ssn = fake.ssn()
        res = inn_control_ok(ssn)
        if res is not None:
            stats["ssn_as_inn"][1] += 1
            stats["ssn_as_inn"][0] += res

        if has_snils:
            res = snils_control_ok(fake.snils())
            if res is not None:
                stats["snils_attr"][1] += 1
                stats["snils_attr"][0] += res

    print("Faker('ru_RU'), итераций: %d, seed=20261016" % iterations)
    print()
    labels = {
        "checking_account": "Faker.checking_account()  против контрольного ключа",
        "correspondent_account": "Faker.correspondent_account() против контрольного ключа",
        "ssn_as_inn": "Faker.ssn() трактуется как ИНН (10/12 цифр)",
        "snils_attr": "Faker.snils() — атрибут",
    }
    for key, (ok, total) in stats.items():
        if total:
            print("  %-58s %5d/%-5d = %5.1f %%"
                  % (labels[key], ok, total, 100.0 * ok / total))
        else:
            print("  %-58s нет данных" % labels[key])

    print()
    if has_inn:
        inns = [fake.inn() for _ in range(3)]
        print("  Faker.inn() доступен, примеры:", inns,
              "валидны:", [inn_control_ok(i) for i in inns])
    else:
        print("  Faker.inn() отсутствует — валидный ИНН Faker не генерирует")

    print("  Faker.snils() %s" % ("присутствует" if has_snils else "отсутствует"))
    print("  Faker.passport_number() примеры:",
          [fake.passport_number() for _ in range(3)],
          "(контрольной суммы у паспорта РФ нет)")

    print()
    for key, rows in samples.items():
        for bik, acc, res in rows:
            print("  пример %-22s БИК=%s СЧЁТ=%s валиден=%s" % (key, bik, acc, res))

    ca = stats["checking_account"]
    if ca[1]:
        rate = 100.0 * ca[0] / ca[1]
        print()
        if 5.0 <= rate <= 15.0:
            print("ВЫВОД: гипотеза подтверждена — контрольный разряд случаен (~10 %%).")
        else:
            print("ВНИМАНИЕ: измеренная доля %.1f %% не согласуется с гипотезой ~10 %%." % rate)
            print("Раздел 6.2 отчёта 03-repos/README.md надо переписать.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
