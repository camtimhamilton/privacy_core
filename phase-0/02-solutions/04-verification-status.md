# Статус проверки: проверено / по документации / не проверено / не найдено

Приложение к [README.md](README.md), раздел 11. Метки: `[ПРОВЕРЕНО]` — открыл источник,
`[ДОК]` — только по документации/аннотации, `[НЕ ПРОВЕРЕНО]` — заявлено, но не подтверждено
первоисточником, `[НЕ НАЙДЕНО]` — искал, нет.

## ПРОВЕРЕНО (открыл источник)

- [x] Presidio: репозиторий переехал в `data-privacy-stack/presidio`; старый URL Microsoft редиректит `[GitHub API]`
- [x] Presidio: лицензия MIT, копирайт «Presidio Contributors» `[raw LICENSE]`
- [x] Presidio: полный список сущностей по регионам (~130 типов) `[raw docs/supported_entities.md]`
- [x] Presidio: `supported_languages: [en]`; ни одного ru-рекогнайзера `[raw default_recognizers.yaml]`
- [x] Presidio: дефолтная модель `en_core_web_lg`; `ORGANIZATION` в `labels_to_ignore`; `low_confidence_score_multiplier: 0.4` `[raw conf/default.yaml]`
- [x] Presidio: 8 операторов анонимайзера + `decrypt`; `surrogate_ahds` требует Azure `[raw docs/anonymizer/index.md]`
- [x] Presidio: `spacy` — обязательная зависимость; extras `server`/`transformers`/`stanza`/`azure-ai-language`/`ahds`/`gliner`/`langextract` `[raw pyproject.toml]`
- [x] Presidio: версия 2.2.364, релиз 2026-07-22; 2.2.363/362/361/360 с датами `[GitHub API releases]`
- [x] Presidio: 11 345 ★, 1 349 форков, 188 open issues, **206 контрибьюторов**, push 2026-10-09, не архивирован `[GitHub API]`
- [x] Presidio: документация прямо требует модификации NlpEngine и рекогнайзеров для нового языка `[raw docs/analyzer/languages.md]`
- [x] Presidio: `presidio-cli` не публикуется в релизном пайплайне `[CHANGELOG 2.2.356]`
- [x] Presidio: hash-оператор с 2.2.361 использует случайную соль по умолчанию `[CHANGELOG]`
- [x] Presidio: немецкий набор из 9–13 рекогнайзеров `[supported_entities.md + default_recognizers.yaml]`
- [x] Presidio: `presidio-analyzer` на PyPI 2.2.364, загружен 2026-07-22, полный `requires_dist` `[PyPI JSON]`
- [x] LLM Guard: **архивирован** `[GitHub API archived=true]` + предупреждение в README `[raw README]`
- [x] LLM Guard: модели на HF Protect AI не поддерживаются `[README]`
- [x] LLM Guard: MIT `[GitHub API + PyPI license]`; 28 зависимостей `[PyPI requires_dist]`; версия 0.3.16 от 2025-05-19
- [x] LLM Guard: 15 Prompt + 21 Output = 36 сканеров, полные списки `[README]`
- [x] LLM Guard: Anonymize наследует Presidio Analyzer; 8 параметров сканера, включая `use_faker` `[документация Anonymize]`
- [x] LLM Guard: «Support of multiple languages: English and Chinese» + «Current entity detection functionality is English-specific» `[документация Anonymize]`
- [x] LLM Guard: 11 групп PII-сущностей, ни одной русской `[документация Anonymize]`
- [x] LLM Guard: **p95 169.13 / 294.57 / 419.60 / 86.47 / 276.45 / 247.22 ms** для 6 конфигураций на входе 317 символов `[документация Anonymize, Benchmarks]`
- [x] LLM Guard: 3 216 ★, 466 форков, 38 open issues, ~41 контрибьютор, push 2026-07-08 `[GitHub API]`
- [x] LLM Guard: список рекомендованных NER-моделей (dslim/bert-base-NER и др.), русских нет `[документация]`
- [x] Portcullis: полная библиография — 6 авторов, Tsinghua + ByteDance, AAAI-25 Vol. 39 No. 1, стр. 1022–1030, 2025-04-11, DOI 10.1609/aaai.v39i1.32088 `[OJS]`
- [x] Portcullis: **точный текст про 96×** — «For masking and unmasking workloads … 96x speed up»; Enron — «surpassing Hide-and-Seek by over 0.1 for GPT-4o mini» `[абстракт OJS]`
- [x] Portcullis: **кода нет** — GitHub Search `portcullis LLM privacy` = 0, `portcullis bytedance` = 0; arXiv API пуст; на странице статьи только PDF и Video/Poster
- [x] SurrogateShield: arXiv 2606.29567, подан 2026-06-28, единственный автор, CC BY 4.0, «14 pages, 1 figure, 9 tables» `[arXiv abs]`
- [x] SurrogateShield: репозиторий Apache-2.0, 2 ★, 0 форков, 0 issues, создан 2026-05-13, push 2026-10-16 `[GitHub API]`
- [x] SurrogateShield: трёхстадийный каскад, 22 типа PII, AES-256-GCM ShadowMap, F1 98.87%, BERTScore +13.26 pp `[абстракт arXiv]`
- [x] SurrogateShield: детальная архитектура (SentinelLayer, MimicGen, ShadowMap, ResolvePass) `[raw README]`
- [x] SurrogateShield: все бенчмарк-цифры (утечки, Nemotron-PII, латентность, память, многоходовость, attacker) `[raw README]`
- [x] SurrogateShield: раздел «Limitations» с самокритикой (29 утечек на 96 значений естественного текста, «plain Presidio is about 5× faster and lighter») `[raw README]`
- [x] SurrogateShield: Security Design (device.key 0600, HKDF, AAD, «never silently replaced by an empty map») `[raw README]`
- [x] SurrogateShield: `en_core_web_lg`, `ssn`, `zip_us`, `phone_us`, `us_bank_number`, `postal_code_uk` — US-специфика `[raw README]`
- [x] SurrogateShield: зависимости включают `faker>=24.0.0`, `presidio-analyzer>=2.2.0` (опционально) `[raw README]`
- [x] pii-guard: 20 типов с полным перечнем; 3 режима; Apache-2.0; веса 678 MiB; 178M параметров; GPLv3-предупреждение про `gender-guesser` `[raw README]`
- [x] pii-guard: архитектура — 16 модулей правил, Presidio AnalyzerEngine + AnonymizerEngine, PyTorch+transformers, окно 512/stride 128, приоритет правил над моделью, расстояния до ключей (BIK 40, паспорт 80/160, p99=83), «наружу отдаётся значение, а не фраза», восстановление падежа pymorphy3+spaCy, смещения от `normalized_text` `[raw docs/architecture.md]`
- [x] rubert-base-pii-ner: Apache-2.0, ru, 21 тип / 43 BIO-метки, 178M параметров, F1 83.6 (модель) / 88.9 (пайплайн), полная таблица per-entity (INN 68.3, IP 36.0), параметры обучения, окно 512/stride 128 `[HF API + raw model card]`
- [x] pii_benchmark: MIT, 2 841 предложение, 1K–10K, 597 загрузок, 23 ❤, disjoint от train `[HF]`
- [x] Natasha: 1 359 ★, MIT, тег v1.6.0, PyPI 2023-07-24, 13 релизов, push 2026-10-14 `[GitHub API + PyPI]`
- [x] Slovnet: NER 2 MB, morph 2 MB, syntax 3 MB; диск 27 MB, RAM 205/115/125 MB; 25.3 статьи/с; полные таблицы F1 по 4 датасетам и таблицы init/disk/ram/speed `[raw README]`
- [x] Exemtik/pii-bench-ru-en: MIT, 13 русских типов, 1 810 документов/1 667 спанов; F1 regex_only 0.800/NAME 0.000, regex+morph 0.846/0.794, regex+spacy_ru 0.897/0.910, regex+morph+spacy 0.873/0.785; EN-цифры; ограничения (ИНН↔паспорт) `[raw README]`
- [x] DeepPavlov: переехал в `deeppavlov/`, 6 997 ★, Apache-2.0, push 2025-08-06, PyTorch, «4 to 16 GB RAM» `[GitHub API + raw README]`
- [x] Yandex Tomita: MPL-2.0, 500 ★, push 2020-11-16 `[GitHub API + raw LICENSE]`
- [x] spaCy ru: sm 14.6 MB, md 39.9 MB, lg 489.6 MB; `en_core_web_lg` 382.1 MB; метки LOC/ORG/PER; MIT; лемматизатор `pymorphy3` `[GitHub Releases + spacy-models meta + spaCy source]`
- [x] llm-pii-proxy-showcase: MIT, KZ_BIN/IIN/IIK с валидацией, pymorphy3, `ru_core_news_md`, blake2b+pepper, отказ старта без pepper, SSE StreamRewriter, lemma-стоп-лист, ORG/LOC ≥0.85/длина≥5, усечение до анонимизации (5.99 M символов), деградация без утечки, аудит без значений, дата только в личном контексте `[raw README]`
- [x] Метаданные 7+ PII-моделей HF (лицензии, языки, параметры, загрузки) `[HF API]`
- [x] scrubadub: 433 ★, Apache-2.0 (репо) vs MIT (PyPI), v2.0.1 2023-09-01, push 2023-09-01, 8 зависимостей вкл. `textblob==0.15.3` `[GitHub API + PyPI]`
- [x] DataFog: 78 ★, MIT, v4.9.0 2026-09-29, 85 MB `[GitHub API]`
- [x] anonymiq-light: 0 ★, EUPL-1.2, `nl_core_news_lg`, ~3 GB GPU-образ, `contextual` не реализован `[GitHub API + raw README]`
- [x] `pydeidentify`: 1 ★, нет лицензии, 2023 `[GitHub API]`; `anonymize-it`: PyPI 404 `[PyPI]`; npm `deidentify@0.2.0`, MIT `[npm registry]`
- [x] `nedap/deidentify` 149 ★ MIT; `philterd/phileas` 110 ★ Apache-2.0 push 2026-10-14; `arx` 738 ★ Apache-2.0 `[GitHub API]`
- [x] private-gpt: 57 583 ★, Apache-2.0, push 2026-10-15 `[GitHub API]`
- [x] mimesis: 4 848 ★, MIT, v22.2.0 2026-09-23; Faker: 19 447 ★, MIT, push 2026-10-09 `[GitHub API]`
- [x] 11 LLM-прокси проектов с лицензиями/звёздами/датами `[GitHub Search]`
- [x] Cloudflare DLP: профили, FLAG/BLOCK, коды 2029/2030, `cf-aig-dlp` `[документация]`
- [x] LiteLLM Presidio guardrail: `MASK`/`BLOCK`, языки `en/es/de`, чанкинг 500 000 байт, `output_parse_pii` только unmask; лицензия MIT `[документация]`
- [x] Purview: список всех SIT, **4 русских SIT**, XML-определения, `recommendedConfidence` 85/75, `patternsProximity="300"`, шкала 65/75/85, русские ключевые слова (с ошибками: `НДС`, `pасспорт`), паспорт РФ `Checksum: No` `[MS Learn]`
- [x] Comprehend: PII только «English and Spanish»; русского нет в базовом списке языков `[AWS docs]`
- [x] Bedrock Guardrails: 32 PII-типа по 6 группам, Block/Mask=`{NAME}`, слепые зоны tool calling, трассировка содержит исходники, CloudWatch `input` содержит исходный запрос, regex без lookaround `[AWS docs]`
- [x] Google SDP: три вида кастомных детекторов, stored dictionary «up to tens of millions», `minLikelihood`, **сэмплы FPE-суррогата (de/re-identify)** `[GCP docs]`

