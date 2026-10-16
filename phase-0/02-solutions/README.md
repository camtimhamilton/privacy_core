# Аудит решений: Presidio, LLM Guard и аналоги

**Статус:** раздел 2 аудита фазы 0 — технический аудит готовых решений и библиотек
**Дата прохода:** 2026-10-16
**Область:** только техника и лицензии. Научные статьи — не здесь. Юридическая часть — не здесь.
**Метод:** каждая проверяемая страница открывалась напрямую (GitHub API, raw-файлы, PyPI JSON,
официальная документация). Метки: `[ПРОВЕРЕНО]` — открыл источник, `[ДОК]` — только по
документации/аннотации, `[НЕ ПРОВЕРЕНО]` — заявлено, но не подтверждено, `[НЕ НАЙДЕНО]` — искал, нет.
**Сводные таблицы в машинночитаемом виде:** см. `01-summary.md` и `02-presidio-tables.md`.

---

## 1. Сводная таблица

### 1.1 Главные фигуранты

| Решение | Язык / стек | Лицензия | Русский | Суррогаты | Вес | Активность (на 2026-10-16) | Вердикт |
|---|---|---|---|---|---|---|---|
| **Microsoft Presidio** (→ data-privacy-stack/presidio) | Python, spaCy/Stanza/transformers | **MIT** `[ПРОВЕРЕНО]` LICENSE «The MIT License (MIT), Copyright (c) Presidio Contributors» | **Нет ни одного ru-рекогнайзера**; `default_recognizers.yaml` → `supported_languages: [en]` | **Только через Azure AHDS** (`surrogate_ahds`); локально — `custom` + Faker вручную | `presidio-analyzer` тянет spaCy; дефолтная модель `en_core_web_lg` = **382.1 MB** | 11 345 ★, 206 контрибьюторов, релиз **2.2.364 от 2026-07-22**, репозиторий живой (push 2026-10-09) | Берём как референс архитектуры и API, не берём как зависимости |
| **LLM Guard** (protectai) | Python | **MIT** `[ПРОВЕРЕНО]` | **Нет.** Документация: «Current entity detection functionality is English-specific», поддержаны «English and Chinese» | Есть флаг `use_faker=True` (Faker), но по умолчанию — `[REDACTED_CUSTOM_1]`-подобные плейсхолдеры | 28 зависимостей; p95 **294.57 ms** (m5.xlarge), **169.13 ms** с ONNX — на входе 317 символов | **АРХИВИРОВАН 2026-07-08** `[ПРОВЕРЕНО]`; последний релиз PyPI 0.3.16 от 2025-05-19 | Отвергаем как зависимость; берём идею «набор сканеров + Vault» |
| **SurrogateShield** (sherwinvishesh) | Python, DeBERTa+spaCy+Faker | **Apache-2.0** `[ПРОВЕРЕНО]` | **Нет.** Всё заточено под en-US: SSN, ZIP, ABA, `en_core_web_lg`, `ADDRESS_MODE` для США | **Да** — это ядро проекта (MimicGen на Faker, формат-сохраняющие суррогаты, Luhn/ABA-валидные) | PII-теггер **~565 MB** + spaCy lg + опц. **~250 MB** distilbert; пик памяти **1 348 MB** | 2 ★, 0 форков, создан 2026-05-13, push 2026-10-16; библиотека 1.9.0 | **Главный конкурент по идее.** Берём архитектурные приёмы, отвергаем как код (язык, зрелость, self-reported бенчи) |
| **Portcullis** (ByteDance/AAAI-25) | — | — | — | — | — | **Кода нет** `[НЕ НАЙДЕНО]` | Только идея: параллельная подстановка + TEE + аттестация. Берём термин «доверенный шлюз», не берём реализацию |
| **pii-guard** (redmadrobot-rnd) | Python 3.10+, Presidio+spaCy+PyTorch | **Apache-2.0** `[ПРОВЕРЕНО]` README + бейдж; веса модели тоже Apache-2.0 | **Да, это русский продукт.** **20 типов**, включая ИНН, СНИЛС, ОМС, паспорт РФ, военный билет, свидетельство о рождении, БИК, расчётный счёт | **Нет.** Только `mask` / `tag` / `pseudonymize` (`<PII type="PERSON" gender="male" id="1" />`) | Веса NER: **678 MiB** safetensors, 178M параметров; движок = Presidio AnalyzerEngine + PyTorch | README живой, есть статья на habr и redmadrobot.ai; на PyPI `pii-guard 0.1.0` с битым `YOUR_USERNAME` в метаданных `[ПРОВЕРЕНО]` | **Главный русский конкурент.** Берём: правила с контрольными суммами, «правило важнее модели», дистанции до ключей, восстановление падежа |
| **Natasha / Slovnet** | Python, NumPy | **MIT** `[ПРОВЕРЕНО]` (PyPI) | **Да.** NER PER/LOC/ORG, морфология, синтаксис, извлечение фактов | Нет (не про суррогаты) | NER-модель **2 MB**, весь диск **27 MB**, RAM **205 MB** `[ПРОВЕРЕНО]` | 1 359 ★, последний тег **v1.6.0 (PyPI 2023-07-24)**, push 2026-10-14 (зависимости) | **Берём.** Самый дешёвый вход в русский NER на CPU |
| **redmadrobot-rnd/rubert-base-pii-ner** | Python/PyTorch (HF) | **Apache-2.0** `[ПРОВЕРЕНО]` HF API | **Да.** 21 тип / 43 BIO-метки | Нет | 178M параметров, ruBert-base, max_len 512, stride 128 | 1 764 загрузок, 6 лайков; датасет-бенчмарк 23 лайка | **Берём как baseline и как датасет для сравнения** |
| **redmadrobot-rnd/pii_benchmark** | CSV, ru | **MIT** `[ПРОВЕРЕНО]` HF | **Да.** 2 841 предложение, 14+ категорий, disjoint от train | — | 1K–10K строк | 597 загрузок, 23 лайка | **Берём как внешний замер** (обязательно: disjoint с нашим dev) |
| **scrubadub** | Python | **Apache-2.0** (репо) / **MIT** (PyPI) `[ПРОВЕРЕНО]` — расхождение | Только en-локали (`scrubadub_spacy`) | Есть Faker в зависимостях, суррогатов по умолчанию нет | Тянет `textblob==0.15.3`, `scikit-learn`, `dateparser` | 433 ★, **релиз v2.0.1 от 2023-09-01**, push 2023-09-01 | **Отвергаем** — мёртвый с 2023 |
| **DataFog** (DataFog/datafog-python) | Python | **MIT** `[ПРОВЕРЕНО]` | Нет данных о ru-поддержке `[НЕ НАЙДЕНО]` | Нет данных | Репо 85 MB | 78 ★, релиз **v4.9.0 от 2026-09-29**, push 2026-10-05 | Наблюдать, не брать |
| **OpenAnonymiser** (ConductionNL/anonymiq-light) | Python, FastAPI+Presidio | **EUPL-1.2** `[ПРОВЕРЕНО]` | **Нет** — `nl_core_news_lg`, нидерландские регексы | Нет, только `[PLACEHOLDER]` | classic = spaCy CPU; gpu-образ **~3 GB** (GLiNER+mdeberta) | 0 ★; репо «Anonymiq Light», push 2026-10-06; прод-эндпоинт `api.openanonymiser.commonground.nu` | Отвергаем (язык, лицензия EUPL-1.2, зрелость) |
| **llm-pii-proxy-showcase** (MihaRS2) | Python 3.12, FastAPI, Presidio, spaCy ru+en, pymorphy3, Redis, Postgres | **MIT** `[ПРОВЕРЕНО]` | **Да** — `ru_core_news_md` + pymorphy3 + любые падежи ФИО | **Нет** — солёные плейсхолдеры `[KIND_<10hex>]` | Presidio + spaCy ru_md + Redis + Postgres | 0 ★, showcase, push 2026-09-21 | **Берём инженерные решения** (SSE-safe восстановление, fail-closed, overlap resolution) |
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

### 1.2 Одной строкой: чего не делает никто

| Возможность | Кто делает | Кто не делает |
|---|---|---|
| Русские форматные ID с контрольной суммой | pii-guard (20 типов), llm-pii-proxy (KZ), rubert-pii-ner (модель) | Presidio (0 ru), LLM Guard, SurrogateShield, все облака кроме Purview |
| **Реалистичные суррогаты локально** | SurrogateShield (en), Google DLP FPE (облако) | Presidio (только Azure), LLM Guard (плейсхолдеры), pii-guard (теги), Bedrock (`{NAME}`) |
| Суррогаты **с валидной контрольной суммой** | SurrogateShield (Luhn/ABA), Google DLP FPE | Presidio, LLM Guard, pii-guard, Bedrock |
| **Русский + суррогаты + локально** | **никто** | — |
| Разрешение ссылок/анафоры в диалоге | **никто** из проверенных | Presidio, LLM Guard, pii-guard, SurrogateShield |
| Восстановление **в правильной падежной форме** | pii-guard (pymorphy3 + spaCy) | все остальные |
| **Один бинарь / Rust / WASM / p95 ≤25 ms** | **никто** | все: Python (кроме `phileas` на Java) |

---

## 2. Presidio — детальный разбор

### 2.1 Что произошло с проектом (важно в первую очередь)

Presidio **больше не проект Microsoft**. Официальная страница
[Project transition](https://presidio.dataprivacystack.org/project_transition/) `[ПРОВЕРЕНО]`:

> «The Presidio project has completed its transition from a Microsoft-owned project to an
> independent, community-governed open source project under the GitHub organization Data Privacy Stack.»

Что это значит практически:

- канонический репозиторий — <https://github.com/data-privacy-stack/presidio> (старые ссылки
  на `microsoft/presidio` редиректят; GitHub API на `microsoft/presidio` возвращает
  `Moved Permanently` `[ПРОВЕРЕНО]`);
- Docker-образы переехали с `mcr.microsoft.com/presidio-*` на
  `ghcr.io/data-privacy-stack/presidio-*`; старые теги в MCR больше не обновляются;
