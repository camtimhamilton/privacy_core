# Прочие open-source решения — детали

Приложение к [README.md](README.md), разделы 5.5 и 5.9.

## 5.5 LLM-прокси и «privacy firewall» проекты

Это самый живой сегмент: `[ПРОВЕРЕНО]` через GitHub Search.

| Репозиторий | ★ | Лицензия | Ключевая деталь |
|---|---|---|---|
| `zeroc00I/DontFeedTheAI` | **665** | MIT | Прозрачный прокси для pentest («strips IPs, credentials, hostnames and PII»), Ollama + regex, per-engagement vault |
| `openbashok/promptzero` | 70 | нет | Прозрачный прокси для Claude API |
| `gbrigandi/mcp-server-conceal` | 18 | MIT | MCP-прокси, «intelligently pseudo-anonymizes PII in real-time» |
| `Athroniaeth/piighost` | 14 | MIT | Reversible masking для агентов (LangChain, Pydantic AI, LlamaIndex, Claude Code) + прокси; 14 868 KB |
| `Aajil-Labs/arabic-pii-py` | 12 | Apache-2.0 | **Арабский + Залив**, local-first — **прямой аналог нашей стратегии для другой юрисдикции** |
| `RPBLC-hq/DAM` | 11 | нет | «local privacy firewall for AI agents» |
| `nikshepsvn/veilstream` | 6 | NOASSERTION | Streaming PII proxy: pluggable detector, per-session vault, крипто |
| `yurivenancio30/llm-dlp` | 5 | MIT | Локальный прокси для Claude Code |
| `whitestag-ai/pii-proxy` | 2 | Apache-2.0 | «GDPR-compliant anonymisation gate for LLM calls» |
| `j911si/local-privacy-gateway` | 1 | MIT | Офлайн-пseudonymization gateway, «reversible tokens» |
| `comptoir-des-signaux/sas-confiance-ia` | 4 | **EUPL-1.2** | Французский: псевдонимизация перед ИИ для территориальных коллективов |

**Что это доказывает.** Ниша «прокси, обезличивающий перед LLM» **уже занята десятком проектов**,
но: **все** либо English-only, либо под одну не-русскую юрисдикцию (арабский/Залив, французский,
нидерландский, Казахстан); **почти все** — про плейсхолдеры/токены, не суррогаты (`piighost` —
«placeholders», `local-privacy-gateway` — «reversible tokens»); **все** на Python/Node, **ни одного
на Rust**; звёзды лидера (665) получены в домене пентеста. **Ниша «русский + суррогаты + один
бинарь» свободна.**


## 5.9 LiteLLM Guardrails

`[ПРОВЕРЕНО]` <https://docs.litellm.ai/docs/proxy/guardrails/pii_masking_v2>:

| Свойство | Значение |
|---|---|
| Провайдер | Microsoft Presidio |
| Сущности | **все типы Presidio** |
| Действия | **`MASK`, `BLOCK`** |
| Режимы | `pre_call`, `during_call`, `post_call`, `logging_only`, `pre_mcp_call` |
| Язык | «Configurable via `presidio_language`»; перечислены **`en` (default), `es`, `de`** |
| Развёртывание | нужны контейнеры Presidio Analyzer + Anonymizer |
| Пороги | `presidio_score_thresholds` с `ALL` и per-entity override |
| Область | `presidio_filter_scope`: `input` / `output` / `both` |
| Крупные тексты | авто-дробление при `/analyze` body > `presidio_analyze_chunk_size_bytes` (default **500 000** байт) |

Дословно про маскирование: «`MASK`: Replace the entity with a **placeholder** (e.g., `<PERSON>`)».
Пример ответа модели подтверждает: `"Hello, <PERSON>! How can I assist you today?"`.

**Важные два факта:**
1. `output_parse_pii` — «this flag only **un-masks tokens back to the originals** after the model
   call; it does not run Presidio detection on outputs». То есть LiteLLM **умеет** восстанавливать
   реальные значения — но через токены, не через суррогаты.