## ТОЛЬКО ПО ДОКУМЕНТАЦИИ / АННОТАЦИИ (не проверял в коде)

- [ ] **Архитектура Portcullis в деталях** — только абстракт; PDF и постер не открывал: «parallel substitution», secure enclave, attestation, конкретные типы сущностей, механизм реконструкции
- [ ] **Hide-and-Seek baseline у Portcullis** — не установил, кто авторы и что именно за нагрузка «masking and unmasking workloads». Требует проверки, если мы будем цитировать сравнение
- [ ] **pii-guard: сам код** — читал README и `docs/architecture.md` через raw, но `api.github.com/repos/redmadrobot-rnd/pii-guard` вернул пустой объект. Не проверял: реализацию контрольных сумм, точные паттерны, как работает `conflict_resolver`, звёзды/контрибьюторы
- [ ] **Задержка pii-guard** — не опубликована; моя оценка «десятки–сотни мс» не замер
- [ ] **Лицензия `pymorphy3`** — предполагаю MIT/permissive, файл не открывал
- [ ] **Русские локали Faker/mimesis** — наличие `ru_RU` беру из общеизвестности, файлы локалей не открывал
- [ ] **Размеры HF-моделей `dslim/bert-base-NER`** — модель-карточки не открывал
- [ ] **Дефолтные регионы `PhoneRecognizer` в Presidio** — входит ли RU
- [ ] **Содержимое `sensisitive_patterns.json`** (LLM Guard) — могут быть не-US паттерны
- [ ] **Обрезанная часть CHANGELOG Presidio** — файл большой, вывод обрезан
- [ ] **Русский в Azure AI Language PII** — не проверял
- [ ] **Почему расхождение лицензий scrubadub** (Apache-2.0 в репо vs MIT на PyPI)
- [ ] **`nedap/deidentify`** — README на `main` вернул 404, структуру не изучал
- [ ] **Детали 11 LLM-прокси проектов** — кроме showcase, читал только описания из GitHub Search
- [ ] **Русские infoType в Google DLP** — страница `language-support` вернула 404; не подтвердил
- [ ] **Число встроенных infoType-детекторов Google** — не получил
- [ ] **Точные размеры моделей по сканерам LLM Guard** — не измерял
- [ ] **Задержка Presidio** — само́й документации нет; 8.2 ms взято из чужого self-reported бенчмарка

