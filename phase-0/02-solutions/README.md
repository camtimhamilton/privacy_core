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

**Расхождение лицензий Apache-2.0 (репо) vs MIT (PyPI)** — `[ПРОВЕРЕНО]`, но причину не выяснял.
При использовании надо смотреть на конкретный файл/пакет.

**Русский:** локалей нет; есть `scrubadub_spacy` (41 ★, Apache-2.0, последний push 2023-09-01),
который даёт детекторы через spaCy-модель — теоретически можно подсунуть `ru_core_news_*`, но
контекстные слова/паттерны — английские `[ДОК]`.

**Суррогаты:** `faker` в зависимостях, но по умолчанию scrubadub возвращает **теги**
(`{{EMAIL}}`), а не суррогаты `[ДОК]`.

**Вердикт: отвергаем.** Мёртв 3 года, `textblob==0.15.3` — жёсткий пин на старую версию
(потенциальный конфликт зависимостей), `scikit-learn` для задачи, которая этого не требует,
расхождение лицензий, ноль русской поддержки.

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

| Репозиторий | ★ | Лицензия | Приём | Ключевая деталь |
|---|---|---|---|---|
| `zeroc00I/DontFeedTheAI` | **665** | MIT | Прозрачный прокси для pentest-задач, локальный Ollama + regex, per-engagement vault | Самый популярный в нише; но домен — пентест, «strips IPs, credentials, hostnames and PII» |
| `openbashok/promptzero` | 70 | нет | Прозрачный прокси для Claude API | — |
| `gbrigandi/mcp-server-conceal` | 18 | MIT | MCP-прокси | «intelligently pseudo-anonymizes PII in real-time» |
| `Athroniaeth/piighost` | 14 | MIT | Reversible PII masking для агентов; LangChain, Pydantic AI, LlamaIndex, Claude Code; OpenAI/Anthropic-совместимый прокси | push 2026-10-09, 14 868 KB — самый зрелый из мелких по объёму кода |
| `Aajil-Labs/arabic-pii-py` | 12 | Apache-2.0 | **Арабский + Залив**, local-first, reversible | **Прямой аналог нашей стратегии для другой юрисдикции** |
| `RPBLC-hq/DAM` | 11 | нет | «local privacy firewall for AI agents» | — |
| `nikshepsvn/veilstream` | 6 | NOASSERTION | Streaming PII proxy: pluggable detector, per-session vault, крипто | — |
| `yurivenancio30/llm-dlp` | 5 | MIT | Локальный прокси для Claude Code | push 2026-10-16 |
| `whitestag-ai/pii-proxy` | 2 | Apache-2.0 | «GDPR-compliant anonymisation gate for LLM calls» | — |
| `j911si/local-privacy-gateway` | 1 | MIT | Офлайн-пseudonymization gateway, «reversible tokens» | — |
| `comptoir-des-signaux/sas-confiance-ia` | 4 | **EUPL-1.2** | Французский: псевдонимизация перед ИИ для территориальных коллективов | — |

**Что это доказывает.** Ниша «прокси, который обезличивает перед LLM» **уже занята десятком
проектов**, но:
- **все** они либо English-only, либо под одну не-русскую юрисдикцию (арабский/Залив, французский,
  нидерландский, Казахстан);
- **почти все** — про плейсхолдеры/токены, а не суррогаты (см. `piighost` — «replaces personal data
  with placeholders», `local-privacy-gateway` — «reversible tokens»);
- **все** — Python/Node, ни одного Rust;
- звёзды у лидера (665) получены в домене пентеста, не в приватности для обычных пользователей.

**Ниша «русский + суррогаты + один бинарь» свободна.** Это подтверждается всем списком.

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

- DLP-профили: **Financial Information** (карты, счета, routing), **Personal Identifiable
  Information** (имена, адреса, телефоны), **Government Identifiers** (SSN, паспорта, права),
  **Healthcare Information**, **Custom Profiles**;
- **действия только `FLAG` или `BLOCK`** — ни маскирования, ни суррогатов;
- проверка `REQUEST` / `RESPONSE` / `BOTH`;
- ошибки: код **2029** (запрос заблокирован), **2030** (ответ заблокирован);
- заголовок `cf-aig-dlp` с массивом `findings`.

**Русский:** `[НЕ НАЙДЕНО]` — в списке профилей нет ничего языкового, профили строятся на
Zero Trust DLP, про русские сущности упоминаний нет.

**Вердикт: отвергаем как механизм обезличивания** (BLOCK — не обезличивание), но берём идею
**профилей как политик** и **машиночитаемого отчёта о срабатывании** (`cf-aig-dlp`). Наш `Report`
в модели данных должен быть таким же структурным.

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

**«Oban»** `[НЕ НАЙДЕНО]` как инструмент обезличивания. В экосистеме есть Elixir-библиотека
`Oban` — это **очередь фоновых задач**, к приватности отношения не имеет. Других совпадений в
контексте PII/анонимизации поиск не дал. Считаю позицию плана закрытой: артефакта нет.

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

**20 типов сущностей** (дословно из README):

| Тег | Где детектится |
|---|---|
| `PERSON` | модель (ФИО) |
| `LOCATION` | модель (адрес и части: страна, регион, район, город, улица, дом) |
| `PHONE_NUMBER` | модель |
| `URL` | модель |
| `EMAIL_ADDRESS` | правила |
| `INN` | правила + контрольная сумма |
| `SNILS` | правила + контрольная сумма |
| `OMS` | правила + контрольная сумма |
| `PASSPORT` | правила + контекст |
| `DRIVER_LICENSE` | правила + контекст |
| `MILITARY_ID` | правила + контекст |
| `BIRTH_CERTIFICATE` | правила + контекст |
| `CREDIT_CARD` | правила + Luhn |
| `BANK_ACCOUNT` | правила |
| `BIK` | правила + расстояние до ключа |
| `POSTAL_CODE` | правила |
| `TELEGRAM` | правила |
| `IP_ADDRESS` | правила |
| `IP_PORT` | правила |
| `DATE_TIME` | правила |

Важно: **правила дают 16 типов**, модель — 4 (`PERSON`, `LOCATION`, `PHONE_NUMBER`, `URL`).
Дословно: «Правила ловят то, что проверяется контрольной суммой, модель — то, что суммой не
проверить.» И: «Без модели работают все типы с контрольными суммами. Недоступны четыре, которые
берёт только модель».

**Это подтверждает наш реестр типов и нашу каскадную архитектуру.** Они пришли к тому же
разделению L0/L2, что и мы, и по той же причине.

**Три архитектурных решения, которые надо забрать дословно** `[ПРОВЕРЕНО]` `docs/architecture.md`:

1. **«Ветки независимы, конфликты разрешаются после. При перекрытии спанов побеждают правила:
   детерминированная контрольная сумма надёжнее вероятностной оценки.»** Это ровно наше правило
   «каскады не отменяют друг друга, а только повышают уверенность», но с явным приоритетом.
   И они честно называют побочный эффект: «ошибочное правило гасит правильное предсказание
   модели, а не наоборот. Именно так выглядели оба дефекта, из-за которых правила пришлось
   сузить по расстоянию.» — читай: у них были FP из-за правил, и они это признают.

2. **«Ключевые слова оцениваются расстоянием, а не присутствием в окне.»** Дословно:
   > «Проверка „есть ли слово где-нибудь в окне ±260 символов“ не выражает конкуренцию: если в
   > одном предложении два числа и два ключа, булев ответ неверен при любом размере окна —
   > „водительское удостоверение 77 АВ 123456, индекс 630009“ требует отклонить первые шесть
   > цифр и принять вторые.»

   И конкретные бюджеты расстояний, **измеренные, а не выдуманные**: «у `BIK` ключ во всей
   разметке стоит не дальше 24 символов слева, поэтому бюджет 40; у паспорта p99 = 83, поэтому
   близкое окно 80 и дальнее 160».

   **Это прямое улучшение нашего L1.** У нас в реестре написано «окно ±N токенов», без указания,
   что делать при двух ключах и двух числах. Они решили это **расстоянием до ближайшего ключа**.
   Мы обязаны это забрать.

3. **«Кандидат ищется по фразе, а отдаётся значение.»** Дословно:
   > «Документ узнаётся по ключевому слову рядом с цифрами: `7518` само по себе не паспорт,
   > `серия 7518, номер 492137` — паспорт. Поэтому классификатор получает фразу целиком, а
   > наружу каркас отдаёт только данные внутри неё… Маска ложится на `серия ****, номер ******`,
   > а не на всю фразу: „серия“ и „номер“ — не персональные данные».

   Это другой ответ, чем у нас. У нас такого требования нет вообще. И это правильно с точки
   зрения сохранения смысла.

**Восстановление в правильной падежной форме** `[ПРОВЕРЕНО]` README:

> «→ `Свяжитесь с Иваном Петровым`. Прямая подстановка дала бы «с Иван Петров».»

Реализация: `pymorphy3` + spaCy. **Никто из западных решений этого не делает.** SurrogateShield
восстанавливает подстановкой строки; Presidio — `decrypt`. Согласование падежа — уникальная
русская продуктовая фича, и она у pii-guard уже есть.

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

1. **Суррогатов нет вообще.** Псевдоним — это `<PII type="PERSON" id="1" />`. Модель видит тип и
   род. Это принципиально отличается от нашей цели «подмена на похожие фейки»: их решение
   сохраняет связность и падеж, но **не маскирует то, что это ПДн**. Дословно: «Только
   `pseudonymize` обратим: **тип сущности виден модели**». Для нашей задачи «внешняя модель не
   должна знать, что это персональные данные» это неверно.
2. **Python + Presidio + PyTorch.** Веса 678 MiB + Presidio + spaCy + torch. В наш профиль
   Balanced (≤250 MB) не влезает; в Fast (≤30 MB) — тем более.
3. **Нет состояния сессии.** «Таблица псевдонимов нигде не хранится. Сервис не имеет состояния:
   ни Redis, ни TTL, ни идентификатора сессии.» Это осознанное решение (карта уезжает вызывающему),
   но оно **перекладывает I5/I2 на клиента**. Для нашего продукта это неприемлемо: у нас сессия
   — часть модели данных.
4. **Задержка не опубликована** `[НЕ НАЙДЕНО]`. При 178M ruBERT + окно 512 со stride 128 на CPU
   ожидать ≤25 ms p95 на 2 000 токенов наивно: 2 000 токенов / (512−128) ≈ 6 окон, каждое —
   forward-проход BERT-base. Это десятки-сотни миллисекунд `[НЕ ПРОВЕРЕНО]`.
5. **Метаданные PyPI битые**: `pii-guard 0.1.0` имеет `Homepage: https://github.com/YOUR_USERNAME/pii-guard`
   — шаблон не заполнен `[ПРОВЕРЕНО]`. Загружен 2025-12-10. То есть релизный процесс сырой.
6. **Кода на GitHub я не открывал** `[НЕ ПРОВЕРЕНО]` — README и docs/architecture.md читаются
   через raw, но `api.github.com/repos/redmadrobot-rnd/pii-guard` вернул пустой объект
   (возможно, из-за rate limit или переименования). Звёзды/контрибьюторы `[НЕ ПРОВЕРЕНО]`.

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

1. **Regex без морфологии даёт F1 по именам ровно 0.000.** Форматные типы (EMAIL, PHONE, СНИЛС,
   ИНН, КПП, ОГРН, карты, токены) покрыты regex с F1 ≈ 0.9–1.0, но имена — нет. Это количественное
   подтверждение того, что наш профиль **Fast (L0+L1, без NER) физически не может находить ФИО**.
   В нашем реестре это уже написано («Для `ru.fio` и `ru.address` без модели детекция невозможна»),
   теперь у этого есть внешняя цифра.
2. **`pymorphy3` даёт F1 0.794 по именам без всяких нейросетей** — то есть морфологический
   разбор (`Surn`/`Name`/`Patr` теги) почти дотягивает до spaCy. Стоимость: `pymorphy3` —
   Python-библиотека со словарями `[НЕ ПРОВЕРЕНО]` по весу, но заведомо **на порядки легче**
   489.6 MB `ru_core_news_lg`. **Это наш кандидат в Balanced L2: морфологический детектор ФИО
   вместо тяжёлой модели.** Работает на словоформах в любом падеже («Игорем Волковым») —
   прямое подтверждение.