2. Признание, что «analyzer latency grows with payload size» и что `/analyze` body капается
   в 1 MB на дефолтных развёртываниях Presidio — прямое подтверждение нашей проблемы масштаба.

**Лицензия LiteLLM: MIT** `[ПРОВЕРЕНО]` (футер docs: «MIT license», ссылка на LICENSE в
`BerriAI/litellm`).

**Вердикт:** берём как **канал интеграции** (наш OpenAI-совместимый прокси должен подключаться к
таким шлюзам как guardrail), но не как движок. Русский подтверждённо не поддержан (список языков
из `en/es/de`).


---

# scrubadub, DataFog, OpenAnonymiser, pydeidentify/anonymize-it (из §5.1–5.4)

### 5.1 scrubadub

`[ПРОВЕРЕНО]`:

| Параметр | Значение | Источник |
|---|---|---|
| Репозиторий | `LeapBeyond/scrubadub` | GitHub API |
| stars | 433 | GitHub API |
| Лицензия в репозитории | **Apache-2.0** | GitHub API `license.spdx_id` |
| Лицензия на PyPI | **MIT** (`info.license = "MIT"`) | PyPI JSON |
| Последний релиз | **v2.0.1, 2023-09-01** | GitHub Releases |
| Последний push | **2023-09-01** | GitHub API |
| Контрибьюторы | ~9 страниц ≈ 17 человек | пагинация `Link` |
| Версия на PyPI | 2.0.1, загружена 2023-09-01 | PyPI |
| Обязательные зависимости | `textblob==0.15.3`, `phonenumbers`, `python-stdnum`, `dateparser`, `catalogue`, `scikit-learn`, `typing-extensions`, `faker` | PyPI `requires_dist` |

**Расхождение лицензий Apache-2.0 (репо) vs MIT (PyPI)** `[ПРОВЕРЕНО]` — причину не выяснял.
**Русский:** локалей нет; `scrubadub_spacy` (41 ★, Apache-2.0, push 2023-09-01) теоретически
позволяет подсунуть `ru_core_news_*`, но контекстные слова и паттерны английские `[ДОК]`.
**Суррогаты:** `faker` в зависимостях, но по умолчанию возвращаются **теги** (`{{EMAIL}}`) `[ДОК]`.
**Вердикт: отвергаем** — мёртв 3 года, `textblob==0.15.3` жёстким пином, `scikit-learn` без
необходимости, расхождение лицензий, ноль русской поддержки.

### 5.2 DataFog

`[ПРОВЕРЕНО]`:

| Параметр | Значение |
|---|---|
| Репозиторий | `DataFog/datafog-python` |
| stars | 78 |
| Лицензия | **MIT** |
| Последний релиз | **v4.9.0, 2026-09-29** (и v4.9.0a2, 2026-10-05) |
| Последний push | 2026-10-05 |
| Контрибьюторы | ~6 страниц ≈ 11 человек |
| Размер репозитория | 85 309 KB (~83 MB) |
| Русский | `[НЕ НАЙДЕНО]` — русских моделей/локалей не нашёл |
| Суррогаты | `[НЕ НАЙДЕНО]` |

Проект живой (релизы в 2026), но маленький. 83 MB репозитория при 78 звёздах — вероятно, много
данных/весов `[НЕ ПРОВЕРЕНО]`. **Вердикт: наблюдаем, не берём** — нет русского, нет
доказательств суррогатов, нет опубликованной задержки.

### 5.3 OpenAnonymiser

`[ПРОВЕРЕНО]` — фактически это `ConductionNL/anonymiq-light` («OpenAnonymiser Light»):

| Параметр | Значение |
|---|---|
| stars | **0** |
| Лицензия | **EUPL-1.2** |
| Последний push | 2026-10-06 |
| Стек | FastAPI + **Presidio** + spaCy |
| NER | **`nl_core_news_lg`** (нидерландский) везде: локально, контейнер, K8s |
| Паттерны | собственные нидерландские regex-рекогнайзеры |
| Эндпоинты | `POST /api/v1/analyze`, `POST /api/v1/anonymize`, `GET /api/v1/health` |
| Flavors | `classic` (spaCy + NL regex, CPU), `gpu` (**~3 GB** образ: GLiNER + mdeberta), `contextual` (**не реализован**) |
| Прод | `api.openanonymiser.commonground.nu` |
| Суррогаты | Нет — «vervangt PII door placeholders» |

