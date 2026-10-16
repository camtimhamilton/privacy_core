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
| **Реалистичные суррогаты локально** | SurrogateShield (en), Google DLP FPE (облако) | Presidio (только через Azure), LLM Guard (плейсхолдеры), pii-guard (теги), Bedrock (`{NAME}`) |
| Суррогаты **с валидной контрольной суммой** | SurrogateShield (Luhn/ABA), Google DLP FPE | Presidio, LLM Guard, pii-guard, Bedrock |
| **Русский + суррогаты + локально** | **никто** | — |
| Разрешение ссылок/анафоры в диалоге | **никто** из проверенных | Presidio, LLM Guard, pii-guard, SurrogateShield |
| Восстановление **в правильной падежной форме** | pii-guard (pymorphy3 + spaCy) | все остальные |

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

| Компонент | Что делает | Ключевой класс |
|---|---|---|
| `presidio-analyzer` | Находит PII, возвращает спаны + score + entity_type | `AnalyzerEngine`, `RecognizerRegistry`, `NlpEngine` |
| `presidio-anonymizer` | Превращает спаны в текст по операторам; умеет обратно | `AnonymizerEngine`, `DeanonymizeEngine`, `Operator` |
| `presidio-image-redactor` | PII на изображениях, DICOM/OCR | — |
| `presidio-structured` | Полу- и структурированные данные | — |

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

### 2.3 Полный перечень рекогнайзеров по регионам

Всё из `[ПРОВЕРЕНО]` `docs/supported_entities.md` (raw, main) — таблица ниже это фактическая
выборка из файла, без домыслов. `RU` — колонка «есть ли русский».

| Регион | Типы сущностей | Кол-во | RU |
|---|---|---|---|
| **Global** | CREDIT_CARD, CRYPTO, DATE_TIME, EMAIL_ADDRESS, IBAN_CODE, IP_ADDRESS, MAC_ADDRESS, NRP, LOCATION, PERSON, PHONE_NUMBER, MEDICAL_LICENSE, URL, UUID | **14** | **нет** |
| USA | US_BANK_NUMBER, US_DRIVER_LICENSE, US_ITIN, US_CLAIM_NUMBER, US_HEALTH_INSURANCE_MEMBER_ID, US_MBI, US_NPI, US_PASSPORT, US_PRESCRIPTION_NUMBER, US_PRIOR_AUTHORIZATION_NUMBER, US_PROVIDER_TAX_ID, US_REFERRAL_NUMBER, US_SSN | 13 | нет |
| UK | UK_DRIVING_LICENCE, UK_NHS, UK_NINO, UK_PASSPORT, UK_POSTCODE, UK_VEHICLE_REGISTRATION | 6 | нет |
| Spain | ES_NIF, ES_NIE, ES_PASSPORT | 3 | нет |
| Italy | IT_FISCAL_CODE, IT_DRIVER_LICENSE, IT_VAT_CODE, IT_PASSPORT, IT_IDENTITY_CARD | 5 | нет |
| Poland | PL_PESEL | 1 | нет |
| Singapore | SG_NRIC_FIN, SG_UEN | 2 | нет |
| Australia | AU_ABN, AU_ACN, AU_TFN, AU_MEDICARE | 4 | нет |
| India | IN_PAN, IN_AADHAAR, IN_VEHICLE_REGISTRATION, IN_VOTER, IN_PASSPORT, IN_GSTIN | 6 | нет |
| Finland | FI_PERSONAL_IDENTITY_CODE | 1 | нет |
| Korea | KR_DRIVER_LICENSE, KR_FRN, KR_PASSPORT, KR_BRN, KR_RRN | 5 | нет |
| Nigeria | NG_NIN, NG_VEHICLE_REGISTRATION | 2 | нет |
| Philippines | PH_PASSPORT, PH_TIN, PH_UMID | 3 | нет |
| Canada | CA_SIN, CA_POSTAL_CODE | 2 | нет |
| Sweden | SE_ORGANISATIONSNUMMER, SE_PERSONNUMMER | 2 | нет |
| South Africa | ZA_ID_NUMBER, ZA_PASSPORT, ZA_INCOME_TAX_NUMBER, ZA_DRIVER_LICENSE, ZA_VAT_NUMBER, ZA_COMPANY_REGISTRATION, ZA_TRAFFIC_REGISTER_NUMBER, ZA_LICENSE_PLATE, ZA_MOBILE_NUMBER, ZA_TELEPHONE_NUMBER | 10 | нет |
| Thai | TH_TNIN | 1 | нет |
| Turkey | TR_NATIONAL_ID, TR_LICENSE_PLATE | 2 | нет |
| Germany | DE_TAX_ID, DE_TAX_NUMBER, DE_PASSPORT, DE_ID_CARD, DE_SOCIAL_SECURITY, DE_HEALTH_INSURANCE, DE_KFZ, DE_HANDELSREGISTER, DE_PLZ (`default_recognizers.yaml` добавляет DE_LANR, DE_BSNR, DE_VAT_ID, DE_FUEHRERSCHEIN) | 9–13 | нет |
| Medical / Clinical | MEDICAL_DISEASE_DISORDER, MEDICAL_MEDICATION, MEDICAL_THERAPEUTIC_PROCEDURE, MEDICAL_CLINICAL_EVENT, MEDICAL_BIOLOGICAL_ATTRIBUTE, MEDICAL_BIOLOGICAL_STRUCTURE, MEDICAL_FAMILY_HISTORY, MEDICAL_HISTORY | 8 | нет |
| **Итого** | | **≈ 130 predefined-типов** | **0** |

Отдельно: Германия получила **цельный национальный набор** (13 рекогнайзеров одной PR-серией,
CHANGELOG 2.2.363, автор @MvdB) `[ПРОВЕРЕНО]`. Вывод: «добавить юрисдикцию» в Presidio — это
пакетный вклад на 12–13 типов. Русский слой — ровно такая пачка, и она уже описана в нашем реестре.

**Русских рекогнайзеров нет ни одного.** В `default_recognizers.yaml` поле верхнего уровня
буквально `supported_languages: - en` `[ПРОВЕРЕНО]`; единственное упоминание `kr` — устаревший
алиас для корейского.

### 2.4 Таблица «тип сущности Presidio → поддержка русского»

Требование задания. Здесь важно правильно прочитать: «поддержка русского» распадается на два
разных вопроса — (а) работает ли детектор на русском тексте, (б) есть ли русская специфика
(контекстные слова, формат). У Presidio ответ (а) часто «случайно да» для форматных типов
и «нет» для NER, а (б) — почти всегда «нет».