3. **`ru_core_news_sm` обгоняет pymorphy3 по именам (0.910 против 0.794) и ловит `ADDRESS` (0.507,
   у regex и pymorphy — 0).** То есть адрес без spaCy-ru не детектится вообще. При цене `sm`
   всего **14.6 MB** это очень выгодная сделка: 14.6 MB за +0.116 F1 по именам и ненулевой адрес.
4. **Аддитивность отрицательная:** `regex+morph+spacy_ru` (0.873) **хуже**, чем `regex+spacy_ru`
   (0.897). Комбинация морфологии и spaCy не улучшает, а ухудшает результат. Вывод: выбирать
   **один** источник NER, не сваливать всё в кучу.

**Ограничения, которые автор сам называет** `[ПРОВЕРЕНО]`: «RU: 10 слитных цифр неоднозначны
(ИНН юрлица ↔ паспорт); контекстные паттерны разводят часть случаев, остальное по умолчанию
относится к паспорту.» — прямое подтверждение нашего пункта про паспорт РФ без контрольной суммы.

**Лицензия: MIT** `[ПРОВЕРЕНО]`, датасеты — по своим лицензиям.

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

Морфология: `slovnet` — **27 MB диск, 115 MB RAM, 532 предложения/с**.
Синтаксис: `slovnet` — **27 MB диск, 125 MB RAM, 450 предложений/с**.

**Насколько быстро это в пересчёте на задержку.** 25.3 новостных статьи/с при «1 article ≈ 1KB»
`[ПРОВЕРЕНО, допущение самого Slovnet]`. То есть **~1 KB за 39.5 ms**. Наши 2 000 токенов —
это грубо 12–16 KB русского текста `[НЕ ПРОВЕРЕНО, оценка]`. Линейная экстраполяция даёт
**~0.5 с**, что в бюджет не влезает вообще. **Нелинейность и батчирование могут изменить картину,
но рассчитывать на 12 ms для L2 по 2 000 токенов, как в нашем бюджете (§7.2 charter), на
основании этих цифр нельзя.** Это важный честный вывод: наш бюджет L2 = 12 ms требует отдельного
подтверждения замером, и Slovnet как есть — не гарантия.

**Оценка качества NER** `[ПРОВЕРЕНО]` Slovnet README, 4 датасета:

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

Собственное резюме авторов: «NER is **1-2% worse** than current BERT SOTA by DeepPavlov but
**60 times smaller in size (~30 MB)** and works **fast on CPU (~25 news articles/sec)**.»

Лучшие датасеты: **factRuEval-2016** (`factru`), `gareev`, `ne5`, **BSNLP** — все доступны через
`natasha/corus` `[ДОК]`. На HuggingFace: `gusevski/factrueval2016` (65 загрузок),
`Vikhrmodels/factru_dataset` (8) `[ПРОВЕРЕНО]` — то есть датасет есть, но почти не используется.

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

`[НЕ НАЙДЕНО]`/`[НЕ ПРОВЕРЕНО]` — здесь честно фиксирую пробелы:

- **Sberbank NLP:** я не нашёл в этом проходе открытой библиотеки для **обезличивания/PII**.
  Модели `sberbank-ai/*` на HuggingFace существуют (ruBert, ruGPT и др.) и используются как
  base-модели (например, нашёл `ai-forever/ruBert-base` как base для `rubert-base-pii-ner`
  `[ПРОВЕРЕНО]`), но **открытого PII-детектора от Sber в этом проходе не нашёл**. Требует
  отдельной проверки в фазе 4.
- **Studio Ousia:** `[НЕ НАЙДЕНО]` — в контексте PII/анонимизации русского ничего не нашёл.
- **SLIM:** `[НЕ ПРОВЕРЕНО]` — не установил, о каком именно «SLIM» речь (возможные кандидаты:
  SPARQL-связанные проекты, `slim-ai` entity linking, «SLIM» как метод дистилляции). Требует
  уточнения постановщиком задачи.
- **NEREL** `[ПРОВЕРЕНО]` — это **датасет** русского NER, представлен на HuggingFace несколькими
  репликами: `iluvvatar/NEREL` (133 загрузки, 9 ❤), `soradan/nerel_short` (50),
  `hvostchedUser/NEREL_inno` (98), `bond005/NEREL_bench` (93), `danasone/nerel` (227),
  `RefalMachine/nerel-bio-simple` (33) и др. Лицензии реплик — по каждой свои `[НЕ ПРОВЕРЕНО]`.
  NEREL даёт более богатую схему, чем PER/LOC/ORG, — это потенциально полезно, но в нашем
  реестре типов таких категорий нет.
- **RuCoCo** `[НЕ НАЙДЕНО]` на HuggingFace (поиск `rucoco` — пусто). Мы в 00-charter ссылаемся
  на RuCoCo (arXiv 2206.04925) для кореференции — датасет в HF-хабе под этим именем не лежит,
  искать надо в другом месте. Это **открытый риск для ворот ContextGuard**.

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
| **Солёные плейсхолдеры** | `[KIND_<10 hex>]`, где хэш — **blake2b с перцем из окружения**. Дословно: «An unsalted hash of a 12-digit BIN is **brute-forceable in minutes** — that would make placeholders reversible. No pepper → the service refuses to start.» |
| Согласованность | «Same value → same placeholder across requests, so the model can tell that „this company“ in two paragraphs is the same party» — реализация нашего I5 через сервис, а не через сессию |
| Разрешение перекрытий | сортировка по confidence → приоритет вида → длина; отбрасывание перекрытий; **пороги ORG/LOC подняты (≥ 0.85, длина ≥ 5)** |
| Стоп-лист по леммам | «Russian legal text inflects everything. The stop-list compares **lemma sequences**, and a span is exempt only when **all** of its significant words are covered» — чтобы «Закону Республики Казахстан» остался читаемым в любом падеже, а «ТОО «Консультант Плюс»» замаскировался |
| **SSE-safe восстановление** | «the model streams chunks, and a placeholder tears across a boundary (`[PERSON_` + `80f18a2b3c]`). `StreamRewriter` keeps a minimal buffer: everything up to the last unclosed `[` is released immediately, the tail is held until the bracket closes. The user sees a live stream with **no torn placeholders and no added latency**.» |
| Fail-closed | `ANONYMIZE_ENABLED=false` работает только вместе с `ALLOW_PASSTHROUGH_FOR_TESTS`; при отсутствии `PLACEHOLDER_PEPPER` сервис **отказывается стартовать** |
| Логи | «The audit log stores only placeholder kinds and counts, **never values**» — наш I10 |
| Усечение до анонимизации | «Tool outputs (web search, page reads) are truncated **before** anonymization — one search result once returned **5.99 M characters**» |
| Деградация | «If Redis is down, **restoration degrades but anonymization keeps working** — nothing leaks.» |
| Тексты | «Long texts are split on sentence boundaries and anonymized **in full** rather than truncated» |

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

Цель раздела — не «что купить», а **какие архитектурные уроки извлечь**. Каждый урок должен
превратиться в требование к нам.

### 7.1 Microsoft Purview — единственный, у кого есть русские SIT

`[ПРОВЕРЕНО]` <https://learn.microsoft.com/en-us/purview/sit-sensitive-information-type-entity-definitions>,
обновлено **2026-06-15**. Это список **всех** SIT-определений; в нём есть раздел по России:

| Русская SIT | Что описано |
|---|---|
| **Russian Passport Number (Domestic)** | 10 цифр: 2 + опц. пробел/дефис + 2 + опц. пробел + 6 |
| **Russian Passport Number (International)** | — |
| **Russia physical addresses** | — |
| **Russia taxpayer identification number** | 10 или 12 цифр с контрольной суммой |

Это **важно**: единственная крупная проприетарная платформа, которая вообще имеет русский слой.
Аргумент «русский никому не нужен» опровергается наличием этих SIT — русский слой востребован
и в enterprise.

**Механика «primary + secondary + confidence»** `[ПРОВЕРЕНО]` — главный урок. На примере ИНН РФ
дословно:

> «A DLP policy has **high confidence** that it detects this type of sensitive information if,
> within a proximity of **300 characters**:
> - The function `Func_Russian_Taxpayer_Identification_Number` finds content that matches the pattern.
> - A keyword from `Keyword_Russian_Taxpayer_Identification_Number` is found.
>
> A DLP policy has **medium confidence** … if, within a proximity of 300 characters:
> - The function `Func_Russian_Taxpayer_Identification_Number` finds content that matches the pattern.»

XML-определение:

```xml
<Entity id="00aa00aa-..." patternsProximity="300" recommendedConfidence="85">
  <Pattern confidenceLevel="85">
    <IdMatch idRef="Func_Russian_Taxpayer_Identification_Number" />
    <Match idRef="Keyword_Russian_Taxpayer_Identification_Number" />
  </Pattern>
  <Pattern confidenceLevel="75">
    <IdMatch idRef="Func_Russian_Taxpayer_Identification_Number" />
  </Pattern>
</Entity>
```

Шкала `[ПРОВЕРЕНО]`: **Low ≤ 65, Medium = 75, High = 85.**

**Русские ключевые слова, дословно** `[ПРОВЕРЕНО]` — обратите внимание на качество списка:

```
НДС, номер налогоплательщика, Налогоплательщика ИД, налог число, налогчисло#, ИНН, НДС#
```

Ошибки в этом списке бросаются в глаза: **`НДС` — это НДС (VAT), а не ИНН**; `Налогоплательщика ИД`
— калька с английского. То есть даже Microsoft в русском списке ключевых слов допустила
семантическую ошибку. **Урок: русские контекстные слова надо собирать и валидировать на корпусе,
а не переводить.** Это подкрепляет наш открытый пункт «определить список отрицательных меток
эмпирически».

**Для паспорта РФ** `[ПРОВЕРЕНО]`:

> «Checksum: **No** …
> A DLP policy has **medium confidence** that it's detected this type of sensitive information if,
> within a proximity of 300 characters:
> - The regex `Regex_Russian_Passport_Number_Domestic` finds content that matches the pattern.
> - A keyword from `Keyword_Russian_Passport_Number` is found.»

То есть **максимум, который даёт Purview для паспорта РФ — medium (75)**, потому что нет
контрольной суммы. **Это независимое подтверждение нашего решения `context_required` для
`ru.passport`.** Microsoft пришла к тому же выводу.

Русские ключевые слова для паспорта содержат **тоже с ошибками**: «паспорт **нет**», «pоссийской
паспорт» (латинская `p`!), «pусский номер паспорта», «паспортid#». Часть — явные опечатки, часть —
OCR-артефакты. `[ПРОВЕРЕНО]`

**Первый/вторичный признак и близость 300 символов** — это **более слабая** версия нашего L1:
у нас расстояние до ближайшего ключа (§6.2, pii-guard), у них — «в пределах 300 символов».
Purview не различает, какой ключ ближе, — ровно та проблема, которую pii-guard решил
расстоянием. **Мы должны быть лучше Purview здесь, и это легко.**

**Про суррогаты в Purview:** `[НЕ НАЙДЕНО]` — в проверенной документации SIT механики генерации
фейковых значений нет. Термины «pseudonymization»/«mock data» в контексте SIT я не подтвердил.
Документация говорит только о детекции и confidence.

**Azure AI Language PII** — Presidio предлагает его как **remote recognizer**
(`docs/samples/python/text_analytics/`), и в `supported_entities.md` есть отдельный раздел
«Complementing Presidio with Azure AI Language PII». То есть Microsoft предлагает **своё облако**
как способ закрыть пробелы Presidio `[ПРОВЕРЕНО]`. Русский в списке языков Azure AI Language PII
`[НЕ ПРОВЕРЕНО]` — в этом проходе не проверял.

### 7.2 Google Cloud DLP / Sensitive Data Protection

`[ПРОВЕРЕНО]` <https://docs.cloud.google.com/sensitive-data-protection/docs/…>, обновлено 2026-10-13.

**Единственный, у кого есть суррогаты как продуктовая функция.** Из списка официальных сэмплов
`[ПРОВЕРЕНО]` (страница samples):