- лицензия осталась MIT, но копирайт в LICENSE изменён с «Microsoft Corporation» на
  «Presidio Contributors» (CHANGELOG, PR #2134) `[ПРОВЕРЕНО]`;
- автор в метаданных PyPI теперь `presidio@dataprivacystack.org` (PR #2128) `[ПРОВЕРЕНО]`;
- governance — Technical Steering Committee из сообщества.

**Следствие для нас.** Аргумент «мы делаем то, что Microsoft не делает» устарел: Microsoft
отошла, проект держит сообщество. Зато появился новый аргумент: у community-проекта нет
интереса вкладываться в русскую юрисдикцию, а у нас это приоритет №1. И ещё: Presidio
теперь можно форкнуть и улучшать без разговора с корпорацией.

### 2.2 Архитектура

Три пакета, разделённые по ответственности `[ПРОВЕРЕНО]` (docs/anonymizer/index.md, pyproject):

Три пакета `[ПРОВЕРЕНО]` (docs/anonymizer, pyproject): `presidio-analyzer` (находит PII, отдаёт
спаны + score + entity_type; `AnalyzerEngine`, `RecognizerRegistry`, `NlpEngine`),
`presidio-anonymizer` (`AnonymizerEngine`, `DeanonymizeEngine`, `Operator`), плюс
`presidio-image-redactor` (DICOM/OCR) и `presidio-structured`.

Конвейер Analyzer:

```
текст
  → NlpEngine (spaCy/Stanza/transformers/NoOp)         # токенизация, леммы, NER
  → NlpArtifacts (сущности, леммы, токены, scores)
  → все зарегистрированные Recognizer'ы                 # pattern recognizer'ы + NER-обёртка
  → LemmaContextAwareEnhancer                          # контекстные слова повышают score
  → список RecognizerResult(entity_type, start, end, score)
```

Ключевая деталь, которую мы обязаны скопировать: **контекстное усиление отделено от детекции**.
`LemmaContextAwareEnhancer` берёт контекстные слова рекогнайзера и повышает score, если они
есть в окне. В версии 2.2.361 добавлен `context_matching_mode` со значениями `substring` и
`whole_word` — исправление бага, когда `lic` матчилось внутри `duplicate` `[ПРОВЕРЕНО]` (CHANGELOG).
Это ровно наш L1-каскад, но примитивнее: у нас требуется отрицательная метка, у Presidio — нет.

### 2.3 Полный перечень рекогнайзеров и таблица «тип → русский»

Полные таблицы (по 20 регионам и покомпонентно по всем ≈130 типам) — в
[`02-presidio-tables.md`](02-presidio-tables.md). Сводка `[ПРОВЕРЕНО]`: **Global — 14 типов**
(CREDIT_CARD, CRYPTO, DATE_TIME, EMAIL_ADDRESS, IBAN_CODE, IP_ADDRESS, MAC_ADDRESS, NRP, LOCATION,
PERSON, PHONE_NUMBER, MEDICAL_LICENSE, URL, UUID); **USA — 13**; **South Africa — 10**;
**UK — 6**; **India — 6**; **Italy — 5**; **Korea — 5**; **Australia — 4**; **Spain — 3**;
**Philippines — 3**; **Singapore / Nigeria / Canada / Sweden / Turkey — по 2**;
**Finland / Poland / Thai — по 1**; **Germany — 9–13**; **Medical / Clinical — 8**
(через `MedicalNERRecognizer`). **Итого ≈ 130 predefined-типов, из них с русским — 0.**
Германия получила **цельный национальный набор** (13 рекогнайзеров одной PR-серией, CHANGELOG
2.2.363, автор @MvdB) — «добавить юрисдикцию» в Presidio это пакетный вклад на 12–13 типов.
**Русских рекогнайзеров нет ни одного:** в `default_recognizers.yaml` поле верхнего уровня
буквально `supported_languages: - en` `[ПРОВЕРЕНО]`; `kr` — устаревший алиас для корейского.

**Таблица «тип сущности → поддержка русского»** (сжато по группам):

**Форматные глобальные** (CREDIT_CARD, IBAN_CODE, IP_ADDRESS, MAC_ADDRESS, EMAIL_ADDRESS, URL,
UUID) — pattern (+ checksum/RFC/mod-97): **работают на русском** (Luhn и mod-97 от языка не зависят),
но **русских контекстных слов нет** (`credit, card, visa…` только en/es/it/pl) → score не поднимется
→ FP на безобидных номерах. **CRYPTO** — pattern + context + checksum, частично (только Bitcoin).
**DATE_TIME** — `DateRecognizer` (regex): да для ISO/числовых форматов, «12 марта 2024 г.» не
распознаётся `[НЕ ПРОВЕРЕНО]`. **PHONE_NUMBER** — `phonenumbers` + регионы: да, но реестр регионов
реально ломался (`"UK"` был no-op, исправлено на `"GB"` только в 2.2.363) `[ПРОВЕРЕНО]`.
**NRP, LOCATION, PERSON** — NER spaCy: **нет из коробки**, только при подмене `NlpEngine` на ru;
класса NORP нет в `ru_core_news_*`, с ru-моделью доступны лишь PER/LOC/ORG. **MEDICAL_LICENSE** —
pattern + context + checksum, US-ориентирован. **13 US-типов** — формально да, но бесполезны и дадут FP
на любых 9-значных числах. **6 UK-типов** — формально да. **ES/IT/PL/FI/SE/TR/TH/KR/NG/PH/CA/ZA/AU/IN/SG**
— формально да, контекст на своих языках (для русского не сработает). **13 DE-типов** — да (немецкие
контекстные слова): бесполезны для русского, но **образец качества** (ISO 7064 Mod 11,10, DRV-алгоритм,
GKV-алгоритм) `[ПРОВЕРЕНО]`. **8 MEDICAL-типов** — `MedicalNERRecognizer` → `blaze999/Medical-NER`,
модель английская.

**Два русских дефицита разной природы.** (1) **Форматные типы работают, но без русской семантики:**
Luhn и mod-97 не знают языка, но контекстные слова — основной механизм борьбы с FP в Presidio — для
русского отсутствуют полностью. Наши `checksum_plus_context` типы (ИНН юрлица, ОГРН, ОГРНИП, БИК,
карта, телефон) в Presidio стали бы `checksum_only` с измеренной вероятностью ложного прохождения
9.9–10% по нашему реестру. (2) **NER-типы (PERSON, LOCATION, NRP, ORGANIZATION) недоступны без замены
NLP-движка**, а русская модель даёт только `PER`, `LOC`, `ORG` — то есть даже после подмены на
`ru_core_news_lg` мы не получим ни адрес, ни паспорт, ни ИНН.


### 2.5 Языки: что реально поддержано

`[ПРОВЕРЕНО]` `docs/analyzer/languages.md`:

> «In its default configuration, it contains recognizers and models for English.
> To extend Presidio to detect PII in an additional language, these modules require modification:
> 1. The `NlpEngine` … 2. PII recognizers … should be adapted or created.»
>
> «While different detection mechanisms such as regular expressions are language agnostic, the
> context words used to increase the PII detection confidence aren't.»

То есть **русского нет и механизма «просто включить» тоже нет**: надо скачать чужую NLP-модель,
написать конфиг и **создать инстанс каждого рекогнайзера с `supported_language="ru"`** и своим
списком контекстных слов.

Дефолтный движок `[ПРОВЕРЕНО]` `conf/default.yaml`: `spacy`, `lang_code: en`,
`model_name: en_core_web_lg`; маппинг `PER/PERSON→PERSON`, `NORP→NRP`, `FAC/LOC/GPE/LOCATION→LOCATION`,
`ORG→ORGANIZATION`, `DATE/TIME→DATE_TIME`; **`low_confidence_score_multiplier: 0.4`**;
**`labels_to_ignore` включает `ORGANIZATION`** с комментарием «Has many false positives».
Множитель 0.4 — признание, что NER-score не равен уверенности. Это честная инженерия, которую
надо повторить.

Доступные NLP-движки `[ПРОВЕРЕНО]` (`docs/analyzer/nlp_engines/`): `SpacyNlpEngine` — дефолт и
**обязательная зависимость** (`spacy>=3.4.4,<4.0.0`); `StanzaNlpEngine` (extra `stanza`);
`TransformersNlpEngine` (extra `transformers`, для ru — напр. `ai-forever/ruBert-base`);
`SlimSpacyNlpEngine` (2.2.363: только токенизация/леммы, NER делегируется, напр. GLiNER);
`NoOpNlpEngine` (unreleased: без NLP-артефактов); `GLiNERRecognizer` (extra `gliner`+`onnxruntime`,
ONNX-бэкенд 2.2.362 «crashes on CPUs without AVX2»). Список языков из документации LiteLLM-интеграции
(проксирует Presidio) — только `en`, `es`, `de` `[ПРОВЕРЕНО]`; не официальный список Presidio, но показательно.

**Сколько стоит русский в Presidio по весам.** `[ПРОВЕРЕНО]` (GitHub Releases `explosion/spacy-models`):

| Модель | Размер wheel | Комментарий |
|---|---|---|
| `en_core_web_lg` (дефолт Presidio) | **382.1 MB** | уже тянется по умолчанию |
| `ru_core_news_sm` | **14.6 MB** | NER: LOC/ORG/PER |
| `ru_core_news_md` | **39.9 MB** | + векторы |
| `ru_core_news_lg` | **489.6 MB** | + векторы |
| `slovnet_ner_news_v1.tar` (Natasha) | **2 MB** | NER: PER/LOC/ORG, диск 27 MB |

Разница между `ru_core_news_lg` (489.6 MB) и Slovnet (2 MB) — **245×**. Это и есть наш аргумент
против «просто подключить ru-модель spaCy»: она в 245 раз тяжелее при сопоставимом качестве
на новостном домене (см. §6, где `regex+spacy_ru` F1 0.897 против `slovnet` F1 0.959 PER на factRuEval).

### 2.6 Anonymizer operators: что делает → даёт ли реалистичный суррогат

`[ПРОВЕРЕНО]` `docs/anonymizer/index.md`. Это ключевая таблица задания.

| Operator | Что делает | Параметры | Формат-сохраняющий? | Валидная контрольная сумма? | Реалистичный суррогат? | Обратим? |
|---|---|---|---|---|---|---|
| `replace` | Меняет PII на заданное значение | `new_value` | Нет | Нет | **Нет** — по умолчанию `<ENTITY_TYPE>`, то есть `<PHONE_NUMBER>` | Нет |
| `redact` | Удаляет PII полностью | — | — | — | Нет | Нет |
| `mask` | Заменяет N символов символом | `chars_to_mask`, `masking_char`, `from_end` | Частично (длина сохраняется) | Нет | **Нет** — `****1234` | Нет |
| `hash` | Солёный SHA-256/512 | `hash_type`, `salt` | Нет (длина меняется) | Нет | Нет | **Только при известном salt** |
| `encrypt` | Шифрует значение ключом | `key` | Нет | Нет | Нет | **Да** (`decrypt`) |
| `custom` | Запускает lambda на PII | `lambda` | **Зависит от lambda** | **Зависит от lambda** | **Возможно, вручную** | Нет по умолчанию (нужен свой mapping) |
| `surrogate_ahds` | «Generate realistic, medically-appropriate surrogates» через Azure Health Data Services | `endpoint`, `entities`, `input_locale` (default `en-US`), `surrogate_locale` (default `en-US`) | Да | Не заявлено | **Да, но только в облаке Azure** | Нет |
| `keep` | Оставляет PII нетронутым, но фиксирует позицию | — | — | — | Нет | — |
| `decrypt` (Deanonymize) | Расшифровывает `encrypt` | `key` | — | — | — | Да |

**Ответ на вопрос задания «can Presidio generate realistic surrogates?»** — **да, но только через
Azure**: `surrogate_ahds` требует `pip install presidio-anonymizer[ahds]`,
`azure-health-deidentification>=1.1.0b1` + `azure-identity` `[ПРОВЕРЕНО]` pyproject. Локально,
офлайн, на CPU — **нет**. Единственный офлайновый путь — `custom` с lambda, и в репозитории есть
ровно один пример такого рода: `example_custom_lambda_anonymizer.py` («Passing a lambda as a
Presidio anonymizer using Faker») `[ДОК]` — по названию файла из списка samples. Это не продуктовая
функция, а пример, который нам придётся написать самим со всей валидацией контрольных сумм,
коллизиями и сессионной согласованностью.

**Три вещи, которые Presidio сделал в анонимайзере правильно** `[ПРОВЕРЕНО]`: (1) **правила
разрешения перекрытий описаны явно** — нет перекрытия → обычная замена; полное → высший score; одна
сущность внутри другой → берётся **более длинная, даже если score ниже**; частичное пересечение →
обе анонимизируются отдельно и результат конкатенируется (`<PERSON><LOCATION>`), и Presidio не
скрывает, что это источник мусора. (2) **Hash по умолчанию перешёл на случайную соль (2.2.361):**
«Same PII value gets different hashes across different entities and calls (unless you provide a
salt)» — ломающее изменение, но правильное по безопасности и прямо противоречащее нашему I5; вывод:
**нельзя брать `hash` как модель для наших подмен**, нужен детерминированный генератор.
(3) **Referential integrity признана ответственностью вызывающего:** «Presidio does not store or
maintain stateful sessions» — сессионную согласованность Presidio **не решает вообще**, это наша
незанятая ниша.

### 2.7 Зависимости и вес

`[ПРОВЕРЕНО]` `presidio-analyzer/pyproject.toml` (version **2.2.364**, `license = "MIT"`):

Обязательные: `spacy (>=3.4.4,!=3.7.0,<4.0.0)`, `numpy (<2.5.0)`, `click`, `regex`, `tldextract`,
`pyyaml`, `phonenumbers (>=9.0.28)`, `pydantic (>=2.12.5)` `[ПРОВЕРЕНО]`.

**`spacy` — обязательная зависимость** (хотя `NoOpNlpEngine` уже есть, но в unreleased). То есть
«ядро без ML» в Presidio невозможно: `pip install presidio-analyzer` всегда тянет spaCy и требует
скачать spaCy-модель. Прямое противоречие нашему запрету «нет обязательной ML-зависимости».

Опциональные extras `[ПРОВЕРЕНО]`: `server` (flask/gunicorn/waitress), `transformers`, `stanza`,
`azure-ai-language`, `ahds`, `gliner`, `langextract`.

Вес типичной установки: spaCy + `en_core_web_lg` (**382.1 MB**) + numpy + pydantic +
`phonenumbers` + `tldextract` — оценочно **≥ 500 MB** на диске без трансформеров `[НЕ ПРОВЕРЕНО,
оценка]`. Slovnet NER для сравнения — **2 MB** весов, **27 MB** диск, **205 MB RAM** `[ПРОВЕРЕНО]`.

### 2.8 Задержка и активность

Presidio **не публикует** замеров задержки `[НЕ НАЙДЕНО]`. Косвенно: `bench.perf_arms`
SurrogateShield `[ПРОВЕРЕНО, self-reported]` — Presidio **медиана 8.2 ms** и пик **1 084 MB**
(+Faker 8.4 ms/1 010 MB; +RoBERTa **83.5 ms**/1 611 MB) на коротких сообщениях; LiteLLM —
«analyzer latency grows with payload size» и дробление при >500 000 байт; LLM Guard — p95
**294.57 ms** (m5.xlarge) и **169.13 ms** с ONNX при входе **317 символов**. **Ключевое:** 8.2 ms —
это **медиана, не p95**, на коротком тексте, от одного автора; 294.57 ms на 317 символах намекает
на нелинейное масштабирование. Наш контракт — p95 25 ms на 2 000 токенов → `[НЕ ПРОВЕРЕНО]` до
собственного замера.

`[ПРОВЕРЕНО]` GitHub API `data-privacy-stack/presidio`, 2026-10-16: **11 345 ★** / 1 349 форков /
99 watchers / 188 open issues / **206 контрибьюторов**; создан 2018-05-04, последний push
2026-10-09; размер 339 965 KB (~332 MB); не архивирован; **MIT**. Релизы: **2.2.364 — 2026-07-22**;
2.2.363 — 2026-06-28; 2.2.362 — 2026-03-18; 2.2.361 — 2026-02-12; 2.2.360 — 2025-09-09 (4 релиза
за 8 месяцев).

### 2.10 Где именно слабое место Presidio для русского

1. **Ноль русских рекогнайзеров при ≈130 типах** — ни ИНН, ни СНИЛС, ни ОГРН, ни паспорта РФ,
   ни адреса РФ, ни российского телефона как отдельного типа с контекстом.
2. **Русский NER недоступен «из коробки»** — нужна замена `NlpEngine` + отдельные инстансы
   рекогнайзеров с русскими контекстными словами (документация это прямо требует).
3. **Контекстные слова — только en/es/it/pl/de/ko/sv/th/tr/fi и др.**; русских нет ни в одном
   рекогнайзере. Для наших `checksum_plus_context` типов фатально: без контекста ~10% прохождения
   случайных цифр (20-entity-registry §2).
4. **Суррогатов локально нет**: `surrogate_ahds` — облако Azure, локали по умолчанию `en-US`.
5. **Нет состояния сессии** — ни I5, ни I2, ни восстановления по карте как первоклассной функции.
6. **Обязательная зависимость от spaCy** противоречит нашему «ядро без ML».
7. **Вес**: 382 MB дефолтная модель + spaCy против 27 MB Natasha.
8. **`presidio-cli` не публикуется** (CHANGELOG 2.2.356) — CLI не продуктовый.
9. **Язык — Python** — для нашего p95-контракта и WASM-цели неприменимо.

---

## 3. LLM Guard

### 3.1 Главное: проект архивирован

`[ПРОВЕРЕНО]` GitHub API, 2026-10-16: `archived = True`, 3 216 ★, 466 форков, 38 open issues,
`license = MIT`, `pushed_at = 2026-07-08`. Релизов в API **нет вообще** (пустой список);
последний на PyPI — **0.3.16 от 2025-05-19**; ~41 контрибьютор (21 страница пагинации).

И в начале `README.md`:

> **THIS PROJECT HAS BEEN ARCHIVED.**
> This project and its associated models on [Hugging Face](https://huggingface.co/protectai)
> are no longer under active development or maintained.

**Не только код, но и модели на HF** больше не поддерживаются — а LLM Guard построен на
HF-моделях для большинства сканеров. Protect AI куплен Palo Alto Networks; архивация — следствие.
Для нас это первый и главный факт: **брать как зависимость нельзя.**

### 3.2 Что это такое, архитектура, связь с Presidio

`[ПРОВЕРЕНО]` README: «The Security Toolkit for LLM Interactions». Конвейер **сканеров** на входе
(`Prompt scanners`) и на выходе (`Output scanners`); каждый возвращает
`(sanitized_text, is_valid, risk_score)`. Разворачивается как библиотека, sidecar-API (FastAPI),
Docker.

**Полный список сканеров** `[ПРОВЕРЕНО]` README: **Prompt scanners (15)** — Anonymize, BanCode,
BanCompetitors, BanSubstrings, BanTopics, Code, Gibberish, InvisibleText, Language, PromptInjection,
Regex, Secrets, Sentiment, TokenLimit, Toxicity; **Output scanners (21)** — BanCode, BanCompetitors,
BanSubstrings, BanTopics, Bias, Code, **Deanonymize**, JSON, Language, LanguageSame, MaliciousURLs,
NoRefusal, ReadingTime, FactualConsistency, Gibberish, Regex, Relevance, Sensitive, Sentiment,
Toxicity, URLReachability. **Итого 36 сканеров**, из которых к нашей задаче относятся ровно два
(`Anonymize`, `Deanonymize`): **LLM Guard — не библиотека обезличивания.**

**Связь с Presidio — точная** `[ПРОВЕРЕНО]` документация Anonymize:

> «**Integration with Presidio Analyzer**: Leverages the Presidio Analyzer library, crafted with
> spaCy, flair and transformers libraries, for precise detection of private data.»

То есть `Anonymize` — **обёртка над Presidio Analyzer** плюс: свой список сущностей по умолчанию
(`anonymize.py#L26-L40`), свои regex-паттерны в `llm_guard/resources/sensisitive_patterns.json`
(опечатка `sensisitive` — в самом репозитории), класс `Vault`, выбор NER через `recognizer_conf`.
**Следствие: все русские дефициты Presidio наследуются, и к ним добавляются свои.**

### 3.3 Суррогаты: `use_faker` и `Vault`

`[ПРОВЕРЕНО]` параметры сканера: `vault` (архив вырезанного для восстановления), `preamble`,
`allowed_names`, `hidden_names` («Transforms specified names to formats like `[REDACTED_CUSTOM_1]`»),
`entity_types`, `regex_pattern_groups_path`, `recognizer_conf`, `threshold` (default `0`), `language`
(«Default is `en`») и **`use_faker`** — «Substitutes **eligible** entities with fabricated data».
**Ответ про суррогаты.** Флаг есть, но: (1) **не по умолчанию** — дефолтный путь даёт плейсхолдеры
`<PERSON>`/`[REDACTED_CUSTOM_1]`; (2) «for **eligible** entities» — критерий eligibility не раскрыт
`[ДОК]`; (3) Faker не умеет **форматно-валидные** российские ID (не знает про mod-11 ИНН или mod-13
ОГРНИП). Восстановление — `Deanonymize` через `Vault`.

### 3.4 Модели, вес, задержка

Рекомендованные модели NER `[ПРОВЕРЕНО]` («Tailored recognizers»): **`dslim/bert-base-NER`**
(«Top Pick»), `dslim/bert-large-NER`, `gyr66/bert-base-chinese-finetuned-ner`,
`Isotonic/distilbert_finetuned_ai4privacy_v2`, `Isotonic/deberta-v3-base_finetuned_ai4privacy_v2`.
**Ни одной русской модели.**

PyPI `llm-guard 0.3.16`: **28 зависимостей** `[ПРОВЕРЕНО]`. Точный список для Anonymize не
декларируется: README говорит, что «Base functionality requires a limited number of libraries. As
you explore more advanced features, necessary libraries will be automatically installed». То есть
**вес зависит от включённых сканеров** — само по себе проблема для контракта по памяти. Нижняя
оценка для одного Anonymize: Presidio + spaCy + `en_core_web_lg` (382 MB) + transformers + torch +
BERT-NER `[НЕ ПРОВЕРЕНО]`. Пик памяти по чужому замеру — **1 565 MB** `[ПРОВЕРЕНО, self-reported]`.

**Задержка опубликована** `[ПРОВЕРЕНО]` документация Anonymize, Benchmarks. Условия: Amazon
Linux 2, Python 3.11.6, **вход 317 символов**, 5 прогонов:

| Инстанс | p90 | **p95** | p99 | средняя | QPS |
|---|---|---|---|---|---|
| AWS m5.xlarge | 255.64 | **294.57** | 325.71 | 177.13 | 1789.64 |
| AWS m5.xlarge with ONNX | 155.64 | **169.13** | 179.93 | 128.64 | 2464.29 |
| AWS g5.xlarge GPU | 321.59 | **419.60** | 498.01 | 125.18 | 2532.35 |
| AWS g5.xlarge GPU + ONNX | 70.49 | **86.47** | 99.26 | 38.11 | 8317.53 |
| AWS r6a.xlarge (AMD) | 266.44 | **276.45** | 284.47 | 244.17 | 1298.29 |
| AWS r6a.xlarge (AMD) + ONNX | 238.15 | **247.22** | 254.47 | 218.91 | 1448.06 |

**Это лучший подарок аудита.** Единственные опубликованные производителем p95 для
Presidio-подобного обезличивания — и они на **317 символах**. Наш контракт — 25–50 ms p95 на
**2 000 токенов** (≈14 000 символов, в 44× больше). Даже лучший CPU-вариант (ONNX, m5.xlarge) —
**169.13 ms p95** на 317 символах, то есть минимум **3.4×** выше потолка при **44×** меньшем
объёме. Это цифра конкурента, а не наша, и её можно цитировать.

### 3.5 Лицензия и русский язык

Лицензия `[ПРОВЕРЕНО]` дважды: GitHub API `license = MIT`; PyPI `info.license` — «The MIT License
(MIT) Copyright (c) Protect AI». Модели под `protectai` — лицензии у каждой свои `[НЕ ПРОВЕРЕНО]`;
они «no longer maintained», но **не отозваны** — использовать юридически можно, поддерживать некому.

`[ПРОВЕРЕНО]` документация Anonymize, дословно, два взаимоисключающих утверждения на одной странице:

> «**Support of multiple languages**: The scanner can detect PII in English and Chinese.»
>
> «Info: **Current entity detection functionality is English-specific.**»

Список PII-сущностей сканера: Credit Cards, Person, PHONE_NUMBER, URL, E-mail (включая
обфусцированные `john.doe[AT]protectai[DOT]com`), IPs, UUID, US SSN, Crypto (Bitcoin), IBAN —
**11 групп, ни одной русской**. Плюс custom regex из `sensisitive_patterns.json` `[НЕ ПРОВЕРЕНО]`.

### 3.6 Где слабое место LLM Guard для русского

1. **Проект архивирован**, модели на HF тоже. Нельзя строить на нём.
2. **Наследует все русские дефициты Presidio** (нет рекогнайзеров, нет русских контекстных слов)
   и **усиливает** их: список из 11 групп заточен под en-US.
3. **«English and Chinese»** официально; Russian не упомянут.
4. `language="en"` по умолчанию; что произойдёт при `language="ru"` — `[НЕ ПРОВЕРЕНО]`, но
   документированной русской модели нет.
5. `use_faker` не дефолт и не даёт валидных по контрольной сумме русских ID.
6. **p95 169–294 ms на 317 символах** — несовместимо с 25–50 ms.
7. Вес не декларируется; пик памяти 1 565 MB по чужому замеру.
8. 34 из 36 сканеров не относятся к задаче — лишняя поверхность атаки и зависимостей.

---

## 4. Portcullis и SurrogateShield

### 4.1 Portcullis (ByteDance / AAAI-25)

**Идентификация** `[ПРОВЕРЕНО]` <https://ojs.aaai.org/index.php/AAAI/article/view/32088>:
**«Portcullis: A Scalable and Verifiable Privacy Gateway for Third-Party LLM Inference»**; авторы —
**Jiangou Zhan (Tsinghua University), Wenhui Zhang, Zheng Zhang, Huanran Xue, Yao Zhang, Ye Wu
(ByteDance Inc.)**; **AAAI-25**, Vol. 39 No. 1, Technical Track on Application Domains, стр.
**1022–1030**; опубликовано **2025-04-11**; DOI **10.1609/aaai.v39i1.32088**;
PDF: <https://ojs.aaai.org/index.php/AAAI/article/view/32088/34243>.

**Уточнение к нашей формулировке:** в 00-charter написано «Portcullis (ByteDance)» — точнее **первый
автор из Tsinghua**, остальные пять — ByteDance.

**Есть ли код: `[НЕ НАЙДЕНО]`.** Проверено: GitHub Search `portcullis LLM privacy` → 0,
`portcullis bytedance` → 0; на странице статьи OJS ссылки на артефакт **нет** (только PDF и
Video/Poster); arXiv API `all:Portcullis AND all:LLM` → **пусто**. Воспроизведение требует
собственной реализации.

**Архитектура по абстракту** `[ДОК]` (PDF не открывал): роль — «mediator» между пользователем и
сторонним LLM; подстановка — «anonymizing sensitive data in prompts through **parallel
substitution**»; среда — «all data processing occurs within **secure encrypted memory**»; доверие —
«The gateway is **attested**»; реконструкция — «accurately reconstructing responses»;
масштабирование — «startup time scales linearly» (замер как confidential container platform).

**Про «96x» — точная формулировка, которую мы обязаны использовать.** Абстракт дословно:

> «For **masking and unmasking workloads**, Portcullis outperforms **Hide-and-Seek** by **96x speed up**,
> while maintaining equal or better false positive and false negative rates compared to existing solutions.
> On the **Enron dataset**, Portcullis achieves notably higher accuracy, surpassing Hide-and-Seek by over **0.1** for **GPT-4o mini**.»

То есть: 96× — **ускорение операций маскирования/снятия маски** против Hide-and-Seek; это **не**
«96× быстрее предшественников» вообще и **не** про качество детекции; на качестве преимущество —
**+0.1 accuracy** для GPT-4o mini, независимая и куда более скромная цифра. Наша правка в
00-charter («96x на маскировании против Hide-and-Seek, плюс аттестация среды») **верна** `[ПРОВЕРЕНО]`.

**Что Portcullis решает:** доверие к среде исполнения и пропускную способность
маскирования/снятия маски. **Чего не решает** `[ДОК — прямо не заявлено в абстракте]`: многоходовый
контекст, анафора и кореференция не адресуются (ни слова в абстракте); не заявлено, что подмены
формат-сохраняющие («parallel substitution» может быть плейсхолдером); **русский язык не упомянут**,
датасеты — **PII** (вероятно английский) и **Enron** (английский); типы сущностей не перечислены;
требуется secure enclave (SGX/TDX) — серверный CPU с TEE, что неприменимо на потребительских CPU и
в WASM; кроссплатформенности нет (confidential container platform).

**Вердикт.** Берём интеллектуальную рамку («шлюз должен быть проверяемым») и сам факт, что
независимая академическая работа назвала именно эту нишу. Отвергаем TEE и аттестацию как основу:
это (а) не решает нашу задачу (русский, суррогаты, контекст), (б) требует серверного железа,
(в) код недоступен, (г) 96× относится к другой метрике. Также отвергаем соблазн цитировать 96×
без оговорки — это единственная цифра, которую проверяющий откроет и найдёт.

### 4.2 SurrogateShield

**Идентификация** `[ПРОВЕРЕНО]`: статья **«SurrogateShield: Beyond Redaction for High-Utility,
Privacy-Preserving LLM Interactions»**, автор **Sherwin Vishesh Jathanna** (единственный), arXiv
**2606.29567** (cs.CR, cs.CL), подан **28 июня 2026**, v1; в arXiv: «14 pages, 1 figure, 9 tables.
**Code and dataset**: <https://github.com/sherwinvishesh/SurrogateShield>»; лицензия статьи
**CC BY 4.0**; репозиторий под **Apache-2.0**; PyPI `surrogateshield` **1.9.0**; GitHub-метрики —
**2 ★, 0 форков, 0 open issues**, создан **2026-05-13**, push **2026-10-16**, Python, 11 439 KB.

**Что заявлено** `[ПРОВЕРЕНО]` (абстракт arXiv):

> «a client-side proxy that substitutes detected PII with locally generated, **type-consistent
> surrogate values** … Detection runs through a **three-stage cascade (PatternScan, EntityTrace, and
> ContextGuard)** covering **22 PII types** and quasi-identifier combinations grounded in **Sweeney's
> k-anonymity** … sealed in an **AES-256-GCM** encrypted per-conversation **ShadowMap** that never
> leaves the device. … **F1 score of 98.87%** … **13.26 pp improvement in BERTScore (roberta-large),
> from 81.59% to 94.85%** … in a 100-query adversarial trial, a prompted LLM adversary recovered
> **no original values**.»

**Что проверяемо.** Статья и код **существуют и открыты** — этим SurrogateShield отличается от
Portcullis: ✓ код на GitHub под Apache-2.0, ✓ статья на arXiv с CC BY 4.0, ✓ PyPI-пакет,
✓ бенчмарк-харнес (`bench/results/README.md`), ✓ 141 pytest-модуль.

**Что не проверено нами: сами цифры.** Все бенчмарки **self-reported одним автором**, репозиторий — 2
звезды, независимых репликаций `[НЕ НАЙДЕНО]`.

**Цифры, которые стоят внимания** `[ПРОВЕРЕНО, self-reported]`. Sealed test-3 (ShareGPT, WildChat,
oasst1, не использованные для настройки):

| arm | утекло значений (из 3 677) | чистых промптов | странных форматов утекло | безобидный текст правлен |
|---|---|---|---|---|
| **SurrogateShield** | **0.8%** (30) | **97.4%** | **0.2%** (1 из 530) | **13.9%** |
| GLiNER-PII (0.3) | 7.3% | 79.5% | 7.5% | 41.3% |
| GLiNER-PII (0.5) | 9.1% | 75.4% | 8.3% | 35.4% |
| **Presidio / Presidio + Faker** | **31.6%** | **30.8%** | **43.6%** | **32.6%** |
| Presidio + RoBERTa | 54.0% | 11.7% | 66.0% | 22.0% |
| **LLM Guard** | **56.3%** | **9.1%** | **61.5%** | **16.4%** |

Второй датасет — **Nemotron-PII** (1 000 записей, 4 627 значений): SurrogateShield утечка 14.5%,
F1 0.886; GLiNER-PII 22.5–25.8% (F1 0.751–0.758); Presidio 40.1% (F1 0.716); Presidio+RoBERTa 56.6%
(F1 0.594); LLM Guard 61.0% (F1 0.530).

**Производительность** (таблица `bench.perf_arms`, `[ПРОВЕРЕНО, self-reported]`):

| arm | медиана задержки | пик памяти |
|---|---|---|
| Presidio | 8.2 ms | 1 084 MB |
| Presidio + Faker | 8.4 ms | 1 010 MB |
| **SurrogateShield** | **43.5 ms** | **1 348 MB** |
| LLM Guard | 58.0 ms | 1 565 MB |
| GLiNER-PII (0.3/0.5) | 79.9 / 82.1 ms | 2 333 / 2 318 MB |
| Presidio + RoBERTa | 83.5 ms | 1 611 MB |

Многоходовый режим (60 диалогов): восстановлено реальных значений **81 из 82** против **0 из 80**
у Presidio + Faker. Это прямое измерение того, что Presidio-плейсхолдеры **не восстанавливаются**.

**Ограничения, которые автор сам называет** `[ПРОВЕРЕНО, раздел Limitations]`: «it trailed on the
**96 real values in natural text** (29 leaked; GLiNER-PII 16–20, Presidio 27), on some single types
(3 of 13 on test-3, 8 of 13 on Nemotron-PII, **URL most of all**) and on **speed and memory** (plain
Presidio is about **5× faster and lighter**)». Это ровно та зона, где мы должны быть сильнее:
**естественный текст**, а не списки значений.

**Архитектура** `[ПРОВЕРЕНО]`: `SentinelLayer` (PatternScan → canonicaliser → EntityTrace → PII
tagger → [ContextGuard] → структурные проходы → relation gate → resolver); `MimicGen` (Faker-суррогаты,
тип-консистентные, без коллизий в сессии); `ShadowMap` (AES-256-GCM, HKDF-SHA256(device_secret,
conv_id, kind), `~/.surrogateshield/`); `ResolvePass` (4 прохода: exact → component → fuzzy с
rapidfuzz, порог 85 → given name/surname).

Ключевые решения, которые стоит забрать (полная таблица — в
[`08-other-oss.md`](08-other-oss.md)): пресеты `balanced`/`strict`/`fast`/`classic`; суррогаты с
**валидной контрольной суммой** («fake SSNs pass format checks, fake Bitcoin addresses match Base58
format»; credit_card «Valid Luhn-format number»; `us_bank_number` «Valid 9-digit ABA routing
number»); `used_surrogates` set против коллизий; device secret 32 байта, `device.key` `0o600`,
каталог `0o700`, AES-256-GCM со свежим 12-байтовым nonce и kind+ID как AAD; **битый файл карты
уезжает в `<name>.corrupt-<ts>`, никогда не подменяется пустой картой**; fail-closed («a missing
detection model stops the request»; `DetectorUnavailable` вместо отправки незамаскированного);
**SHA-256 pin весов** на фиксированный коммит HF; квази-идентификаторный скоринг (10 комбинаций
Sweeney); восстановление **только по целым значениям** («"Lee" never matches inside "Leeds"») с
категоризацией `exact_miss`/`fuzzy_hit`/`fuzzy_miss`.


**Это почти дословно наш проектный документ, но на английском и на US-сущностях:** `en_core_web_lg`,
`ssn`, `zip_us`, `phone_us`, `us_bank_number`, `ABA`, `postal_code_uk` — **ни одной русской
сущности**. Их `ADDRESS_MODE=shift` (сдвиг номера дома на ±1) применим и у нас, но их address
parser — американский (штаты, ZIP+4, directionals).

**Вердикт.** Главный ориентир по идее и главный конкурент по духу. Берём ShadowMap-модель,
fail-closed, форматную валидность суррогатов, пресеты, whole-value matching, категоризацию
ошибок, квази-идентификаторный скоринг. Отвергаем как код (Python, US-only, 2 звезды, bus factor = 1),
как источник цифр (self-reported), как архитектуру задержки (43.5 ms медиана, 1 348 MB).
Мы **не можем** утверждать «мы первые делаем суррогаты» — только «мы первые делаем их для
русского, на CPU, в одном бинаре».

---

## 5. Прочие open-source решения

### 5.1 scrubadub

Факты `[ПРОВЕРЕНО]`: `LeapBeyond/scrubadub`, **433 ★**; лицензия **Apache-2.0** в репозитории, но
**MIT** на PyPI (`info.license`) — расхождение, причину не выяснял; последний релиз **v2.0.1 от
2023-09-01** и последний push **2023-09-01**; ~17 контрибьюторов; зависимости — `textblob==0.15.3`,
`phonenumbers`, `python-stdnum`, `dateparser`, `catalogue`, `scikit-learn`, `typing-extensions`,
`faker` `[PyPI requires_dist]`. **Русский:** локалей нет; `scrubadub_spacy` (41 ★, Apache-2.0,
push 2023-09-01) теоретически позволяет подсунуть `ru_core_news_*`, но контекстные слова и паттерны
английские `[ДОК]`. **Суррогаты:** `faker` в зависимостях, но по умолчанию возвращаются **теги**
(`{{EMAIL}}`) `[ДОК]`. **Вердикт: отвергаем** — мёртв 3 года, `textblob==0.15.3` жёстким пином,
`scikit-learn` без необходимости, расхождение лицензий, ноль русской поддержки.

### 5.2 DataFog

`[ПРОВЕРЕНО]`: `DataFog/datafog-python`, **78 ★**, **MIT**, релиз **v4.9.0 от 2026-09-29**
(+v4.9.0a2 от 2026-10-05), push 2026-10-05, ~11 контрибьюторов, репозиторий **85 309 KB (~83 MB)**.
**Русский:** `[НЕ НАЙДЕНО]`. **Суррогаты:** `[НЕ НАЙДЕНО]`. Проект живой, но маленький; 83 MB при
78 звёздах — вероятно, много данных/весов `[НЕ ПРОВЕРЕНО]`. **Вердикт: наблюдаем, не берём** — нет
русского, нет доказательств суррогатов, нет опубликованной задержки.

### 5.3 OpenAnonymiser

`[ПРОВЕРЕНО]` — фактически это `ConductionNL/anonymiq-light` («OpenAnonymiser Light»): **0 ★**,
**EUPL-1.2**, push 2026-10-06; стек FastAPI + **Presidio** + spaCy; NER — **`nl_core_news_lg`**
(нидерландский) везде (локально, контейнер, K8s); собственные нидерландские regex-рекогнайзеры;
эндпоинты `POST /api/v1/analyze`, `POST /api/v1/anonymize`, `GET /api/v1/health`; flavors —
`classic` (spaCy + NL regex, CPU), `gpu` (**~3 GB** образ: GLiNER + mdeberta), `contextual`
(**не реализован**); прод — `api.openanonymiser.commonground.nu`; суррогатов нет («vervangt PII
door placeholders»). Python-репо `openanonymiser` в GitHub Search **не найден** (`total_count = 0`) —
то есть «OpenAnonymiser» как отдельный проект не существует, есть только `anonymiq-light`.
**Вердикт: отвергаем** — нидерландский язык, EUPL-1.2 (копилефт-совместимость требует проверки при
линковке), 0 звёзд, `contextual` не реализован, суррогатов нет.

### 5.4 pydeidentify, deidentify, anonymize-it

`[ПРОВЕРЕНО]`: `pydeidentify` (`Lucasc-99/pydeidentify`, 1 ★, spaCy-NER, **нет лицензии**, push
2023-03-21, на PyPI есть) — отвергаем. npm `deidentify@0.2.0` — **MIT**, наблюдаем.
Из 226 GitHub-репо `deidentify` релевантны `nedap/deidentify` (149 ★, MIT, медзаписи, push
2025-11-17), `philterd/phileas` (110 ★, Apache-2.0, Java, push 2026-10-14), `pydicom/deid` (180 ★,
MIT, медизображения), `Planeshifter/deidentify` (41 ★, GPL-2.0, HIPAA PHI) — **`phileas`
единственный живой движок, но Java и без русского**. `anonymize-it` — на PyPI **404**, GitHub Search
выдал только посторонние проекты (`nucleuscloud/neosync` 4 133 ★, `arx-deidentifier/arx` 738 ★
Apache-2.0) → **не существует**.

**Полезный побочный результат:** `arx-deidentifier/arx` (738 ★, **Apache-2.0**, push 2025-10-01) —
«comprehensive open source data anonymization tool». Это **не** про текст для LLM, а про табличные
данные и k-anonymity, но лицензия и зрелость делают его референсом по **методологии k-anonymity**
`[ДОК]`. `nucleuscloud/neosync` (4 133 ★) — про базы данных, не про промпты. Оба — «соседние рынки,
где анонимизация уже commoditised».


### 5.5 LLM-прокси и «privacy firewall» проекты

Полная таблица (**11 проектов** с лицензиями, звёздами и датами) — в
[`08-other-oss.md`](08-other-oss.md). Ключевые:

| Репозиторий | ★ | Лицензия | Ключевая деталь |
|---|---|---|---|
| `zeroc00I/DontFeedTheAI` | **665** | MIT | Прозрачный прокси для pentest («strips IPs, credentials, hostnames and PII»), Ollama + regex, per-engagement vault |
| `openbashok/promptzero` | 70 | нет | Прозрачный прокси для Claude API |
| `Athroniaeth/piighost` | 14 | MIT | Reversible masking для агентов (LangChain, Pydantic AI, LlamaIndex, Claude Code) + прокси; 14 868 KB |
| `Aajil-Labs/arabic-pii-py` | 12 | Apache-2.0 | **Арабский + Залив**, local-first — **прямой аналог нашей стратегии для другой юрисдикции** |
| `gbrigandi/mcp-server-conceal`, `RPBLC-hq/DAM`, `nikshepsvn/veilstream`, `yurivenancio30/llm-dlp`, `whitestag-ai/pii-proxy`, `j911si/local-privacy-gateway`, `comptoir-des-signaux/sas-confiance-ia` | 1–18 | MIT / нет / NOASSERTION / | **EUPL-1.2** | MCP-прокси, «local privacy firewall», streaming PII proxy, локальный прокси для Claude Code, «GDPR anonymisation gate», офлайн-пseudonymization gateway, французская псевдонимизация (EUPL-1.2) |

**Что это доказывает.** Ниша «прокси, обезличивающий перед LLM» **уже занята десятком проектов**,
но: **все** либо English-only, либо под одну не-русскую юрисдикцию (арабский/Залив, французский,
нидерландский, Казахстан); **почти все** про плейсхолдеры/токены, не суррогаты; **все** на
Python/Node, **ни одного на Rust**; звёзды лидера (665) получены в домене пентеста. **Ниша
«русский + суррогаты + один бинарь» свободна.**


### 5.6 Источники суррогатов: mimesis и Faker

Ключевой практический вопрос — **откуда брать русские фейки**.

**Faker** `[ПРОВЕРЕНО]`: `joke2k/faker`, **19 447 ★**, **MIT**, push 2026-10-09; локаль **`ru_RU`**
существует — стандартный источник русских имён/адресов/телефонов `[ДОК]`; зависимость SurrogateShield
(`faker>=24.0.0`) и scrubadub.

**mimesis** `[ПРОВЕРЕНО]`: `lk-geimfari/mimesis`, **4 848 ★**, **MIT**, релиз **v22.2.0 от
2026-09-23**, push 2026-10-06, ~114 страниц контрибьюторов (≈228 человек); поддерживает локали,
включая русскую `[ДОК]`; чистый Python без ML-зависимостей — лёгкий `[НЕ ПРОВЕРЕНО]` по размеру.

**Но ни Faker, ни mimesis не генерируют форматно-валидные российские ID.** «ИНН физлица, проходящий
mod-11 ×2» — это **не** задача генератора фейковых персон, а отдельный генератор на нашей стороне:
взять формат, сгенерировать первые N−1 цифр, **посчитать контрольную сумму по нашему реестру** и
подставить. То же для ОГРН (mod 11), ОГРНИП (mod 13), СНИЛС (mod 101), карты (Luhn). Ни Presidio,
ни LLM Guard, ни pii-guard этого не делают — они маскируют, а не подменяют валидным. Единственный,
кто заявляет валидные суррогаты, — SurrogateShield, и только для Luhn/ABA/Base58 `[ПРОВЕРЕНО]`.

### 5.7 nedap/deidentify, Phileas и мелкие движки

`[ПРОВЕРЕНО]`: `nedap/deidentify` — 149 ★, **MIT**, медицинские записи, «state-of-the-art NLP»;
язык нидерландский/английский; README на `main` вернул 404 `[ПРОВЕРЕНО]` (структура репо другая).
`philterd/phileas` — 110 ★, **Apache-2.0**, push 2026-10-14, Java-движок PII/PHI-редакции с
«replacement»-стратегиями: **единственный живой не-Python движок** в обзоре, русского нет
`[НЕ НАЙДЕНО]`. Стоит перепроверить в фазе 4 как источник идей по стратегиям замены.

### 5.8 Cloudflare AI Gateway DLP

`[ПРОВЕРЕНО]` [set-up-dlp](https://developers.cloudflare.com/ai-gateway/features/dlp/set-up-dlp/),
обновлено 2026-09-25. Профили: **Financial Information** (карты, счета, routing), **PII** (имена,
адреса, телефоны), **Government Identifiers** (SSN, паспорта, права), **Healthcare Information**,
**Custom Profiles**. **Действия только `FLAG` или `BLOCK`** — ни маскирования, ни суррогатов.
Проверка `REQUEST`/`RESPONSE`/`BOTH`. Ошибки: код **2029** (запрос) / **2030** (ответ). Заголовок
`cf-aig-dlp` с массивом `findings`. **Русский:** `[НЕ НАЙДЕНО]` — профили строятся на Zero Trust
DLP, про русские сущности упоминаний нет. **Вердикт: отвергаем как механизм обезличивания**
(BLOCK — не обезличивание), но берём идею **профилей как политик** и **машиночитаемого отчёта о
срабатывании**; наш `Report` должен быть таким же структурным.

### 5.9 LiteLLM Guardrails

Полный разбор — в [`08-other-oss.md`](08-other-oss.md). Главное `[ПРОВЕРЕНО]` по
[документации](https://docs.litellm.ai/docs/proxy/guardrails/pii_masking_v2): провайдер — Presidio;
сущности — **все типы Presidio**; действия **`MASK` и `BLOCK`**, где «`MASK`: Replace the entity
with a **placeholder** (e.g., `<PERSON>`)»; языки — только **`en` (default), `es`, `de`**; требуются
развёрнутые контейнеры Presidio Analyzer + Anonymizer. Два важных факта: (1) `output_parse_pii`
«only **un-masks tokens back to the originals** after the model call» — LiteLLM **умеет**
восстанавливать реальные значения, но через токены, не суррогаты; (2) «analyzer latency grows with
payload size» и `/analyze` body капается в 1 MB на дефолтных развёртываниях Presidio — прямое
подтверждение нашей проблемы масштаба. **Лицензия LiteLLM: MIT** `[ПРОВЕРЕНО]`. **Вердикт:** берём
как **канал интеграции**, не как движок; русский подтверждённо не поддержан.

### 5.10 Прочее, что проверено и отвергнуто

| Проект | Лицензия | Активность | Причина отказа |
|---|---|---|---|
| `zylon-ai/private-gpt` | Apache-2.0 | **57 583 ★**, push 2026-10-15 | Приватность = **локальный инференс**, а не обезличивание; Presidio в зависимостях не нашли — другая задача |
| `yandex/tomita-parser` | **MPL-2.0** | 500 ★, **push 2020-11-16** | Мёртв 6 лет; сборка требует lua5.2/gcc 4.8/cmake 2.8 и отдельно `libmystem-c-binding.so` из релиза v1.0; свой язык конфигов. Берём идею rule-based извлечения (Natasha/Yargy — живая реализация) |
| `LeapBeyond/scrubadub_spacy` | Apache-2.0 | 41 ★, push 2023-09-01 | Мёртв |
| `LeapBeyond/scrubadub_address` | Apache-2.0 | **1 ★**, push 2023-09-01 | Мёртв |

**«Oban»** `[НЕ НАЙДЕНО]` как инструмент обезличивания: в экосистеме есть Elixir-библиотека `Oban`
— **очередь фоновых задач**, к приватности отношения не имеет. Позиция плана закрыта: артефакта нет.


## 6. Русскоязычные инструменты

### 6.1 Сводная таблица

Полная таблица с весами и активностью (26 строк, вкл. модели HuggingFace) — в
[`06-russian-tools-table.md`](06-russian-tools-table.md). Здесь — только то, чего нет в §1.1:
типы сущностей и наличие суррогатов.

| Инструмент | Лицензия | Типы сущностей | Суррогаты | Вердикт |
|---|---|---|---|---|
| **pii-guard** | **Apache-2.0** | **20 типов:** модель — PERSON, LOCATION, PHONE_NUMBER, URL; правила — EMAIL_ADDRESS, INN, SNILS, OMS, PASSPORT, DRIVER_LICENSE, MILITARY_ID, BIRTH_CERTIFICATE, CREDIT_CARD, BANK_ACCOUNT, BIK, POSTAL_CODE, TELEGRAM, IP_ADDRESS, IP_PORT, DATE_TIME | **Нет** (mask/tag/pseudonymize) | **Главный русский конкурент** |
| **rubert-base-pii-ner** | **Apache-2.0** | 21 тип / 43 BIO-метки | Нет | **Берём как baseline** |
| **pii_benchmark / pii_train** | **MIT** / — | 14+ категорий (2 841 предл.) / 17 137 предл., 39 687 спанов | — | **Берём как внешние замеры** |
| **Natasha / Slovnet** | **MIT** | PER, LOC, ORG (+ морфология, синтаксис, факты; Yargy — даты/деньги/адреса) | Нет | **Берём NER-провайдером** |
| **spaCy `ru_core_news_*`** | **MIT** | PER, LOC, ORG | Нет | Baseline для сравнения |
| **exemtik/pii-bench-ru-en** | **MIT** | 13 типов (RU): NAME, PHONE_NUMBER, EMAIL, ADDRESS, BANK_CARD_NUMBER, CVC, INN, KPP, OGRN, OGRNIP, SNILS, PASSPORT_NUMBER, TOKEN | Нет, токены `[ИМЯ_1]` | **Берём метод (pymorphy3) и цифры** |
| **llm-pii-proxy-showcase** | **MIT** | KZ_BIN/IIN/IIK + PERSON/ORG/LOC (Presidio ru) | Нет (солёные плейсхолдеры) | **Берём инженерию** |
| **DeepPavlov** | **Apache-2.0** | PER/LOC/ORG (ru-модели) | Нет | Отвергаем (вес) |
| **SergeyK88i/pii-obfuscator** | **нет лицензии** | `[НЕ ПРОВЕРЕНО]` | Псевдонимы | Наблюдать (нет лицензии!) |
| **ru-pii-detection-system / yashirgich / kazpii-bench** | **MIT** / нет / нет | `[НЕ ПРОВЕРЕНО]` / `[НЕ ПРОВЕРЕНО]` / KazNERD | — | Наблюдать (у двух нет лицензии) |
| **rubert/спaCy-модели ru-PII** (`alrosait`, `dzanozin`, `HikmaAI`, `Wismut`, `tabularisai`, `LiquidAI`, `Gherman`, `yqelz`) | MIT / Apache-2.0 / other | `[НЕ ПРОВЕРЕНО]`; у `dzanozin` — **1,4 млрд параметров**, у `tabularisai` — 559M и 4 151 загрузка, у `LiquidAI` — лицензия «other» | — | **Стоит проверить `tabularisai/eu-pii-safeguard` в фазе 4**; остальные — наблюдать |
| **SLIM / NEREL / Sber / Studio Ousia** | — | — | — | См. §6.6 |


### 6.2 pii-guard — детальный разбор (главный русский конкурент)

`[ПРОВЕРЕНО]` README `redmadrobot-rnd/pii-guard` + `docs/architecture.md`. «Обезличивание
персональных данных в русском тексте … маской или тегом — насовсем, псевдонимом — обратимо.
Работает и сам по себе, и прослойкой перед LLM».

**20 типов сущностей** `[ПРОВЕРЕНО]` (дословно из README): **модель даёт 4** — `PERSON` (ФИО),
`LOCATION` (адрес и части: страна/регион/район/город/улица/дом), `PHONE_NUMBER`, `URL`.
**Правила дают 16**: `EMAIL_ADDRESS`; `INN`, `SNILS`, `OMS` (правила + контрольная сумма);
`PASSPORT`, `DRIVER_LICENSE`, `MILITARY_ID`, `BIRTH_CERTIFICATE` (правила + контекст);
`CREDIT_CARD` (Luhn); `BANK_ACCOUNT`, `POSTAL_CODE`, `TELEGRAM`, `IP_ADDRESS`, `IP_PORT`,
`DATE_TIME`; `BIK` (правила + **расстояние до ключа**).

Дословно: «Правила ловят то, что проверяется контрольной суммой, модель — то, что суммой не
проверить»; «Без модели работают все типы с контрольными суммами. Недоступны четыре, которые берёт
только модель». **Это подтверждает наш реестр типов и нашу каскадную архитектуру** — они пришли к
тому же разделению L0/L2 и по той же причине.

**Три архитектурных решения, которые надо забрать дословно** `[ПРОВЕРЕНО]` `docs/architecture.md`:

1. **«Ветки независимы, конфликты разрешаются после. При перекрытии спанов побеждают правила:
   детерминированная контрольная сумма надёжнее вероятностной оценки»** — наше правило «каскады не
   отменяют друг друга», но с явным приоритетом. Побочный эффект честно назван: «ошибочное правило
   гасит правильное предсказание модели, а не наоборот. Именно так выглядели оба дефекта, из-за
   которых правила пришлось сузить по расстоянию».
2. **«Ключевые слова оцениваются расстоянием, а не присутствием в окне»**: «если в одном предложении
   два числа и два ключа, булев ответ неверен при любом размере окна — „водительское удостоверение
   77 АВ 123456, индекс 630009“ требует отклонить первые шесть цифр и принять вторые». Бюджеты
   **измеренные**: «у `BIK` ключ во всей разметке стоит не дальше 24 символов слева, поэтому бюджет
   40; у паспорта p99 = 83, поэтому близкое окно 80 и дальнее 160». **Прямое улучшение нашего L1**:
   у нас записано «окно ±N токенов», без ответа на случай двух ключей и двух чисел.
3. **«Кандидат ищется по фразе, а отдаётся значение»**: «`7518` само по себе не паспорт, `серия 7518,
   номер 492137` — паспорт … Маска ложится на `серия ****, номер ******`, а не на всю фразу: „серия“
   и „номер“ — не персональные данные». У нас такого требования нет вообще, а оно правильно с точки
   зрения сохранения смысла.

**Восстановление в правильной падежной форме** `[ПРОВЕРЕНО]` README: «→ `Свяжитесь с Иваном
Петровым`. Прямая подстановка дала бы „с Иван Петров“.» Реализация — `pymorphy3` + spaCy.
**Никто из западных решений этого не делает:** SurrogateShield восстанавливает подстановкой строки,
Presidio — `decrypt`. Согласование падежа — уникальная русская продуктовая фича, и у pii-guard она есть.

**Режимы** `[ПРОВЕРЕНО]`: `mask` (`Иван Петров` → `***********`, необратим), `tag` (→ `[PERSON]`,
необратим), `pseudonymize` (→ `<PII type="PERSON" gender="male" id="1" />`, **обратим**). Псевдоним
несёт **тип, род и стабильный id** — «одинаковые значения получают один `id` даже в разных
грамматических формах» (реализация нашего I5). И это **не суррогат**: тип сущности виден модели.

**Предобработка** `[ПРОВЕРЕНО]`: нормализация, **транслит в кириллицу**, числа словами,
исключение base64-блобов. Это наш I1 («проверка сравнивает E с исходными спанами по всем
нормализациям: регистр, пробелы, дефисы, **транслит**, юникод-гомоглифы, цифры словами»).
У них это уже реализовано. Отдельно: **смещения указывают на `normalized_text`, а не на
присланный текст** — и они это документируют как подводный камень.

**Данные** `[ПРОВЕРЕНО]` (полная таблица — в [`06-russian-tools-table.md`](06-russian-tools-table.md)):
NER-модель `rubert-base-pii-ner` (ruBert-base, 178M, веса **678 MiB**); обучение — `pii_train`,
**17 137 предложений / 39 687 спанов**; оценка — `pii_benchmark`, **2 841 предложение**, disjoint от
train; 10 эпох, lr 3e-5 linear без warmup, batch 16, AdamW wd 0.01, fp32, max_len 512, seed 42;
инференс — окно 512 подтокенов, **stride 128**, `min_confidence=0.3`; **слияние фрагментов
обязательно** (без него `elena@pochta.ru` распадается на `el` + `ena@pochta.ru`).


**Качество** `[ПРОВЕРЕНО]` (span-level micro-F1, exact matching, 14 категорий):

| Система | P | R | F1 |
|---|---|---|---|
| Модель отдельно | 81.9 | 85.5 | **83.6** |
| Пайплайн `pii-guard` (правила + модель) | 90.4 | 87.5 | **88.9** |
| На протоколе лидерборда (PERSON+LOCATION, overlap) | — | — | 94.7 / 95.0 |

Per-entity F1 модели (полная таблица по 21 типу — в [`06-russian-tools-table.md`](06-russian-tools-table.md)):
FIRST_NAME 89.8, MIDDLE_NAME 91.5, LAST_NAME 84.5, HOUSE 92.7, EMAIL 97.6, URL 89.7, PHONE 86.9,
STREET 87.3, OMS 90.1, CREDIT_CARD 87.9, BIRTH_CERTIFICATE 83.7, CITY 82.5, DRIVER_LICENSE 81.8,
PASSPORT 81.3, REGION 80.3, DISTRICT 78.3, SNILS 78.5, MILITARY_ID 78.3, COUNTRY 86.1,
**INN 68.3**, **IP_ADDRESS 36.0**.


**Критично для нас:** `INN` F1 **68.3** — при том, что у ИНН есть контрольная сумма, и в
пайплайне правила её проверяют. То есть даже с checksum+context ИНН детектится хуже остальных
документов. `IP_ADDRESS` F1 36.0 — но они сами пишут, что в пайплайне это regex-правило.
**Вывод: наш реестр не должен обещать высокий recall по ИНН без замеров; 68–80% — реальная
планка для чистой модели.**

**Лицензия** `[ПРОВЕРЕНО]`: «Apache-2.0, веса модели тоже. **Копилефта в установке по умолчанию
нет.** Единственное исключение придётся подключить руками. Extra `latin-gender` ставит
`gender-guesser` под **GPLv3**.» — Это ровно та честность про лицензии, которую мы требуем от
себя. И важное предупреждение: **опциональная extra с GPLv3** — если мы будем определять род
русского имени, нельзя случайно затащить GPL.

**Слабые места pii-guard:** (лицензию `pymorphy3` надо проверить — предполагаю MIT, `[НЕ ПРОВЕРЕНО]`)


1. **Суррогатов нет вообще.** Псевдоним — `<PII type="PERSON" id="1" />`, модель видит тип и род;
   дословно «Только `pseudonymize` обратим: **тип сущности виден модели**» — для нашей задачи
   «внешняя модель не должна знать, что это ПДн» это неверно.
2. **Python + Presidio + PyTorch:** 678 MiB весов + Presidio + spaCy + torch — в Balanced (≤250 MB)
   не влезает, в Fast (≤30 MB) тем более.
3. **Нет состояния сессии:** «Таблица псевдонимов нигде не хранится. Сервис не имеет состояния: ни
   Redis, ни TTL, ни идентификатора сессии» — осознанное решение, но **перекладывает I5/I2 на клиента**.
4. **Задержка не опубликована** `[НЕ НАЙДЕНО]`: 178M ruBERT + окно 512/stride 128 на CPU, 2 000
   токенов ≈ 6 окон по forward-проходу BERT-base — ≤25 ms p95 наивно ожидать нельзя
   `[НЕ ПРОВЕРЕНО, оценка]`.
5. **Метаданные PyPI битые:** `pii-guard 0.1.0` содержит `Homepage: https://github.com/YOUR_USERNAME/pii-guard`
   — шаблон не заполнен `[ПРОВЕРЕНО]`; загружен 2025-12-10.
6. **Код на GitHub не открывал** `[НЕ ПРОВЕРЕНО]`: API по репо вернул пустой объект, звёзды и
   контрибьюторы не подтверждены.

### 6.3 Exemtik/pii-bench-ru-en — единственные публичные цифры по русскому персоналу

`[ПРОВЕРЕНО]` README (raw):

**Данные.** RU: `hivetrace/pii-bench` — **1 810 документов, 1 667 спанов**, **13 типов**: NAME,
PHONE_NUMBER, EMAIL, ADDRESS, BANK_CARD_NUMBER, CVC, **INN, KPP, OGRN, OGRNIP, SNILS,
PASSPORT_NUMBER**, TOKEN. EN: `alejopaullier/pii-external-dataset` — 4 434 документа, 22 226 спанов.
**Метрика:** char-overlap F1 + span-метрики + token-F5 для английского.

**Результаты по русскому** `[ПРОВЕРЕНО]`:

| Конфигурация | overall F1 | F1 по `NAME` |
|---|---|---|
| `regex_only` | 0.800 | **0.000** |
| `regex+morph` (pymorphy3) | 0.846 | **0.794** |
| `regex+spacy_ru` (baseline) | **0.897** | **0.910** |
| `regex+morph+spacy_ru` | 0.873 | 0.785 |

**Результаты по английскому** (те же метрики): `regex_only` 0.586/**NAME_STUDENT 0.000**;
`regex+spacy` **0.745**/0.728 (token-F5 0.727); `regex+spacy_aggr` 0.703/0.637 (token-F5 0.765).


**Четыре вывода:**

1. **Regex без морфологии даёт F1 по именам ровно 0.000.** Форматные типы (EMAIL, PHONE, СНИЛС, ИНН,
   КПП, ОГРН, карты, токены) покрыты regex с F1 ≈ 0.9–1.0, имена — нет. Количественное подтверждение,
   что профиль **Fast (L0+L1, без NER) физически не может находить ФИО**.
2. **`pymorphy3` даёт F1 0.794 по именам без нейросетей** — разбор (`Surn`/`Name`/`Patr`) почти
   дотягивает до spaCy и работает в любом падеже («Игорем Волковым»). Вес `[НЕ ПРОВЕРЕНО]`, но
   заведомо **на порядки легче** 489.6 MB `ru_core_news_lg` → **кандидат в Balanced L2**.
3. **`ru_core_news_sm` обгоняет pymorphy3 (0.910 против 0.794) и ловит `ADDRESS` (0.507, у regex и
   pymorphy — 0)** — за **14.6 MB**.
4. **Аддитивность отрицательная:** `regex+morph+spacy_ru` (0.873) **хуже** `regex+spacy_ru` (0.897) —
   выбирать **один** источник NER, не сваливать всё в кучу.

**Ограничения автора** `[ПРОВЕРЕНО]`: «RU: 10 слитных цифр неоднозначны (ИНН юрлица ↔ паспорт);
контекстные паттерны разводят часть случаев, остальное по умолчанию относится к паспорту» —
подтверждение нашего пункта про паспорт РФ без контрольной суммы. **Лицензия MIT** `[ПРОВЕРЕНО]`.

### 6.4 Natasha / Slovnet — лучший CPU-провайдер для русского

`[ПРОВЕРЕНО]` README Natasha + Slovnet + PyPI:

- **Задачи:** токенизация/сегментация (Razdel), эмбеддинги (Navec), морфология, лемматизация
  (Pymorphy), синтаксис, NER (Slovnet), извлечение фактов (Yargy), визуализация (Ipymarkup).
- **Версия:** `natasha 1.6.0`, PyPI загружен **2023-07-24**, 13 релизов; тег **v1.6.0**;
  1 359 ★, push **2026-10-14** (изменения идут, но без релизов). **Лицензия MIT** `[ПРОВЕРЕНО]`.
- **Зависимости:** `pymorphy2`, `razdel>=0.5.0`, `navec>=0.9.0`, `slovnet>=0.6.0`, `yargy>=0.16.0`,
  `ipymarkup>=0.8.0` `[ПРОВЕРЕНО]`.
- **Позиционирование (важно для нас):** «Natasha is **not a research project** … built for
  production. **We pay attention to model size, RAM usage and performance. Models run on CPU, use
  Numpy for inference.**» Предупреждение: «Models optimized for **news articles**, quality on other
  domain may be lower»; «API may change».

**Размеры моделей** `[ПРОВЕРЕНО]` Slovnet README, таблица Downloads: `slovnet_ner_news_v1.tar` —
**2 MB** (русский NER, стандартная аннотация **PER, LOC, ORG**, обучен на новостях);
`slovnet_morph_news_v1.tar` — **2 MB**; `slovnet_syntax_news_v1.tar` — **3 MB**.

**Производительность и вес** `[ПРОВЕРЕНО]` Slovnet README (полная таблица — в
[`06-russian-tools-table.md`](06-russian-tools-table.md)): **slovnet** (NER) — init **1.0 s**,
диск **27 MB**, RAM **205 MB**, **25.3** новостных статей/с; `spacy` (ru) — 140 MB / 625 MB (8.0);
**tomita** — 64 MB / **63 MB** (29.8); `slovnet_bert` — 473 MB / 9 500 MB (40 it/s GPU);
`deeppavlov*` — 1 024–2 048 MB диск и 3 072–6 144 MB RAM (8–24 it/s GPU); `pullenti` — 16 MB / 253 MB
(6.0); `stanza` — 591 MB / 11 264 MB (3.0 GPU); `mitie` — 327 MB / 261 MB (32.8). Морфология:
`slovnet` — **27 MB диск, 115 MB RAM, 532 предложения/с**; синтаксис — **27 MB, 125 MB RAM,
450 предложений/с**.

**Пересчёт на задержку.** 25.3 новостных статьи/с при «1 article ≈ 1KB» `[ПРОВЕРЕНО, допущение
Slovnet]` → **~1 KB за 39.5 ms**. Наши 2 000 токенов — грубо 12–16 KB `[НЕ ПРОВЕРЕНО, оценка]`.
Линейная экстраполяция даёт **~0.5 с**, что в бюджет не влезает. Нелинейность и батчирование
могут изменить картину, но **рассчитывать на 12 ms для L2 по 2 000 токенов, как в нашем бюджете
(§7.2 charter), на основании этих цифр нельзя** — бюджет L2 требует собственного замера, Slovnet
как есть не гарантия.

**Качество NER** `[ПРОВЕРЕНО]` Slovnet README, F1 по 4 датасетам (factRu/gareev/ne5/bsnlp):

| Система | factRu PER | factRu LOC | factRu ORG | gareev PER | ne5 PER | ne5 LOC | bsnlp PER |
|---|---|---|---|---|---|---|---|
| **slovnet** | **0.959** | **0.915** | 0.825 | **0.977** | **0.984** | **0.973** | 0.944 |
| slovnet_bert | 0.973 | 0.928 | 0.831 | 0.991 | 0.996 | 0.989 | 0.960 |
| deeppavlov | 0.910 | 0.886 | 0.742 | 0.944 | 0.942 | 0.919 | 0.866 |
| deeppavlov_bert | 0.971 | 0.928 | 0.825 | 0.980 | 0.997 | 0.990 | 0.954 |
| deeppavlov_slavic | 0.956 | 0.884 | 0.714 | 0.976 | 0.984 | 0.817 | **0.965** |
| **spacy** (ru) | 0.901 | 0.886 | 0.765 | 0.970 | 0.967 | 0.928 | 0.919 |
| stanza | 0.943 | 0.865 | 0.687 | 0.953 | 0.923 | 0.753 | 0.938 |
| pullenti | 0.905 | 0.814 | 0.686 | 0.939 | 0.952 | 0.862 | 0.900 |
| tomita | 0.929 | — | — | 0.921 | 0.945 | — | 0.881 |

Резюме авторов: «NER is **1-2% worse** than current BERT SOTA by DeepPavlov but **60 times smaller
in size (~30 MB)** and works **fast on CPU (~25 news articles/sec)**.» Датасеты (factRuEval-2016,
gareev, ne5, BSNLP) доступны через `natasha/corus` `[ДОК]`; на HF — `gusevski/factrueval2016`
(65 загрузок) `[ПРОВЕРЕНО]`.

**Вердикт по Natasha.** **Берём.** 2 MB модель, 27 MB диск, 205 MB RAM, MIT, CPU-only, Numpy-only
инференс, F1 по PER 0.959 на factRuEval, честное позиционирование «не research project», и прямо
заявленная оптимизация под размер/RAM/скорость. Это **ровно** наш профиль Balanced. Риски:
только PER/LOC/ORG (нет PII-типов), домен — новости, последний релиз 2023, API нестабилен.

### 6.5 spacy ru-модели — детально

`[ПРОВЕРЕНО]`: `ru_core_news_sm` — **14.6 MB**, `ru_core_news_md` — **39.9 MB**,
`ru_core_news_lg` — **489.6 MB**; все — **MIT**, NER-метки **LOC, ORG, PER**. Pipeline
`ru_core_news_sm-3.8.0`: `morphologizer`, `parser`, `attribute_ruler`, `lemmatizer`, `ner`;
лексемы — полный Universal Dependencies; лемматизатор по умолчанию **`mode: "pymorphy3"`**
`[ПРОВЕРЕНО]` (`spacy/lang/ru/__init__.py`).

**Важно:** spaCy `ru` **сам использует `pymorphy3`** для лемматизации — «spaCy против pymorphy3»
ложная дихотомия: `ru_core_news_sm` = статистический NER + pymorphy3-лемматизатор. Именно поэтому
`regex+spacy_ru` обгоняет `regex+morph`: pymorphy3 работает **внутри** spaCy, плюс есть NER и парсер.

**Слабые места** `[ПРОВЕРЕНО]`: (1) только **три** метки — LOC/ORG/PER; **нет** адреса, даты,
документа, ИНН, телефона. (2) Обучены на **новостях**, не на личных документах/переписке.
(3) У Presidio по умолчанию **`ORGANIZATION` в игнор-листе** «Has many false positives» — и это
про английскую `en_core_web_lg`; про русскую надёжных данных нет. Практика из showcase: **«ORG/LOC
thresholds are raised (≥ 0.85, length ≥ 5) — spaCy is noisy on them in legal text»** — русский
юридический текст не тот домен, где spaCy LOC/ORG можно брать с дефолтным порогом.
(4) Вес: 489.6 MB для lg против 2 MB Slovnet.

### 6.6 DeepPavlov — берём цифры, не берём код

`[ПРОВЕРЕНО]` GitHub API (репозиторий переехал с `deepmipt/DeepPavlov` на **`deeppavlov/DeepPavlov`**,
API отдаёт `Moved Permanently`, id 111113343): **6 997 ★**, **Apache-2.0**, последний push
**2025-08-06**, не архивирован. README: «built on **PyTorch** and **transformers**»;
**«Depending on the model used, you may need from 4 to 16 GB RAM»** — это убивает применимость
(наш Balanced ≤250 MB). Модели русского NER из таблиц Slovnet `[ПРОВЕРЕНО]`:

| Модель | init, s | диск, MB | RAM, MB | Скорость |
|---|---|---|---|---|
| `deeppavlov` (NER) | 5.9 | **1 024** | **3 072** | 24.3 it/s (GPU) |
| `deeppavlov_bert` | 34.5 | **2 048** | **6 144** | 13.1 it/s (GPU) |
| `deeppavlov_slavic` | 35.0 | **2 048** | **4 096** | 8.0 it/s (GPU) |

**Все три — на GPU** в этих замерах; на CPU не заявлено. **Вердикт:** берём **цифры качества**
(`deeppavlov_bert` F1 0.971 PER / 0.928 LOC / 0.825 ORG на factRuEval — верхняя планка русского NER)
и **цифры веса** для обоснования Slovnet. Код не берём: PyTorch, 1–2 GB диск, ≥4 GB RAM, 35 s init.

### 6.7 Sberbank NLP, Studio Ousia, SLIM/NEREL

`[НЕ НАЙДЕНО]`/`[НЕ ПРОВЕРЕНО]` — честно фиксирую пробелы:

- **Sberbank NLP:** открытой библиотеки для **обезличивания/PII** не нашёл. Модели `sberbank-ai/*`
  на HF существуют и используются как base (`ai-forever/ruBert-base` — base для
  `rubert-base-pii-ner` `[ПРОВЕРЕНО]`), но **открытого PII-детектора от Sber не нашёл**. Проверить
  в фазе 4.
- **Studio Ousia:** `[НЕ НАЙДЕНО]` в контексте PII/анонимизации русского.
- **SLIM:** `[НЕ ПРОВЕРЕНО]` — не установил, о каком проекте речь (возможные кандидаты:
  SPARQL-связанные проекты, `slim-ai` entity linking, метод дистилляции). **Требует уточнения
  постановщиком.**
- **NEREL** `[ПРОВЕРЕНО]` — это **датасет** русского NER, на HF несколько реплик:
  `iluvvatar/NEREL` (133 загрузки, 9 ❤), `danasone/nerel` (227), `hvostchedUser/NEREL_inno` (98),
  `bond005/NEREL_bench` (93), `soradan/nerel_short` (50), `RefalMachine/nerel-bio-simple` (33).
  Лицензии реплик — по каждой свои `[НЕ ПРОВЕРЕНО]`. Схема богаче PER/LOC/ORG, но категорий нашего
  реестра там нет.
- **RuCoCo** `[НЕ НАЙДЕНО]` на HuggingFace (поиск `rucoco` пуст). Мы в 00-charter §2.6 ссылаемся
  на RuCoCo (arXiv 2206.04925) для кореференции — под этим именем датасета в HF нет, искать надо
  в другом месте. **Открытый риск для ворот ContextGuard.**

### 6.8 Юрисдикции СНГ

`[ПРОВЕРЕНО]` — нашлось ровно то, что нужно нашему §4.2 реестра. **MihaRS2/llm-pii-proxy-showcase**
(Kazakhstan, 0 ★, MIT, push 2026-09-21) — прод-прокси для юридической платформы в Казахстане,
**самый близкий к нам проект по постановке**:

Сущности и валидация: `KZ_BIN` (12 цифр) — национальная контрольная сумма, без метки «БИН:»
считаются только checksum-valid; `KZ_IIN` — checksum **плюс реальная календарная дата рождения
внутри номера**; `KZ_IIK` — полный IBAN mod-97 (ISO 13616); `KZ_PHONE`, `KZ_COMPANY`, `KZ_ADDRESS` —
казахстанские паттерны (ТОО/АО/ИП/LLC…, «г./ул./д.»); `PERSON` — ФИО **в любом падеже**, только
отчество, казахские «…улы/…кызы»; `EMAIL`, `URL`, `LOCATION`, `ORGANIZATION` — Presidio built-ins с
повышенными порогами; `DATE` маскируется **только в личном контексте** (дни рождения, паспорта),
сроки договоров остаются читаемыми.

Ключевое про IIN: **«checksum plus a real calendar birth date embedded in the number — „Feb 31“
used to slip through range checks»** `[ПРОВЕРЕНО]`. Это ровно наш урок про паспорт РФ: одной
форматной проверки мало, нужен второй признак, и второй признак — **не любая проверка, а
семантически связанная** (дата должна быть реальной датой, а не в диапазоне 1–31).

**Другие инженерные решения, которые надо забрать** (полная таблица — в
[`06-russian-tools-table.md`](06-russian-tools-table.md)): солёные плейсхолдеры (`[KIND_<10 hex>]`,
**blake2b с перцем из окружения**; нет перца → сервис отказывается стартовать); согласованность
«same value → same placeholder»; разрешение перекрытий по confidence → приоритет вида → длина;
стоп-лист по **последовательностям лемм**; **SSE-safe восстановление** с буфером до последней
незакрытой `[`; fail-closed (`ANONYMIZE_ENABLED=false` только с `ALLOW_PASSTHROUGH_FOR_TESTS`);
аудит-лог только видов и количеств; усечение инструментального вывода **до** анонимизации
(«one search result once returned 5.99 M characters»); деградация без утечки при падении Redis;
длинные тексты дробятся по границам предложений и анонимизируются **целиком**.


**Стек:** Python 3.12, FastAPI, httpx (streaming), **Microsoft Presidio**, spaCy
(**`ru_core_news_md`**, `en_core_web_sm`), **pymorphy3**, Redis (mapping, TTL 24 ч), PostgreSQL
(audit), Docker. **32 теста. MIT.** Это прецедент, доказывающий наш тезис: Presidio +
`ru_core_news_md` + pymorphy3 — рабочая связка для русского, но она даёт **плейсхолдеры**, а не
суррогаты, и требует Redis+Postgres. Наш ход — тот же результат в одном Rust-бинаре, с суррогатами
и без внешних хранилищ.

**Другие СНГ-проекты** `[ПРОВЕРЕНО]`: `k1ddy/kazpii-bench` (0 ★, без лицензии, push 2026-10-12) —
«Open benchmark for PII detection in **Kazakh and Russian** texts: generator, **KazNERD** ingest,
harness, leaderboard», 762 KB; `bekzod/yashirgich` (3 ★, без лицензии, 2026-02-15) — «**Uzbek/Russian**
PII detection and masking using ML (**rubai**) + rule-based (Presidio)»; `geekpartnerslab-kz/llmguard`
(2 ★, без лицензии, 2026-04-24) — «Open-source PII anonymization proxy between your apps and LLMs».

**Вывод по СНГ.** Рынок **уже шевелится**, но всё это 0–3 звезды, без лицензий в двух случаях из
трёх, всё на Python + Presidio. **Казахстан — единственная юрисдикция СНГ с реальным прод-опытом**
(llm-pii-proxy-showcase); **Беларуси (УНП), Узбекистана, Украины — открытых обезличивателей не
нашёл** `[НЕ НАЙДЕНО]` (есть только валидаторы внутри мультиязычных пакетов, напр. `python-stdnum` в
зависимостях scrubadub `[ДОК]`); **Россия** — есть `pii-guard` (сильный) и разрозненные заготовки.

**Что это значит для нашего §4.2 реестра:** пометки `unverified` у `kz.iin`, `kz.bin`, `by.unp`
подтверждаются как верные — **открытых эталонных реализаций для сверки почти нет**. Единственная
найденная реализация казахских контрольных сумм — `kz_recognizers.py` в
`MihaRS2/llm-pii-proxy-showcase` (32 теста на синтетике, MIT) `[ПРОВЕРЕНО по README]`. Это
лучший кандидат для сверки формул — но именно **кандидат**, не эталон: авторы сами пишут
«brand, domains, hosts and the client's own test suite removed».

---

## 7. Проприетарные ориентиры

Цель — не «что купить», а **какие архитектурные уроки извлечь**. Каждый урок должен стать
требованием к нам.

### 7.1 Microsoft Purview — единственный, у кого есть русские SIT

`[ПРОВЕРЕНО]` [SIT entity definitions](https://learn.microsoft.com/en-us/purview/sit-sensitive-information-type-entity-definitions),
обновлено **2026-06-15**. В полном списке SIT есть раздел по России:

| Русская SIT | Что описано |
|---|---|
| **Russian Passport Number (Domestic)** | 10 цифр: 2 + опц. пробел/дефис + 2 + опц. пробел + 6; **`Checksum: No`** |
| **Russian Passport Number (International)** | — |
| **Russia physical addresses** | — |
| **Russia taxpayer identification number** | 10 или 12 цифр, **с контрольной суммой** |

Аргумент «русский никому не нужен» опровергается наличием этих SIT: русский слой востребован и в
enterprise.

**Механика «primary + secondary + confidence» — главный урок** `[ПРОВЕРЕНО]`. Для ИНН РФ дословно:
**high confidence** — если в пределах **300 символов** найдены и `Func_Russian_Taxpayer_Identification_Number`
(паттерн), и ключевое слово; **medium confidence** — если найдена только функция. XML-определение:
`<Entity patternsProximity="300" recommendedConfidence="85">` с двумя `<Pattern>` —
`confidenceLevel="85"` (функция + ключ) и `confidenceLevel="75"` (только функция). Публичная шкала
`[ПРОВЕРЕНО]`: **Low ≤ 65, Medium = 75, High = 85.**

**Русские ключевые слова, дословно** `[ПРОВЕРЕНО]`:
`НДС, номер налогоплательщика, Налогоплательщика ИД, налог число, налогчисло#, ИНН, НДС#`. Ошибки
бросаются в глаза: **`НДС` — это НДС (VAT), а не ИНН**; `Налогоплательщика ИД` — калька с
английского. Для паспорта РФ список содержит «паспорт **нет**», «**p**оссийской паспорт» (латинская
`p`), «паспортid#» — часть явные опечатки, часть OCR-артефакты. **Урок: даже Microsoft в русском
списке ключевых слов ошиблась — русские контекстные слова надо собирать и валидировать на корпусе,
а не переводить.** Это подкрепляет наш пункт «определить отрицательные метки эмпирически».

**Для паспорта РФ** `[ПРОВЕРЕНО]`: «Checksum: **No**», максимум **medium confidence (75)** —
**независимое подтверждение нашего `context_required` для `ru.passport`**. **Близость 300 символов —
более слабая версия нашего L1:** Purview не различает, какой ключ ближе, а pii-guard решил это
расстоянием (§6.2), так что **здесь мы легко можем быть лучше Purview**.

**Про суррогаты в Purview:** `[НЕ НАЙДЕНО]` механики генерации фейковых значений в SIT-документации.
Azure AI Language PII Presidio предлагает как **remote recognizer**
(`docs/samples/python/text_analytics/`) — Microsoft предлагает **своё облако** как способ закрыть
пробелы Presidio `[ПРОВЕРЕНО]`.

### 7.2 Google Cloud DLP / Sensitive Data Protection

`[ПРОВЕРЕНО]`, документация обновлена 2026-10-13. **Единственный, у кого суррогаты — продуктовая
функция** (из официального списка сэмплов):

- **«De-identify free text with FPE by using a surrogate»** (`dlp-deidentify-free-text-with-fpe-using-surrogate`)
- «Re-identify free text with FPE using a surrogate»
- «Format-preserving encryption (FPE)», «De-identify table data with format-preserving encryption»
- «De-identify content through **deterministic** encryption», «Character masking», «Date shifting»,
  «table bucketing», «Redacting with matched input values», «replacing with infoType», «conditional logic»

**Концептуальный вывод, который мы обязаны забрать.** Google решает задачу «реалистичный суррогат»
**через Format-Preserving Encryption, а не через генератор фейков**. Свойства FPE: результат
**сохраняет формат** (16-значная карта → 16-значная); **обратим** детерминированно тем же ключом;
**не требует справочников** и не может «попасть в реальный реестр»; детерминирован (одно значение →
один суррогат), то есть закрывает наш I5.

Наш `similar_fake` (00-charter §5.4) — это **гибрид**: FPE даёт формат и детерминизм, но не даёт
правдоподобия (распределение по регионам, валидная контрольная сумма). Пункт 3 нашего §5.4
(«правдоподобие») FPE **не закрывает**. Значит, нужен либо FPE + пост-коррекция контрольной суммы,
либо явный генератор с валидацией. **Это архитектурный выбор, который аудит вытаскивает наверх.**

**Кастомные детекторы и уверенность** `[ПРОВЕРЕНО]`: три вида — regular dictionary, **stored
dictionary (до десятков миллионов фраз, Bloom-фильтры в Cloud Storage)**, regex; уровень
`minLikelihood` от `VERY_UNLIKELY` до `VERY_LIKELY`; hotword rules; exclusion lists.

**Русские infoType и языковая поддержка:** `[НЕ ПРОВЕРЕНО]` — страница `language-support` вернула
**404**; число встроенных infoType-детекторов не получил. Русские типы **не подтвердил** →
«допроверить в фазе 4».

**Что берём:** FPE-суррогат как механизм; полный чек-лист типов трансформаций (redact, replace, mask,
crypto-hash, deterministic crypto, FPE, bucketing, date shifting, generalization, conditional) как
проверку полноты нашей `Substitution.kind`; разделение де- и ре-идентификации как двух API; stored
dictionary до десятков миллионов фраз.

### 7.3 AWS Comprehend и Bedrock Guardrails

**Comprehend — русского нет вообще** `[ПРОВЕРЕНО]`
([supported-languages](https://docs.aws.amazon.com/comprehend/latest/dg/supported-languages.html)):
базовый список — de, en, es, it, pt, fr, ja, ko, hi, ar, zh, zh-TW; **русского нет**.
«Detecting PII entities» и «Labeling PII entities» — **только English and Spanish**. То есть
Comprehend не может обработать русский текст **ни в одной** функции.

**Bedrock Guardrails — механика, которую надо забрать** `[ПРОВЕРЕНО]`
([sensitive information filters](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-sensitive-filters.html)):
«a **probabilistic ML based solution** that is **context-dependent**»; режимы **Block** и **Mask**,
где Mask «replaces it with the PII type (for example, `{NAME}` or `{EMAIL}`)» — **это плейсхолдер,
не суррогат**. Типы — **32 PII** в 6 группах: General (ADDRESS, AGE, NAME, EMAIL, PHONE, USERNAME,
PASSWORD, DRIVER_ID, LICENSE_PLATE, VIN), Finance (CVV, EXPIRY, CARD_NUMBER, PIN, IBAN, SWIFT),
IT (IP, MAC, URL, AWS_ACCESS_KEY, AWS_SECRET_KEY), USA (5), Canada (2), UK (3) + Custom regex.
**Русских типов нет ни одного.**

Четыре урока: (1) **явное требование контекста** — «The PII model performs more effectively when it
is provided with **sufficient context**. … **avoid submitting single words or short phrases**. … a
string of digits might represent an AWS KMS key or a user ID depending on the surrounding
information» — **это наш L1-каскад, сформулированный вендором**. (2) **Явные слепые зоны** — «it does
not evaluate … PII the model generates into **tool call arguments** … PII in **tool results** … PII in
the **tool definitions**» — для нас это будущая проблема v0.2, и раздел «чего не умеем» должен её
предвидеть. (3) **Утечка через трассировку by design** — «the `match` field in
`GuardrailPiiEntityFilter` … contains the **original PII value**»; «the `input` field in Amazon
CloudWatch Logs always contains the **original, unmodified request** regardless of guardrail
intervention» → **наш I10 — прямое конкурентное преимущество**. (4) **Ограничение кастомного regex** —
«does **not support a regex lookaround match**».

### 7.4 Сводная таблица проприетарных ориентиров

| | **Purview** | **Google DLP / SDP** | **AWS Comprehend** | **Bedrock Guardrails** | **Cloudflare AI Gateway** |
|---|---|---|---|---|---|
| **(a) Русский** | **Да**, 4 SIT: паспорт РФ ×2, адреса РФ, ИНН РФ `[ПРОВЕРЕНО]` | `[НЕ ПРОВЕРЕНО]` — страница language support 404 | **Нет** — PII только «English and Spanish»; русского нет в списке языков `[ПРОВЕРЕНО]` | **Нет** — 0 ru-типов из 32 `[ПРОВЕРЕНО]` | `[НЕ НАЙДЕНО]` ru-профилей `[ПРОВЕРЕНО]` |
| **(b) Суррогаты** | `[НЕ НАЙДЕНО]` в SIT-документации | **Да** — FPE-суррогат, отдельные сэмплы de/re-identify `[ПРОВЕРЕНО]` | Нет | **Нет** — маска = `{NAME}`, `{EMAIL}` `[ПРОВЕРЕНО]` | **Нет** — только FLAG/BLOCK `[ПРОВЕРЕНО]` |
| **(c) Формат / checksum** | Формат да (паттерны); валидность суррогата не заявлена | FPE сохраняет формат by design; валидность checksum не заявлена | — | Плейсхолдер, формат не сохраняется | — |
| **(d) Офлайн** | Нет (облако) | Нет | Нет | Нет | Нет |
| **(e) Кастомные типы** | Да — настраиваемые SIT + trainable classifiers `[ДОК]` | Да — 3 вида: regular dictionary, **stored до десятков миллионов**, regex `[ПРОВЕРЕНО]` | Да — Custom Entity Recognition, но не для ru | Да — **Regex filter** (без lookaround) `[ПРОВЕРЕНО]` | Да — Custom Profiles `[ПРОВЕРЕНО]` |
| **(f) Confidence + вторичные признаки** | **Образцово:** `recommendedConfidence=85`, два Pattern (85 = функция+ключ, 75 = только функция), `patternsProximity="300"`, шкала Low≤65/Medium=75/High=85 `[ПРОВЕРЕНО]` | Да — `minLikelihood` VERY_UNLIKELY…VERY_LIKELY, hotword rules, exclusion lists `[ПРОВЕРЕНО]` | Да — confidence scores | — | Да — «Tune confidence levels» `[ПРОВЕРЕНО]` |

**Главный вывод раздела 7.** Из четырёх крупных облаков **только Purview имеет русский слой**
(4 SIT), **только Google имеет суррогаты** (через FPE), и **ни одно не имеет русских суррогатов**.
Пересечение «русский × суррогаты × офлайн» пусто **даже среди проприетарных**.

**И третий сильный аргумент: у вендоров сломаны базовые вещи**, которые мы объявили инвариантами:
AWS **by design** пишет исходный текст в CloudWatch и возвращает исходное значение в trace → наш I10
выигрывает; Purview **не может** дать выше medium для паспорта РФ → наше `context_required` верно;
Purview **ошибается** в русских контекстных словах (`НДС` вместо `ИНН`) → наш пункт «определить метки
эмпирически» обоснован, а не перфекционизм; Cloudflare **не умеет** обезличивать вообще (только
блокировать) → ниша обезличивания в шлюзах открыта.

---

## 8. Что берём, что отвергаем и почему

### 8.1 Что берём — по решениям

Полная таблица (49 пунктов с источниками) — в [`05-take-reject.md`](05-take-reject.md). Сводка по источникам:

| Откуда | Что берём | Зачем нам |
|---|---|---|
| **pii-guard** | Приоритет правил над моделью; **расстояние до ближайшего ключа** вместо «есть ли ключ в окне» (измеренные бюджеты: BIK 40, паспорт 80/160, p99=83); **наружу отдаётся значение, а не фраза**; восстановление с согласованием падежа (pymorphy3+spaCy); предобработка (транслит → кириллица, числа словами, исключение base64); пометка «смещения относятся к `normalized_text`»; честное предупреждение про GPLv3-extra | Прямое усиление нашего L1 и русской части; падежное восстановление не делает никто из западных решений |
| **Presidio** | Явные правила разрешения перекрытий; контекстное усиление отдельным слоем + `context_matching_mode`; `low_confidence_score_multiplier: 0.4` и `labels_to_ignore` для шумных NER-меток; задокументированная модель угроз «нет аутентификации by design»; немецкий набор как **шаблон пакетного вклада юрисдикции** (13 рекогнайзеров, ISO 7064 Mod 11,10, DRV, GKV, legal basis) | Шаблон для `rules/ru/`; честность про шумные метки; формат публичной модели угроз |
| **SurrogateShield** | ShadowMap (AES-256-GCM + HKDF + AAD); **битый файл карты не заменяется пустым**; fail-closed с `DetectorUnavailable`; **проверка весов по SHA-256** на фиксированный коммит; whole-value matching при восстановлении + `exact_miss`/`fuzzy_hit`/`fuzzy_miss`; пресеты; k-anonymity-скоринг (10 комбинаций); форматная валидность суррогатов | Наши I3/I4/I6 и качество восстановления; защита от подмены модели |
| **llm-pii-proxy-showcase** | **Солёный хэш фейка с перцем из окружения** («unsalted hash of a 12-digit BIN is brute-forceable in minutes») и отказ стартовать без перца; **SSE-safe восстановление** (буфер до последней незакрытой `[`); стоп-лист по **последовательностям лемм**; маскирование даты только в личном контексте; второй признак **семантически связан** (реальная календарная дата внутри KZ_IIN против range-check); пороги ORG/LOC ≥0.85 и длина ≥5 для юридического русского текста; усечение инструментального вывода **до** анонимизации; деградация без утечки при падении Redis; аудит-лог без значений | I5, I9, I10, стриминг, сохранение смысла |
| **Natasha / Slovnet** | NER-провайдер: **2 MB модель, 27 MB диск, 205 MB RAM, CPU, Numpy, MIT** | Единственный инструмент, попадающий в профиль Balanced |
| **pii-bench-ru-en** | **`pymorphy3`** как дешёвый детектор ФИО (F1 NAME 0.794, любой падеж); **`ru_core_news_sm`** — 14.6 MB за +0.116 F1 по именам и ненулевой ADDRESS (0.507); **отрицательная аддитивность** (morph+spacy хуже, чем spacy); датасеты `hivetrace/pii-bench` и `pii_benchmark` | Выбор L2-провайдера и внешние замеры |
| **Google DLP** | **FPE-суррогат** (формат-сохраняющий, детерминированный, обратимый, без справочников); полный чек-лист типов трансформаций; stored dictionary до десятков миллионов фраз | Механизм суррогатов и проверка полноты нашей `Substitution.kind` |
| **Purview** | Primary/secondary + числовой confidence: `recommendedConfidence=85`, два Pattern, `patternsProximity="300"`, шкала Low≤65/Medium=75/High=85; **независимое подтверждение `context_required` для паспорта РФ**; юридическое основание в описании типа | Структура уверенности и внешняя валидация нашего реестра |
| **AWS Bedrock / Cloudflare / LiteLLM / arx / arabic-pii-py** | Явное требование контекста и запрет «single words or short phrases»; список слепых зон tool calling; ошибка вендора как урок (trace и CloudWatch содержат исходники); профили как политики + `cf-aig-dlp`; режимы guardrail и авто-дробление длинных текстов; k-anonymity-методология ARX; подтверждение стратегии «local-first reversible для одной юрисдикции» | Обоснование I10, структура `Report`, путь интеграции, методология |

### 8.2 Что отвергаем — по решениям

Полная таблица (32 пункта) — в [`05-take-reject.md`](05-take-reject.md). Ключевое:

| Что отвергаем | Причина |
|---|---|
| **Presidio как зависимость** | Python; `spacy` **обязательна** (противоречит «ядро без ML»); `en_core_web_lg` **382 MB**; **0 русских рекогнайзеров** и ни одного русского контекстного слова; нет состояния сессии; `presidio-cli` не публикуется |
| Presidio `hash` / `replace` / `mask` / `redact` / `surrogate_ahds` как целевое поведение | `hash` с 2.2.361 даёт **случайную соль по умолчанию** → нарушает I5; `replace`/`mask`/`redact` дают `<PHONE_NUMBER>`/`****1234`/удаление — потеря смысла; `surrogate_ahds` требует **облака Azure** с локалями `en-US` |
| **LLM Guard как зависимость и как «решение»** | **Архивирован 2026-07-08**, модели на HF тоже; 36 сканеров, из которых анонимизационный — один; p95 **169–294 ms на 317 символах** |
| **Portcullis как реализацию** | **Кода нет** (3 независимые проверки); TEE требует серверного CPU и не работает в WASM; 96× относится к маскированию, а не к детекции |
| **SurrogateShield как код** | Python; en-US-сущности; **2 ★**, 0 форков, bus factor = 1; цифры **self-reported**; 43.5 ms медиана и 1 348 MB пик не влезают в Balanced |
| **scrubadub / OpenAnonymiser / pydeidentify / anonymize-it / «Oban»** | Мертвы (2023), не тот язык (nl), нет лицензии, не существуют (PyPI 404), не существуют как инструмент обезличивания |
| **DeepPavlov / Tomita / `ru_core_news_lg`** | 1–2 GB диск и 3–6 GB RAM / MPL-2.0 и мёртв с 2020 / **489.6 MB** против 2 MB Slovnet |
| **Cloudflare / Comprehend / Bedrock `Mask` / LiteLLM-guardrail** | Только FLAG/BLOCK; русского нет вообще; `{NAME}` — плейсхолдер; `en/es/de` и `<PERSON>` |
| **Приватность = локальный инференс** (`private-gpt`) и **идея «мы первые делаем суррогаты»** | Первое — другая задача (данные не уходят, но не обезличиваются); второе неверно: SurrogateShield делает суррогаты с мая 2026, корректно — «первые для русского, локально, на CPU» |

### 8.3 Что меняется в наших документах по итогам аудита

Полная таблица (20 правок с обоснованиями и URL) — в [`05-take-reject.md`](05-take-reject.md).
Ключевое:

| Наш документ | Что поправить |
|---|---|
| 00-charter §2.7 | Portcullis: первый автор — Tsinghua, пятеро — ByteDance; 96× — **маскирование/снятие маски** против Hide-and-Seek |
| 00-charter §2.6 | Тезис «ни у Presidio, ни у LLM Guard, ни у Portcullis этого нет» **подтверждён**, но добавить: **ни у кого** из проверенных, включая SurrogateShield, pii-guard и все облака |
| 00-charter §2.2 п.2 | «доступ из Presidio-подобных пайплайнов» уточнить: Presidio **архивировал** `presidio-cli` и **обязательно тянет spaCy** — наша ценность в **замене**, а не «дополнении» |
| 00-charter §5.4 | Добавить требование **форматной валидности**; рассмотреть **FPE** как альтернативу генератору; разделить «формат+детерминизм» (FPE) и «правдоподобие распределения» (генератор + валидация); **фейк не должен быть валидным реальным идентификатором** (иначе атака «инверсия фейка»); **суррогат хэшируется с перцем из окружения**, без перца — отказ; детерминированность должна быть **криптографической**, а не «от seed» |
| 00-charter §5.2 | Добавить: **смещения относятся к нормализованному тексту**; для подсветки в оригинале — отдельный API переноса смещений |
| 00-charter §6.5 | Adversarial: критерий **«фейк не попадает в реальные реестры»**; добавить атаку **«брутфорс плейсхолдера»** (несолёный хэш) |
| 00-charter §7.1–7.2 | Профиль Fast **обязан** публиковать «ФИО и адрес не обнаруживаются» (подкреплено F1 NAME = 0.000); **бюджет L2 NER 12 ms не обоснован** (Slovnet ~39.5 ms/KB) → нужен собственный замер; `ru_core_news_sm` (14.6 MB) — компактная альтернатива L2 |
| 00-charter §5.1, §10, анти-цели | Добавить в `Report` машиночитаемую структуру о срабатывании (по образцу `cf-aig-dlp`); добавить риск **PII в аргументах/результатах инструментов** (tool calling); **не затаскивать копилефт через optional extra** — проверка лицензий дерева зависимостей в CI |
| 20-entity-registry §5 | Каскад L1: **заменить «окно ±N токенов» на расстояние до ближайшего ключа** с измеренными бюджетами; добавить приём «наружу отдаётся значение, а не фраза» |
| 20-entity-registry §4.1 | Для `ru.passport` **независимое подтверждение**: Purview ставит максимум **medium (75)**, `Checksum: No`. Для ИНН: **реальный F1 — 68.3 у чистой модели**, 88.9 у пайплайна — публиковать отдельно, не средним |
| 20-entity-registry §3, §4.2 | Сверка весов СНГ: **единственный кандидат** — `kz_recognizers.py` в showcase (MIT, 32 теста), но клиентский набор убран → **остаётся `unverified`**; для `kz.iin` второй признак = **реальная календарная дата внутри номера**, а не range-check |


## 9. Пробелы рынка, которые мы займём

Полный разбор (три пробела + второстепенные + «чего мы не должны обещать») — в
[`07-market-gaps.md`](07-market-gaps.md). Суть:

**Пробел 1: русский + реалистичные суррогаты + локально.** Пусто **и** в open-source (§5, §6),
**и** у проприетарных (§7). Единственные русские суррогаты не существуют; единственные суррогаты
с русским — не существуют; Google имеет суррогаты, но не имеет подтверждённого русского; Purview
имеет русский, но не имеет суррогатов. Полная матрица — в Приложении B.

**Пробел 2: связность текста и контекстные утечки в диалоге** `[ПРОВЕРЕНО]` — этого нет **ни у
кого** из проверенных: Presidio прямо говорит «Presidio does not store or maintain stateful
sessions»; pii-guard — «Сервис не имеет состояния: ни Redis, ни TTL, ни идентификатора сессии»;
SurrogateShield держит карту, но не моделирует анафору; Portcullis не адресует; облака — тоже.
Наш ContextGuard (L3) и I2 — **полностью свободная ниша**.

**Пробел 3: один бинарь, Rust, влезающий в WASM и в 25 ms** `[ПРОВЕРЕНО]` — **все** проверенные
решения на Python (кроме `phileas` на Java), **ни одного на Rust**. Balanced (≤25 ms, ≤250 MB)
недостижим для Python-стека: LLM Guard p95 169 ms на 317 символах, SurrogateShield 43.5 ms медиана
и 1 348 MB пик, Presidio 8.2 ms медиана и 1 084 MB пик (на коротком тексте). Проверенный минимум
по весу в русском стеке — Slovnet **27 MB диск / 205 MB RAM**, и он на Python+Numpy. Наш Fast-профиль
(≤5 ms, ≤30 MB, WASM) **аналогов не имеет вообще**.

**Второстепенные пробелы:** восстановление **с согласованием падежа** — только pii-guard и только
для псевдонима (для суррогата-имени не решено никем); суррогаты **с валидной контрольной суммой** для
русских ID — только SurrogateShield и только для США; **проверка весов модели по хэшу** — только
SurrogateShield; **fail-closed как проверяемый инвариант** — SurrogateShield и showcase, но никто не
формулирует это как C4 с 8 сценариями сбоя; **публикация FP(clean) на чистом корпусе** — `[НЕ НАЙДЕНО]`
ни у кого; **стоп-лист по леммам** и **SSE-safe восстановление** — только showcase на 0 звёзд;
**обезличивание логов** — **никто** (у AWS Bedrock trace и CloudWatch `input` содержат исходники by
design); **юрисдикции СНГ кроме Казахстана** — `[НЕ НАЙДЕНО]` для BY, UZ, UA.

**Чего мы не должны обещать** (аудит собрал ровно те места, где конкуренты переобещали):
«ноль ложных срабатываний» (наш замер: 9.91% для ИНН юрлица, 80.98% для паспорта РФ; Purview для
паспорта тоже не даёт выше medium) · «93% recall по ИНН» (реально 68.3 у чистой модели, 88.9 у
пайплайна) · «находим ФИО в профиле Fast» (regex-only F1 по NAME = 0.000) · «96× быстрее» без
оговорки · «PII не уходит наружу» без оговорки про tool calling · «логи не содержат PII» без
проверки · «суррогаты не восстановимы» (FPE обратим ключом; плейсхолдер с несолёным хэшем
брутфорсится за минуты) · «мы первые делаем суррогаты» (SurrogateShield с мая 2026) ·
«Balanced в 25 ms» без замера L2.

---

## 10. Библиография

Полный список из **129 URL с разбивкой по 10 тематическим группам** — в [`03-bibliography.md`](03-bibliography.md).
Ключевые источники по каждому блоку:

**Presidio** — [project transition](https://presidio.dataprivacystack.org/project_transition/) ·
[supported entities](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/supported_entities.md) ·
[languages](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/analyzer/languages.md) ·
[anonymizer operators](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/anonymizer/index.md) ·
[default.yaml](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default.yaml) ·
[default_recognizers.yaml](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default_recognizers.yaml) ·
[pyproject](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/pyproject.toml) ·
[CHANGELOG](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/CHANGELOG.md) ·
[LICENSE](https://raw.githubusercontent.com/data-privacy-stack/presidio/main/LICENSE) ·
[PyPI](https://pypi.org/pypi/presidio-analyzer/json)

**LLM Guard** — [репозиторий](https://github.com/protectai/llm-guard) ·
[README с предупреждением об архивации](https://raw.githubusercontent.com/protectai/llm-guard/main/README.md) ·
[Anonymize scanner (сущности, `use_faker`, бенчмарки p95)](https://protectai.github.io/llm-guard/input_scanners/anonymize/) ·
[PyPI](https://pypi.org/pypi/llm-guard/json) ·
[HF-модели Protect AI](https://huggingface.co/protectai)

**Portcullis / SurrogateShield** — [AAAI-25, DOI, «96x»](https://ojs.aaai.org/index.php/AAAI/article/view/32088) ·
[Portcullis PDF](https://ojs.aaai.org/index.php/AAAI/article/view/32088/34243) ·
[SurrogateShield arXiv:2606.29567](https://arxiv.org/abs/2606.29567) ·
[репозиторий](https://github.com/sherwinvishesh/SurrogateShield) ·
[README (архитектура, бенчмарки, ограничения)](https://raw.githubusercontent.com/sherwinvishesh/SurrogateShield/main/README.md)

**Русскоязычные** — [pii-guard](https://github.com/redmadrobot-rnd/pii-guard) ·
[pii-guard README](https://raw.githubusercontent.com/redmadrobot-rnd/pii-guard/main/README.md) ·
[pii-guard docs/architecture.md](https://raw.githubusercontent.com/redmadrobot-rnd/pii-guard/main/docs/architecture.md) ·
[rubert-base-pii-ner](https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner) ·
[pii_benchmark](https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark) ·
[Natasha](https://github.com/natasha/natasha) ·
[Slovnet (размеры, F1, speed)](https://raw.githubusercontent.com/natasha/slovnet/master/README.md) ·
[pii-bench-ru-en](https://raw.githubusercontent.com/Exemtik/pii-bench-ru-en/main/README.md) ·
[DeepPavlov](https://github.com/deeppavlov/DeepPavlov) ·
[spaCy ru](https://spacy.io/models/ru) ·
[Tomita](https://github.com/yandex/tomita-parser) ·
[hivetrace/pii-bench](https://huggingface.co/datasets/hivetrace/pii-bench)

**Остальные OSS** — [scrubadub](https://github.com/LeapBeyond/scrubadub) ·
[DataFog](https://github.com/DataFog/datafog-python) ·
[anonymiq-light / OpenAnonymiser](https://github.com/ConductionNL/anonymiq-light) ·
[phileas](https://github.com/philterd/phileas) ·
[arx](https://github.com/arx-deidentifier/arx) ·
[private-gpt](https://github.com/zylon-ai/private-gpt) ·
[mimesis](https://github.com/lk-geimfari/mimesis) ·
[Faker](https://github.com/joke2k/faker)

**Прокси и СНГ** — [DontFeedTheAI](https://github.com/zeroc00I/DontFeedTheAI) ·
[piighost](https://github.com/Athroniaeth/piighost) ·
[arabic-pii-py](https://github.com/Aajil-Labs/arabic-pii-py) ·
[llm-pii-proxy-showcase (Казахстан)](https://raw.githubusercontent.com/MihaRS2/llm-pii-proxy-showcase/main/README.md) ·
[kazpii-bench](https://github.com/k1ddy/kazpii-bench)

**Проприетарные** — [Purview: список SIT с 4 русскими](https://learn.microsoft.com/en-us/purview/sit-sensitive-information-type-entity-definitions) ·
[Purview: ИНН РФ](https://learn.microsoft.com/en-us/purview/sit-defn-russia-taxpayer-identification-number) ·
[Purview: паспорт РФ](https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-domestic) ·
[Google SDP: де-идентификация](https://docs.cloud.google.com/sensitive-data-protection/docs/concepts-de-identification) ·
[Google: FPE-суррогат](https://docs.cloud.google.com/sensitive-data-protection/docs/samples/dlp-deidentify-free-text-with-fpe-using-surrogate) ·
[AWS Comprehend: языки](https://docs.aws.amazon.com/comprehend/latest/dg/supported-languages.html) ·
[AWS Bedrock Guardrails](https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-sensitive-filters.html) ·
[Cloudflare AI Gateway DLP](https://developers.cloudflare.com/ai-gateway/features/dlp/set-up-dlp/) ·
[LiteLLM + Presidio](https://docs.litellm.ai/docs/proxy/guardrails/pii_masking_v2)

## 11. Что проверено / по документации / не проверено

Полные чек-листы (≈140 пунктов) — в [`04-verification-status.md`](04-verification-status.md).
Сводка:

**ПРОВЕРЕНО (открыл источник).** Presidio: переезд в `data-privacy-stack`, MIT, ~130 типов сущностей,
`supported_languages: [en]`, `en_core_web_lg`, `ORGANIZATION` в игнор-листе, 8 операторов + `decrypt`,
`surrogate_ahds` требует Azure, обязательная зависимость spaCy, версия 2.2.364 (2026-07-22),
11 345 ★ / 206 контрибьюторов, случайная соль в `hash` с 2.2.361, `presidio-cli` не публикуется. ·
LLM Guard: **архивирован**, модели на HF не поддерживаются, MIT, 28 зависимостей, 0.3.16 (2025-05-19),
36 сканеров, наследование Presidio, «English and Chinese» + «English-specific», **p95 169.13 / 294.57 ms
на 317 символах**. · Portcullis: полная библиография (Tsinghua + ByteDance, AAAI-25, стр. 1022–1030,
DOI), точный текст про 96×, **кода нет** (3 независимые проверки). · SurrogateShield: arXiv 2606.29567,
Apache-2.0, 2 ★, вся архитектура и все бенчмарк-цифры, раздел Limitations, Security Design. ·
pii-guard: 20 типов, 3 режима, Apache-2.0, 678 MiB весов, приоритет правил, расстояния до ключей
(BIK 40, паспорт 80/160, p99=83), восстановление падежа, GPLv3-предупреждение. · rubert-base-pii-ner:
Apache-2.0, 21 тип/43 метки, F1 83.6/88.9, полная таблица per-entity (INN 68.3, IP 36.0). ·
Natasha/Slovnet: 1 359 ★, MIT, NER 2 MB, диск 27 MB, RAM 205 MB, 25.3 статьи/с, полные таблицы F1. ·
pii-bench-ru-en: MIT, F1 regex_only 0.800/**NAME 0.000**, +morph 0.794, +spacy_ru 0.910. ·
llm-pii-proxy-showcase: KZ-валидация, blake2b+pepper, SSE, fail-closed, lemma-стоп-лист. ·
spaCy ru: sm 14.6 / md 39.9 / lg 489.6 MB, MIT, LOC/ORG/PER, лемматизатор pymorphy3. ·
DeepPavlov (6 997 ★, Apache-2.0, «4 to 16 GB RAM»), Tomita (MPL-2.0, мёртв с 2020). ·
scrubadub, DataFog, anonymiq-light, pydeidentify, `anonymize-it` (404), phileas, arx, mimesis, Faker,
11 прокси-проектов, Cloudflare DLP, LiteLLM — метрики и лицензии. ·
Purview: 4 русских SIT, `recommendedConfidence` 85/75, `patternsProximity="300"`, шкала 65/75/85,
русские ключевые слова (с ошибками `НДС`/`pасспорт`), паспорт РФ `Checksum: No`. ·
Comprehend: PII только «English and Spanish». Bedrock: 32 PII, `Mask` = `{NAME}`, слепые зоны tool
calling, trace и CloudWatch `input` содержат исходники. Google SDP: 3 вида кастомных детекторов,
stored dictionary «up to tens of millions», **сэмплы FPE-суррогата**.

**ТОЛЬКО ПО ДОКУМЕНТАЦИИ (не проверял в коде/исполнении).** Архитектура Portcullis в деталях (только
абстракт); кто такие Hide-and-Seek и что за нагрузка «masking and unmasking workloads»; **код pii-guard**
(читал README и docs/architecture.md, но GitHub API по репо вернул пустой объект); задержка pii-guard
(не опубликована, моя оценка «десятки–сотни мс» — не замер); лицензия `pymorphy3`; наличие русских
локалей Faker/mimesis в файлах; размеры HF-моделей `dslim/bert-base-NER`; дефолтные регионы
`PhoneRecognizer` (входит ли RU); содержимое `sensisitive_patterns.json`; русский в Azure AI Language
PII; причина расхождения лицензий scrubadub; детали 11 прокси-проектов (кроме showcase); **русские
infoTypes в Google DLP**; число встроенных infoType-детекторов Google; точные размеры моделей LLM Guard
по сканерам; **задержка Presidio** (8.2 ms — из чужого self-reported бенчмарка, не наш замер).

**НЕ НАЙДЕНО (искал, артефакта нет).** Код Portcullis (GitHub Search ×2 = 0, arXiv API пуст, на
странице статьи только PDF и постер) · `anonymize-it` (PyPI 404) · `Oban` как инструмент обезличивания
(существует только Elixir-очередь задач) · `OpenAnonymiser` как отдельный проект (есть только
`anonymiq-light`) · русские рекогнайзеры в Presidio (0) · русские PII-типы в Bedrock (0 из 32) ·
русский в Comprehend (нет в списке языков вообще) · обезличиватели для Беларуси (УНП), Узбекистана,
Украины · открытый PII-детектор от Sberbank (есть только base-модели) · Studio Ousia в контексте
PII/русского · **RuCoCo на HuggingFace** (поиск пуст — а мы ссылаемся на него в 00-charter §2.6 для
ворот ContextGuard → это открытый риск) · открытый датасет кореференции для русского ·
замеры задержки Presidio от авторов · русские контекстные слова в Presidio (ни одного) ·
русские локали `surrogate_ahds` (только `en-US`).

**ТРЕБУЕТ УТОЧНЕНИЯ ПОСТАНОВЩИКОМ.** «SLIM» — не установил, о каком проекте речь (возможные
кандидаты: SPARQL-интерфейсы, `slim-ai` entity linking, методы дистилляции) · «SurrogateShield» —
если имелось в виду что-то кроме `arXiv:2606.29567` + `github.com/sherwinvishesh/SurrogateShield`
(других артефактов под этим именем поиск не дал) · «Yandex NER» — Tomita найден и разобран, отдельного
публичного «Yandex NER» нет; Yandex Cloud NLP API — проприетарный и в список не входил ·
«Studio Ousia» — связи с PII/русским не нашёл.

---

## Приложение A. Пять самых важных находок

1. **LLM Guard архивирован, и вместе с ним модели на HuggingFace** `[ПРОВЕРЕНО]` — снимает его с
   любой повестки как зависимость и заметно обнуляет конкурентное поле в open-source.
2. **pii-guard (Apache-2.0, русский, 20 типов) делает почти всё, кроме того, что делаем мы**
   `[ПРОВЕРЕНО]`. Их `docs/architecture.md` — готовый учебник по русской детекции: приоритет правил
   над моделью, расстояние до ближайшего ключа, «наружу отдаётся значение, а не фраза», восстановление
   падежа, транслит-нормализация. Берём эти четыре решения и отличаемся в двух: **суррогаты**
   (у них псевдонимы с видимым типом) и **состояние сессии** (у них его нет вообще).
3. **SurrogateShield — главный ориентир по идее и главный конкурент по духу:** тот же набор решений,
   но English-only, self-reported, 2 звезды, 1 348 MB RAM, 43.5 ms медиана `[ПРОВЕРЕНО]`. Ниша
   реальна, и наша версия для русского, на CPU, в одном бинаре — не то же самое.
4. **Единственные опубликованные p95 для Presidio-подобного стека — у LLM Guard: 169.13 ms (ONNX) /
   294.57 ms на 317 символах** `[ПРОВЕРЕНО]`. Наш бюджет 25–50 ms на 2 000 токенов — цифра
   конкурента объективно доказывает, что Python+spaCy в наш контракт не влезает.
5. **У проприетарных вендоров сломаны базовые вещи, которые мы объявили инвариантами**
   `[ПРОВЕРЕНО]`: AWS **by design** пишет исходный запрос в CloudWatch и исходное значение в trace;
   Cloudflare **не умеет** обезличивать (только FLAG/BLOCK); Purview **не может** дать выше medium
   для паспорта РФ и **ошибается** в русских ключевых словах (`НДС` вместо `ИНН`); только Google
   имеет суррогаты — и только через облачный FPE.

## Приложение B. Пересечение «русский × суррогаты × офлайн»

Главная таблица всего аудита; клетки — из источников, перечисленных выше.

| Решение | Детектит ru | Суррогаты | Формат-валидны | Офлайн | Состояние сессии | Открытый код | Лицензия |
|---|:---:|:---:|:---:|:---:|:---:|:---:|---|
| Presidio | ✗ | ✗ (только Azure AHDS) | ✗ | ✓ | ✗ | ✓ | MIT |
| LLM Guard | ✗ | ~ (`use_faker`, не дефолт) | ✗ | ✓ | ✗ (Vault в процессе) | ✓ (архив) | MIT |
| SurrogateShield | ✗ | ✓ | ✓ (Luhn/ABA/Base58) | ✓ | ✓ (ShadowMap) | ✓ | Apache-2.0 |
| Portcullis | ✗ | ? | ? | ✗ (TEE) | ? | **✗** | — |
| pii-guard | **✓** (20 типов) | ✗ (псевдонимы) | — | ✓ | ✗ (stateless) | ✓ | Apache-2.0 |
| llm-pii-proxy-showcase | **✓** (ru + KZ) | ✗ (солёные плейсхолдеры) | — | ~ (Redis+Postgres) | ✓ (через Redis) | ✓ | MIT |
| Natasha/Slovnet | ✓ (PER/LOC/ORG) | ✗ | — | ✓ | ✗ | ✓ | MIT |
| spaCy ru | ✓ (PER/LOC/ORG) | ✗ | — | ✓ | ✗ | ✓ | MIT |
| scrubadub | ✗ | ✗ | — | ✓ | ✗ | ✓ | Apache-2.0/MIT |
| Cloudflare DLP | ✗ | ✗ (FLAG/BLOCK) | — | ✗ | ✗ | ✗ | проприетарная |
| LiteLLM + Presidio | ✗ (`en/es/de`) | ✗ (`<PERSON>`) | ✗ | ~ | ~ (unmask токенов) | ✓ | MIT |
| Microsoft Purview | **✓** (4 SIT) | ✗ | — | ✗ | ✗ | ✗ | проприетарная |
| Google DLP | ? | **✓** (FPE) | формат да, checksum ? | ✗ | ✗ | ✗ | проприетарная |
| AWS Comprehend | ✗ | ✗ | — | ✗ | ✗ | ✗ | проприетарная |
| AWS Bedrock Guardrails | ✗ | ✗ (`{NAME}`) | ✗ | ✗ | ✗ | ✗ | проприетарная |
| **privacy-core (наш план)** | **✓** | **✓** | **✓ (mod 11/13/101, Luhn)** | **✓** | **✓** | **✓** | **Apache-2.0** |

**Ни одна строка, кроме последней, не имеет пяти «✓». Пересечение «русский × суррогаты × локально ×
состояние сессии × форматная валидность» пусто** — это формулировка нашего позиционирования,
подкреплённая источниками выше.

---

## Приложения (отдельные файлы)

| Файл | Содержание |
|---|---|
| [`02-presidio-tables.md`](02-presidio-tables.md) | Presidio: полный перечень рекогнайзеров по 20 регионам и покомпонентная матрица «тип → русский» |
| [`03-bibliography.md`](03-bibliography.md) | Библиография: 127 URL в 10 тематических группах |
| [`04-verification-status.md`](04-verification-status.md) | Статус проверки: ≈140 пунктов (проверено / по документации / не найдено / требует уточнения) |
| [`05-take-reject.md`](05-take-reject.md) | Полные таблицы «что берём» (49) и «что отвергаем» (32) + 20 правок наших документов |
| [`06-russian-tools-table.md`](06-russian-tools-table.md) | Русскоязычные инструменты: полная таблица, данные и качество pii-guard, EN-результаты, инженерия КЗ |
| [`07-market-gaps.md`](07-market-gaps.md) | Пробелы рынка: три главных и второстепенные, «чего мы не должны обещать» |
| [`08-other-oss.md`](08-other-oss.md) | Прочие OSS: scrubadub/DataFog/OpenAnonymiser, 11 прокси-проектов, LiteLLM, инженерные решения SurrogateShield |
| [`09-not-found.md`](09-not-found.md) | Что не найдено и что требует уточнения постановщиком |
