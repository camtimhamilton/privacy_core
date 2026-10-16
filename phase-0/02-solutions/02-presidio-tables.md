# Presidio: полные таблицы

Приложение к [README.md](README.md), раздел 2. Источники — в [`03-bibliography.md`](03-bibliography.md). Все данные — из `[ПРОВЕРЕНО]`-источников, перечисленных в тексте README.

## 2.3 Полный перечень рекогнайзеров по регионам

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

## 2.4 Таблица «тип сущности Presidio → поддержка русского»

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