- **«De-identify free text with FPE by using a **surrogate**»** ← `dlp-deidentify-free-text-with-fpe-using-surrogate`
- «Re-identify free text with FPE using a **surrogate**» ← `dlp-reidentify-free-text-with-fpe-using-surrogate`
- «Format-preserving encryption (FPE)» ← `dlp-deidentify-fpe`
- «De-identify table data with format-preserving encryption»
- «De-identify content through **deterministic** encryption»
- «De-identify data: Redacting with matched input values»
- «De-identify sensitive data by replacing with infoType»
- «Character masking»
- «Date shifting», «table bucketing», «conditional logic»

**Концептуальный вывод, который мы обязаны забрать.** Google решает задачу «реалистичный суррогат»
**через Format-Preserving Encryption, а не через генератор фейков**. Свойства FPE:
- результат **сохраняет формат** (16-значная карта → 16-значная карта);
- **обратим** детерминированно (re-identify тем же ключом);
- **не требует справочников** и не может «попасть в реальный реестр» (это не реальный ИНН);
- детерминирован: одно значение → один суррогат (то, что нам нужно для I5).

Наш `similar_fake` из 00-charter (§5.4) — это **гибрид**: FPE даёт формат и детерминизм, но не
даёт правдоподобия (распределение по регионам, валидная контрольная сумма). Наш пункт 3
«Правдоподобие: сохранение распределения по регионам, годам, префиксам операторов» FPE **не
закрывает**. Значит, нам нужен либо FPE + пост-коррекция контрольной суммы, либо явный
генератор с валидацией. **Это важный архитектурный выбор, который аудит вытаскивает наверх.**

**InfoTypes:** `[ПРОВЕРЕНО]` — есть страница «InfoType detector reference» и метод API
`infoTypes.list`; точное **число** встроенных детекторов я в этом проходе не получил
`[НЕ ПРОВЕРЕНО]`.

**Русские infoType и языковая поддержка:** `[НЕ ПРОВЕРЕНО]` — страница `language-support`
вернула **404** (и на `cloud.google.com`, и на `docs.cloud.google.com`). Русские infoTypes я
**не подтвердил**. Google исторически имеет `RUSSIA_*` детекторы (например, для паспорта РФ) —
но утверждать это без источника не буду. **Ставлю в список «допроверить в фазе 4»**.

**Что берём:**
1. **FPE-суррогат как механизм** — формат-сохраняющая обратимая подмена без справочников.
2. **Список transformation-типов** как чек-лист полноты: redact, replace, mask, crypto-hash,
   deterministic crypto, FPE, bucketing, date shifting, generalization, conditional logic.
   Наша модель `Substitution.kind` (`similar_fake` / `placeholder` / `category_label`) — беднее.
3. **Разделение «де-идентификация» и «ре-идентификация» как двух API**, а не одного.
4. **Уровни likelihood** (у Google — от `VERY_UNLIKELY` до `VERY_LIKELY`), как у Purview
   confidence 65/75/85.

### 7.3 AWS Comprehend и Bedrock Guardrails

**Comprehend PII — русского нет** `[ПРОВЕРЕНО]`
<https://docs.aws.amazon.com/comprehend/latest/dg/supported-languages.html>:

| Feature | Supported languages |
|---|---|
| Entities | All supported languages |
| Key phrases | All supported languages |
| **Detecting PII entities** | **English and Spanish** |
| **Labeling PII entities** | **English and Spanish** |
| Sentiment | All supported languages |
| Targeted sentiment | English |
| Syntax analysis | de, en, es, fr, it, pt |
| Custom entity recognition | de, en, es, fr, it, pt |

Базовый список языков Comprehend: de, en, es, it, pt, fr, ja, ko, hi, ar, zh, zh-TW —
**русского нет вообще**. То есть AWS Comprehend **не может** обрабатывать русский текст ни в
одной функции. Это чистый минус в сравнительной таблице.

**Bedrock Guardrails — механика, которую надо забрать** `[ПРОВЕРЕНО]`
<https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-sensitive-filters.html>:

- «a **probabilistic ML based solution** that is **context-dependent** and detects sensitive
  information based on the context within input prompts or model responses»;
- два режима: **Block** (блокирует всё содержимое) и **Mask** («replaces it with the PII type
  (for example, `{NAME}` or `{EMAIL}`)») — то есть **маска = плейсхолдер, не суррогат**;
- список PII **34 типа** по группам (я насчитал по документу):

| Группа | Типы | Кол-во |
|---|---|---|
| General | ADDRESS, AGE, NAME, EMAIL, PHONE, USERNAME, PASSWORD, DRIVER_ID, LICENSE_PLATE, VEHICLE_IDENTIFICATION_NUMBER | 10 |
| Finance | CREDIT_DEBIT_CARD_CVV, CREDIT_DEBIT_CARD_EXPIRY, CREDIT_DEBIT_CARD_NUMBER, PIN, INTERNATIONAL_BANK_ACCOUNT_NUMBER, SWIFT_CODE | 6 |
| IT | IP_ADDRESS, MAC_ADDRESS, URL, AWS_ACCESS_KEY, AWS_SECRET_KEY | 5 |
| USA specific | US_BANK_ACCOUNT_NUMBER, US_BANK_ROUTING_NUMBER, US_INDIVIDUAL_TAX_IDENTIFICATION_NUMBER, US_PASSPORT_NUMBER, US_SOCIAL_SECURITY_NUMBER | 5 |
| Canada specific | CA_HEALTH_NUMBER, CA_SOCIAL_INSURANCE_NUMBER | 2 |
| UK specific | UK_NATIONAL_HEALTH_SERVICE_NUMBER, UK_NATIONAL_INSURANCE_NUMBER, UK_UNIQUE_TAXPAYER_REFERENCE_NUMBER | 3 |
| Custom | Regex filter | 1 |
| **Итого** | | **32 + regex** |

**Русских типов нет ни одного.** Группы: General, Finance, IT, USA, Canada, UK. Ни Russia,
ни EU, ни СНГ.

**Четыре урока, которые надо забрать из документации Bedrock:**

1. **Явное предупреждение про контекст:** «The PII model performs more effectively when it is
   provided with **sufficient context**. To enhance its accuracy, include more contextual
   information and **avoid submitting single words or short phrases**. Since PII can be
   context-dependent (for example, a string of digits might represent an AWS KMS key or a user ID
   depending on the surrounding information), providing comprehensive context is crucial.»
   **Это ровно наш L1-каскад, сформулированный вендором.** Мы можем цитировать AWS как
   подтверждение необходимости контекста.
2. **Явно перечисленные слепые зоны** (то, чего мы тоже не должны обещать):
   > «it does not evaluate the following, so PII in these fields is neither blocked nor masked:
   > PII the model generates into **tool call arguments** … PII in **tool results** … PII in the
   > **tool definitions**.»
   Для нас это будущая проблема: если пользователь даёт LLM инструменты, PII утечёт через
   аргументы вызова. В нашем v0.1 инструментов нет, но раздел «чего не умеем» должен это
   предвидеть.
3. **Утечка через трассировку** — признание, которое мы обязаны учесть в I10:
   > «the `match` field in `GuardrailPiiEntityFilter` … contains the **original PII value**, not the
   > masked output. This behavior is by design so that your application can use the detection result
   > for its own logic.»
   И: «**Model invocation logs** — the `input` field in Amazon CloudWatch Logs always contains the
   **original, unmodified request** regardless of guardrail intervention.»
   То есть **обезличивание в AWS не покрывает логи**. Наш I10 («логи, телеметрия, отладочный
   вывод не содержат исходных значений») — прямое конкурентное преимущество, потому что у
   крупнейшего вендора это сломано by design.
4. **Ограничение кастомных regex:** «A custom regex filter of sensitive information filters does
   **not support a regex lookaround match**.» — полезная деталь при проектировании нашего
   regex-движка, если мы захотим совместимость.

### 7.4 Сводная таблица проприетарных ориентиров

Требование задания — «(a) детектит русский, (b) генерирует реалистичные суррогаты,
(c) сохраняет формат/валидность контрольной суммы, (d) работает офлайн, (e) поддерживает
кастомные типы/regex, (f) confidence со вторичными признаками». Источник на ячейку.

| | **Purview** | **Google DLP / SDP** | **AWS Comprehend** | **AWS Bedrock Guardrails** | **Cloudflare AI Gateway** |
|---|---|---|---|---|---|
| **(a) Русский** | **Да**, 4 SIT: паспорт РФ ×2, адреса РФ, ИНН РФ `[ПРОВЕРЕНО]` | `[НЕ ПРОВЕРЕНО]` — страница language support вернула 404 | **Нет** — PII только «English and Spanish»; русского нет в списке языков вообще `[ПРОВЕРЕНО]` | **Нет** — 0 ru-типов из 32 `[ПРОВЕРЕНО]` | `[НЕ НАЙДЕНО]` ru-профилей `[ПРОВЕРЕНО]` |
| **(b) Суррогаты** | `[НЕ НАЙДЕНО]` механики в SIT-документации | **Да** — FPE-суррогат, отдельные сэмплы de/re-identify `[ПРОВЕРЕНО]` | Нет | **Нет** — маска = `{NAME}`, `{EMAIL}` `[ПРОВЕРЕНО]` | **Нет** — только FLAG/BLOCK `[ПРОВЕРЕНО]` |
| **(c) Формат / checksum** | Формат да (паттерны); **валидность суррогата не заявлена** | FPE сохраняет формат by design; **валидность контрольной суммы не заявлена** | — | Плейсхолдер — формат не сохраняется | — |
| **(d) Офлайн** | Только как облачный сервис (Purview/Azure) | Нет (облако) | Нет | Нет | Нет |
| **(e) Кастомные типы** | Да — настраиваемые SIT + trainable classifiers `[ДОК]` | Да — 3 вида кастомных детекторов: regular dictionary, **stored dictionary до десятков миллионов**, regex `[ПРОВЕРЕНО]` | Да — Custom Entity Recognition, но не для ru | Да — **Regex filter** (без lookaround) `[ПРОВЕРЕНО]` | Да — Custom Profiles `[ПРОВЕРЕНО]` |
| **(f) Confidence + вторичные признаки** | **Да, образцово:** `recommendedConfidence=85`, два Pattern'а (85 = primary+keyword, 75 = primary), `patternsProximity="300"`, шкала Low≤65/Medium=75/High=85 `[ПРОВЕРЕНО]` | Да — `minLikelihood` от VERY_UNLIKELY до VERY_LIKELY, hotword rules, exclusion lists `[ПРОВЕРЕНО]` | Да — confidence scores у Comprehend | — | Да — «Tune confidence levels», настройка чувствительности `[ПРОВЕРЕНО]` |
| **Лицензия** | Проприетарная | Проприетарная | Проприетарная | Проприетарная | Проприетарная |

**Главный вывод раздела 7.** Из четырёх крупных облаков **только Purview имеет русский слой**
(4 SIT), **только Google имеет суррогаты** (через FPE), и **ни одно не имеет русских суррогатов**.
Пересечение «русский × суррогаты × офлайн» пусто **даже среди проприетарных**, не говоря об
open-source. Это и есть наша ниша, и она подтверждена с двух сторон — и open-source обзором (§5),
и вендорским (§7).

**И третий сильный аргумент: у vendoров сломаны базовые вещи**, которые мы объявили инвариантами:
- AWS **by design** пишет исходный текст в CloudWatch и возвращает исходное значение в trace
  → наш I10 выигрывает;
- Purview **не может** дать ничего выше medium для паспорта РФ → наше `context_required` верно;
- Purview **ошибается** в русских контекстных словах (`НДС` вместо `ИНН`) → наш пункт
  «определить метки эмпирически» обоснован, а не перфекционизм;
- Cloudflare **не умеет** обезличивать вообще (только блокировать) → вся ниша обезличивания
  в шлюзах открыта.

---

## 8. Что берём, что отвергаем и почему

### 8.1 Что берём — по решениям