Python-репо `openanonymiser` в GitHub Search **не найден** (`total_count = 0`) `[ПРОВЕРЕНО]` —
то есть «OpenAnonymiser» как отдельный проект не существует, существует `anonymiq-light`.

**Вердикт: отвергаем.** Нидерландский язык, EUPL-1.2 (копилефт-совместимость требует проверки
при линковке), 0 звёзд, `contextual`-flavor не реализован, суррогатов нет.

### 5.4 pydeidentify, deidentify, anonymize-it

`[ПРОВЕРЕНО]`:

| Имя | Что нашлось | Лицензия | Активность | Русский | Вердикт |
|---|---|---|---|---|---|
| `pydeidentify` | `Lucasc-99/pydeidentify`, 1 ★, «simple tool for text deidentification built on spaCy's NER pipeline»; на PyPI **есть** | **нет лицензии** | push 2023-03-21 | нет | Отвергаем |
| `deidentify` (npm) | npm `deidentify@0.2.0` | **MIT** | — | нет | Наблюдать |
| `deidentify` (GitHub, 226 репо) | релевантные: `nedap/deidentify` (149 ★, MIT, медзаписи, push 2025-11-17), `philterd/phileas` (110 ★, Apache-2.0, Java, push 2026-10-14), `pydicom/deid` (180 ★, MIT, медизображения), `Planeshifter/deidentify` (41 ★, GPL-2.0, HIPAA PHI) | разные | — | нет | `phileas` — единственный живой движок, но Java и без ru |
| `anonymize-it` | на PyPI — **404**; GitHub Search выдал только посторонние проекты (`nucleuscloud/neosync` 4 133 ★, `arx-deidentifier/arx` 738 ★ Apache-2.0) | — | — | — | **Не существует** |

**Полезный побочный результат:** `arx-deidentifier/arx` (738 ★, **Apache-2.0**, push 2025-10-01) —
«comprehensive open source data anonymization tool aiming to provide scalability». Это **не** про
текст для LLM, это про табличные данные и k-anonymity. Но лицензия Apache-2.0 и зрелость делают
его референсом по **методологии k-anonymity** `[ДОК]`. И `nucleuscloud/neosync` (4 133 ★) —
«Open Source Data Security Platform … Monitor and Detect PII, Anonymize» — про базы данных, не про
промпты. Оба стоит держать в поле зрения как «соседние рынки, где анонимизация уже commoditised».


---

# SurrogateShield: инженерные решения (из §4.2)

Ключевые решения, которые стоит забрать:

| Решение | Деталь |
|---|---|
| Presets | `balanced` (default), `strict`, `fast`, `classic` |
| Суррогаты с валидной checksum | «fake SSNs pass format checks, fake Bitcoin addresses match Base58 format»; credit_card «Valid Luhn-format number»; `us_bank_number` «Valid 9-digit ABA routing number» |
| Коллизии | `used_surrogates` set, суррогат уникален в сессии |
| Ключ / шифрование | device secret 32 байта, `device.key` `0o600`, каталог `0o700`; AES-256-GCM, свежий 12-байтовый nonce, kind+ID как AAD |
| Битый файл карты | **«Moved aside to `<name>.corrupt-<ts>` … never silently replaced by an empty map»** |
| Fail-closed | «a missing detection model stops the request»; «detection raises `DetectorUnavailable` instead of sending unmasked text» |
| Проверка весов | SHA-256 pin на фиксированный коммит HF |
| Квази-идентификаторы | 10 комбинаций по Sweeney (ZIP+DOB+Gender = «87% of US population uniquely identifiable») |
| Восстановление | «matches **whole values only** ("Lee" never matches inside "Leeds")» — ровно наш I3 |
| Категоризация ошибок | `exact_miss`, `fuzzy_hit`, `fuzzy_miss` |