## НЕ НАЙДЕНО (искал, артефакта нет)

- [x] **Код Portcullis** — GitHub Search `portcullis LLM privacy` = 0, `portcullis bytedance` = 0; arXiv API пуст; на странице статьи нет ссылки на артефакт
- [x] **`anonymize-it`** — PyPI 404; в GitHub Search только посторонние проекты
- [x] **`Oban` как инструмент обезличивания** — не найден в этом качестве; существует Elixir `Oban` (очередь задач)
- [x] **`OpenAnonymiser` как отдельный проект** — GitHub Search `openanonymiser OR openanonymizer` = 0; существует только `ConductionNL/anonymiq-light` («OpenAnonymiser Light»)
- [x] **Русские рекогнайзеры в Presidio** — ни одного
- [x] **Русские PII-типы в AWS Bedrock Guardrails** — 0 из 32
- [x] **Русский в AWS Comprehend** — отсутствует в списке языков Comprehend вообще
- [x] **Открытые обезличиватели для Беларуси (УНП), Узбекистана, Украины** — есть только валидаторы внутри мультиязычных пакетов
- [x] **Открытый PII-детектор от Sberbank** — не найден (есть base-модели `ai-forever/ruBert-base`, используемые другими)
- [x] **Studio Ousia в контексте PII/анонимизации русского** — не найдено
- [x] **RuCoCo на HuggingFace** — поиск `rucoco` пуст; наш 00-charter §2.6 ссылается на arXiv-статью, но датасета в HF под этим именем нет → **риск для ворот ContextGuard**
- [x] **Открытый датасет кореференции для русского** — не подтверждён
- [x] **Замеры задержки Presidio от авторов** — не публикуются
- [x] **Русские контекстные слова в Presidio** — ни одного
- [x] **Русские локали `surrogate_ahds`** — только `en-US` в дефолтах

## ТРЕБУЕТ УТОЧНЕНИЯ ПОСТАНОВЩИКОМ

- **«SLIM»** — не установил, о каком проекте речь. Возможные кандидаты: SPARQL-интерфейсы,
  `slim-ai` entity linking, методы дистилляции. В контексте PII/русского ничего не нашёл.
- **«SurrogateShield»** — если имелось в виду что-то другое, кроме `arXiv:2606.29567` +
  `github.com/sherwinvishesh/SurrogateShield`: других артефактов под этим именем поиск не дал
  (GitHub Search `SurrogateShield` → 1 результат).
- **«Yandex Tomita/ner»** — Tomita найден и разобран; **отдельного** публичного «Yandex NER» нет.
  Если имелся в виду Yandex Cloud NLP API — это проприетарный облачный сервис, в список
  проприетарных ориентиров он не входил.
- **«Studio Ousia»** — в списке был, но связи с PII/русским не нашёл.