| № | Откуда | Что берём | Почему (с источником) |
|---|---|---|---|
| 1 | **pii-guard** | **Разделение «правила + модель» с приоритетом правил** и признание побочного эффекта | `[ПРОВЕРЕНО]` docs/architecture.md: «правила побеждают модель: детерминированная контрольная сумма надёжнее вероятностной оценки»; и честно про «ошибочное правило гасит правильное предсказание модели» |
| 2 | **pii-guard** | **Расстояние до ближайшего ключа вместо «есть ли ключ в окне»** | `[ПРОВЕРЕНО]`: контрпример «водительское удостоверение 77 АВ 123456, индекс 630009» + измеренные бюджеты (BIK 40, паспорт 80/160, p99=83) |
| 3 | **pii-guard** | **Наружу отдаётся значение, а не фраза** | `[ПРОВЕРЕНО]`: `серия ****, номер ******`, а не `**** **** **** ****` — «плейсхолдер на всю фразу ломает предложение сильнее, чем требуется» |
| 4 | **pii-guard** | **Восстановление с согласованием падежа** (pymorphy3 + spaCy) | `[ПРОВЕРЕНО]`: `Свяжитесь с Иваном Петровым`, а не `с Иван Петров`. Никто из западных решений этого не делает |
| 5 | **pii-guard** | **Предобработка: транслит → кириллица, числа словами, исключение base64** | `[ПРОВЕРЕНО]`: наш I1 требует ровно этих нормализаций; у них реализовано |
| 6 | **pii-guard** | **Явное предупреждение про лицензию optional extra** (gender-guesser = GPLv3) | `[ПРОВЕРЕНО]` README: «Копилефта в установке по умолчанию нет. Единственное исключение придётся подключить руками» |
| 7 | **pii-guard / Exemtik** | **Пометка, что смещения относятся к `normalized_text`, а не к исходному тексту** | `[ПРОВЕРЕНО]` оба источника. У нас в модели данных это не оговорено — исправить |
| 8 | **Presidio** | **Явные правила разрешения перекрытий**, включая случай «внутри другой → берём длинную» | `[ПРОВЕРЕНО]` docs/anonymizer. И честное признание, что при частичном пересечении получается `<PERSON><LOCATION>` |
| 9 | **Presidio** | **Контекстное усиление как отдельный слой** (`LemmaContextAwareEnhancer`) + `context_matching_mode` | `[ПРОВЕРЕНО]`: word-boundary матчинг контекстных слов; баг `lic` внутри `duplicate` |
| 10 | **Presidio** | **`low_confidence_score_multiplier: 0.4` и `labels_to_ignore`** для шумных NER-меток | `[ПРОВЕРЕНО]` conf/default.yaml: ORGANIZATION в игнор-листе «Has many false positives» |
| 11 | **Presidio** | **Модель угроз «нет аутентификации by design»** задокументирована | `[ПРОВЕРЕНО]` CHANGELOG 2.2.362, #1903. Мы обязаны сделать то же — и у нас это уже есть («раздел чего не умеем обязателен») |
| 12 | **Presidio** | **Немецкий набор как образец пакетного вклада юрисдикции** (13 рекогнайзеров, ISO 7064 Mod 11,10, DRV, GKV, legal basis в описании) | `[ПРОВЕРЕНО]` supported_entities.md. Шаблон для нашего `rules/ru/` |
| 13 | **SurrogateShield** | **ShadowMap: AES-256-GCM + HKDF(device_secret, conv_id, kind) + associated data** | `[ПРОВЕРЕНО]` README «Security Design»: kind+ID как AAD → «a file only opens as what it was written for» |
| 14 | **SurrogateShield** | **Битый файл карты НЕ заменяется пустым**: «Moved aside to `<name>.corrupt-<ts>` and an error is raised: never silently replaced by an empty map» | `[ПРОВЕРЕНО]` — прямое усиление нашей I4/I6 |
| 15 | **SurrogateShield** | **Fail-closed с явным исключением**: `DetectorUnavailable` вместо отправки незамаскированного; `ALLOW_EPHEMERAL_KEY` как opt-in | `[ПРОВЕРЕНО]` — ровно наша I4 «значение по умолчанию — блокировать отправку» |
| 16 | **SurrogateShield** | **Проверка весов модели по SHA-256 на фиксированный коммит HF** | `[ПРОВЕРЕНО]` — защита от подмены модели, у нас этого нет |
| 17 | **SurrogateShield** | **Whole-value matching при восстановлении** («Lee» never matches inside «Leeds») + категоризация `exact_miss`/`fuzzy_hit`/`fuzzy_miss` | `[ПРОВЕРЕНО]` — наша I3 + метрика качества восстановления |
| 18 | **SurrogateShield** | **Пресеты `balanced`/`strict`/`fast`/`classic`** | `[ПРОВЕРЕНО]` — совпадает с нашей идеей профилей |
| 19 | **SurrogateShield** | **Квази-идентификаторный скоринг по k-anonymity** с явным списком комбинаций | `[ПРОВЕРЕНО]` 10 комбинаций, «ZIP+DOB+Gender = 87%» (Sweeney 2000) |
| 20 | **SurrogateShield** | **Форматная валидность суррогатов как требование** (Luhn, ABA, Base58) | `[ПРОВЕРЕНО]` — подтверждает наш §5.4 п.1 |
| 21 | **llm-pii-proxy-showcase** | **Солёный хэш фейка с перцем из окружения; отказ стартовать без перца**; обоснование: «An unsalted hash of a 12-digit BIN is brute-forceable in minutes» | `[ПРОВЕРЕНО]` — прямое требование к нашим подменам и к I9 |
| 22 | **llm-pii-proxy-showcase** | **SSE-safe восстановление** с буфером до последней незакрытой `[` | `[ПРОВЕРЕНО]` — обязательное требование, если наш прокси поддерживает стриминг |
| 23 | **llm-pii-proxy-showcase** | **Стоп-лист по последовательностям лемм**, а не по словам | `[ПРОВЕРЕНО]`: «Закону Республики Казахстан» остаётся читаемым в любом падеже |
| 24 | **llm-pii-proxy-showcase** | **Маскирование даты только в личном контексте** (дни рождения/паспорта), сроки договоров — нет | `[ПРОВЕРЕНО]` — сохранение смысла, наш §6.3 «Сохранение смысла» |
| 25 | **llm-pii-proxy-showcase** | **Второй признак должен быть семантически связан**: у KZ_IIN внутри номера проверяется реальная календарная дата («Feb 31» проходил range-check) | `[ПРОВЕРЕНО]` — усиливает наш §4.1 реестра (для паспорта РФ нужен второй признак, но не «любая проверка») |
| 26 | **llm-pii-proxy-showcase** | **Пороги ORG/LOC подняты (≥0.85, длина ≥5) для юридического русского текста** | `[ПРОВЕРЕНО]`: «spaCy is noisy on them in legal text» |
| 27 | **llm-pii-proxy-showcase** | **Усечение инструментального вывода ДО анонимизации** («one search result once returned 5.99 M characters») | `[ПРОВЕРЕНО]` — защита от исчерпания и от пропуска PII за лимитом |
| 28 | **llm-pii-proxy-showcase** | **Деградация без утечки**: «If Redis is down, restoration degrades but anonymization keeps working» | `[ПРОВЕРЕНО]` — архитектурный принцип: хранилище не на критическом пути обезличивания |
| 29 | **llm-pii-proxy-showcase** | **Аудит-лог хранит только виды и количества, никогда значения** | `[ПРОВЕРЕНО]` — наша I10 |
| 30 | **Natasha / Slovnet** | Сам NER-провайдер: **2 MB модель, 27 MB диск, 205 MB RAM, CPU, Numpy, MIT** | `[ПРОВЕРЕНО]` — единственный инструмент, попадающий в профиль Balanced |
| 31 | **Exemtik/pii-bench-ru-en** | **`pymorphy3` как дешёвый детектор ФИО**: F1 по NAME 0.794 без нейросетей, работает в любом падеже | `[ПРОВЕРЕНО]` — кандидат в L2 для профиля с малым весом |
| 32 | **Exemtik/pii-bench-ru-en** | **`ru_core_news_sm` (14.6 MB) как дешёвый апгрейд**: +0.116 F1 по именам и ненулевой ADDRESS (0.507) | `[ПРОВЕРЕНО]` — 14.6 MB за адрес, которого regex не даёт вообще |
| 33 | **Exemtik/pii-bench-ru-en** | **Отрицательная аддитивность**: `regex+morph+spacy_ru` (0.873) **хуже** `regex+spacy_ru` (0.897) | `[ПРОВЕРЕНО]` — не сваливать все детекторы в кучу; выбирать источник NER |
| 34 | **Exemtik/pii-bench-ru-en** | Датасет **`hivetrace/pii-bench`** с 13 русскими типами (INN, KPP, OGRN, OGRNIP, SNILS, PASSPORT_NUMBER…) | `[ПРОВЕРЕНО]` — готовый внешний тест для нашего Extended-уровня |
| 35 | **redmadrobot** | Датасет **`pii_benchmark`** (2 841 предложение, MIT), disjoint от train | `[ПРОВЕРЕНО]` — второй внешний тест; правило «не пересекается с нашим dev» соблюсти |
| 36 | **Google DLP** | **FPE-суррогат как механизм**: формат-сохраняющая, детерминированная, обратимая подмена без справочников | `[ПРОВЕРЕНО]` официальные сэмплы de/re-identify free text with FPE using a surrogate |
| 37 | **Google DLP** | **Полный чек-лист типов трансформаций** (redact, replace, mask, crypto-hash, deterministic crypto, FPE, bucketing, date shifting, generalization, conditional) | `[ПРОВЕРЕНО]` transformations-reference + список сэмплов |
| 38 | **Google DLP** | **Stored custom dictionary до десятков миллионов фраз** (Bloom-фильтры в Cloud Storage) | `[ПРОВЕРЕНО]` concepts-de-identification: «up to tens of millions» |
| 39 | **Purview** | **Primary/secondary признак с числовым confidence**: `recommendedConfidence=85`, два Pattern (85 = функция+ключ, 75 = функция), `patternsProximity="300"` | `[ПРОВЕРЕНО]` XML SIT-определения |
| 40 | **Purview** | **Публичная шкала** Low ≤ 65 / Medium 75 / High 85 | `[ПРОВЕРЕНО]` |
| 41 | **Purview** | **Независимое подтверждение `context_required` для паспорта РФ**: «Checksum: No», максимум «medium confidence» | `[ПРОВЕРЕНО]` sit-defn-russia-passport-number-domestic |
| 42 | **Purview** | **Юридическое основание в описании типа** (`Legal basis: §§ 139a–139e AO` у немецких SIT) | `[ПРОВЕРЕНО]` supported_entities.md — у нас это часть «раздела чего не умеем» (юрисдикция) |
| 43 | **AWS Bedrock** | **Явное требование контекста** и запрет на «single words or short phrases» как подтверждение нашего L1 | `[ПРОВЕРЕНО]` guardrails-sensitive-filters |
| 44 | **AWS Bedrock** | **Явный список слепых зон** (tool call arguments, tool results, tool definitions) | `[ПРОВЕРЕНО]` — заготовка для нашего раздела «чего не умеем» |
| 45 | **AWS Bedrock** | **Ошибка вендора как урок**: `match` в trace и CloudWatch `input` содержат **исходные** значения | `[ПРОВЕРЕНО]` — обоснование нашего I10 как конкурентного преимущества |
| 46 | **Cloudflare** | **Профили как политики** + машиночитаемый отчёт о срабатывании (`cf-aig-dlp`, коды 2029/2030) | `[ПРОВЕРЕНО]` — образец для нашей структуры `Report` |
| 47 | **LiteLLM** | **Наша роль как guardrail-провайдера** в чужих шлюзах; готовый набор режимов (`pre_call`/`post_call`/`logging_only`/`pre_mcp_call`) | `[ПРОВЕРЕНО]` — конкретный путь интеграции фазы 7 |
| 48 | **LiteLLM** | **Авто-дробление длинных текстов** с перекрытием и слиянием детекций | `[ПРОВЕРЕНО]` `presidio_analyze_chunk_size_bytes` (default 500 000) — у нас нет стратегии для текстов > лимита |
| 49 | **arx-deidentifier/arx** | Методология **k-anonymity** (Apache-2.0, 738 ★) как референс для квази-идентификаторов | `[ПРОВЕРЕНО]` GitHub API — соседний зрелый проект |
| 50 | **Aajil-Labs/arabic-pii-py** | **Подтверждение, что «local-first, reversible PII для одной юрисдикции» — рабочая стратегия** (арабский/Залив, Apache-2.0) | `[ПРОВЕРЕНО]` — 12 ★, но паттерн ровно наш |