---

# Строки сводной таблицы §1.1 по мелким решениям

| Решение | Стек | Лицензия | Русский | Суррогаты | Вес | Активность | Вердикт |
|---|---|---|---|---|---|---|---|
| **Ivan Jurin / «Oban»** | — | — | — | — | — | `[НЕ НАЙДЕНО]` как инструмент обезличивания | Отвергаем (не существует в этом качестве) |
| **anonymize-it** | — | — | — | — | — | `[НЕ НАЙДЕНО]` на PyPI (404), npm/GitHub — только чужие проекты | Отвергаем (артефакта нет) |
| **pydeidentify** | Python, spaCy | **без лицензии** `[ПРОВЕРЕНО]` | Нет | Нет | — | 1 ★, push 2023-03-21, на PyPI есть | Отвергаем |
| **deidentify** (npm) | JS | **MIT** `[ПРОВЕРЕНО]` | Нет | Нет | — | npm `deidentify@0.2.0` | Наблюдать |
| **nedap/deidentify** | Python, медицинские записи | **MIT** `[ПРОВЕРЕНО]` | Нет (нидерландский/английский) | Нет | — | 149 ★, push 2025-11-17 | Отвергаем (домен: медицина, не ru) |
| **Phileas** (philterd/phileas) | Java | **Apache-2.0** `[ПРОВЕРЕНО]` | Нет `[НЕ НАЙДЕНО]` | Есть «replacement»-стратегии | JVM | 110 ★, push 2026-10-14 | Наблюдать |
| **mimesis** | Python | **MIT** `[ПРОВЕРЕНО]` | **Да, есть локали ru** `[ДОК]` | **Да, это генератор фейков** | Чистый Python, лёгкий | 4 848 ★, релиз **v22.2.0 от 2026-09-23** | **Берём как источник суррогатов** |
| **Faker** | Python | **MIT** `[ПРОВЕРЕНО]` | Локаль `ru_RU` | **Да** | Чистый Python | 19 447 ★, push 2026-10-09 | **Берём** (валидировать русскую морфологию) |
| **Cloudflare AI Gateway DLP** | Проприетарный облачный | Проприетарная | `[НЕ НАЙДЕНО]` ru-профилей | **Нет** — только FLAG/BLOCK | — | Документация обновлена 2026-09-25 | Отвергаем (нет surrogates, облако) |
| **LiteLLM Guardrails (Presidio)** | Python, MIT | **MIT** | Через `presidio_language`; в документации только `en`, `es`, `de` | **Нет** — `MASK` заменяет на `<PERSON>` | Требует развёрнутых контейнеров Presidio | Официальный guardrail-провайдер | Берём как канал интеграции, не как движок |
| **Microsoft Purview** | Проприетарный | Проприетарная | **Да, 4 SIT:** паспорт РФ (внутр.), паспорт РФ (загр.), адреса РФ, ИНН РФ | **Нет** | — | Документация обновлена 2026-06-15 | Берём как ориентир: primary/secondary + confidence 65/75/85 |
| **Google DLP / Sensitive Data Protection** | Проприетарный | Проприетарная | `[НЕ ПРОВЕРЕНО]` — страницу language support открыть не удалось (404) | **Да — FPE-суррогат** (`dlp-deidentify-free-text-with-fpe-using-surrogate`) | — | Документация обновлена 2026-10-13 | **Берём идею FPE-суррогата**, реализуем локально |
| **AWS Comprehend PII** | Проприетарный | Проприетарная | **Нет.** PII: «Detecting PII entities — English and Spanish» `[ПРОВЕРЕНО]` | Нет | — | — | Отвергаем для ru |
| **AWS Bedrock Guardrails** | Проприетарный | Проприетарная | `[НЕ НАЙДЕНО]` ru в списке PII | **Маска = плейсхолдер** `{NAME}`, `{EMAIL}` — **не суррогат** | — | — | Ориентир по видам сущностей |

