# Русскоязычные инструменты: полные таблицы

Приложение к [README.md](README.md), раздел 6. Веса, активность, данные и качество pii-guard, EN-результаты, инженерные решения казахстанского прокси.

---

# pii-guard: данные и качество (из §6.2)

## Per-entity F1 модели rubert-base-pii-ner (baseline для нас) `[ПРОВЕРЕНО]`:

| Сущность | gold | F1 | | Сущность | gold | F1 |
|---|---|---|---|---|---|---|
| FIRST_NAME | 499 | 89.8 | | IP_ADDRESS | 155 | **36.0** |
| LAST_NAME | 457 | 84.5 | | PASSPORT | 494 | 81.3 |
| MIDDLE_NAME | 308 | 91.5 | | INN | 263 | **68.3** |
| COUNTRY | 242 | 86.1 | | SNILS | 223 | 78.5 |
| REGION | 176 | 80.3 | | OMS | 170 | 90.1 |
| CITY | 339 | 82.5 | | CREDIT_CARD | 203 | 87.9 |
| DISTRICT | 161 | 78.3 | | DRIVER_LICENSE | 371 | 81.8 |
| STREET | 171 | 87.3 | | MILITARY_ID | 278 | 78.3 |
| HOUSE | 163 | 92.7 | | BIRTH_CERTIFICATE | 341 | 83.7 |
| EMAIL | 221 | 97.6 | | | | |
| PHONE | 173 | 86.9 | | | | |
| URL | 206 | 89.7 | | | | |

---

# Дополнительные таблицы раздела 6

**Данные** `[ПРОВЕРЕНО]`:

| Артефакт | Значение |
|---|---|
| NER-модель | `redmadrobot-rnd/rubert-base-pii-ner`, ruBert-base, 178M, веса **678 MiB** |
| Обучение | `redmadrobot-rnd/pii_train` — **17 137 предложений, 39 687 спанов** |
| Оценка | `redmadrobot-rnd/pii_benchmark` — **2 841 предложение**, disjoint от train |
| Метод обучения | 10 эпох, lr 3e-5 linear без warmup, batch 16, AdamW wd 0.01, fp32, max_len 512, seed 42 |
| Инференс | скользящее окно 512 подтокенов, **stride 128**, `min_confidence=0.3` |
| Слияние фрагментов | **обязательно**: без него `elena@pochta.ru` распадается на `el` + `ena@pochta.ru` |

**Результаты по английскому:**

| Конфигурация | overall F1 | F1 по `NAME_STUDENT` | token-F5 |
|---|---|---|---|
| `regex_only` | 0.586 | **0.000** | 0.386 |
| `regex+spacy` | **0.745** | 0.728 | 0.727 |
| `regex+spacy_aggr` | 0.703 | 0.637 | 0.765 |

Другие инженерные решения, которые надо забрать:

| Решение | Деталь |
|---|---|
| **Солёные плейсхолдеры** | `[KIND_<10 hex>]`, хэш — **blake2b с перцем из окружения**: «An unsalted hash of a 12-digit BIN is **brute-forceable in minutes**». Нет перца → сервис **отказывается стартовать** |
| Согласованность | «Same value → same placeholder across requests» — реализация нашего I5 через сервис |
| Разрешение перекрытий | сортировка по confidence → приоритет вида → длина; **пороги ORG/LOC подняты (≥ 0.85, длина ≥ 5)** |
| Стоп-лист по леммам | «The stop-list compares **lemma sequences**, and a span is exempt only when **all** of its significant words are covered» — «Закону Республики Казахстан» читаем в любом падеже, «ТОО «Консультант Плюс»» маскируется |
| **SSE-safe восстановление** | `StreamRewriter` держит буфер до последней незакрытой `[` — «no torn placeholders and no added latency» |
| Fail-closed | `ANONYMIZE_ENABLED=false` только вместе с `ALLOW_PASSTHROUGH_FOR_TESTS` |
| Логи | «audit log stores only placeholder kinds and counts, **never values**» — наш I10 |
| Усечение до анонимизации | «Tool outputs … are truncated **before** anonymization — one search result once returned **5.99 M characters**» |
| Деградация | «If Redis is down, **restoration degrades but anonymization keeps working** — nothing leaks» |
| Тексты | «split on sentence boundaries and anonymized **in full** rather than truncated» |