### 8.2 Что отвергаем — по решениям, с причиной

| № | Что отвергаем | Причина |
|---|---|---|
| 1 | **Presidio как зависимость** | Python; `spacy` — **обязательная** зависимость (противоречит «ядро без ML»); `en_core_web_lg` **382 MB**; **0 русских рекогнайзеров**; русских контекстных слов нет ни в одном рекогнайзере; нет состояния сессии; `presidio-cli` не публикуется |
| 2 | Presidio `hash` как модель подмен | С 2.2.361 **случайная соль по умолчанию** → одно значение даёт разные хэши, что **нарушает наш I5** `[ПРОВЕРЕНО]` CHANGELOG |
| 3 | Presidio `replace`/`mask`/`redact` как целевое поведение | Дают `<PHONE_NUMBER>`, `****1234`, удаление — это **не подмена на похожие фейки**, а потеря смысла |
| 4 | Presidio `surrogate_ahds` как путь к суррогатам | Требует **облака Azure**; `input_locale`/`surrogate_locale` default **`en-US`**; русского locale не заявлено; противоречит «локально, без сети» |
| 5 | Presidio-конвенция «одна сущность внутри другой → берём длинную» **без оговорок** | Приводит к подмене более широкого спана, чем нужно; у нас нужна проверка `checksum`/`context` перед расширением |
| 6 | **LLM Guard как зависимость** | **Архивирован 2026-07-08**; **модели на HF тоже не поддерживаются**; последний релиз PyPI 2025-05-19; 41 контрибьютор, но развития нет |
| 7 | LLM Guard как «решение для обезличивания» | Это 36 сканеров, из которых анонимизационный — **один**; остальные 34 не относятся к задаче и добавляют зависимостей и поверхности атаки |
| 8 | LLM Guard `use_faker` | Не дефолт; «for **eligible** entities» — критерий не раскрыт; Faker не даёт валидных по контрольной сумме русских ID |
| 9 | Ориентация на p95 LLM Guard | **169.13 ms (ONNX) / 294.57 ms** p95 на **317 символах** `[ПРОВЕРЕНО]` — в 3.4–5.9× выше нашего потолка 50 ms при в 44× меньшем объёме |
| 10 | **Portcullis как реализацию** | **Кода нет** `[НЕ НАЙДЕНО]` (GitHub Search = 0, arXiv = 0, на странице статьи только PDF и постер) |
| 11 | Portcullis TEE/аттестацию как основу | Требует серверного CPU с confidential computing; не работает на потребительском CPU и в WASM; не решает ни русский, ни суррогаты, ни контекст |
| 12 | Цитату «96× быстрее предшественников» | Абстракт: 96× — это **«for masking and unmasking workloads»** против **Hide-and-Seek**; качество на Enron — **+0.1 accuracy**, а не 96× |
| 13 | **SurrogateShield как код** | Python; `en_core_web_lg`; `ssn`/`zip_us`/`us_bank_number`/ABA/`postal_code_uk` — **ни одной русской сущности**; **2 ★**, 0 форков, единственный автор (bus factor = 1) |
| 14 | **Цифры SurrogateShield как эталон** | Все **self-reported** одним автором; независимых репликаций `[НЕ НАЙДЕНО]`; репозиторий создан 5 месяцев назад |
| 15 | Модель задержки SurrogateShield | **43.5 ms медиана, 1 348 MB пик** `[ПРОВЕРЕНО, self-reported]` — не влезает в Balanced (25 ms, 250 MB) |
| 16 | Surrogates SurrogateShield `ADDRESS_MODE=shift` как есть | Американский address parser (штаты, ZIP+4, directionals), `ZIP + DOB + Gender` — US-специфика Sweeney |
| 17 | **scrubadub** | Последний релиз и push **2023-09-01**; `textblob==0.15.3` жёстким пином; `scikit-learn` без необходимости; **расхождение лицензий** Apache-2.0 (репо) vs MIT (PyPI); нет ru |
| 18 | **OpenAnonymiser / anonymiq-light** | Только `nl_core_news_lg`; EUPL-1.2; **0 ★**; flavor `contextual` **не реализован**; суррогатов нет |
| 19 | **pydeidentify** | **Нет лицензии**; 1 ★; 2023; spaCy-only; нет ru |
| 20 | **anonymize-it** | **Не существует** (PyPI 404, GitHub Search — только посторонние проекты) |
| 21 | **«Oban» как инструмент обезличивания** | `[НЕ НАЙДЕНО]`; в экосистеме `Oban` — очередь задач на Elixir, к PII отношения не имеет |
| 22 | **DeepPavlov как зависимость** | PyTorch; **1 024–2 048 MB диск**, **3 072–6 144 MB RAM**; «from 4 to 16 GB RAM»; **34.5 s инициализации**; замеры на GPU |
| 23 | **Yandex Tomita как код** | **MPL-2.0**; последний push **2020-11-16** (мёртв); сборка через cmake 2.8 + lua 5.2 + gcc 4.8 + отдельный `libmystem-c-binding.so`; свой язык конфигов |
| 24 | **spaCy `ru_core_news_lg`** | **489.6 MB** против **2 MB** Slovnet при сопоставимом качестве на новостях; только LOC/ORG/PER; обучена на новостях |
| 25 | **Никакую русскую модель, не прошедшую наш замер** | Это правило, а не отказ: `Wismut/nym-pii-multilingual`, `tabularisai/eu-pii-safeguard`, `dzanozin/traciora-opf-ru-pii-v6` (1,4 млрд параметров!) — все `[НЕ ПРОВЕРЕНО]` и в фазе 4 должны измеряться нашим харнесом |
| 26 | **Cloudflare AI Gateway как механизм обезличивания** | Только **FLAG/BLOCK**; обезличивания нет вообще; ru-профилей нет |
| 27 | **AWS Comprehend для русского** | PII только «English and Spanish»; русского нет ни в одной функции Comprehend |
| 28 | **AWS Bedrock `Mask`** | Заменяет на `{NAME}`/`{EMAIL}` — плейсхолдер, не суррогат |
| 29 | **LiteLLM Presidio-guardrail как движок** | Русский в списке языков — `en`/`es`/`de`; `MASK` = `<PERSON>`; требует развёрнутых контейнеров Presidio; рекурсивно наследует все минусы Presidio |
| 30 | **Приватность = локальный инференс** (`private-gpt`) | `zylon-ai/private-gpt` (57 583 ★) решает **другую задачу**: данные не уходят, но и не обезличиваются. Это не наш конкурент и не наш ориентир |
| 31 | **Presidio как «эталон, который мы догоняем»** | Он не эталон по нашей задаче: 8.2 ms медиана на коротком тексте + 31.6% утечки на реальных сообщениях (замер SurrogateShield) — то есть по **главной** метрике (доля запросов с утечкой) Presidio слаб |
| 32 | **Идею «мы первые делаем суррогаты»** | SurrogateShield делает суррогаты с мая 2026, arXiv с июня 2026. Корректная формулировка: **«мы первые делаем суррогаты для русского, локально, на CPU»** |

### 8.3 Что меняется в наших документах по итогам аудита

| Наш документ | Что поправить | Основание (URL) |
|---|---|---|
| 00-charter §2.7 | Формулировку про Portcullis уточнить: первый автор — Tsinghua, пятеро — ByteDance; 96× — **маскирование/снятие маски** против Hide-and-Seek | <https://ojs.aaai.org/index.php/AAAI/article/view/32088> |
| 00-charter §2.6 | Тезис «ни у Presidio, ни у LLM Guard, ни у Portcullis этого нет» **подтверждён**, но добавить: **ни у кого** из проверенных (§4, §5, §6, §7) — включая SurrogateShield, pii-guard и все облака | §1.2 этого отчёта |
| 00-charter §2.2 п.2 | «доступ из Presidio-подобных пайплайнов» уточнить: Presidio **архивировал** `presidio-cli` из релизов и **обязательно тянет spaCy** — значит, наша ценность в том, что мы **заменяем**, а не «дополняем» | CHANGELOG 2.2.356, `presidio-analyzer/pyproject.toml` |
| 00-charter §5.4 `similar_fake` | Добавить требование **форматной валидности** и рассмотреть **FPE** как альтернативу генератору; разделить «формат+детерминизм» (FPE) и «правдоподобие распределения» (генератор с валидацией) | <https://docs.cloud.google.com/sensitive-data-protection/docs/samples/dlp-deidentify-free-text-with-fpe-using-surrogate> |
| 00-charter §5.4 | Добавить: **фейк не должен быть валидным реальным идентификатором** — иначе adversarial-атака «инверсия фейка» (`§6.5` плана) даёт попадание в реестр. FPE это закрывает by design | та же ссылка |
| 00-charter §5.2 | Добавить пункт: **смещения относятся к нормализованному тексту**; публичный контракт должен это фиксировать; для подсветки в оригинале — отдельный API переноса смещений | pii-guard README, llm-pii-proxy-showcase README |
| 00-charter §5.4 | Добавить: **суррогат хэшируется с перцем из окружения**; без перца — отказ; несолёный хэш 12-значного ID брутфорсится за минуты | llm-pii-proxy-showcase README |
| 00-charter §5.4 п.4 | «Детерминированность от seed» — оставить, но добавить: **детерминированность должна быть криптографической** (FPE/ключ), а не «от seed», иначе подмена обратима без карты (наша adversarial-атака «через формат») | Google FPE docs |
| 20-entity-registry §5 | Наш контекстный каскад L1 описан как «окно ±N токенов». **Заменить на расстояние до ближайшего ключа** с измеренными бюджетами | pii-guard docs/architecture.md |
| 20-entity-registry §5 п.3 | «Отрицательная метка важнее положительной» — **подтверждается**, добавить: у pii-guard есть готовый приём (сужение спана до данных внутри фразы: `серия ****, номер ******`) | pii-guard docs/architecture.md |
| 20-entity-registry §4.1 | Для `ru.passport` **независимое подтверждение**: Purview ставит максимум **medium (75)**, `Checksum: No` | <https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-domestic> |
| 20-entity-registry §4.1 | Добавить в `ru.inn_legal`/`ru.inn_person`: **реальный F1 для ИНН — 68.3 у чистой модели**, 88.9 у пайплайна. Публиковать recall по ИНН отдельно, а не средним | <https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner> |
| 20-entity-registry §3 | Открытый пункт «сверить веса счёта ЦБ и идентификаторов СНГ»: **единственный найденный кандидат для сверки KZ** — `kz_recognizers.py` в showcase (MIT, 32 теста), но авторы сами убрали клиентский тест-набор. **Остаётся `unverified`** | llm-pii-proxy-showcase README |
| 20-entity-registry §4.2 | `kz.iin` — добавить требование: **второй признак = реальная календарная дата внутри номера**, а не range-check | llm-pii-proxy-showcase README |
| 00-charter §6.5 | Adversarial-атака «инверсия фейка»: добавить критерий **«фейк не попадает в реальные реестры»** → FPE предпочтительнее генератора; и добавить атаку **«брутфорс плейсхолдера»** (несолёный хэш) | Google FPE, llm-pii-proxy-showcase |
| 00-charter §7.2 | Бюджет **L2 NER 12 ms** — сейчас не обоснован. Slovnet на 25.3 новостных статьи/с (~1 KB) даёт **~39.5 ms/KB**; линейная экстраполяция на 2 000 токенов (~12–16 KB) — **~0.5 с**. **Бюджет требует собственного замера, а не заимствования** | <https://github.com/natasha/slovnet> (таблица `ner2`) |
| 00-charter §7.2 | Добавить, что `ru_core_news_sm` (**14.6 MB**) — компактная альтернатива для L2 с лучшим F1 по именам (0.910), чем pymorphy3 (0.794), и с ненулевым ADDRESS | Exemtik/pii-bench-ru-en README |
| 00-charter §7.1 | Профиль Fast (L0+L1) **обязан** публиковать «ФИО и адрес не обнаруживаются» — теперь это подкреплено цифрой **F1 по NAME = 0.000** для regex-only | Exemtik/pii-bench-ru-en README |
| 00-charter §5.1 / модель данных | Добавить в `Report` **машиночитаемую структуру о срабатывании** (виды, количества, политики) по образцу `cf-aig-dlp` | Cloudflare AI Gateway DLP docs |
| 00-charter §10 | Добавить риск: **PII в аргументах/результатах инструментов (tool calling)** — у Bedrock это задокументированная слепая зона; для нас это будущий v0.2 | AWS Bedrock Guardrails docs |
| Новый пункт анти-целей | **Не затаскивать копилефт через optional extra** (пример: `gender-guesser` GPLv3 у pii-guard) — добавить в CI проверку лицензий дерева зависимостей | pii-guard README |

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