| Тип сущности | Механизм детекции в Presidio | Работает ли на русском тексте | Русские контекстные слова | Что ломается на русском |
|---|---|---|---|---|
| CREDIT_CARD | pattern + checksum | **Да** (Luhn от языка не зависит) | **Нет** (`credit, card, visa…` только en/es/it/pl) | Контекст не сработает → score не поднимется → FP на порядковых номерах |
| CRYPTO | pattern + context + checksum | Частично (только Bitcoin) | Нет | — |
| DATE_TIME | `DateRecognizer` (regex) | **Да** для ISO/числовых форматов | Нет | «12 марта 2024 г.» не распознаётся `[НЕ ПРОВЕРЕНО]` — не тестировалось |
| EMAIL_ADDRESS | pattern + RFC-822 + `tldextract` | **Да** | Нет | Куриллические домены (punycode) — есть фикс #2077 `[ПРОВЕРЕНО]` |
| IBAN_CODE | pattern + mod-97 | **Да** | Нет | Российских IBAN нет; реестр дополняли UA/EG/IQ/LY/LC/SC (#2078) — РФ не добавляли |
| IP_ADDRESS | pattern + context | **Да** | Нет | — |
| MAC_ADDRESS | pattern + context | **Да** | Нет | — |
| NRP | NER (spaCy) | **Нет** (нужна ru-модель, у которой этого класса нет) | — | Класса NORP нет в `ru_core_news_*` |
| LOCATION | NER + маппинг | **Только при подмене NLP-движка** на ru | — | Дефолт en → на русском не находит |
| PERSON | NER (spaCy `PER`/`PERSON`) | **Только при подмене NLP-движка** | — | То же; качество см. §6 |
| PHONE_NUMBER | `phonenumbers` (libphonenumber) + регионы | **Да** — `supported_regions` включают RU `[НЕ ПРОВЕРЕНО]` (список дефолтных регионов не открывал) | **Нет** | Баг `UK`→`GB` исправлен только в 2.2.363: «UK entry was a no-op» `[ПРОВЕРЕНО]` — то есть реестр регионов реально ломался |
| MEDICAL_LICENSE | pattern + context + checksum | Нет (US-ориентированный) | Нет | — |
| URL | pattern + tldextract | **Да** | Нет | — |
| UUID | pattern + version/variant nibbles | **Да** | Нет | — |
| **US_\*** (13) | pattern + context | Формально да | **Нет** | Бесполезны; в РФ-тексте дадут FP на любых 9-значных числах |
| **UK_\*** (6) | pattern + context (+ checksum у NHS) | Формально да | Нет | — |
| **ES/IT/PL/FI/SE/TR/TH/KR/NG/PH/CA/ZA/AU/IN/SG** | pattern + context | Формально да | **Нет** | Контекстные слова на этих языках; для русского текста не сработают |
| **DE_\*** (13) | pattern + context + checksum | **Да** (немецкие контекстные слова) | Немецкие | Для русского бесполезны, но это **образец качества реализации**: ISO 7064 Mod 11,10, DRV-алгоритм, GKV-алгоритм `[ПРОВЕРЕНО]` |
| MEDICAL_\* (8) | `MedicalNERRecognizer` → `blaze999/Medical-NER` | Нет (модель английская) | — | — |

**Главный вывод таблицы.** У Presidio ровно два русских дефицита, и они разной природы:

1. **Форматные типы работают, но без русской семантики.** Luhn и mod-97 не знают языка. Но
   контекстные слова, которые в Presidio — основной механизм борьбы с FP, для русского
   отсутствуют полностью. То есть наши `checksum_plus_context` типы (ИНН юрлица, ОГРН, ОГРНИП,
   БИК, карта, телефон) в Presidio превратились бы в `checksum_only` с измеренной вероятностью
   ложного прохождения 9.9–10% по нашему реестру.
2. **NER-типы (PERSON, LOCATION, NRP, ORGANIZATION) недоступны без замены NLP-движка**, а
   русская модель даёт только `PER`, `LOC`, `ORG` — то есть даже после подмены на
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

Доступные NLP-движки `[ПРОВЕРЕНО]` (структура `docs/analyzer/nlp_engines/`):

| Движок | Пакет | Комментарий |
|---|---|---|
| `SpacyNlpEngine` | `spacy>=3.4.4,<4.0.0` — **обязательная зависимость** | дефолт |
| `StanzaNlpEngine` | extra `stanza` (`>=1.11.1`) | альтернатива |
| `TransformersNlpEngine` | extra `transformers` (+`accelerate`, `huggingface_hub`, `spacy_huggingface_pipelines`) | для ru — напр. `ai-forever/ruBert-base` |
| `SlimSpacyNlpEngine` | добавлен в 2.2.363 | только токенизация/леммы; NER делегируется, напр. GLiNER |
| `NoOpNlpEngine` | добавлен в unreleased | без NLP-артефактов; `HuggingFaceNerRecognizer` без spaCy-модели |
| `GLiNERRecognizer` | extra `gliner` + `onnxruntime` | ONNX-бэкенд (2.2.362, «crashes on CPUs without AVX2») |

Список языков из документации LiteLLM-интеграции (проксирует Presidio) — только `en`, `es`, `de`
`[ПРОВЕРЕНО]`. Не официальный список Presidio, но показательно.

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

### 2.6 Anonymizer operators: таблица «что делает → даёт ли реалистичный суррогат»

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

**Ответ на вопрос задания «can Presidio generate realistic surrogates?»** — **да, но только
через Azure** (`surrogate_ahds`, требует `pip install presidio-anonymizer[ahds]`,
`azure-health-deidentification>=1.1.0b1` + `azure-identity`, `[ПРОВЕРЕНО]` pyproject).
Локально, офлайн, на CPU — **нет**. Единственный офлайновый путь — `custom` с lambda, и в
репозитории есть ровно один пример такого рода: `example_custom_lambda_anonymizer.py`
(«Passing a lambda as a Presidio anonymizer using Faker») `[ДОК]` — по названию файла, из
списка samples. Это не продуктовая функция, а пример на 20 строк, который мы должны
написать сами со всей валидацией контрольных сумм, коллизиями и сессионной согласованностью.

**Три вещи, которые Presidio сделал в анонимайзере правильно и которые надо забрать:**

1. **Правила разрешения перекрытий описаны явно** `[ПРОВЕРЕНО]`: нет перекрытия → обычная
   замена; полное перекрытие → берётся высший score; одна сущность внутри другой → берётся
   **более длинная, даже если score ниже**; частичное пересечение → обе сущности
   анонимизируются отдельно и результат конкатенируется (`<PERSON><LOCATION>`). Последний
   случай — источник мусора, и Presidio его не скрывает.
2. **Hash по умолчанию перешёл на случайную соль (2.2.361)** `[ПРОВЕРЕНО]`: «Same PII value gets
   different hashes across different entities and calls (unless you provide a salt)». Это
   ломающее изменение, но правильное по безопасности — и прямо противоречит нашему инварианту
   I5 (одно значение → одна подмена в сессии). Вывод: **нельзя брать `hash` как модель для
   наших подмен**, нужен собственный деterministic-генератор с seed.
3. **Referential integrity признана ответственностью вызывающего**: «Presidio does not store or
   maintain stateful sessions. For referential integrity across records or calls, users must
   securely manage and provide their own salt.» То есть сессионную согласованность Presidio
   **не решает вообще** — это наша незанятая ниша.

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

### 2.8 Задержка

Presidio **не публикует** замеров задержки `[НЕ НАЙДЕНО]`. Косвенные источники:

| Источник | Число | Контекст |
|---|---|---|
| `bench.perf_arms` SurrogateShield `[ПРОВЕРЕНО, self-reported]` | Presidio **медиана 8.2 ms**, пик **1 084 MB**; +Faker 8.4 ms/1 010 MB; +RoBERTa **83.5 ms**/1 611 MB | короткие сообщения |
| LiteLLM docs `[ПРОВЕРЕНО]` | «analyzer latency grows with payload size»; дробление при >500 000 байт | признание нелинейности |
| LLM Guard benchmarks `[ПРОВЕРЕНО]` | p95 **294.57 ms** (m5.xlarge); **169.13 ms** с ONNX | вход **317 символов** |

**Ключевое наблюдение.** 8.2 ms — замер на **коротком** тексте с `en_core_web_lg`, и это
**медиана, не p95**, полученная одним автором. Наш контракт — p95 25 ms на 2 000 токенов.
294.57 ms на 317 символах намекает на нелинейное масштабирование. Честная формулировка:
`[НЕ ПРОВЕРЕНО]` до нашего собственного замера.

### 2.9 Активность и сообщество

`[ПРОВЕРЕНО]` GitHub API `data-privacy-stack/presidio`, 2026-10-16:

| Метрика | Значение |
|---|---|
| stars / forks / watchers | **11 345** / 1 349 / 99 |
| open issues | 188 |
| контрибьюторы | **206** (пагинация `Link`) |
| создан / последний push | 2018-05-04 / 2026-10-09 |
| размер / archived / лицензия | 339 965 KB (~332 MB) / false / **MIT** |

Релизы `[ПРОВЕРЕНО]`: **2.2.364 — 2026-07-22**; 2.2.363 — 2026-06-28; 2.2.362 — 2026-03-18;
2.2.361 — 2026-02-12; 2.2.360 — 2025-09-09. Темп: 4 релиза за 8 месяцев.

### 2.10 Где именно слабое место Presidio для русского

1. **Ноль русских рекогнайзеров при 130 типах.** Ни ИНН, ни СНИЛС, ни ОГРН, ни паспорта РФ,
   ни адреса РФ, ни российского телефона как отдельного типа с контекстом.
2. **Русский NER недоступен «из коробки»** — нужна замена `NlpEngine` + отдельные инстансы
   рекогнайзеров с русскими контекстными словами (документация это прямо требует).
3. **Контекстные слова — только en/es/it/pl/de/ko/sv/th/tr/fi и др.** Русских нет ни в одном
   рекогнайзере. Для наших `checksum_plus_context` типов это фатально: без контекста карта
   даёт ~10% прохождения случайных цифр (см. 20-entity-registry §2).
4. **Суррогатов локально нет**: `surrogate_ahds` — облако Azure, `input_locale`/`surrogate_locale`
   по умолчанию `en-US`, русского locale не заявлено.
5. **Нет состояния сессии** — ни I5, ни I2, ни восстановления по карте как первоклассной функции.
6. **Обязательная зависимость от spaCy** противоречит нашему «ядро без ML».
7. **Вес**: 382 MB дефолтная модель + spaCy против 27 MB Natasha.
8. **`presidio-cli` не публикуется** (CHANGELOG 2.2.356: «Do not release `presidio-cli` as part of
   the release pipeline») — CLI у них не продуктовый.
9. **Язык реализации — Python.** Для нашего p95-контракта и WASM-цели неприменимо.

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

**Полный список сканеров** `[ПРОВЕРЕНО]` README:

- **Prompt scanners (15):** Anonymize, BanCode, BanCompetitors, BanSubstrings, BanTopics, Code,
  Gibberish, InvisibleText, Language, PromptInjection, Regex, Secrets, Sentiment, TokenLimit, Toxicity.
- **Output scanners (21):** BanCode, BanCompetitors, BanSubstrings, BanTopics, Bias, Code,
  **Deanonymize**, JSON, Language, LanguageSame, MaliciousURLs, NoRefusal, ReadingTime,
  FactualConsistency, Gibberish, Regex, Relevance, Sensitive, Sentiment, Toxicity, URLReachability.

**Итого 36 сканеров**, из которых к нашей задаче относятся ровно два (`Anonymize`, `Deanonymize`).
**LLM Guard — не библиотека обезличивания:** обезличивание — один сканер из 36, остальные 34 про
prompt injection, токсичность, секреты, конкурентов и языки.

**Связь с Presidio — точная** `[ПРОВЕРЕНО]` документация Anonymize:

> «**Integration with Presidio Analyzer**: Leverages the Presidio Analyzer library, crafted with
> spaCy, flair and transformers libraries, for precise detection of private data.»

То есть `Anonymize` — **обёртка над Presidio Analyzer** плюс: свой список сущностей по умолчанию
(`anonymize.py#L26-L40`), свои regex-паттерны в `llm_guard/resources/sensisitive_patterns.json`
(опечатка `sensisitive` — в самом репозитории), класс `Vault`, выбор NER через `recognizer_conf`.
**Следствие: все русские дефициты Presidio наследуются, и к ним добавляются свои.**

### 3.3 Суррогаты: `use_faker` и `Vault`

`[ПРОВЕРЕНО]` параметры сканера: `vault` (архив вырезанного для восстановления), `preamble`,
`allowed_names` (белый список), `hidden_names` («Transforms specified names to formats like
`[REDACTED_CUSTOM_1]`»), `entity_types`, `regex_pattern_groups_path`, `recognizer_conf`,
`threshold` (default `0`), `language` («Default is `en`») и **`use_faker`** —
«Substitutes **eligible** entities with fabricated data».

**Ответ про суррогаты.** Флаг есть, но: (1) он **не по умолчанию** — дефолтный путь даёт
плейсхолдеры `<PERSON>`/`[REDACTED_CUSTOM_1]`; (2) «for **eligible** entities» — критерий
eligibility в документации не раскрыт `[ДОК]`; (3) Faker не умеет **форматно-валидные** российские
ID — он не знает про mod-11 ИНН или mod-13 ОГРНИП. Восстановление — `Deanonymize` через `Vault`.

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

- Название: **«Portcullis: A Scalable and Verifiable Privacy Gateway for Third-Party LLM Inference»**
- Авторы: **Jiangou Zhan (Tsinghua University), Wenhui Zhang, Zheng Zhang, Huanran Xue, Yao Zhang, Ye Wu (ByteDance Inc.)**
- Venue: **AAAI-25**, Vol. 39 No. 1, Technical Track on Application Domains, стр. **1022–1030**
- Опубликовано **2025-04-11**
- DOI: **10.1609/aaai.v39i1.32088**
- PDF: <https://ojs.aaai.org/index.php/AAAI/article/view/32088/34243>

**Важное уточнение.** Мы в 00-charter пишем «Portcullis (ByteDance)». Точнее: **первый автор из
Tsinghua**, остальные пять — ByteDance. Формулировка «Portcullis, ByteDance + Tsinghua, AAAI-25»
корректнее.

**Есть ли код: `[НЕ НАЙДЕНО]`.** Что проверено: GitHub Search `portcullis LLM privacy` → 0,
`portcullis bytedance` → 0; на странице статьи OJS ссылки на артефакт **нет** (только PDF и
Video/Poster); arXiv API `all:Portcullis AND all:LLM` → **пусто** (arXiv-версии нет). Воспроизведение
требует собственной реализации.

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
маскирования/снятия маски. **Чего не решает** `[ДОК — прямо не заявлено в абстракте]`:

| Вопрос | Ответ |
|---|---|
| Многоходовый контекст, анафора, кореференция | Не адресуется, ни слова в абстракте |
| Реалистичность подмен (формат, checksum) | Не заявлено, что подмены формат-сохраняющие; «parallel substitution» может быть плейсхолдером |
| Русский язык | Не упомянут; датасеты — **PII** (вероятно английский) и **Enron** (английский) |
| Типы сущностей | Не перечислены в абстракте |
| Требования к железу | Secure enclave (SGX/TDX) — серверный CPU с TEE; на потребительских CPU и в WASM неприменимо |
| Кроссплатформенность | Нет: confidential container platform |

**Вердикт.** Берём интеллектуальную рамку («шлюз должен быть проверяемым») и сам факт, что
независимая академическая работа назвала именно эту нишу. Отвергаем TEE и аттестацию как основу:
это (а) не решает нашу задачу (русский, суррогаты, контекст), (б) требует серверного железа,
(в) код недоступен, (г) 96× относится к другой метрике. Также отвергаем соблазн цитировать 96×
без оговорки — это единственная цифра, которую проверяющий откроет и найдёт.

### 4.2 SurrogateShield

**Идентификация** `[ПРОВЕРЕНО]`:

- Статья: **«SurrogateShield: Beyond Redaction for High-Utility, Privacy-Preserving LLM Interactions»**
- Автор: **Sherwin Vishesh Jathanna** (единственный автор)
- arXiv: **2606.29567** (cs.CR, cs.CL), подан **28 июня 2026**, v1
- Комментарий в arXiv: «14 pages, 1 figure, 9 tables. **Code and dataset**: <https://github.com/sherwinvishesh/SurrogateShield>»
- Лицензия статьи: **CC BY 4.0**
- Репозиторий: <https://github.com/sherwinvishesh/SurrogateShield>, лицензия **Apache-2.0**
- PyPI: `surrogateshield`, версия библиотеки **1.9.0**
- GitHub-метрики: **2 ★, 0 форков, 0 open issues**, создан **2026-05-13**, push **2026-10-16**,
  язык Python, размер 11 439 KB

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

**Ограничения, которые автор сам называет** `[ПРОВЕРЕНО, раздел Limitations]`:

> «it trailed on the **96 real values in natural text** (29 leaked; GLiNER-PII 16–20, Presidio 27),
> on some single types (3 of 13 on test-3, 8 of 13 on Nemotron-PII, **URL most of all**) and on
> **speed and memory** (plain Presidio is about **5× faster and lighter**).»

Это честно и это ровно та зона, где мы должны быть сильнее: **естественный текст**, а не списки значений.

**Архитектура** `[ПРОВЕРЕНО]`: `SentinelLayer` (PatternScan → canonicaliser → EntityTrace →
PII tagger → [ContextGuard] → структурные проходы → relation gate → resolver); `MimicGen`
(Faker-суррогаты, тип-консистентные, без коллизий в сессии); `ShadowMap` (AES-256-GCM,
HKDF-SHA256(device_secret, conv_id, kind), `~/.surrogateshield/`); `ResolvePass` (4 прохода:
exact → component → fuzzy с rapidfuzz, порог 85 → given name/surname).

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

### 5.5 LLM-прокси и «privacy firewall» проекты

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

### 5.6 Источники суррогатов: mimesis и Faker

Ключевой практический вопрос — **откуда брать русские фейки**.

**Faker** `[ПРОВЕРЕНО]` GitHub API + PyPI-метаданные через SurrogateShield:
- репозиторий `joke2k/faker`, **19 447 ★**, **MIT**, push 2026-10-09;
- локаль **`ru_RU`** существует — используется в `pii-bench-ru-en`? нет, там pymorphy3; но
  Faker `ru_RU` — стандартный источник русских имён/адресов/телефонов `[ДОК]`;
- зависимость SurrogateShield (`faker>=24.0.0`), scrubadub (без версии) `[ПРОВЕРЕНО]`.

**mimesis** `[ПРОВЕРЕНО]` GitHub API:
- репозиторий `lk-geimfari/mimesis`, **4 848 ★**, **MIT**, последний релиз **v22.2.0 от 2026-09-23**,
  push 2026-10-06, ~114 страниц контрибьюторов (≈ 228 человек);
- поддерживает локали, включая русскую `[ДОК]` — «list locales incl. ru_RU»;
- чистый Python, без ML-зависимостей — лёгкий `[НЕ ПРОВЕРЕНО]` по точному размеру.

**Но ни Faker, ни mimesis не генерируют форматно-валидные российские ID.** Ключевое:
«ИНН физлица, проходящий mod-11 ×2» — это **не** задача генератора фейковых персон. Это
отдельный генератор на нашем стороне: взять формат, сгенерировать первые N−1 цифр, **посчитать
контрольную сумму по нашему реестру** и подставить. То же для ОГРН (mod 11), ОГРНИП (mod 13),
СНИЛС (mod 101), карты (Luhn). Ни Presidio, ни LLM Guard, ни pii-guard этого не делают — они
маскируют, а не подменяют валидным.

Единственный, кто заявляет валидные суррогаты, — SurrogateShield, и только для Luhn/ABA/Base58
`[ПРОВЕРЕНО]`, а для США.

### 5.7 Файл `nedap/deidentify` и Phileas — кратко

- `nedap/deidentify` 149 ★ MIT: медицинские записи, «state-of-the-art NLP». Язык — нидерландский/
  английский. README на `main` вернул 404 `[ПРОВЕРЕНО]` (структура репо другая).
- `philterd/phileas` 110 ★ **Apache-2.0**, push 2026-10-14 — живой Java-движок PII/PHI-редакции
  с «replacement»-стратегиями. Единственный живой не-Python движок в обзоре. Русского нет
  `[НЕ НАЙДЕНО]`. Стоит перепроверить в фазе 4 как источник идей по стратегиям замены — но
  лицензия Apache-2.0 и Java делают его неудобным для нас.

### 5.8 Cloudflare AI Gateway DLP

`[ПРОВЕРЕНО]` <https://developers.cloudflare.com/ai-gateway/features/dlp/set-up-dlp/>,
обновлено 2026-09-25:

Профили: **Financial Information** (карты, счета, routing), **PII** (имена, адреса, телефоны),
**Government Identifiers** (SSN, паспорта, права), **Healthcare Information**, **Custom Profiles**.
**Действия только `FLAG` или `BLOCK`** — ни маскирования, ни суррогатов. Проверка `REQUEST` /
`RESPONSE` / `BOTH`. Ошибки: код **2029** (запрос) / **2030** (ответ). Заголовок `cf-aig-dlp`
с массивом `findings`. **Русский:** `[НЕ НАЙДЕНО]` — профили строятся на Zero Trust DLP, про русские
сущности упоминаний нет. **Вердикт: отвергаем как механизм обезличивания** (BLOCK — не
обезличивание), но берём идею **профилей как политик** и **машиночитаемого отчёта о срабатывании**;
наш `Report` должен быть таким же структурным.

### 5.9 LiteLLM Guardrails

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

### 5.10 Прочее, что проверено и отвергнуто

| Проект | Лицензия | Активность | Причина отказа |
|---|---|---|---|
| `zylon-ai/private-gpt` | Apache-2.0 | **57 583 ★**, push 2026-10-15 | Приватность = **локальный инференс**, а не обезличивание. Presidio в зависимостях не нашли. Это другая задача. |
| `yandex/tomita-parser` | **MPL-2.0** | 500 ★, **последний push 2020-11-16** | Мёртв 6 лет; сборка требует lua5.2/gcc 4.8/cmake 2.8 и **отдельно `libmystem-c-binding.so`** из релиза v1.0; язык конфигов — свой. Отвергаем как код, берём идею rule-based извлечения (Natasha/Yargy — живая реализация той же идеи). |
| `LeapBeyond/scrubadub_spacy` | Apache-2.0 | 41 ★, push 2023-09-01 | Мёртв |
| `LeapBeyond/scrubadub_address` | Apache-2.0 | **1 ★**, push 2023-09-01 | Мёртв |

**«Oban»** `[НЕ НАЙДЕНО]` как инструмент обезличивания: в экосистеме есть Elixir-библиотека `Oban`
— **очередь фоновых задач**, к приватности отношения не имеет. Других совпадений в контексте
PII/анонимизации поиск не дал. Позиция плана закрыта: артефакта нет.

---

## 6. Русскоязычные инструменты

### 6.1 Сводная таблица

| Инструмент | Что это | Лицензия | Типы сущностей | Русский | Вес | Активность | Суррогаты | Вердикт |
|---|---|---|---|---|---|---|---|---|
| **pii-guard** (redmadrobot-rnd) | Обезличивание русского текста: правила + ruBERT + Presidio | **Apache-2.0** | **20 типов** | **Да, ядро** | Веса **678 MiB** (178M параметров, ruBert-base) | README живой; статья на habr/redmadrobot.ai | **Нет** (mask/tag/pseudonymize) | **Главный русский конкурент** |
| **rubert-base-pii-ner** | HF-модель NER | **Apache-2.0** | 21 тип / 43 BIO-метки | **Да** | 178M параметров | 1 764 загрузок, 6 ❤ | Нет | **Берём как baseline** |
| **pii_benchmark** | HF-датасет, оценка | **MIT** | 14+ категорий | **Да** | 2 841 предложение | 597 загрузок, 23 ❤ | — | **Берём как внешний замер** |
| **pii_train** | HF-датасет, обучение | — | 17 137 предложений, 39 687 спанов | **Да** | — | — | — | Референс для нашего набора |
| **Natasha / Slovnet** | NLP-фреймворк для русского | **MIT** | PER, LOC, ORG | **Да** | NER **2 MB**, диск **27 MB**, RAM **205 MB** | 1 359 ★, тег v1.6.0 (PyPI 2023-07-24), push 2026-10-14 | Нет | **Берём NER-провайдером** |
| **DeepPavlov** | NLP-фреймворк (PyTorch) | **Apache-2.0** | PER/LOC/ORG (ru-модели) | **Да** | модель **1024–2048 MB** диск, **3072–6144 MB RAM** | 6 997 ★, push 2025-08-06 | Нет | Отвергаем (вес) |
| **spaCy `ru_core_news_*`** | NER-пайплайны | **MIT** | PER, LOC, ORG | **Да** | sm **14.6 MB**, md **39.9 MB**, lg **489.6 MB** | живее всех | Нет | Берём как baseline для сравнения |
| **exemtik/pii-bench-ru-en** | Детектор + токенизатор + бенчмарк | **MIT** | 13 типов (RU) | **Да** | pymorphy3 + spaCy | 0 ★, push 2026-05-31 | Нет, токены `[ИМЯ_1]` | **Берём метод (pymorphy3) и цифры** |
| **llm-pii-proxy-showcase** | Прод-прокси (Казахстан) | **MIT** | KZ_BIN/IIN/IIK + PERSON/ORG/LOC (Presidio ru) | **Да** | Presidio + `ru_core_news_md` | 0 ★, push 2026-09-21 | Нет (солёные плейсхолдеры) | **Берём инженерию** |
| **SergeyK88i/pii-obfuscator** | Детекция + детерминированная псевдонимизация ru | **нет лицензии** | `[НЕ ПРОВЕРЕНО]` | Да (по описанию) | 18 KB | 0 ★, создан и последний push **2026-09-24** | Псевдонимы | Наблюдать (нет лицензии!) |
| **feuerstrahll/ru-pii-detection-system** | Детекция ПДн в русских документах | **MIT** | `[НЕ ПРОВЕРЕНО]` | Да | **1 KB** (!) | 0 ★, push 2026-09-22 | — | Заявка без кода (1 KB) |
| **bekzod/yashirgich** | PII для узбекского/русского | **нет лицензии** | `[НЕ ПРОВЕРЕНО]` | Да | 50 KB | 3 ★, push 2026-02-15 | — | Наблюдать |
| **k1ddy/kazpii-bench** | Бенчмарк PII для казахского и русского | **нет лицензии** | KazNERD | Да | 762 KB | 0 ★, push 2026-10-12 | — | Наблюдать (свежий) |
| **alrosait/spacy_ru_core_news_lg_pii** | spaCy-пайплайн ru+PII | **MIT** | `[НЕ ПРОВЕРЕНО]` | Да | lg | **0 загрузок**, 2 ❤ | Нет | Непроверяемо |
| **dzanozin/traciora-opf-ru-pii-v6** | openai-privacy-filter ru | **Apache-2.0** | `[НЕ ПРОВЕРЕНО]` | Да | **1,4 млрд параметров** | 28 загрузок, 4 ❤ | — | Отвергаем (вес) |
| **HikmaAI mdeberta-v3-base-pii-multilingual** | Мультиязычная PII-модель | **Apache-2.0** | `[НЕ ПРОВЕРЕНО]` | ru в списке языков | — | 27 загрузок | — | Наблюдать |
| **Wismut/nym-pii-multilingual** | Мультиязычная PII-модель (mmBERT) | **MIT** | `[НЕ ПРОВЕРЕНО]` | ru+uk в языках | 308M параметров | 2 785 загрузок | — | Наблюдать |
| **tabularisai/eu-pii-safeguard** | XLM-R PII, 26 языков ЕС | **Apache-2.0** | `[НЕ ПРОВЕРЕНО]` | **ru** в списке | xlm-roberta-large, 559M | **4 151 загрузка, 26 ❤** | — | **Стоит проверить в фазе 4** |
| **LiquidAI LFM2.5-Encoder-350M-PII-Detector** | Мультиязычный PII-детектор | **other** | `[НЕ ПРОВЕРЕНО]` | ru в списке | 355M | **2 086 загрузок, 45 ❤** | — | Стоит проверить; лицензия «other» — читать |
| **Gherman/bert-base-NER-Russian** | Русский NER | `[НЕ ПРОВЕРЕНО]` | PER/LOC/ORG | Да | bert-base | 1 855 загрузок, 25 ❤ | Нет | Наблюдать |
| **yqelz/xml-roberta-large-ner-russian** | Русский NER | `[НЕ ПРОВЕРЕНО]` | — | Да | xlm-roberta-large | 766 загрузок, 10 ❤ | Нет | Наблюдать |
| **SLIM / NEREL** | — | — | — | — | — | `[НЕ ПРОВЕРЕНО]` | — | См. §6.6 |
| **Sber / Studio Ousia** | — | — | — | — | — | `[НЕ НАЙДЕНО]` в контексте PII | — | См. §6.6 |
| **natasha/slovnet** (Navec) | Компактные русские эмбеддинги | **MIT** | — | Да | — | часть Natasha | — | Берём вместе с NER |

### 6.2 pii-guard — детальный разбор (главный русский конкурент)

`[ПРОВЕРЕНО]` README `redmadrobot-rnd/pii-guard` + `docs/architecture.md`:

**Что делает:** «Обезличивание персональных данных в русском тексте. Находит персональные данные
и убирает их из текста: маской или тегом — насовсем, псевдонимом — обратимо. Работает и сам по
себе, и прослойкой перед LLM».

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
   детерминированная контрольная сумма надёжнее вероятностной оценки.»** Это наше правило
   «каскады не отменяют друг друга», но с явным приоритетом. Побочный эффект они называют честно:
   «ошибочное правило гасит правильное предсказание модели, а не наоборот. Именно так выглядели
   оба дефекта, из-за которых правила пришлось сузить по расстоянию».

2. **«Ключевые слова оцениваются расстоянием, а не присутствием в окне.»**
   > «Проверка „есть ли слово где-нибудь в окне ±260 символов“ не выражает конкуренцию: если в
   > одном предложении два числа и два ключа, булев ответ неверен при любом размере окна —
   > „водительское удостоверение 77 АВ 123456, индекс 630009“ требует отклонить первые шесть
   > цифр и принять вторые.»

   Бюджеты расстояний **измеренные, а не выдуманные**: «у `BIK` ключ во всей разметке стоит не
   дальше 24 символов слева, поэтому бюджет 40; у паспорта p99 = 83, поэтому близкое окно 80 и
   дальнее 160». **Это прямое улучшение нашего L1**: у нас записано «окно ±N токенов», без ответа
   на случай двух ключей и двух чисел.

3. **«Кандидат ищется по фразе, а отдаётся значение.»**
   > «`7518` само по себе не паспорт, `серия 7518, номер 492137` — паспорт … наружу каркас отдаёт
   > только данные внутри неё. Маска ложится на `серия ****, номер ******`, а не на всю фразу:
   > „серия“ и „номер“ — не персональные данные».

   У нас такого требования нет вообще, а оно правильно с точки зрения сохранения смысла.

**Восстановление в правильной падежной форме** `[ПРОВЕРЕНО]` README: «→ `Свяжитесь с Иваном
Петровым`. Прямая подстановка дала бы „с Иван Петров“.» Реализация — `pymorphy3` + spaCy.
**Никто из западных решений этого не делает:** SurrogateShield восстанавливает подстановкой строки,
Presidio — `decrypt`. Согласование падежа — уникальная русская продуктовая фича, и у pii-guard она есть.

**Режимы** `[ПРОВЕРЕНО]`:

| Режим | Пример | Обратим |
|---|---|---|
| `mask` | `Иван Петров` → `***********` | нет |
| `tag` | → `[PERSON]` | нет |
| `pseudonymize` | → `<PII type="PERSON" gender="male" id="1" />` | **да** |

Псевдоним несёт **тип, род и стабильный id** — «одинаковые значения получают один `id` даже в
разных грамматических формах». Это реализация нашего I5. И это **не суррогат**: тип сущности
виден модели.

**Предобработка** `[ПРОВЕРЕНО]`: нормализация, **транслит в кириллицу**, числа словами,
исключение base64-блобов. Это наш I1 («проверка сравнивает E с исходными спанами по всем
нормализациям: регистр, пробелы, дефисы, **транслит**, юникод-гомоглифы, цифры словами»).
У них это уже реализовано. Отдельно: **смещения указывают на `normalized_text`, а не на
присланный текст** — и они это документируют как подводный камень.

**Данные** `[ПРОВЕРЕНО]`:

| Артефакт | Значение |
|---|---|
| NER-модель | `redmadrobot-rnd/rubert-base-pii-ner`, ruBert-base, 178M, веса **678 MiB** |
| Обучение | `redmadrobot-rnd/pii_train` — **17 137 предложений, 39 687 спанов** |
| Оценка | `redmadrobot-rnd/pii_benchmark` — **2 841 предложение**, disjoint от train |
| Метод обучения | 10 эпох, lr 3e-5 linear без warmup, batch 16, AdamW wd 0.01, fp32, max_len 512, seed 42 |
| Инференс | скользящее окно 512 подтокенов, **stride 128**, `min_confidence=0.3` |
| Слияние фрагментов | **обязательно**: без него `elena@pochta.ru` распадается на `el` + `ena@pochta.ru` |

**Качество** `[ПРОВЕРЕНО]` (span-level micro-F1, exact matching, 14 категорий):

| Система | P | R | F1 |
|---|---|---|---|
| Модель отдельно | 81.9 | 85.5 | **83.6** |
| Пайплайн `pii-guard` (правила + модель) | 90.4 | 87.5 | **88.9** |
| На протоколе лидерборда (PERSON+LOCATION, overlap) | — | — | 94.7 / 95.0 |

Per-entity F1 модели — то, что нам нужно как baseline `[ПРОВЕРЕНО]`:

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

**Восстановление падежа** (`pymorphy3`) — тоже надо проверить на лицензию (`pymorphy3` — MIT
`[НЕ ПРОВЕРЕНО]`).

**Слабые места pii-guard:**

1. **Суррогатов нет вообще.** Псевдоним — `<PII type="PERSON" id="1" />`; модель видит тип и род.
   Дословно: «Только `pseudonymize` обратим: **тип сущности виден модели**». Для нашей задачи
   «внешняя модель не должна знать, что это ПДн» это неверно.
2. **Python + Presidio + PyTorch.** Веса 678 MiB + Presidio + spaCy + torch — в Balanced (≤250 MB)
   не влезает, в Fast (≤30 MB) тем более.
3. **Нет состояния сессии.** «Таблица псевдонимов нигде не хранится. Сервис не имеет состояния:
   ни Redis, ни TTL, ни идентификатора сессии.» Осознанное решение (карта уезжает вызывающему),
   но оно **перекладывает I5/I2 на клиента**.
4. **Задержка не опубликована** `[НЕ НАЙДЕНО]`. 178M ruBERT + окно 512/stride 128 на CPU: 2 000
   токенов ≈ 6 окон, каждое — forward-проход BERT-base; ≤25 ms p95 наивно ожидать нельзя
   `[НЕ ПРОВЕРЕНО, оценка]`.
5. **Метаданные PyPI битые**: `pii-guard 0.1.0` содержит `Homepage: https://github.com/YOUR_USERNAME/pii-guard`
   — шаблон не заполнен `[ПРОВЕРЕНО]`; загружен 2025-12-10. Релизный процесс сырой.
6. **Код на GitHub не открывал** `[НЕ ПРОВЕРЕНО]` — `api.github.com/repos/redmadrobot-rnd/pii-guard`
   вернул пустой объект. Звёзды/контрибьюторы `[НЕ ПРОВЕРЕНО]`.

### 6.3 Exemtik/pii-bench-ru-en — единственные публичные цифры по русскому персоналу

`[ПРОВЕРЕНО]` README (raw):

**Данные.** RU: `hivetrace/pii-bench` — **1 810 документов, 1 667 спанов**, **13 типов**:
NAME, PHONE_NUMBER, EMAIL, ADDRESS, BANK_CARD_NUMBER, CVC, **INN, KPP, OGRN, OGRNIP, SNILS,
PASSPORT_NUMBER**, TOKEN. EN: `alejopaullier/pii-external-dataset` — 4 434 документа, 22 226 спанов.

**Метрика:** char-overlap F1 (на уровне символов) + span-метрики + token-F5 для английского.

**Результаты по русскому** `[ПРОВЕРЕНО]`:

| Конфигурация | overall F1 | F1 по `NAME` |
|---|---|---|
| `regex_only` | 0.800 | **0.000** |
| `regex+morph` (pymorphy3) | 0.846 | **0.794** |
| `regex+spacy_ru` (baseline) | **0.897** | **0.910** |
| `regex+morph+spacy_ru` | 0.873 | 0.785 |

**Результаты по английскому:**

| Конфигурация | overall F1 | F1 по `NAME_STUDENT` | token-F5 |
|---|---|---|---|
| `regex_only` | 0.586 | **0.000** | 0.386 |
| `regex+spacy` | **0.745** | 0.728 | 0.727 |
| `regex+spacy_aggr` | 0.703 | 0.637 | 0.765 |

**Четыре вывода, которые мы обязаны учесть:**

1. **Regex без морфологии даёт F1 по именам ровно 0.000.** Форматные типы (EMAIL, PHONE, СНИЛС, ИНН,
   КПП, ОГРН, карты, токены) покрыты regex с F1 ≈ 0.9–1.0, имена — нет. Количественное подтверждение,
   что профиль **Fast (L0+L1, без NER) физически не может находить ФИО** (у нас это уже записано).
2. **`pymorphy3` даёт F1 0.794 по именам без нейросетей** — морфологический разбор (`Surn`/`Name`/
   `Patr`) почти дотягивает до spaCy, работает на словоформах в любом падеже («Игорем Волковым»).
   Вес `[НЕ ПРОВЕРЕНО]`, но заведомо **на порядки легче** 489.6 MB `ru_core_news_lg`. **Кандидат
   в Balanced L2 вместо тяжёлой модели.**
3. **`ru_core_news_sm` обгоняет pymorphy3 по именам (0.910 против 0.794) и ловит `ADDRESS` (0.507,
   у regex и pymorphy — 0).** За **14.6 MB** — +0.116 F1 по именам и ненулевой адрес.
4. **Аддитивность отрицательная:** `regex+morph+spacy_ru` (0.873) **хуже** `regex+spacy_ru` (0.897).
   Вывод: выбирать **один** источник NER, не сваливать всё в кучу.

**Ограничения автора** `[ПРОВЕРЕНО]`: «RU: 10 слитных цифр неоднозначны (ИНН юрлица ↔ паспорт);
контекстные паттерны разводят часть случаев, остальное по умолчанию относится к паспорту» —
подтверждение нашего пункта про паспорт РФ без контрольной суммы. **Лицензия MIT** `[ПРОВЕРЕНО]`.

### 6.4 Natasha / Slovnet — лучший CPU-провайдер для русского

`[ПРОВЕРЕНО]` `natasha/natasha` README + `natasha/slovnet` README + PyPI:

- **Задачи:** токенизация/сегментация (Razdel), эмбеддинги (Navec), морфология, лемматизация
  (Pymorphy), синтаксис, NER (Slovnet), извлечение фактов (Yargy), визуализация (Ipymarkup).
- **Версия:** `natasha 1.6.0`, PyPI загружен **2023-07-24**, **13 релизов**; последний тег
  **v1.6.0**; репозиторий 1 359 ★, push **2026-10-14** (то есть изменения идут, но без релизов).
- **Лицензия: MIT** `[ПРОВЕРЕНО]` (PyPI `info.license = "MIT"`), GitHub API `license = MIT`.
- **Зависимости:** `pymorphy2`, `razdel>=0.5.0`, `navec>=0.9.0`, `slovnet>=0.6.0`, `yargy>=0.16.0`,
  `ipymarkup>=0.8.0` `[ПРОВЕРЕНО]` PyPI `requires_dist`.
- **Позиционирование (важно для нас):** «Natasha is **not a research project**, underlying
  technologies are built for production. **We pay attention to model size, RAM usage and performance.
  Models run on CPU, use Numpy for inference.**»
- **Предупреждение:** «Models optimized for **news articles**, quality on other domain may be lower.»
  И: «API may change, for realworld tasks consider using low level libraries».

**Размеры моделей** `[ПРОВЕРЕНО]` Slovnet README, дословно из таблицы Downloads:

| Модель | Размер | Что |
|---|---|---|
| `slovnet_ner_news_v1.tar` | **2 MB** | Русский NER, **стандартная аннотация PER, LOC, ORG**, обучен на новостях |
| `slovnet_morph_news_v1.tar` | **2 MB** | Морфология |
| `slovnet_syntax_news_v1.tar` | **3 MB** | Синтаксис |

**Производительность и вес** `[ПРОВЕРЕНО]` Slovnet README, таблицы `ner2` / `morph2` / `syntax2`:

| Система | init, s | диск, MB | RAM, MB | скорость, it/s |
|---|---|---|---|---|
| **slovnet** (NER) | **1.0** | **27** | **205** | **25.3** (новостных статей/с) |
| slovnet_bert | 5.0 | 473 | 9 500 | 40.0 (GPU) |
| deeppavlov | 5.9 | 1 024 | 3 072 | 24.3 (GPU) |
| deeppavlov_bert | 34.5 | 2 048 | 6 144 | 13.1 (GPU) |
| deeppavlov_slavic | 35.0 | 2 048 | 4 096 | 8.0 (GPU) |
| pullenti | 2.9 | 16 | 253 | 6.0 |
| **spacy** (ru) | 8.0 | **140** | **625** | 8.0 |
| stanza | 3.0 | 591 | 11 264 | 3.0 (GPU) |
| **tomita** | 2.0 | 64 | **63** | **29.8** |
| mitie | 28.3 | 327 | 261 | 32.8 |

Морфология: `slovnet` — **27 MB диск, 115 MB RAM, 532 предложения/с**. Синтаксис: **27 MB,
125 MB RAM, 450 предложений/с**.

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

`[ПРОВЕРЕНО]`:

| Модель | Размер wheel | Тип | NER-метки | Лицензия |
|---|---|---|---|---|
| `ru_core_news_sm` | **14.6 MB** | core, vocab, syntax, entities | **LOC, ORG, PER** | **MIT** |
| `ru_core_news_md` | **39.9 MB** | + vectors | LOC, ORG, PER | MIT |
| `ru_core_news_lg` | **489.6 MB** | + vectors | LOC, ORG, PER | MIT |

Из метаданных `ru_core_news_sm-3.8.0.json` `[ПРОВЕРЕНО]`: `license: MIT`, `size: 14 MB`,
pipeline включает `morphologizer`, `parser`, `attribute_ruler`, `lemmatizer`, `ner`;
лексемы — полный набор Universal Dependencies; лемматизатор по умолчанию
**`mode: "pymorphy3"`** `[ПРОВЕРЕНО]` (`spacy/lang/ru/__init__.py`).

**Важно:** spaCy `ru` **сам использует `pymorphy3`** для лемматизации. То есть «spaCy против
pymorphy3» — ложная дихотомия: `ru_core_news_sm` = статистический NER + pymorphy3-лемматизатор.
Именно поэтому `regex+spacy_ru` обгоняет `regex+morph`: там pymorphy3 работает **внутри** spaCy,
плюс есть NER и парсер.

**Слабые места ru-моделей spaCy** `[ПРОВЕРЕНО]` `docs/analyzer/...` и внешние замеры:
1. Только **три** метки: LOC/ORG/PER. **Нет** адреса, даты, документа, ИНН, телефона.
2. Обучены на **новостях** (Wikipedia/новости), не на личных документах/переписке.
3. У Presidio по умолчанию **`ORGANIZATION` в игнор-листе** «Has many false positives» —
   и это про английскую `en_core_web_lg`. Про русскую модели надёжных данных нет. Практика
   из `llm-pii-proxy-showcase`: **«ORG/LOC thresholds are raised (≥ 0.85, length ≥ 5) — spaCy
   is noisy on them in legal text»** `[ПРОВЕРЕНО]`. Это прямая рекомендация: русский юридический
   текст — не тот домен, где spaCy LOC/ORG можно брать с дефолтным порогом.
4. Вес: 489.6 MB для lg против 2 MB Slovnet.

### 6.6 DeepPavlov — берём цифры, не берём код

`[ПРОВЕРЕНО]` GitHub API (репозиторий переехал с `deepmipt/DeepPavlov` на **`deeppavlov/DeepPavlov`**,
API отдаёт `Moved Permanently`, новый id 111113343):

| Параметр | Значение |
|---|---|
| stars | **6 997** |
| Лицензия | **Apache-2.0** |
| Последний push | **2025-08-06** |
| archived | false |
| Описание | «An open source library for deep learning end-to-end dialog systems and chatbots» |

README: «DeepPavlov 1.0 is an open-source NLP framework built on **PyTorch** and **transformers**».
**Требования: «Depending on the model used, you may need from 4 to 16 GB RAM.»** `[ПРОВЕРЕНО]`
Это убивает применимость: наш профиль Balanced — ≤250 MB.

Из таблиц Slovnet `[ПРОВЕРЕНО]`, модели DeepPavlov для русского NER:

| Модель | init, s | диск, MB | RAM, MB | Скорость |
|---|---|---|---|---|
| `deeppavlov` (NER) | 5.9 | **1 024** | **3 072** | 24.3 it/s (GPU) |
| `deeppavlov_bert` | 34.5 | **2 048** | **6 144** | 13.1 it/s (GPU) |
| `deeppavlov_slavic` | 35.0 | **2 048** | **4 096** | 8.0 it/s (GPU) |

**Все три работают на GPU** в этих замерах. На CPU — не заявлено.

**Вердикт.** Берём **цифры качества** (`deeppavlov_bert` F1 0.971/0.928/0.825 на factRuEval —
это верхняя планка русского NER) и **цифры веса**, чтобы обосновать выбор Slovnet. Не берём код:
PyTorch, 1–2 GB на диске, минимум 4 GB RAM, 35 секунд инициализации.

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

`[ПРОВЕРЕНО]` — нашлось ровно то, что нужно нашему §4.2 реестра:

**MihaRS2/llm-pii-proxy-showcase** (Kazakhstan, 0 ★, MIT, push 2026-09-21) — прод-прокси для
юридической платформы в Казахстане. Это **самый близкий к нам проект по постановке**:

| Сущность | Валидация |
|---|---|
| `KZ_BIN` (12 цифр) | национальная контрольная сумма; без метки «БИН:» считаются только checksum-valid |
| `KZ_IIN` | checksum **плюс реальная календарная дата рождения внутри номера** |
| `KZ_IIK` | полный IBAN mod-97 (ISO 13616) |
| `KZ_PHONE`, `KZ_COMPANY`, `KZ_ADDRESS` | казахстанские паттерны (ТОО/АО/ИП/LLC…, «г./ул./д.») |
| `PERSON` | ФИО **в любом падеже**, только отчество, казахские «…улы/…кызы» |
| `EMAIL`, `URL`, `LOCATION`, `ORGANIZATION` | Presidio built-ins с повышенными порогами |
| `DATE` | маскируется **только в личном контексте** (дни рождения, паспорта) — сроки договоров остаются читаемыми |

Ключевое про IIN: **«checksum plus a real calendar birth date embedded in the number — „Feb 31“
used to slip through range checks»** `[ПРОВЕРЕНО]`. Это ровно наш урок про паспорт РФ: одной
форматной проверки мало, нужен второй признак, и второй признак — **не любая проверка, а
семантически связанная** (дата должна быть реальной датой, а не в диапазоне 1–31).

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

**Стек:** Python 3.12, FastAPI, httpx (streaming), **Microsoft Presidio**, spaCy
(**`ru_core_news_md`**, `en_core_web_sm`), **pymorphy3**, Redis (mapping, TTL 24 ч), PostgreSQL
(audit), Docker. **32 теста. MIT.**

**Это прецедент, доказывающий наш тезис:** Presidio + `ru_core_news_md` + pymorphy3 — рабочая
связка для русского, но она даёт **плейсхолдеры**, а не суррогаты, и требует Redis+Postgres.
Наш ход — тот же результат в одном Rust-бинаре, с суррогатами и без внешних хранилищ.

**Другие СНГ-проекты** `[ПРОВЕРЕНО]`:

| Репозиторий | ★ | Лицензия | Push | Что |
|---|---|---|---|---|
| `k1ddy/kazpii-bench` | 0 | нет | 2026-10-12 | «Open benchmark for PII detection in **Kazakh and Russian** texts: generator, **KazNERD** ingest, harness, leaderboard» — свежий, 762 KB |
| `bekzod/yashirgich` | 3 | нет | 2026-02-15 | «**Uzbek/Russian** PII detection and masking using ML (**rubai**) + rule-based (Presidio)» |
| `geekpartnerslab-kz/llmguard` | 2 | нет | 2026-04-24 | «Open-source PII anonymization proxy between your apps and LLMs (Claude, GPT, local)» — КЗ |

**Вывод по СНГ.** Рынок **уже шевелится**, но:
- всё это 0–3 звезды, без лицензий в двух случаях из трёх;
- все на Python + Presidio;
- **Казахстан — единственная юрисдикция СНГ с реальным прод-опытом** (llm-pii-proxy-showcase);
- **Беларуси (УНП), Узбекистана, Украины — открытых обезличивателей не нашёл** `[НЕ НАЙДЕНО]`;
  есть только валидаторы в составе мультиязычных пакетов (`python-stdnum` в зависимостях
  scrubadub `[ДОК]`).
- **Россия** — есть `pii-guard` (сильный) и разрозненные заготовки.

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

**Механика «primary + secondary + confidence» — главный урок** `[ПРОВЕРЕНО]`. На примере ИНН РФ
дословно:

> «A DLP policy has **high confidence** … if, within a proximity of **300 characters**:
> - The function `Func_Russian_Taxpayer_Identification_Number` finds content that matches the pattern.
> - A keyword from `Keyword_Russian_Taxpayer_Identification_Number` is found.
>
> A DLP policy has **medium confidence** … if … only the function finds a match.»

XML-определение: `<Entity patternsProximity="300" recommendedConfidence="85">` с двумя `<Pattern>`:
`confidenceLevel="85"` (функция + ключ) и `confidenceLevel="75"` (только функция). Публичная шкала
`[ПРОВЕРЕНО]`: **Low ≤ 65, Medium = 75, High = 85.**

**Русские ключевые слова, дословно** `[ПРОВЕРЕНО]`:
`НДС, номер налогоплательщика, Налогоплательщика ИД, налог число, налогчисло#, ИНН, НДС#`. Ошибки
бросаются в глаза: **`НДС` — это НДС (VAT), а не ИНН**; `Налогоплательщика ИД` — калька с
английского. Для паспорта РФ список содержит «паспорт **нет**», «**p**оссийской паспорт» (латинская
`p`), «паспортid#» — часть явные опечатки, часть OCR-артефакты. **Урок: даже Microsoft в русском
списке ключевых слов ошиблась. Русские контекстные слова надо собирать и валидировать на корпусе,
а не переводить.** Это подкрепляет наш пункт «определить список отрицательных меток эмпирически».

**Для паспорта РФ** `[ПРОВЕРЕНО]`: «Checksum: **No**», максимум **medium confidence (75)**.
Это **независимое подтверждение нашего `context_required` для `ru.passport`** — Microsoft пришла к
тому же выводу.

**Близость 300 символов — более слабая версия нашего L1:** Purview не различает, какой ключ ближе,
а pii-guard решил это расстоянием (§6.2). **Здесь мы легко можем быть лучше Purview.**

**Про суррогаты в Purview:** `[НЕ НАЙДЕНО]` механики генерации фейковых значений в SIT-документации.
Azure AI Language PII Presidio предлагает как **remote recognizer** (`docs/samples/python/text_analytics/`)
— то есть Microsoft предлагает **своё облако** как способ закрыть пробелы Presidio `[ПРОВЕРЕНО]`.
Русский там `[НЕ ПРОВЕРЕНО]`.

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

**Кастомные детекторы** `[ПРОВЕРЕНО]`: три вида — regular dictionary, **stored dictionary (до десятков
миллионов фраз, Bloom-фильтры в Cloud Storage)**, regex; уровень `minLikelihood` от `VERY_UNLIKELY`
до `VERY_LIKELY`; hotword rules; exclusion lists.

**Русские infoType и языковая поддержка:** `[НЕ ПРОВЕРЕНО]` — страница `language-support` вернула
**404**; число встроенных infoType-детекторов не получил. Русские типы **не подтвердил**. В список
«допроверить в фазе 4».

**Что берём:** FPE-суррогат как механизм; полный чек-лист типов трансформаций (redact, replace, mask,
crypto-hash, deterministic crypto, FPE, bucketing, date shifting, generalization, conditional) как
проверку полноты нашей `Substitution.kind`; разделение «де-идентификация» и «ре-идентификация» как
двух API; stored dictionary до десятков миллионов фраз.

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

Четыре урока:

1. **Явное требование контекста:** «The PII model performs more effectively when it is provided with
   **sufficient context**. … **avoid submitting single words or short phrases**. … a string of digits
   might represent an AWS KMS key or a user ID depending on the surrounding information.» —
   **это наш L1-каскад, сформулированный вендором.**
2. **Явные слепые зоны:** «it does not evaluate … PII the model generates into **tool call arguments**
   … PII in **tool results** … PII in the **tool definitions**.» — для нас это будущая проблема v0.2
   (инструменты), и раздел «чего не умеем» должен её предвидеть.
3. **Утечка через трассировку by design:** «the `match` field in `GuardrailPiiEntityFilter` … contains
   the **original PII value**»; «the `input` field in Amazon CloudWatch Logs always contains the
   **original, unmodified request** regardless of guardrail intervention». **Наш I10 — прямое
   конкурентное преимущество.**
4. **Ограничение кастомного regex:** «does **not support a regex lookaround match**.»

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

| № | Откуда | Что берём | Почему (с источником) |
|---|---|---|---|
| 1 | **pii-guard** | **Приоритет правил над моделью** + признание побочного эффекта | «детерминированная контрольная сумма надёжнее вероятностной оценки»; честно про «ошибочное правило гасит правильное предсказание модели» `[docs/architecture.md]` |
| 2 | **pii-guard** | **Расстояние до ближайшего ключа вместо «есть ли ключ в окне»** | контрпример «водительское удостоверение 77 АВ 123456, индекс 630009»; измеренные бюджеты (BIK 40, паспорт 80/160, p99=83) |
| 3 | **pii-guard** | **Наружу отдаётся значение, а не фраза** | `серия ****, номер ******`, а не вся фраза — «плейсхолдер на всю фразу ломает предложение сильнее, чем требуется» |
| 4 | **pii-guard** | **Восстановление с согласованием падежа** (pymorphy3 + spaCy) | `Свяжитесь с Иваном Петровым`, а не `с Иван Петров`. Никто из западных решений этого не делает |
| 5 | **pii-guard** | **Предобработка: транслит → кириллица, числа словами, исключение base64** | наш I1 требует ровно этих нормализаций |
| 6 | **pii-guard** | **Пометка, что смещения относятся к `normalized_text`**, а не к исходному тексту | у нас в модели данных это не оговорено — исправить |
| 7 | **pii-guard** | **Явное предупреждение про лицензию optional extra** (`gender-guesser` = GPLv3) | «Копилефта в установке по умолчанию нет. Единственное исключение придётся подключить руками» |
| 8 | **Presidio** | **Явные правила разрешения перекрытий**, включая «внутри другой → берём длинную» | docs/anonymizer; там же честно про `<PERSON><LOCATION>` при частичном пересечении |
| 9 | **Presidio** | **Контекстное усиление отдельным слоем** (`LemmaContextAwareEnhancer`) + `context_matching_mode` | word-boundary матчинг; баг `lic` внутри `duplicate` |
| 10 | **Presidio** | **`low_confidence_score_multiplier: 0.4` и `labels_to_ignore`** для шумных NER-меток | `ORGANIZATION` в игнор-листе «Has many false positives» |
| 11 | **Presidio** | Модель угроз «нет аутентификации by design» **задокументирована** | CHANGELOG 2.2.362 / #1903; у нас «раздел чего не умеем» уже обязателен |
| 12 | **Presidio** | **Немецкий набор как образец пакетного вклада юрисдикции** (13 рекогнайзеров, ISO 7064 Mod 11,10, DRV, GKV, legal basis) | шаблон для нашего `rules/ru/` |
| 13 | **SurrogateShield** | **ShadowMap: AES-256-GCM + HKDF(device_secret, conv_id, kind) + AAD** | «a file only opens as what it was written for» |
| 14 | **SurrogateShield** | **Битый файл карты НЕ заменяется пустым**: «Moved aside to `<name>.corrupt-<ts>` … never silently replaced by an empty map» | усиление наших I4/I6 |
| 15 | **SurrogateShield** | **Fail-closed с явным исключением**: `DetectorUnavailable` вместо отправки незамаскированного; `ALLOW_EPHEMERAL_KEY` как opt-in | ровно наша I4 |
| 16 | **SurrogateShield** | **Проверка весов модели по SHA-256 на фиксированный коммит HF** | защита от подмены модели; у нас этого нет |
| 17 | **SurrogateShield** | **Whole-value matching при восстановлении** + категоризация `exact_miss`/`fuzzy_hit`/`fuzzy_miss` | наша I3 + метрика качества восстановления |
| 18 | **SurrogateShield** | **Пресеты `balanced`/`strict`/`fast`/`classic`** | совпадает с нашей идеей профилей |
| 19 | **SurrogateShield** | **Квази-идентификаторный скоринг по k-anonymity** с явным списком комбинаций | 10 комбинаций; «ZIP+DOB+Gender = 87%» (Sweeney 2000) |
| 20 | **SurrogateShield** | **Форматная валидность суррогатов как требование** (Luhn, ABA, Base58) | подтверждает наш §5.4 п.1 |
| 21 | **llm-pii-proxy-showcase** | **Солёный хэш фейка с перцем из окружения; отказ стартовать без перца** | «An unsalted hash of a 12-digit BIN is brute-forceable in minutes» — требование к подменам и к I9 |
| 22 | **llm-pii-proxy-showcase** | **SSE-safe восстановление** с буфером до последней незакрытой `[` | обязательно, если прокси поддерживает стриминг |
| 23 | **llm-pii-proxy-showcase** | **Стоп-лист по последовательностям лемм**, а не по словам | «Закону Республики Казахстан» остаётся читаемым в любом падеже |
| 24 | **llm-pii-proxy-showcase** | **Маскирование даты только в личном контексте** (дни рождения/паспорта) | сохранение смысла — наш §6.3 |
| 25 | **llm-pii-proxy-showcase** | **Второй признак должен быть семантически связан**: у KZ_IIN проверяется реальная календарная дата («Feb 31» проходил range-check) | усиливает наш §4.1 реестра |
| 26 | **llm-pii-proxy-showcase** | **Пороги ORG/LOC подняты (≥0.85, длина ≥5) для юридического русского текста** | «spaCy is noisy on them in legal text» |
| 27 | **llm-pii-proxy-showcase** | **Усечение инструментального вывода ДО анонимизации** | «one search result once returned 5.99 M characters» |
| 28 | **llm-pii-proxy-showcase** | **Деградация без утечки**: «If Redis is down, restoration degrades but anonymization keeps working» | хранилище не на критическом пути обезличивания |
| 29 | **llm-pii-proxy-showcase** | **Аудит-лог хранит только виды и количества, никогда значения** | наша I10 |
| 30 | **Natasha / Slovnet** | NER-провайдер: **2 MB модель, 27 MB диск, 205 MB RAM, CPU, Numpy, MIT** | единственный инструмент, попадающий в профиль Balanced |
| 31 | **pii-bench-ru-en** | **`pymorphy3` как дешёвый детектор ФИО**: F1 по NAME 0.794 без нейросетей, любой падеж | кандидат в L2 для профиля с малым весом |
| 32 | **pii-bench-ru-en** | **`ru_core_news_sm` (14.6 MB) как дешёвый апгрейд**: +0.116 F1 по именам и ненулевой ADDRESS (0.507) | 14.6 MB за адрес, которого regex не даёт вообще |
| 33 | **pii-bench-ru-en** | **Отрицательная аддитивность**: `regex+morph+spacy_ru` (0.873) хуже `regex+spacy_ru` (0.897) | не сваливать все детекторы в кучу; выбирать источник NER |
| 34 | **pii-bench-ru-en** | Датасет **`hivetrace/pii-bench`**, 13 русских типов | готовый внешний тест для Extended-уровня |
| 35 | **redmadrobot** | Датасет **`pii_benchmark`** (2 841 предложение, MIT), disjoint от train | второй внешний тест |
| 36 | **Google DLP** | **FPE-суррогат**: формат-сохраняющая, детерминированная, обратимая подмена без справочников | официальные сэмплы de/re-identify free text with FPE using a surrogate |
| 37 | **Google DLP** | **Чек-лист типов трансформаций** (redact, replace, mask, crypto-hash, deterministic crypto, FPE, bucketing, date shifting, generalization, conditional) | transformations-reference |
| 38 | **Google DLP** | **Stored custom dictionary до десятков миллионов фраз** | concepts-de-identification |
| 39 | **Purview** | **Primary/secondary + числовой confidence**: `recommendedConfidence=85`, два Pattern, `patternsProximity="300"`, шкала 65/75/85 | XML SIT-определения |
| 40 | **Purview** | **Независимое подтверждение `context_required` для паспорта РФ**: «Checksum: No», максимум medium | sit-defn-russia-passport-number-domestic |
| 41 | **Purview** | **Юридическое основание в описании типа** (`Legal basis: §§ 139a–139e AO`) | у нас это часть «раздела чего не умеем» |
| 42 | **AWS Bedrock** | **Явное требование контекста** и запрет на «single words or short phrases» | подтверждение необходимости нашего L1 |
| 43 | **AWS Bedrock** | **Явный список слепых зон** (tool call arguments, tool results, tool definitions) | заготовка для «чего не умеем» (v0.2) |
| 44 | **AWS Bedrock** | **Ошибка вендора как урок**: `match` в trace и CloudWatch `input` содержат **исходные** значения | обоснование I10 как конкурентного преимущества |
| 45 | **Cloudflare** | **Профили как политики** + машиночитаемый отчёт (`cf-aig-dlp`, коды 2029/2030) | образец для нашей структуры `Report` |
| 46 | **LiteLLM** | **Наша роль как guardrail-провайдера**; готовый набор режимов (`pre_call`/`post_call`/`logging_only`/`pre_mcp_call`) | конкретный путь интеграции фазы 7 |
| 47 | **LiteLLM** | **Авто-дробление длинных текстов** с перекрытием и слиянием детекций (`presidio_analyze_chunk_size_bytes`, default 500 000) | у нас нет стратегии для текстов > лимита |
| 48 | **arx-deidentifier/arx** | Методология **k-anonymity** (Apache-2.0, 738 ★) | зрелый соседний проект |
| 49 | **arabic-pii-py** | Подтверждение, что **«local-first reversible PII для одной юрисдикции» — рабочая стратегия** | Apache-2.0, арабский/Залив |

### 8.2 Что отвергаем — по решениям, с причиной

| № | Что отвергаем | Причина |
|---|---|---|
| 1 | **Presidio как зависимость** | Python; `spacy` **обязательна** (противоречит «ядро без ML»); `en_core_web_lg` **382 MB**; **0 русских рекогнайзеров**; русских контекстных слов нет ни в одном рекогнайзере; нет состояния сессии; `presidio-cli` не публикуется |
| 2 | Presidio `hash` как модель подмен | С 2.2.361 **случайная соль по умолчанию** → одно значение даёт разные хэши, что **нарушает наш I5** |
| 3 | Presidio `replace`/`mask`/`redact` как целевое поведение | `<PHONE_NUMBER>`, `****1234`, удаление — не подмена на похожие фейки, а потеря смысла |
| 4 | Presidio `surrogate_ahds` как путь к суррогатам | Требует **облака Azure**; `input_locale`/`surrogate_locale` default **`en-US`**; русского locale нет; противоречит «локально, без сети» |
| 5 | Presidio-конвенцию «внутри другой → берём длинную» **без оговорок** | Подменяет более широкий спан, чем нужно; нужна проверка `checksum`/`context` перед расширением |
| 6 | **LLM Guard как зависимость** | **Архивирован 2026-07-08**; **модели на HF тоже не поддерживаются**; последний релиз PyPI 2025-05-19 |
| 7 | LLM Guard как «решение для обезличивания» | 36 сканеров, из которых анонимизационный — **один**; остальные 34 не относятся к задаче и добавляют зависимостей и поверхности атаки |
| 8 | LLM Guard `use_faker` | Не дефолт; «for **eligible** entities» — критерий не раскрыт; Faker не даёт валидных по checksum русских ID |
| 9 | Ориентацию на p95 LLM Guard | **169.13 ms (ONNX) / 294.57 ms** p95 на **317 символах** — в 3.4–5.9× выше нашего потолка 50 ms при 44× меньшем объёме |
| 10 | **Portcullis как реализацию** | **Кода нет** (GitHub Search = 0 ×2, arXiv = 0, на странице статьи только PDF и постер) |
| 11 | Portcullis TEE/аттестацию как основу | Требует серверного CPU с confidential computing; не работает на потребительском CPU и в WASM; не решает ни русский, ни суррогаты, ни контекст |
| 12 | Цитату «96× быстрее предшественников» | Абстракт: 96× — **«for masking and unmasking workloads»** против **Hide-and-Seek**; качество на Enron — **+0.1 accuracy** |
| 13 | **SurrogateShield как код** | Python; `en_core_web_lg`; `ssn`/`zip_us`/`us_bank_number`/ABA/`postal_code_uk` — **ни одной русской сущности**; **2 ★**, 0 форков, единственный автор (bus factor = 1) |
| 14 | **Цифры SurrogateShield как эталон** | Все **self-reported** одним автором; независимых репликаций `[НЕ НАЙДЕНО]`; репозиторий создан 5 месяцев назад |
| 15 | Модель задержки SurrogateShield | **43.5 ms медиана, 1 348 MB пик** — не влезает в Balanced (25 ms, 250 MB) |
| 16 | SurrogateShield `ADDRESS_MODE=shift` как есть | Американский address parser (штаты, ZIP+4, directionals); `ZIP + DOB + Gender` — US-специфика Sweeney |
| 17 | **scrubadub** | Релиз и push **2023-09-01**; `textblob==0.15.3` жёстким пином; `scikit-learn` без необходимости; **расхождение лицензий**; нет ru |
| 18 | **OpenAnonymiser / anonymiq-light** | Только `nl_core_news_lg`; EUPL-1.2; **0 ★**; flavor `contextual` **не реализован**; суррогатов нет |
| 19 | **pydeidentify** | **Нет лицензии**; 1 ★; 2023; spaCy-only; нет ru |
| 20 | **anonymize-it** | **Не существует** (PyPI 404, GitHub Search — только посторонние проекты) |
| 21 | **«Oban» как инструмент обезличивания** | `[НЕ НАЙДЕНО]`; `Oban` — очередь задач на Elixir, к PII отношения не имеет |
| 22 | **DeepPavlov как зависимость** | PyTorch; **1 024–2 048 MB диск**, **3 072–6 144 MB RAM**; «from 4 to 16 GB RAM»; **34.5 s инициализации**; замеры на GPU |
| 23 | **Yandex Tomita как код** | **MPL-2.0**; push **2020-11-16** (мёртв); cmake 2.8 + lua 5.2 + gcc 4.8 + `libmystem-c-binding.so`; свой язык конфигов |
| 24 | **spaCy `ru_core_news_lg`** | **489.6 MB** против **2 MB** Slovnet при сопоставимом качестве на новостях; только LOC/ORG/PER |
| 25 | **Любую русскую модель, не прошедшую наш замер** | Правило, а не отказ: `Wismut/nym-pii-multilingual`, `tabularisai/eu-pii-safeguard`, `dzanozin/traciora-opf-ru-pii-v6` (1,4 млрд параметров) — все `[НЕ ПРОВЕРЕНО]`, измерять нашим харнесом в фазе 4 |
| 26 | **Cloudflare AI Gateway как механизм обезличивания** | Только **FLAG/BLOCK**; обезличивания нет вообще; ru-профилей нет |
| 27 | **AWS Comprehend для русского** | PII только «English and Spanish»; русского нет ни в одной функции |
| 28 | **AWS Bedrock `Mask`** | Заменяет на `{NAME}`/`{EMAIL}` — плейсхолдер, не суррогат |
| 29 | **LiteLLM Presidio-guardrail как движок** | Языки — `en`/`es`/`de`; `MASK` = `<PERSON>`; требует развёрнутых контейнеров Presidio; рекурсивно наследует минусы Presidio |
| 30 | **Приватность = локальный инференс** (`private-gpt`) | 57 583 ★ решают **другую задачу**: данные не уходят, но и не обезличиваются |
| 31 | **Presidio как «эталон, который мы догоняем»** | По **главной** метрике (доля запросов с утечкой) Presidio слаб: 31.6% утечки на реальных сообщениях (замер SurrogateShield) |
| 32 | **Идею «мы первые делаем суррогаты»** | SurrogateShield делает суррогаты с мая 2026. Корректно: **«мы первые делаем суррогаты для русского, локально, на CPU»** |

### 8.3 Что меняется в наших документах по итогам аудита

| Наш документ | Что поправить | Основание |
|---|---|---|
| 00-charter §2.7 | Portcullis: первый автор — Tsinghua, пятеро — ByteDance; 96× — **маскирование/снятие маски** против Hide-and-Seek | [AAAI-25](https://ojs.aaai.org/index.php/AAAI/article/view/32088) |
| 00-charter §2.6 | Тезис «ни у Presidio, ни у LLM Guard, ни у Portcullis этого нет» **подтверждён**, но добавить: **ни у кого** из проверенных, включая SurrogateShield, pii-guard и все облака | §1.2 этого отчёта |
| 00-charter §2.2 п.2 | «доступ из Presidio-подобных пайплайнов» уточнить: Presidio **архивировал** `presidio-cli` и **обязательно тянет spaCy** — наша ценность в **замене**, а не «дополнении» | CHANGELOG 2.2.356, pyproject |
| 00-charter §5.4 `similar_fake` | Добавить требование **форматной валидности**; рассмотреть **FPE** как альтернативу генератору; разделить «формат+детерминизм» (FPE) и «правдоподобие распределения» (генератор + валидация) | [Google FPE-суррогат](https://docs.cloud.google.com/sensitive-data-protection/docs/samples/dlp-deidentify-free-text-with-fpe-using-surrogate) |
| 00-charter §5.4 | Добавить: **фейк не должен быть валидным реальным идентификатором** — иначе adversarial-атака «инверсия фейка» (§6.5) даёт попадание в реестр. FPE закрывает это by design | та же ссылка |
| 00-charter §5.2 | Добавить: **смещения относятся к нормализованному тексту**; публичный контракт должен это фиксировать; для подсветки в оригинале — отдельный API переноса смещений | pii-guard README, showcase README |
| 00-charter §5.4 | Добавить: **суррогат хэшируется с перцем из окружения**; без перца — отказ; несолёный хэш 12-значного ID брутфорсится за минуты | showcase README |
| 00-charter §5.4 п.4 | «Детерминированность от seed» оставить, но добавить: **детерминированность должна быть криптографической** (FPE/ключ), иначе подмена обратима без карты (атака «через формат») | Google FPE docs |
| 20-entity-registry §5 | Каскад L1 описан как «окно ±N токенов». **Заменить на расстояние до ближайшего ключа** с измеренными бюджетами | pii-guard docs/architecture.md |
| 20-entity-registry §5 п.3 | «Отрицательная метка важнее положительной» **подтверждается**; добавить приём pii-guard: сужение спана до данных внутри фразы (`серия ****, номер ******`) | pii-guard docs/architecture.md |
| 20-entity-registry §4.1 | Для `ru.passport` **независимое подтверждение**: Purview ставит максимум **medium (75)**, `Checksum: No` | [Purview паспорт РФ](https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-domestic) |
| 20-entity-registry §4.1 | Добавить для ИНН: **реальный F1 — 68.3 у чистой модели**, 88.9 у пайплайна. Публиковать recall по ИНН отдельно, не средним | [rubert-base-pii-ner](https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner) |
| 20-entity-registry §3 | Открытый пункт про сверку весов СНГ: **единственный кандидат** — `kz_recognizers.py` в showcase (MIT, 32 теста), но клиентский тест-набор убран авторами. **Остаётся `unverified`** | showcase README |
| 20-entity-registry §4.2 | `kz.iin`: второй признак = **реальная календарная дата внутри номера**, а не range-check | showcase README |
| 00-charter §6.5 | Adversarial: добавить критерий **«фейк не попадает в реальные реестры»** → FPE предпочтительнее генератора; добавить атаку **«брутфорс плейсхолдера»** (несолёный хэш) | Google FPE, showcase README |
| 00-charter §7.2 | Бюджет **L2 NER 12 ms** не обоснован: Slovnet даёт **~39.5 ms/KB**; экстраполяция на 2 000 токенов — **~0.5 с**. **Нужен собственный замер** | [Slovnet](https://github.com/natasha/slovnet) (таблица `ner2`) |
| 00-charter §7.2 | Добавить, что `ru_core_news_sm` (**14.6 MB**) — компактная альтернатива L2 с лучшим F1 по именам (0.910 против 0.794) и ненулевым ADDRESS | pii-bench-ru-en README |
| 00-charter §7.1 | Профиль Fast (L0+L1) **обязан** публиковать «ФИО и адрес не обнаруживаются» — подкреплено **F1 по NAME = 0.000** для regex-only | pii-bench-ru-en README |
| 00-charter §5.1 | Добавить в `Report` **машиночитаемую структуру о срабатывании** (виды, количества, политики) по образцу `cf-aig-dlp` | Cloudflare DLP docs |
| 00-charter §10 | Добавить риск: **PII в аргументах/результатах инструментов (tool calling)** — у Bedrock это задокументированная слепая зона; для нас v0.2 | AWS Bedrock docs |
| Анти-цели (новый пункт) | **Не затаскивать копилефт через optional extra** (пример: `gender-guesser` GPLv3 у pii-guard) — проверка лицензий дерева зависимостей в CI | pii-guard README |

---

## 9. Пробелы рынка, которые мы займём

### 9.1 Три пробела, подтверждённых с двух сторон

**Пробел 1: русский язык + реалистичные суррогаты + локально.**

Пусто **и** в open-source (§5, §6), **и** у проприетарных (§7). Единственные русские
суррогаты не существуют; единственные суррогаты с русским — не существуют; Google имеет
суррогаты, но не имеет подтверждённого русского; Purview имеет русский, но не имеет суррогатов.

| | Русский | Суррогаты | Локально/офлайн |
|---|---|---|---|
| Presidio | ✗ | ✗ (только Azure) | ✓ |
| LLM Guard | ✗ | ~ (`use_faker`, не дефолт) | ✓ |
| SurrogateShield | ✗ | ✓ | ✓ |
| pii-guard | ✓ | ✗ | ✓ |
| Portcullis | ✗ | ? (не заявлено) | ✗ (TEE-сервер) |
| llm-pii-proxy-showcase | ✓ (ru+kz) | ✗ | ~ (Redis+Postgres) |
| Purview | ✓ (4 SIT) | ✗ | ✗ |
| Google DLP | ? | ✓ (FPE) | ✗ |
| Bedrock | ✗ | ✗ | ✗ |
| **privacy-core** | **✓** | **✓** | **✓** |

**Пробел 2: связность текста и контекстные утечки в диалоге.**

`[ПРОВЕРЕНО]` — этого нет **ни у кого** из проверенных: ни Presidio (нет состояния сессии,
документация прямо говорит «Presidio does not store or maintain stateful sessions»), ни LLM Guard,
ни pii-guard («Сервис не имеет состояния: ни Redis, ни TTL, ни идентификатора сессии»),
ни SurrogateShield, ни Portcullis (не адресует), ни облака. Наш ContextGuard (L3) и инвариант I2 —
**полностью свободная ниша**.

**Пробел 3: один бинарь, Rust, влезающий в WASM и в 25 ms.**

`[ПРОВЕРЕНО]` — **все** проверенные решения на Python (кроме `phileas` на Java), **ни одного на
Rust**. Профиль Balanced (≤25 ms, ≤250 MB) недостижим для Python-стека: LLM Guard p95 169 ms на
317 символах, SurrogateShield 43.5 ms медиана / 1 348 MB пик, Presidio 8.2 ms медиана / 1 084 MB
пик (и это на коротком тексте). Проверенный минимум по весу в русском стеке — Slovnet
**27 MB диск / 205 MB RAM** — и он на Python+Numpy.

Наш Fast-профиль (≤5 ms, ≤30 MB, влезает в WASM) **не имеет аналогов вообще**: ни одно решение
не заявляет WASM-сборку.

### 9.2 Второстепенные пробелы

| Пробел | Кто ближе всех | Что у них не так |
|---|---|---|
| **Восстановление с согласованием падежа** для русского | pii-guard (единственный) | Только для псевдонима `<PII type=... id=...>`; для суррогата-имени не решено **никем** |
| **Суррогаты с валидной контрольной суммой** для русских ID | SurrogateShield (только US: Luhn/ABA/Base58) | Никаких mod-11/mod-13/mod-101 для ИНН/ОГРН/ОГРНИП/СНИЛС |
| **Проверка весов модели по хэшу** | SurrogateShield | У нас нет; у Presidio/LLM Guard/pii-guard тоже нет |
| **Fail-closed как проверяемый инвариант** | SurrogateShield, llm-pii-proxy-showcase | Никто не формулирует это как машино-проверяемый инвариант с 8 сценариями сбоя (наш C4) |
| **Публикация FP(clean) на чистом корпусе** | `[НЕ НАЙДЕНО]` ни у кого | Мы публикуем — это наш Core/Extended протокол |
| **Стоп-лист по леммам для русского** | llm-pii-proxy-showcase | Только как часть showcase на 0 звёзд; не продуктово |
| **SSE-safe восстановление** | llm-pii-proxy-showcase | Только в showcase; у SurrogateShield ResolvePass не про стриминг |
| **Обезличивание логов** | **никто** — у Bedrock trace и CloudWatch `input` содержат исходники by design | Наш I10 |
| **Юрисдикции СНГ кроме Казахстана** | `[НЕ НАЙДЕНО]` для BY, UZ, UA | Наш §4.2 |

### 9.3 Чего мы **не** должны обещать (по итогам аудита)

Это важная часть раздела, потому что аудит собрал ровно те места, где конкуренты переобещали.

| Не обещать | Почему | Источник |
|---|---|---|
| «Ноль ложных срабатываний» | Наш же замер: 9.91% для ИНН юрлица, 80.98% для паспорта РФ. Purview для паспорта тоже не даёт выше medium | 20-entity-registry §2; Purview SIT |
| «93% recall по ИНН» | Реальные цифры: чистая модель **68.3**, пайплайн **88.9**; пайплайн — upper bound для правил+модели | rubert-base-pii-ner model card |
| «Находим ФИО в профиле Fast» | Regex-only F1 по NAME = **0.000** | Exemtik/pii-ben-ru-en |
| «96× быстрее» без оговорки | Это маскирование/снятие маски против Hide-and-Seek | AAAI-25 абстракт |
| «PII не уходит наружу» без оговорки про tool calling | Bedrock документирует, что аргументы/результаты/определения инструментов **не проверяются** | Bedrock Guardrails docs |
| «Логи не содержат PII» без проверки | У Bedrock CloudWatch `input` содержит исходный запрос **by design**; `match` в trace — исходное значение | Bedrock Guardrails docs |
| «Суррогаты не восстановимы» | FPE-суррогат **обратим ключом**; плейсхолдер с несолёным хэшем брутфорсится за минуты | Google FPE; llm-pii-proxy-showcase |
| «Мы первые делаем суррогаты» | SurrogateShield с мая 2026 | arXiv 2606.29567 |
| «Balanced в 25 ms» без замера L2 | Бюджет L2=12 ms не обоснован; Slovnet даёт ~39.5 ms/KB | Slovnet README |

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

---

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

1. **LLM Guard архивирован, и вместе с ним модели на HuggingFace** `[ПРОВЕРЕНО]`. Это снимает его с
   любой повестки как зависимость и заметно обнуляет конкурентное поле в open-source.
2. **pii-guard (Apache-2.0, русский, 20 типов) существует и делает почти всё, кроме того, что
   делаем мы** `[ПРОВЕРЕНО]`. Их `docs/architecture.md` — готовый учебник по русской детекции:
   приоритет правил над моделью, расстояние до ближайшего ключа, «наружу отдаётся значение, а не
   фраза», восстановление падежа, транслит-нормализация. Обязаны взять эти четыре решения и
   отличаться в двух: **суррогаты** (у них псевдонимы с видимым типом) и **состояние сессии**
   (у них его нет вообще).
3. **SurrogateShield — главный ориентир по идее и главный конкурент по духу.** Тот же набор решений
   (суррогаты, ShadowMap, fail-closed, whole-value restore, k-anonymity), но English-only,
   self-reported, 2 звезды, 1 348 MB RAM, 43.5 ms медиана `[ПРОВЕРЕНО]`. Ниша реальна, и наша версия
   для русского, на CPU, в одном бинаре — не то же самое.
4. **Единственные опубликованные p95 для Presidio-подобного стека — у LLM Guard: 169.13 ms (ONNX) /
   294.57 ms на 317 символах** `[ПРОВЕРЕНО]`. Наш бюджет 25–50 ms на 2 000 токенов. Цифра
   конкурента — объективное доказательство, что Python+spaCy в наш контракт не влезает.
5. **У проприетарных вендоров сломаны базовые вещи, которые мы объявили инвариантами** `[ПРОВЕРЕНО]`.
   AWS **by design** пишет исходный запрос в CloudWatch и исходное значение в trace guardrail.
   Cloudflare **вообще не умеет** обезличивать — только FLAG/BLOCK. Purview **не может** дать выше
   medium для паспорта РФ и **ошибается** в русских ключевых словах (`НДС` вместо `ИНН`).
   Только Google имеет суррогаты — и только через облачный FPE.

---

## Приложение B. Пересечение «русский × суррогаты × офлайн»

Главная таблица всего аудита. Клетки — из источников, перечисленных выше.

| Решение | Детектит ru | Суррогаты | Формат-валидны | Офлайн | Состояние сессии | Открытый код | Лицензия |
|---|:---:|:---:|:---:|:---:|:---:|:---:|---|
| Presidio | ✗ | ✗ (только Azure AHDS) | ✗ | ✓ | ✗ | ✓ | MIT |
| LLM Guard | ✗ | ~ (`use_faker`, не дефолт) | ✗ | ✓ | ✗ (Vault в процессе) | ✓ (архив) | MIT |
| SurrogateShield | ✗ | ✓ | ✓ (Luhn/ABA/Base58) | ✓ | ✓ (ShadowMap) | ✓ | Apache-2.0 |
| Portcullis | ✗ | ? | ? | ✗ (TEE) | ? | **✗** | — |
| pii-guard | **✓** (20 типов) | ✗ (псевдонимы) | — | ✓ | ✗ (stateless) | ✓ | Apache-2.0 |
| llm-pii-proxy-showcase | **✓** (ru + KZ) | ✗ (солёные плейсхолдеры) | — | ~ (Redis+Postgres) | ✓ (через Redis) | ✓ | MIT |
| Natasha/Slovnet | ✓ (только PER/LOC/ORG) | ✗ | — | ✓ | ✗ | ✓ | MIT |
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
состояние сессии × форматная валидность» пусто.** Это и есть формулировка нашего позиционирования,
подкреплённая источниками выше.