## 10. Библиография с URL

### 10.1 Presidio

1. Project transition (переход к community ownership) — <https://presidio.dataprivacystack.org/project_transition/>
2. Документация, поддерживаемые сущности — <https://presidio.dataprivacystack.org/supported_entities/>
3. Raw-источник списка сущностей — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/supported_entities.md>
4. Языки и NLP-движки — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/analyzer/languages.md>
5. spaCy/Stanza NLP engine — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/analyzer/nlp_engines/spacy_stanza.md>
6. Anonymizer: операторы и разрешение перекрытий — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/anonymizer/index.md>
7. Дефолтный NLP-конфиг — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default.yaml>
8. Дефолтный реестр рекогнайзеров — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default_recognizers.yaml>
9. CHANGELOG (случайная соль в hash, ORGANIZATION в игнор-листе, чистка CLI, AEHD) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/CHANGELOG.md>
10. `presidio-analyzer` pyproject (зависимости, версия 2.2.364, MIT) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/pyproject.toml>
11. LICENSE (MIT, Presidio Contributors) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/LICENSE>
12. Репозиторий — <https://github.com/data-privacy-stack/presidio>
13. PyPI `presidio-analyzer` 2.2.364 — <https://pypi.org/pypi/presidio-analyzer/json>
14. Presidio на PyPI (мета-пакет) — <https://pypi.org/project/presidio/>

### 10.2 LLM Guard

15. Репозиторий (архивирован) — <https://github.com/protectai/llm-guard>
16. README с предупреждением об архивации — <https://raw.githubusercontent.com/protectai/llm-guard/main/README.md>
17. Документация Anonymize-сканера (сущности, модели, `use_faker`, бенчмарки p95) — <https://protectai.github.io/llm-guard/input_scanners/anonymize/>
18. PyPI `llm-guard` 0.3.16 — <https://pypi.org/pypi/llm-guard/json>
19. Модели Protect AI на HF (не поддерживаются) — <https://huggingface.co/protectai>
20. Список сущностей Anonymize в коде — <https://github.com/protectai/llm-guard/blob/main/llm_guard/input_scanners/anonymize.py>
21. Кастомные regex-паттерны — <https://github.com/protectai/llm-guard/blob/main/llm_guard/resources/sensisitive_patterns.json>

### 10.3 Portcullis и SurrogateShield

22. Portcullis, AAAI-25 (страница статьи, DOI, авторы, абстракт, «96x») — <https://ojs.aaai.org/index.php/AAAI/article/view/32088>
23. Portcullis PDF — <https://ojs.aaai.org/index.php/AAAI/article/view/32088/34243>
24. Portcullis DOI — <https://doi.org/10.1609/aaai.v39i1.32088>
25. SurrogateShield, arXiv:2606.29567 — <https://arxiv.org/abs/2606.29567>
26. SurrogateShield PDF — <https://arxiv.org/pdf/2606.29567>
27. SurrogateShield HTML — <https://arxiv.org/html/2606.29567v1>
28. SurrogateShield, репозиторий — <https://github.com/sherwinvishesh/SurrogateShield>
29. SurrogateShield README (детальная архитектура, бенчмарки, ограничения) — <https://raw.githubusercontent.com/sherwinvishesh/SurrogateShield/main/README.md>
30. SurrogateShield, PyPI — <https://pypi.org/project/surrogateshield/>
31. SurrogateShield, сайт — <https://sherwinvishesh.github.io/SurrogateShield/>
32. SurrogateShield в EmergentMind (аннотация) — <https://www.emergentmind.com/papers/2606.29567>

### 10.4 Русскоязычные инструменты

33. **pii-guard**, репозиторий — <https://github.com/redmadrobot-rnd/pii-guard>
34. **pii-guard**, README (20 типов, режимы, лицензия, GPLv3-предупреждение) — <https://raw.githubusercontent.com/redmadrobot-rnd/pii-guard/main/README.md>
35. **pii-guard**, архитектура (C4, приоритет правил, расстояния до ключей) — <https://raw.githubusercontent.com/redmadrobot-rnd/pii-guard/main/docs/architecture.md>
36. pii-guard, статья Redmadrobot — <https://redmadrobot.ai/articles/pii-guard>
37. pii-guard, статья на Habr — <https://habr.com/ru/articles/1080440/>
38. pii-guard, PyPI (метаданные с `YOUR_USERNAME`) — <https://pypi.org/pypi/pii-guard/json>
39. **rubert-base-pii-ner**, модель — <https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner>
40. rubert-base-pii-ner, model card (метрики по типам, обучение) — <https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner/raw/main/README.md>
41. **pii_benchmark**, датасет (MIT) — <https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark>
42. pii_train, датасет — <https://huggingface.co/datasets/redmadrobot-rnd/pii_train>
43. **Natasha**, репозиторий — <https://github.com/natasha/natasha>
44. Natasha README — <https://raw.githubusercontent.com/natasha/natasha/master/README.md>
45. **Slovnet**, README (размеры моделей, таблицы качества, диск/RAM/скорость) — <https://raw.githubusercontent.com/natasha/slovnet/master/README.md>
46. Natasha на PyPI 1.6.0 (MIT) — <https://pypi.org/pypi/natasha/json>
47. Navec (эмбеддинги) — <https://github.com/natasha/navec>
48. Yargy (rule-based извлечение, аналог Tomita) — <https://github.com/natasha/yargy>
49. Corus (загрузка датасетов factRuEval, ne5, bsnlp, gareev) — <https://github.com/natasha/corus>
50. **DeepPavlov**, репозиторий — <https://github.com/deeppavlov/DeepPavlov>
51. DeepPavlov README (PyTorch, 4–16 GB RAM, Apache-2.0) — <https://raw.githubusercontent.com/deeppavlov/DeepPavlov/master/README.md>
52. **Exemtik/pii-bench-ru-en**, репозиторий — <https://github.com/Exemtik/pii-bench-ru-en>
53. pii-bench-ru-en README (цифры по русскому, 13 типов, ограничения) — <https://raw.githubusercontent.com/Exemtik/pii-bench-ru-en/main/README.md>
54. Датасет `hivetrace/pii-bench` — <https://huggingface.co/datasets/hivetrace/pii-bench>
55. **Yandex Tomita** (MPL-2.0, мёртв с 2020) — <https://github.com/yandex/tomita-parser>
56. Tomita LICENSE — <https://raw.githubusercontent.com/yandex/tomita-parser/master/LICENSE>
57. **spaCy модели для русского** — <https://spacy.io/models/ru>
58. spaCy `ru` lang-модуль (лемматизатор pymorphy3) — <https://raw.githubusercontent.com/explosion/spaCy/master/spacy/lang/ru/__init__.py>
59. Метаданные `ru_core_news_sm` (MIT, LOC/ORG/PER) — <https://raw.githubusercontent.com/explosion/spacy-models/master/meta/ru_core_news_sm-3.8.0.json>
60. Релизы spaCy-моделей (размеры wheel) — <https://github.com/explosion/spacy-models/releases>
61. NEREL, реплики на HF — <https://huggingface.co/datasets/iluvvatar/NEREL>
62. factRuEval-2016 на HF — <https://huggingface.co/datasets/gusevski/factrueval2016>

### 10.5 Остальные open-source

63. **scrubadub** — <https://github.com/LeapBeyond/scrubadub>
64. scrubadub PyPI (MIT, зависимости) — <https://pypi.org/pypi/scrubadub/json>
65. scrubadub_spacy — <https://github.com/LeapBeyond/scrubadub_spacy>
66. **DataFog** — <https://github.com/DataFog/datafog-python>
67. **OpenAnonymiser / Anonymiq Light** — <https://github.com/ConductionNL/anonymiq-light>
68. OpenAnonymiser README (flavors, `nl_core_news_lg`, ~3 GB GPU) — <https://raw.githubusercontent.com/ConductionNL/anonymiq-light/main/README.md>
69. Прод-эндпоинт OpenAnonymiser — <https://api.openanonymiser.commonground.nu/api/v1/docs>
70. **pydeidentify** — <https://github.com/Lucasc-99/pydeidentify>
71. **nedap/deidentify** (медзаписи) — <https://github.com/nedap/deidentify>
72. **Phileas** (Java, Apache-2.0) — <https://github.com/philterd/phileas>
73. **ARX** (k-anonymity, Apache-2.0) — <https://github.com/arx-deidentifier/arx>
74. **neosync** — <https://github.com/nucleuscloud/neosync>
75. **PrivateGPT** (другая задача) — <https://github.com/zylon-ai/private-gpt>
76. **mimesis** (MIT, локали) — <https://github.com/lk-geimfari/mimesis>
77. **Faker** (MIT, `ru_RU`) — <https://github.com/joke2k/faker>
78. npm `deidentify` — <https://registry.npmjs.org/deidentify>

### 10.6 LLM-прокси и privacy firewall

79. **DontFeedTheAI** (665 ★) — <https://github.com/zeroc00I/DontFeedTheAI>
80. **piighost** — <https://github.com/Athroniaeth/piighost>
81. **arabic-pii-py** (Apache-2.0, арабский/Залив) — <https://github.com/Aajil-Labs/arabic-pii-py>
82. **mcp-server-conceal** — <https://github.com/gbrigandi/mcp-server-conceal>
83. **veilstream** — <https://github.com/nikshepsvn/veilstream>
84. **llm-dlp** — <https://github.com/yurivenancio30/llm-dlp>
85. **pii-proxy** (Apache-2.0, GDPR) — <https://github.com/whitestag-ai/pii-proxy>
86. **local-privacy-gateway** — <https://github.com/j911si/local-privacy-gateway>
87. **promptzero** — <https://github.com/openbashok/promptzero>
88. **DAM** — <https://github.com/RPBLC-hq/DAM>
89. **sas-confiance-ia** (EUPL-1.2) — <https://github.com/comptoir-des-signaux/sas-confiance-ia>
90. **llmguard** (KZ) — <https://github.com/geekpartnerslab-kz/llmguard>

### 10.7 СНГ

91. **llm-pii-proxy-showcase** (Казахстан, MIT) — <https://github.com/MihaRS2/llm-pii-proxy-showcase>
92. llm-pii-proxy-showcase README (KZ-рекогнайзеры, солёные плейсхолдеры, SSE, fail-closed) — <https://raw.githubusercontent.com/MihaRS2/llm-pii-proxy-showcase/main/README.md>
93. **kazpii-bench** (бенчмарк KZ/RU) — <https://github.com/k1ddy/kazpii-bench>
94. **yashirgich** (узбекский/русский) — <https://github.com/bekzod/yashirgich>
95. **pii-obfuscator** (ru, без лицензии) — <https://github.com/SergeyK88i/pii-obfuscator>
96. **ru-pii-detection-system** (ru, MIT) — <https://github.com/feuerstrahll/ru-pii-detection-system>

### 10.8 Модели PII на HuggingFace (проверенные метаданные)

97. `redmadrobot-rnd/rubert-base-pii-ner` (Apache-2.0, ru, 1 764 загрузок) — <https://huggingface.co/redmadrobot-rnd/rubert-base-pii-ner>
98. `alrosait/spacy_ru_core_news_lg_pii` (MIT, ru, 0 загрузок) — <https://huggingface.co/alrosait/spacy_ru_core_news_lg_pii>
99. `dzanozin/traciora-opf-ru-pii-v6` (Apache-2.0, 1,4 млрд параметров) — <https://huggingface.co/dzanozin/traciora-opf-ru-pii-v6>
100. `HikmaAI/hikmaai-mdeberta-v3-base-pii-multilingual` (Apache-2.0, 11 языков вкл. ru) — <https://huggingface.co/HikmaAI/hikmaai-mdeberta-v3-base-pii-multilingual>
101. `Wismut/nym-pii-multilingual` (MIT, mmBERT, 22 языка вкл. ru/uk) — <https://huggingface.co/Wismut/nym-pii-multilingual>
102. `tabularisai/eu-pii-safeguard` (Apache-2.0, xlm-roberta-large, 26 языков вкл. ru) — <https://huggingface.co/tabularisai/eu-pii-safeguard>
103. `LiquidAI/LFM2.5-Encoder-350M-PII-Detector` (license «other», 16 языков вкл. ru) — <https://huggingface.co/LiquidAI/LFM2.5-Encoder-350M-PII-Detector>
104. `Gherman/bert-base-NER-Russian` — <https://huggingface.co/Gherman/bert-base-NER-Russian>
105. `yqelz/xml-roberta-large-ner-russian` — <https://huggingface.co/yqelz/xml-roberta-large-ner-russian>
106. `onnx-community/bert-base-NER-Russian-ONNX` — <https://huggingface.co/onnx-community/bert-base-NER-Russian-ONNX>
107. `nvidia/Nemotron-PII` (датасет, использован SurrogateShield как внешний) — <https://huggingface.co/datasets/nvidia/Nemotron-PII>
108. `redmadrobot-rnd/pii_benchmark` (MIT) — <https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark>
109. `Powpowpow23/ru-pii-ner-data` — <https://huggingface.co/datasets/Powpowpow23/ru-pii-ner-data>

### 10.9 Проприетарные

110. Purview, список всех SIT-определений (вкл. 4 русских) — <https://learn.microsoft.com/en-us/purview/sit-sensitive-information-type-entity-definitions>
111. Purview, Russian taxpayer identification number — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-taxpayer-identification-number>
112. Purview, Russian Passport Number (Domestic) — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-domestic>
113. Purview, Russian Passport Number (International) — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-international>
114. Purview, Russia physical addresses — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-physical-addresses>
115. Google Sensitive Data Protection, де-идентификация (infoTypes, кастомные детекторы) — <https://docs.cloud.google.com/sensitive-data-protection/docs/concepts-de-identification>
116. Google, reference по трансформациям — <https://docs.cloud.google.com/sensitive-data-protection/docs/transformations-reference>
117. Google, **сэмпл: FPE-суррогат для свободного текста** — <https://docs.cloud.google.com/sensitive-data-protection/docs/samples/dlp-deidentify-free-text-with-fpe-using-surrogate>
118. Google, re-identify free text with FPE using a surrogate — <https://docs.cloud.google.com/sensitive-data-protection/docs/samples/dlp-reidentify-free-text-with-fpe-using-surrogate>
119. Google, список встроенных infoType-детекторов — <https://docs.cloud.google.com/sensitive-data-protection/docs/listing-infotypes>
120. Google, InfoType detector reference — <https://docs.cloud.google.com/sensitive-data-protection/docs/infotypes-reference>
121. Google, псевдонимизация — <https://docs.cloud.google.com/sensitive-data-protection/docs/pseudonymization>
122. AWS Comprehend, поддерживаемые языки (PII: English and Spanish) — <https://docs.aws.amazon.com/comprehend/latest/dg/supported-languages.html>
123. AWS Bedrock, sensitive information filters (32 PII, Block/Mask, слепые зоны, трассировка) — <https://docs.aws.amazon.com/bedrock/latest/userguide/guardrails-sensitive-filters.html>
124. Cloudflare AI Gateway, настройка DLP (профили, FLAG/BLOCK, `cf-aig-dlp`) — <https://developers.cloudflare.com/ai-gateway/features/dlp/set-up-dlp/>
125. Cloudflare Zero Trust DLP профили — <https://developers.cloudflare.com/cloudflare-one/data-loss-prevention/dlp-profiles/>
126. LiteLLM, PII/PHI Masking — Presidio (языки, режимы, чанкинг) — <https://docs.litellm.ai/docs/proxy/guardrails/pii_masking_v2>
127. LiteLLM, MIT лицензия — <https://github.com/BerriAI/litellm/blob/main/LICENSE>

### 10.10 Внутренние документы проекта

128. `docs/00-charter.md` — рабочий документ (инварианты I1–I10, профили задержки, модель данных)
129. `docs/20-entity-registry.md` — реестр типов сущностей и контрольные суммы

---

## 11. Что проверено / по документации / не проверено

### 11.1 ПРОВЕРЕНО (открыл источник)

**Presidio:**
- [x] Репозиторий переехал в `data-privacy-stack/presidio`; старый URL Microsoft редиректит `[GitHub API]`
- [x] Лицензия MIT, копирайт «Presidio Contributors» `[raw LICENSE]`
- [x] Полный список сущностей по регионам (~130 типов) `[raw docs/supported_entities.md]`
- [x] `supported_languages: [en]`; ни одного ru-рекогнайзера `[raw default_recognizers.yaml]`
- [x] Дефолтная NLP-модель `en_core_web_lg`; `ORGANIZATION` в `labels_to_ignore`; `low_confidence_score_multiplier: 0.4` `[raw conf/default.yaml]`
- [x] 8 операторов анонимайзера + `decrypt`; `surrogate_ahds` требует Azure `[raw docs/anonymizer/index.md]`
- [x] Зависимости: `spacy` обязательна; extras `server`/`transformers`/`stanza`/`azure-ai-language`/`ahds`/`gliner`/`langextract` `[raw pyproject.toml]`
- [x] Версия 2.2.364, релиз 2026-07-22, релизы 2.2.363/362/361/360 с датами `[GitHub API releases]`
- [x] 11 345 ★, 1 349 форков, 188 open issues, **206 контрибьюторов**, push 2026-10-09, не архивирован `[GitHub API]`
- [x] Документация прямо требует модификации NlpEngine и рекогнайзеров для нового языка `[raw docs/analyzer/languages.md]`
- [x] `presidio-cli` не публикуется в релизном пайплайне `[CHANGELOG 2.2.356]`
- [x] Hash-оператор с 2.2.361 использует случайную соль по умолчанию `[CHANGELOG]`
- [x] Немецкий набор из 9–13 рекогнайзеров `[docs/supported_entities.md + default_recognizers.yaml]`
- [x] `presidio-analyzer` на PyPI: 2.2.364, загружен 2026-07-22, полный `requires_dist` `[PyPI JSON]`

**LLM Guard:**
- [x] **Архивирован** `[GitHub API archived=true]` + предупреждение в README `[raw README]`
- [x] Модели на HF Protect AI не поддерживаются `[README]`
- [x] MIT `[GitHub API + PyPI license]`
- [x] 28 зависимостей `[PyPI requires_dist]`; версия 0.3.16 от 2025-05-19
- [x] 15 Prompt + 21 Output = 36 сканеров, полные списки `[README]`
- [x] Anonymize наследует Presidio Analyzer `[документация Anonymize]`
- [x] 8 параметров сканера, включая `use_faker` `[документация Anonymize]`
- [x] «Support of multiple languages: English and Chinese» + «Current entity detection functionality is English-specific» `[документация Anonymize]`
- [x] 11 групп PII-сущностей, ни одной русской `[документация Anonymize]`
- [x] **p95 169.13 / 294.57 / 419.60 / 86.47 / 276.45 / 247.22 ms** для 6 конфигураций на входе 317 символов `[документация Anonymize, Benchmarks]`
- [x] 3 216 ★, 466 форков, 38 open issues, ~41 контрибьютор, push 2026-07-08 `[GitHub API]`
- [x] Список рекомендованных NER-моделей (dslim/bert-base-NER и др.), русских нет `[документация]`

**Portcullis:**
- [x] Полная библиография: 6 авторов, Tsinghua + ByteDance, AAAI-25 Vol. 39 No. 1, стр. 1022–1030, опубликовано 2025-04-11, DOI 10.1609/aaai.v39i1.32088 `[OJS]`
- [x] **Точный текст про 96×**: «For masking and unmasking workloads, Portcullis outperforms Hide-and-Seek by 96x speed up»; про Enron: «surpassing Hide-and-Seek by over 0.1 for GPT-4o mini» `[абстракт OJS]`
- [x] **Кода нет**: GitHub Search `portcullis LLM privacy` = 0, `portcullis bytedance` = 0; arXiv API пуст; на странице статьи только PDF и Video/Poster

**SurrogateShield:**
- [x] arXiv 2606.29567, подан 2026-06-28, единственный автор, CC BY 4.0, «14 pages, 1 figure, 9 tables» `[arXiv abs]`
- [x] Репозиторий Apache-2.0, 2 ★, 0 форков, 0 issues, создан 2026-05-13, push 2026-10-16 `[GitHub API]`
- [x] Трёхстадийный каскад, 22 типа PII, AES-256-GCM ShadowMap, F1 98.87%, BERTScore +13.26 pp `[абстракт arXiv]`
- [x] Детальная архитектура: SentinelLayer, MimicGen, ShadowMap, ResolvePass `[raw README]`
- [x] Все бенчмарк-цифры (утечки, Nemotron-PII, латентность, память, многоходовость, attacker) `[raw README]`
- [x] Раздел «Limitations» с самокритикой (29 утечек на 96 значений естественного текста, «plain Presidio is about 5× faster and lighter») `[raw README]`
- [x] Security Design: device.key 0600, HKDF, AAD, «never silently replaced by an empty map» `[raw README]`
- [x] `en_core_web_lg`, `ssn`, `zip_us`, `phone_us`, `us_bank_number`, `postal_code_uk` — US-специфика `[raw README]`
- [x] Зависимости включают `faker>=24.0.0`, `presidio-analyzer>=2.2.0` (опционально) `[raw README]`

**Русскоязычные:**
- [x] **pii-guard**: 20 типов с полным перечнем; 3 режима; Apache-2.0; веса 678 MiB; 178M параметров; GPLv3-предупреждение про `gender-guesser` `[raw README]`
- [x] **pii-guard** архитектура: 16 модулей правил, Presidio AnalyzerEngine + AnonymizerEngine, PyTorch+transformers, окно 512/stride 128, приоритет правил над моделью, расстояния до ключей (BIK 40, паспорт 80/160, p99=83), «наружу отдаётся значение, а не фраза», восстановление падежа pymorphy3+spaCy, смещения от `normalized_text` `[raw docs/architecture.md]`
- [x] **rubert-base-pii-ner**: Apache-2.0, ru, apache-2.0, 21 тип / 43 BIO-метки, 178M параметров, F1 83.6 (модель) / 88.9 (пайплайн), полная таблица per-entity F1 (INN 68.3, IP 36.0), параметры обучения, окно 512/stride 128 `[HF API + raw model card]`
- [x] **pii_benchmark**: MIT, 2 841 предложение, 1K–10K, 597 загрузок, 23 ❤, disjoint от train `[HF]`
- [x] **Natasha**: 1 359 ★, MIT, тег v1.6.0, PyPI 2023-07-24, 13 релизов, push 2026-10-14 `[GitHub API + PyPI]`
- [x] **Slovnet**: NER 2 MB, morph 2 MB, syntax 3 MB; диск 27 MB, RAM 205/115/125 MB; 25.3 статьи/с; полные таблицы F1 по 4 датасетам и таблицы init/disk/ram/speed `[raw README]`
- [x] **Exemtik/pii-bench-ru-en**: MIT, 13 русских типов, 1 810 документов/1 667 спанов; F1 regex_only 0.800/NAME 0.000, regex+morph 0.846/0.794, regex+spacy_ru 0.897/0.910, regex+morph+spacy 0.873/0.785; EN-цифры; ограничения (ИНН↔паспорт) `[raw README]`
- [x] **DeepPavlov**: переехал в `deeppavlov/`, 6 997 ★, Apache-2.0, push 2025-08-06, PyTorch, «4 to 16 GB RAM» `[GitHub API + raw README]`
- [x] **Yandex Tomita**: MPL-2.0, 500 ★, push 2020-11-16 `[GitHub API + raw LICENSE]`
- [x] **spaCy ru**: sm 14.6 MB, md 39.9 MB, lg 489.6 MB; `en_core_web_lg` 382.1 MB; метки LOC/ORG/PER; MIT; лемматизатор `pymorphy3` `[GitHub Releases + spacy-models meta + spaCy source]`
- [x] **llm-pii-proxy-showcase**: MIT, KZ_BIN/IIN/IIK с валидацией, pymorphy3, `ru_core_news_md`, blake2b+pepper, отказ старта без pepper, SSE StreamRewriter, lemma-стоп-лист, ORG/LOC ≥0.85/длина≥5, усечение до анонимизации (5.99 M символов), деградация без утечки, аудит без значений, дата только в личном контексте `[raw README]`
- [x] Метаданные 7+ PII-моделей HF (лицензии, языки, параметры, загрузки) `[HF API]`

**Остальное open-source:**
- [x] scrubadub: 433 ★, Apache-2.0 (репо) vs MIT (PyPI), релиз v2.0.1 2023-09-01, push 2023-09-01, 8 зависимостей вкл. `textblob==0.15.3` `[GitHub API + PyPI]`
- [x] DataFog: 78 ★, MIT, v4.9.0 2026-09-29, 85 MB `[GitHub API]`
- [x] anonymiq-light: 0 ★, EUPL-1.2, `nl_core_news_lg`, ~3 GB GPU-образ, `contextual` не реализован `[GitHub API + raw README]`
- [x] `pydeidentify`: 1 ★, нет лицензии, 2023 `[GitHub API]`
- [x] `anonymize-it`: PyPI 404 `[PyPI]`
- [x] npm `deidentify@0.2.0`, MIT `[npm registry]`
- [x] `nedap/deidentify` 149 ★ MIT; `philterd/phileas` 110 ★ Apache-2.0 push 2026-10-14; `arx` 738 ★ Apache-2.0 `[GitHub API]`
- [x] private-gpt: 57 583 ★, Apache-2.0, push 2026-10-15 `[GitHub API]`
- [x] mimesis: 4 848 ★, MIT, v22.2.0 2026-09-23 `[GitHub API]`
- [x] Faker: 19 447 ★, MIT, push 2026-10-09 `[GitHub API]`
- [x] 11 LLM-прокси проектов с лицензиями/звёздами/датами `[GitHub Search]`
- [x] Cloudflare DLP: профили, FLAG/BLOCK, коды 2029/2030, `cf-aig-dlp` `[документация]`
- [x] LiteLLM Presidio guardrail: `MASK`/`BLOCK`, языки `en/es/de`, чанкинг 500 000 байт, `output_parse_pii` только unmask `[документация]`
- [x] LiteLLM лицензия MIT `[футер документации]`

**Проприетарные:**
- [x] Purview: список всех SIT, **4 русских SIT**, XML-определения, `recommendedConfidence` 85/75, `patternsProximity="300"`, шкала 65/75/85, русские ключевые слова (с ошибками: `НДС`, `pасспорт`), паспорт РФ `Checksum: No` `[MS Learn]`
- [x] Comprehend: PII только «English and Spanish»; русского нет в базовом списке языков `[AWS docs]`
- [x] Bedrock Guardrails: 32 PII-типа по 6 группам, Block/Mask=`{NAME}`, слепые зоны tool calling, трассировка содержит исходники, CloudWatch `input` содержит исходный запрос, regex без lookaround `[AWS docs]`
- [x] Google SDP: три вида кастомных детекторов, stored dictionary «up to tens of millions», `minLikelihood`, **сэмплы FPE-суррогата (de/re-identify)** `[GCP docs]`

### 11.2 ТОЛЬКО ПО ДОКУМЕНТАЦИИ / АННОТАЦИИ (не проверял в коде)

- [ ] **Portcullis архитектура в деталях** — только абстракт; PDF и постер не открывал. «Parallel substitution», secure enclave, attestation, конкретные типы сущностей, механизм реконструкции — `[ДОК]`
- [ ] **Portcullis Hide-and-Seek baseline** — я не установил, кто авторы Hide-and-Seek и что именно за нагрузка «masking and unmasking workloads» `[НЕ ПРОВЕРЕНО]`. **Требует проверки, если мы будем цитировать сравнение**
- [ ] **pii-guard: сам код** — читал README и `docs/architecture.md` через raw, но `api.github.com/repos/redmadrobot-rnd/pii-guard` вернул пустой объект (`[НЕ ПРОВЕРЕНО]` звёзды, контрибьюторы, дата создания). **Не проверял:** реализацию контрольных сумм, точные паттерны, как именно работает `conflict_resolver`
- [ ] **pii-guard задержка** — не опубликована, не измерял. Оценка «десятки–сотни мс» — моя, `[НЕ ПРОВЕРЕНО]`
- [ ] **pymorphy3 лицензия** — предполагаю MIT/permissive, но не открывал `[НЕ ПРОВЕРЕНО]`
- [ ] **Faker/mimesis русские локали** — наличие `ru_RU` беру из общеизвестности, конкретные файлы локалей не открывал `[ДОК]`
- [ ] **`dslim/bert-base-NER` размеры** — не открывал модель-карточки `[НЕ ПРОВЕРЕНО]`
- [ ] **Presidio: список дефолтных регионов `PhoneRecognizer`** — включает ли RU, не проверял `[НЕ ПРОВЕРЕНО]`
- [ ] **Presidio `sensisitive_patterns.json`** (LLM Guard) — содержимое не открывал, могут быть не-US паттерны `[НЕ ПРОВЕРЕНО]`
- [ ] **Presidio CHANGELOG (обрезанная часть)** — часть changelog не прочитана (файл большой, вывод обрезан) `[ДОК]`
- [ ] **Azure AI Language PII: русский** — не проверял `[НЕ ПРОВЕРЕНО]`
- [ ] **scrubadub: почему расхождение лицензий** Apache-2.0 (репо) vs MIT (PyPI) `[НЕ ПРОВЕРЕНО]`
- [ ] **Allure of `nedap/deidentify`** — README на `main` вернул 404, структуру не изучал `[НЕ ПРОВЕРЕНО]`
- [ ] **Детали 11 LLM-прокси проектов** — кроме `llm-pii-proxy-showcase`, читал только описания из GitHub Search, README не открывал `[ДОК]`
- [ ] **Google DLP: русские infoTypes** — не подтвердил `[НЕ ПРОВЕРЕНО]`
- [ ] **Google DLP: число встроенных infoType-детекторов** — не получил `[НЕ ПРОВЕРЕНО]`
- [ ] **Точные размеры моделей в МБ для каждого сканера LLM Guard** — не измерял `[НЕ ПРОВЕРЕНО]`
- [ ] **Presidio задержка** — само́й документации нет; цифра 8.2 ms взята из чужого self-reported бенчмарка `[НЕ ПРОВЕРЕНО нашим харнесом]`

### 11.3 НЕ НАЙДЕНО (искал, артефакта нет)

- [x] **Код Portcullis** — GitHub Search: `portcullis LLM privacy` = 0, `portcullis bytedance` = 0; arXiv API пуст; на странице статьи нет ссылки на артефакт
- [x] **`anonymize-it`** — PyPI 404; в GitHub Search только посторонние проекты
- [x] **`Oban` как инструмент обезличивания PII** — не найден в этом качестве; существует Elixir `Oban` (очередь задач)
- [x] **`OpenAnonymiser` как отдельный проект** — GitHub Search `openanonymiser OR openanonymizer` = 0; существует только `ConductionNL/anonymiq-light` («OpenAnonymiser Light»)
- [x] **Русские рекогнайзеры в Presidio** — ни одного
- [x] **Русские PII-типы в AWS Bedrock Guardrails** — 0 из 32
- [x] **Русский в AWS Comprehend** — отсутствует в списке языков Comprehend вообще
- [x] **Открытые обезличиватели для Беларуси (УНП), Узбекистана, Украины** — не найдено; есть только валидаторы внутри мультиязычных пакетов
- [x] **Открытый PII-детектор от Sberbank** — не найден (есть base-модели `ai-forever/ruBert-base`, используемые другими)
- [x] **Studio Ousia в контексте PII/анонимизации русского** — не найдено
- [x] **RuCoCo на HuggingFace** — поиск `rucoco` пуст; наш §2.6 charter ссылается на arXiv-статью, но датасета в HF под этим именем нет → **риск для ворот ContextGuard**
- [x] **Датасет с разметкой кореференции для русского в открытом доступе** — не подтверждён (см. выше)
- [x] **Замеры задержки Presidio от самих авторов** — не публикуются
- [x] **Русские контекстные слова в Presidio** — ни одного (только en/es/it/pl/de/ko/sv/th/tr/fi)
- [x] **Русские локали `surrogate_ahds`** — только `en-US` в дефолтах

### 11.4 ТРЕБУЕТ УТОЧНЕНИЯ ПОСТАНОВЩИКОМ

- **«SLIM»** — не установил, о каком проекте речь. Возможные кандидаты: SPARQL-интерфейсы,
  `slim-ai` entity linking, методы дистилляции SLIM. В контексте PII/русского ничего не нашёл.
- **«SurrogateShield»** — если у постановщика было в виду что-то другое, кроме
  `arXiv:2606.29567` + `github.com/sherwinvishesh/SurrogateShield`, это надо сказать: других
  артефактов под этим именем поиск не дал (GitHub Search `SurrogateShield` → 1 результат).
- **«Yandex Tomita/ner»** — Tomita найден и разобран; **отдельного** «Yandex NER» как публичного
  артефакта не нашёл. Если имелся в виду Yandex Cloud NLP API — это проприетарный облачный
  сервис, я его в аудит не включал (не было в списке проприетарных ориентиров).
- **«Studio Ousia»** — в списке был, но связи с PII/русским не нашёл. Уточнить, что имелось в виду.

---

## Приложение A. Пять самых важных находок этого аудита

1. **LLM Guard архивирован, и вместе с ним модели на HuggingFace.** Это снимает его с любой
   повестки как зависимость и обнуляет значительную часть «конкурентного поля» в open-source.
   `[ПРОВЕРЕНО]`

2. **pii-guard (Apache-2.0, русский, 20 типов) существует и делает почти всё, кроме того, что
   делаем мы.** Их `docs/architecture.md` — это готовый учебник по русской детекции: приоритет
   правил над моделью, расстояние до ближайшего ключа, «наружу отдаётся значение, а не фраза»,
   восстановление падежа, транслит-нормализация. Мы обязаны взять эти четыре решения и обязаны
   отличаться в двух: **суррогаты** (у них псевдонимы с видимым типом) и **состояние сессии**
   (у них его нет вообще). `[ПРОВЕРЕНО]`

3. **SurrogateShield — наш главный ориентир по идее и главный конкурент по духу.** Тот же
   набор решений (суррогаты, ShadowMap, fail-closed, whole-value restore, k-anonymity), но
   English-only, self-reported, 2 звезды, 1 348 MB RAM, 43.5 ms медиана. Это доказывает, что
   ниша реальна, и что наша версия для русского, на CPU, в одном бинаре — это не то же самое.
   `[ПРОВЕРЕНО]`

4. **Единственные опубликованные p95 для Presidio-подобного стека — у LLM Guard: 169.13 ms
   (ONNX) / 294.57 ms на 317 символах.** Наш бюджет 25–50 ms на 2 000 токенов. Это цифра
   конкурента, которую мы можем цитировать как объективное доказательство, что Python+spaCy
   в наш контракт не влезает. `[ПРОВЕРЕНО]`

5. **У проприетарных вендоров сломаны базовые вещи, которые мы объявили инвариантами.**
   AWS **by design** пишет исходный запрос в CloudWatch и исходное значение в trace guardrail.
   Cloudflare **вообще не умеет** обезличивать — только FLAG/BLOCK. Purview **не может** дать
   выше medium для паспорта РФ и **ошибается** в русских ключевых словах (`НДС` вместо `ИНН`).
   Только Google имеет суррогаты — и только через облачный FPE. `[ПРОВЕРЕНО]`

---

## Приложение B. Одна страница для сверки: пересечение «русский × суррогаты × офлайн»

Это главная таблица всего аудита. Клетки — из источников, перечисленных выше.

| Решение | Детектит ru | Генерирует суррогаты | Суррогаты формат-валидны | Офлайн / локально | Состояние сессии | Открытый код | Лицензия |
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

**Ни одна колонка не заполнена «✓» у всех конкурентов одновременно, и ни одна строка, кроме
последней, не имеет пяти «✓». Пересечение «русский × суррогаты × локально × состояние сессии ×
форматная валидность» пусто.** Это и есть формулировка нашего позиционирования, и она
подкреплена 129 источниками выше.
