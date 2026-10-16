# Аудит репозиториев, датасетов и бенчмарков

**Поток:** 4 — репозитории, бенчмарки, датасеты, генераторы синтетики
**Дата аудита:** 2026-10-16
**Автор:** исследовательский подагент (поток «репозитории/датасеты/рынок»)
**Связанные документы:** [`../01-audit.md`](../01-audit.md), [`../04-market/README.md`](../04-market/README.md), [`../02-solutions/`](../02-solutions/), [`../appendix/benchmark-protocol.md`](../appendix/benchmark-protocol.md)
**Артефакты и воспроизводимость:** [`scripts/verification-log.md`](scripts/verification-log.md) — журнал всех проверок, команды, что не удалось проверить и почему; [`scripts/`](scripts/) — скрипты проверки, сырые данные (276 репозиториев) и генератор валидных реквизитов.

**Метод.** Звёзды, лицензия, признак «архивный» и описание сняты с HTML-страницы `github.com/<owner>/<repo>` (в HTML встроен JSON с полем `"stargazerCount"`, `"license":{"spdxId":…}`, `"isArchived"`). Дата последнего коммита — из фида `https://github.com/<owner>/<repo>/commits.atom`. Топ-язык — из `img.shields.io/github/languages/top/…json`. Существование репозитория подтверждается кодом ответа HTTP 200 на `https://github.com/<owner>/<repo>`; переименованные репозитории GitHub редиректит, и каноническое имя зафиксировано в колонке «канонический URL». Датасеты проверены через `https://huggingface.co/api/datasets/<id>` (HTTP 200 и наличие поля `id`), размеры — через `https://datasets-server.huggingface.co/size`.

**Важно про дату.** Аудит проведён **2026-10-16**. Порог «мёртвый проект» — нет коммитов более 12 месяцев, то есть последний коммит раньше **2026-10-16**. Все выводы о смерти проектов привязаны к этой дате.

**Ограничение среды.** REST API GitHub (`api.github.com`) имеет лимит 60 запросов в час для анонимного доступа и был исчерпан в ходе аудита; поэтому массовые проверки выполнены через HTML и atom-фиды (лимита не имеют). Установка Python-пакетов в песочнице запрещена (PEP 668 + `Permission denied` на `~/.local`), поэтому выводы о поведении `Faker` сделаны **по исходному коду**, а не запуском — это помечено в разделе 6.

---

## 1. GitHub-репозитории

Раздел разбит на четыре таблицы: инфраструктура PII/детекции, шлюзы приватности перед LLM, русскоязычные проекты, мёртвые и архивные. Колонка «Что берём» — решение по каждому репозиторию с точки зрения `privacy_core`.

### 1.1 Инфраструктура детекции и обезличивания PII

| Имя | URL (проверен, HTTP 200) | Звёзды | Последний коммит | Лицензия | Стек | Русский | Что берём |
|---|---|---:|---|---|---|---|---|
| data-privacy-stack/presidio | https://github.com/data-privacy-stack/presidio | 11 345 | 2026-10-08 | MIT | Python 99.5% | через `ru_core_news_*` (слабо) | Берём как эталон архитектуры «regex + NER + консолидация». **Проект переехал**: `microsoft/presidio` теперь редиректит на `data-privacy-stack/presidio` (подтверждено: `curl -I` → `https://github.com/data-privacy-stack/presidio`, 200). Смена владельца — риск для планирования. |
| data-privacy-stack/presidio-research | https://github.com/data-privacy-stack/presidio-research | 317 | 2026-10-14 | MIT | Jupyter 66.6% | — | Берём **харнесс оценки** (`PresidioEvaluator`): он считает precision/recall по спанам и умеет сравнивать конфигурации распознавателей. Это готовый каркас для нашего бенчмарка. Тоже переехал из `microsoft/`. |
| openai/privacy-filter | https://github.com/openai/privacy-filter | 2 745 | 2026-04-22 | Apache-2.0 | Python 100% | **нет** | Не берём код, но берём **идею таксономии из 8 категорий** и «preset operating points» (переключатель precision/recall). Модель: 1.5 млрд параметров всего / 50 млн активных, контекст 128k. Только английский. |
| localai-org/privacy-filter.cpp | https://github.com/localai-org/privacy-filter.cpp | 305 | 2026-07-01 | MIT | C++ | нет | **Важный замер для нас.** Реализация модели OpenAI Privacy Filter на GGML. Опубликована таблица латентности на CPU (Ryzen 9 7900, 12 потоков), и она **опровергает** пригодность нейросетевого детектора под наш бюджет: 512 токенов — 0.14 с (140 мс), 2 048 токенов — 0.59 с, 8 192 токена — 3.51 с. Это в 3–6 раз выше нашей цели p95 25–50 мс уже на коротком промпте. На Raspberry Pi 5 — 360 ток/с (1 360 токенов за 3.8 с). |
| urchade/GLiNER | https://github.com/urchade/GLiNER | 4 106 | 2026-10-05 | Apache-2.0 | Python 99.5% | мультиязычно | Берём как **внешний baseline** в бенчмарке (zero-shot NER по текстовому описанию метки). Именно GLiNER лежит в основе лидеров PII Arena и в основе детектора AGIMA. |
| Knowledgator/GLiNER | https://github.com/Knowledgator/GLiNER | 8 | 2026-04-26 | Apache-2.0 | Python 99.3% | мультиязычно | Не берём: почти пустой форк-зеркало (8 звёзд), оригинал — у `urchade`. |
| HiveTrace/gliner-guard | https://github.com/HiveTrace/gliner-guard | 0 | 2026-07-13 | Apache-2.0 | — | есть | Смотрим: код и харнесс к датасету `hivetrace/pii-bench` и семейству GLiNER Guard. Ноль звёзд, но это **вендорский бенчмарк с открытым кодом** — полезен как источник протокола. |
| guardrails-ai/guardrails | https://github.com/guardrails-ai/guardrails | 7 527 | 2026-08-26 | Apache-2.0 | Python 99.7% | нет | Не берём целиком (тяжёлый фреймворк валидации), но смотрим модель хуков `Guard`/`Validator` — она удобна для встраивания наших проверок. |
| guardrails-ai/detect_pii | https://github.com/guardrails-ai/detect_pii | 18 | 2026-07-27 | Apache-2.0 | Python | нет | Смотрим как минимальный пример «валидатор PII». Англоцентричен. |
| guardrails-ai/guardrails_pii | https://github.com/guardrails-ai/guardrails_pii | 23 | 2026-07-27 | Apache-2.0 | Python | нет | То же. |
| NVIDIA-NeMo/Guardrails | https://github.com/NVIDIA-NeMo/Guardrails | 7 305 | 2026-10-15 | Other (не OSI) | Python 99.3% | нет | **Проект переименован** (`NVIDIA/NeMo-Guardrails` → `NVIDIA-NeMo/Guardrails`). Лицензия «Other» — не Apache/MIT, юридически требует отдельной проверки. Не берём. |
| protectai/llm-guard | https://github.com/protectai/llm-guard | 3 216 | 2026-07-08 | MIT | Python 99.0% | нет | **РЕПОЗИТОРИЙ АРХИВИРОВАН** (`"isArchived":true`). Ровно тот инструмент, который в плане назван ориентиром. Брать нечего: развитие остановлено, ядро всё равно англоязычное. |
| maziyarpanahi/openmed | https://github.com/maziyarpanahi/openmed | 5 693 | 2026-10-14 | Apache-2.0 | Python | нет | Не берём (медицина, HIPAA, англ.), но смотрим: это пример «локального» детектора PII с большой аудиторией — подтверждает спрос на локальный запуск. |
| Rizzo-AI-Academy/rizzo-pii | https://github.com/Rizzo-AI-Academy/rizzo-pii | 1 183 | 2026-09-25 | MIT | Python | нет (итал. ветка) | Смотрим как конкурента по позиционированию «local-first privacy guard для документов». Русского нет. |
| zeroc00I/DontFeedTheAI | https://github.com/zeroc00I/DontFeedTheAI | 665 | 2026-07-08 | MIT | Python | нет | Не берём (пентест-специфика, IP/креды). |
| redhuntlabs/Octopii | https://github.com/redhuntlabs/Octopii | 747 | 2025-01-22 | Other | Python | нет | Не берём. **21 месяц без коммитов** — фактически заброшен. |
| arx-deidentifier/arx | https://github.com/arx-deidentifier/arx | 738 | 2025-10-01 | Apache-2.0 | Java | нет | Не берём (Java, k-анонимность табличных данных, не текст). Живой, но не наш профиль. |
| GreenmaskIO/greenmask | https://github.com/GreenmaskIO/greenmask | 1 785 | 2026-10-10 | Apache-2.0 | Go | нет | Не берём (анонимизация дампов БД, не промпты). Но идея **детерминированного маппинга «одинаковый вход → одинаковый выход»** у них реализована зрело — полезно для наших подмен. |
| censgate/redact | https://github.com/censgate/redact | 22 | 2026-10-01 | Apache-2.0 | **Rust** | нет | **Смотрим внимательно.** Быстрый Rust-движок детекции и редактирования PII и секретов в промптах, коде и БД. Единственный Rust-проект в выборке, кроме `clawshell`/`gaze`/`muninn`. Проверить их архитектуру матчинга — прямая польза для нашего Rust-ядра. |
| hoophq/alcatraz | https://github.com/hoophq/alcatraz | 38 | 2026-10-14 | MIT | Go | нет | Смотрим: компактный детектор PII с маскированием и кастомными правилами. 38 звёзд, но от компании (hoop.dev). |
| thyrisAI/safe-zone | https://github.com/thyrisAI/safe-zone | 30 | 2026-10-15 | Apache-2.0 | Go | нет | Смотрим: движок детекции PII + guardrails. |
| anonde-io/anonde | https://github.com/anonde-io/anonde | 18 | 2026-10-11 | Apache-2.0 | Go | мультиязычно через GLiNER | Смотрим: **73 паттерн-распознавателя + GLiNER**, обратимая анонимизация на Go. Хороший ориентир по числу паттернов. |
| CertaMesh/gaze | https://github.com/CertaMesh/gaze | 12 | 2026-10-15 | Apache-2.0 | **Rust** | нет | Смотрим: «reversible PII pseudonymization runtime для агентных LLM-процессов». Наш профиль, но 12 звёзд и нет русского. |
| colliery-io/muninn | https://github.com/colliery-io/muninn | 14 | 2026-10-06 | Apache-2.0 | **Rust** | нет | Смотрим: privacy-first рекурсивный контекстный шлюз для кодинг-агентов. |
| gbrigandi/mcp-server-conceal | https://github.com/gbrigandi/mcp-server-conceal | 18 | 2025-07-24 | MIT | **Rust** | нет | Смотрим, но проект **14 месяцев без коммитов** — вероятно заброшен. |
| RPBLC-hq/DAM | https://github.com/RPBLC-hq/DAM | 11 | 2026-06-27 | нет файла лицензии | Rust | нет | Берём **с осторожностью**: лицензии нет, значит по умолчанию «все права защищены» — код переиспользовать нельзя. Учитывать только как идею. |
| awslabs/ferret-scan | https://github.com/awslabs/ferret-scan | 98 | 2026-10-16 | Apache-2.0 | Go | нет | Смотрим: офлайн CLI и Go-библиотека поиска и редактирования чувствительных данных. Свежий, от AWS Labs. |
| thoughtbot/top_secret | https://github.com/thoughtbot/top_secret | 419 | 2026-05-14 | MIT | Ruby | нет | Не берём (Ruby), но концепция «фильтровать свободный текст перед отправкой во внешний API» — ровно наша. |
| gitleaks/gitleaks | https://github.com/gitleaks/gitleaks | 29 985 | 2026-07-22 | MIT | Go | нет | **Берём как источник правил.** Канонический набор правил поиска секретов. `packyme/privacy-filter` скомпилировал все 222 правила gitleaks в Go-RE2 (отмечая, что ранний Python-порт потерял 26 правил из-за несовместимости с RE2). Для нас — готовый датасет правил и тест-кейсов. |
| betterleaks/betterleaks | https://github.com/betterleaks/betterleaks | 2 238 | не установлено | не установлено | Go | нет | Смотрим: альтернативный набор правил поиска утечек, используется в PII Arena как `secrets-rules` вместе с gitleaks. |
| EdyVision/pii-codex | https://github.com/EdyVision/pii-codex | 102 | 2026-02-15 | нет в HTML | Python 64.8% | нет | Смотрим: «severity»-оценка PII, любопытная идея приоритизации. 8 месяцев без коммитов. |
| LeapBeyond/scrubadub | https://github.com/LeapBeyond/scrubadub | 433 | 2023-09-01 | Apache-2.0 | Python | нет | **НЕ БЕРЁМ: мёртв 3 года.** `scrubadub/scrubadub` больше не существует (404) — проект уехал под `LeapBeyond/scrubadub` и там остановился 2023-09-01. Ещё один «стандарт» из плана оказался заброшен. |
| davidsbatista/NER-Evaluation | https://github.com/davidsbatista/NER-Evaluation | 224 | 2024-07-02 | MIT | Python | нет | **Берём утилиту подсчёта метрик** (seqeval-подобная, с отчётом по типам). 2 года без коммитов, но код стабилен и лицензия MIT. |
| NorskRegnesentral/text-anonymization-benchmark | https://github.com/NorskRegnesentral/text-anonymization-benchmark | 81 | 2026-01-19 | MIT | Python 100% | нет | **Берём протокол.** Это TAB (Text Anonymization Benchmark) — эталонный корпус и метрики для текстовой анонимизации. Английский, но метрики переносимы. |
| stefan-it/hmbench | https://github.com/stefan-it/hmbench | 6 | 2023-11-14 | нет | — | нет | Не берём: 3 года без коммитов, 6 звёзд. |

### 1.2 Шлюзы приватности перед LLM (парадигма «маскировать запрос → восстановить ответ»)

| Имя | URL (проверен, HTTP 200) | Звёзды | Последний коммит | Лицензия | Стек | Русский | Что берём |
|---|---|---:|---|---|---|---|---|
| packyme/privacy-filter | https://github.com/packyme/privacy-filter | 338 | 2026-06-09 | MIT | **Go** | нет (плейсхолдеры на китайском) | **Главный ориентир по латентности.** Чистый Go, без модели, без GPU, без CGO, один статический бинарник. Опубликован замер: ~50 Б — **0.01 мс**, ~2 КБ — **0.46 мс**, ~32 КБ — **9 мс**. Работает в проде в PackyCode (packyapi.com) как компонент комплаенса API-релея. **Ключевая цитата из README:** «No person / place / organization name recognition — that needs an NER model, which costs seconds of CPU time on long text and was removed per requirements». То есть **продакшн-шлюз выбросил распознавание имён, потому что NER слишком медленный** — это прямое подтверждение нашей гипотезы о rule-first и одновременно указание на свободную нишу (имена по-русски). |
| dataiku/kiji-proxy | https://github.com/dataiku/kiji-proxy | 437 | 2026-10-15 | Apache-2.0 | Go + Node + Python | нет | **Берём как самый близкий по функции западный аналог.** Прозрачный прокси перед AI API: ML-детекция 26 типов PII, замена на **реалистичные фиктивные значения** (не заглушки!), прозрачное восстановление ответа, ONNX-модель локально, Chrome-расширение, mapping с возможностью просмотра и удаления. От Dataiku (575 Lab). Изучить их модель восстановления и UI-контроль маппинга. Есть публичный обучающий датасет `DataikuNLP/kiji-pii-training-data` — но **без русского** (da/de/en/es/fr/nl). |
| sgasser/pasteguard | https://github.com/sgasser/pasteguard | 765 | 2026-10-13 | Apache-2.0 | TypeScript | нет | Смотрим: «AI gets the context. Not your private data» — локальный прокси для браузера, AI API и кодинг-агентов. Самый звёздный в нише локального прокси (765). |
| xiaYuTian11/maskit | https://github.com/xiaYuTian11/maskit | 310 | 2026-10-14 | **AGPL-3.0** | Python | нет (кит.) | **Смотрим, но лицензия AGPL-3.0 — копилефт.** Для Apache-2.0-проекта переиспользование кода запрещено; изучаем только архитектуру. Реализует «запрос автоматически маскируется, ответ потоково восстанавливается», поддержка Cursor/Claude Code/Codex. Потоковое восстановление — то, что нам понадобится. |
| runta-dev/clawshell | https://github.com/runta-dev/clawshell | 345 | 2026-07-09 | Apache-2.0 | **Rust** | нет | Смотрим: локальный runtime-контроль для OpenClaw/Hermes-agent, защита PII и кредов. Rust-архитектура. |
| CassiopeiaCode/CosyRedactGateway | https://github.com/CassiopeiaCode/CosyRedactGateway | 103 | 2026-10-10 | Apache-2.0 | JS | нет | Смотрим: stateless-редактирование секретов перед OpenAI/Anthropic с восстановлением. Идея «stateless» важна — совпадает с решением redmadrobot (mapping не хранится на сервере). |
| voidmind-io/voidllm | https://github.com/voidmind-io/voidllm | 141 | 2026-09-30 | NOASSERTION | Go | нет | Не берём (лицензия неопределённая + в основном роутинг/балансировка). |
| illfilter-ai/ilter | https://github.com/illfilter-ai/ilter | 14 | 2026-10-03 | NOASSERTION | Go | нет | Не берём (лицензия неопределённая). |
| yatuk/tamga | https://github.com/yatuk/tamga | 44 | 2026-10-12 | AGPL-3.0 | Go | нет | Не берём (AGPL-3.0). Интересна заявка «sub-millisecond» и комплаенс KVKK/GDPR/PCI-DSS — показательно для турецкого рынка. |
| zolderio/token-proxy | https://github.com/zolderio/token-proxy | 31 | 2026-04-13 | Apache-2.0 | Python | нет | Смотрим: «прозрачный PII-редактирующий прокси для трафика LLM API». |
| k13lucien/maskme | https://github.com/k13lucien/maskme | 13 | 2026-07-17 | MIT | Python | нет | Не берём (структурированные данные). |
| DataFog/datafog-python | https://github.com/DataFog/datafog-python | 78 | 2026-10-05 | MIT | Python | нет | Смотрим: «офлайн PII-фаервол для AI-агентов», хук для Claude Code, интеграция с LiteLLM. |
| crp4222/PrivAiTe | https://github.com/crp4222/PrivAiTe | 54 | 2026-10-09 | BSD-3-Clause | Python | нет | Смотрим: self-hosted прокси редактирования PII для LLM API и CLI агентов. |
| postralai/masquerade | https://github.com/postralai/masquerade | 77 | 2025-08-11 | MIT | Python | нет | **Не берём: 14 месяцев без коммитов.** «The Privacy Firewall for LLMs» — красивое позиционирование, но проект встал. |
| takashiishida/cleanprompt | https://github.com/takashiishida/cleanprompt | 26 | 2024-03-24 | MIT | Python | нет | **Не берём: мёртв 2.5 года.** Один из первых в нише «анонимизировать промпт перед LLM» — и он остановился. Показательно: ранние проекты ниши не выжили. |
| amanvirparhar/elara | https://github.com/amanvirparhar/elara | 69 | 2025-01-26 | MIT | Svelte | нет | **Не берём: 21 месяц без коммитов.** «Anonymize LLM prompts». |
| Guardrails/LLM-Guard и др. | см. 1.1 | | | | | | |
| sherwinvishesh/SurrogateShield | https://github.com/sherwinvishesh/SurrogateShield | 2 | 2026-10-16 | Apache-2.0 | Python | **нет** | Уже разобран в [`../01-audit.md`](../01-audit.md). Подтверждаю фактически: репозиторий живой (коммит сегодня), Apache-2.0, 2 звезды. Русского нет, требуется torch. |

### 1.3 Русскоязычные репозитории (прямые конкуренты и смежники)

**Это самая важная таблица аудита.** Здесь собраны проекты, которые делают ровно то же, что планирует `privacy_core`, на русском языке.

| Имя | URL (проверен, HTTP 200) | Звёзды | Последний коммит | Лицензия | Стек | Что делает | Что берём / чем опасен |
|---|---|---:|---|---|---|---|---|
| **redmadrobot-rnd/pii-guard** | https://github.com/redmadrobot-rnd/pii-guard | 12 | 2026-09-09 | **Apache-2.0** | Python (FastAPI + spaCy + ruBERT) | **Прямой конкурент №1.** 20 типов сущностей: PERSON, LOCATION (страна/регион/район/город/улица/дом), PHONE, URL, EMAIL, **INN, SNILS, OMS, PASSPORT, DRIVER_LICENSE, MILITARY_ID, BIRTH_CERTIFICATE, CREDIT_CARD, BANK_ACCOUNT, BIK, POSTAL_CODE, TELEGRAM, IP_ADDRESS, IP_PORT, DATE_TIME**. Три режима: `mask`, `tag`, `pseudonymize` (обратимый). Есть `/anonymize` и `/deanonymize`, **восстановление с восстановлением падежа**: «Свяжитесь с \<PII…\>» → «Свяжитесь с Иваном Петровым» (прямая подстановка дала бы «с Иван Петров»). Работает как библиотека, как HTTP-сервис (порт 8080, `127.0.0.1` по умолчанию) и как Docker. Mapping **не хранится** на сервере — уезжает вызывающему. Правила с контрольными суммами + модель ruBERT только для имён/адресов/телефонов/URL. Можно собрать образ **без весов** (`NER_DISABLED=1`) — тогда остаются только типы с контрольными суммами. | **Подтверждает и отменяет часть нашей дифференциации.** Русский, обратимость, контрольные суммы, падежи — уже сделано, открыто, Apache-2.0, с обучающим датасетом, весами модели и бенчмарком. Наши оставшиеся отличия: (1) **латентность и вес** — у них torch + ruBERT, замеров латентности в README **нет**; (2) контекст/анафора; (3) Rust-ядро. Смотреть обязательно весь код. |
| redmadrobot-rnd/pii_mask (org) | https://github.com/redmadrobot-rnd | — | — | — | — | Организация, выложившая `pii-guard`, датасеты `pii_train`/`pii_benchmark` и модель `rubert-base-pii-ner`. | Смотреть: у них есть статья `redmadrobot.ai/articles/pii-guard` и статья на Habr (habr.com/ru/articles/1080440/). |
| dewil/pii-mask | https://github.com/dewil/pii-mask | 4 | 2026-10-09 | MIT | Python | «Локальный шлюз маскировки персональных данных перед отправкой текста в облачные LLM: mask/unmask, stateless ma…» | Смотрим: свежий (неделю назад), тот же сценарий, MIT. |
| Kirill-Murashev/mithril-veil | https://github.com/Kirill-Murashev/mithril-veil | 1 | 2026-05-20 | Apache-2.0 | Python 3.12+ (FastAPI) | «Self-hosted Russian PII anonymization gateway». Детерминированные regex/checksum-детекторы: email, телефон, **ИНН, СНИЛС, паспорт, ОГРН/ОГРНИП, КПП, БИК, банковские/корр. счета, карты (Luhn), кадастровые/судебные/договорные номера, IP, URL, Telegram**. Опционально Natasha NER и GLiNER (по умолчанию **выключены**). Пресеты политик: `general_ru`, `legal_ru`, `valuation_ru`, `banking_ru`, `court_case_ru`. Режимы: replace/redact/pseudonymize (обратимый, mapping в шифрованном `.json.enc` только через CLI). Документы: .txt/.md/.docx/.odt/.rtf/.pdf. | **Очень близко к нашей архитектуре** (rule-first, NER опционально). Но: **восстановление не реализовано** («Restore / de-anonymization is not implemented — design only»), OCR/UI/формат-сохраняющий вывод отсутствуют, v0.1.0, 1 звезда. Взять их **модель угроз и пресеты политик** как образец честной документации. |
| zubarevdm/pii-gateway | https://github.com/zubarevdm/pii-gateway | 0 | 2026-09-29 | нет файла лицензии | Python | «Локальный OpenAI-совместимый прокси, который находит и обезличивает ПД (152-ФЗ) в запросе **до** отправки в зарубежную модель, а в ответе восстанавливает». Слой 1 — regex + контрольные суммы (телефон, email, **ИНН, СНИЛС, ОГРН, паспорт, карта, счёт, госномер, IP**); слой 2 — RU NER (Natasha): ФИО, организации, адреса. Vault в памяти, не покидает контур. Явный юридический дисклеймер (`docs/legal/ОТКАЗ.md`). | **Прямое попадание в наш сценарий, но лицензии нет** → юридически брать нельзя. Взять структуру дисклеймера и идею «аудит: факт+тип маскировки, без ПД». |
| vovkins/ru-llm-proxy | https://github.com/vovkins/ru-llm-proxy | 1 | 2026-10-13 | нет файла лицензии | Python (LiteLLM) | Прокси на LiteLLM: regex + spaCy + **специализированная BERT-модель**; маскирование/блокировка/восстановление; **временные сопоставления в Redis**; восстановление **потокового** текста и JSON-аргументов инструментов, включая разорванные между событиями метки; NVIDIA CUDA-профиль Analyzer, **CPU по умолчанию**; словарные подстановки для десяти крупных российских банков; список разрешённых синтетических тестовых ПД; Prometheus, аудит. | **Самый функционально зрелый открытый русский прокси из найденных.** Ключевое для нас: они решают **потоковое восстановление и разрывы меток** — то, что мы ещё не проектировали. Есть `docs/research/gpu-analyzer-benchmark.md`. Лицензии нет. |
| NeoflexDognauts/dognauts-guardrails | https://github.com/NeoflexDognauts/dognauts-guardrails | 0 | 2026-09-29 | Apache-2.0 | Python (Guardrails AI + Presidio) | Guard-server на базе `guardrails-api`: guard `pii-anonymizer` (обратимые плейсхолдеры `<PERSON_1>`, кэш по session id), `pii-deanonymizer`, `phone-check`, `secrets-check`, `contains-card-number`, `toxicity-check`, `sensitive-data-check`. **Кастомные русские распознаватели Presidio**: паспорт, водительское удостоверение, КПП + spaCy NER для имён/локаций. | **Опасен по происхождению:** за org стоит **Neoflex** — крупный российский ИТ-интегратор. Даже при 0 звёзд это корпоративная разработка с ресурсами. Взять их `recognizers.yaml` и `sensitive_data/pii/ru_*.py` (Apache-2.0 позволяет). |
| Quakeer444/pii-arena | https://github.com/Quakeer444/pii-arena | 0 | 2026-09-22 | MIT | Python 3.12+ | **Независимый бенчмарк детекторов PII и секретов.** 41 датасет, 41 643 строки, 227 466 нормализованных gold-спанов, 2 591 сохранённый прогон предсказаний, замороженный эксперимент 2026-09-09. Живой лидерборд: https://www.piiarena.com/leaderboard (проверено, HTTP 200). Метрики: **Fully Hidden** (первичная), Detected/Overlapped, Extra Masking, char F1, и **скорость**: секунды/10k символов, символов/с, **амортизированные мс/строку p50 и p95**, пиковый RSS — на AMD EPYC 9K84, 16 потоков, 24 воркера. Русский срез присутствует. | **Берём целиком как основу нашего протокола бенчмарка.** Это ровно то, чего требует критерий выхода фазы 0 («черновик протокола бенчмарка с номером версии»): готовая методология, метрики, воспроизводимость, публичные CSV (`results/summary.csv`, `results/category-metrics.csv`) и команда `Evaluate your detector`. |
| Exemtik/pii-bench-ru-en | https://github.com/Exemtik/pii-bench-ru-en | 0 | 2026-05-31 | NOASSERTION | Python (pymorphy3 + spaCy) | Двуязычный детектор и токенизатор PII с оценкой на открытых датасетах. Русское ядро — **морфологический анализ pymorphy3** (имя/фамилия/отчество «по самим словам», без опоры на метки «фамилия:»). Оценка precision/recall/F1 по типам и языкам, сравнение `regex-only` против `regex + морфология/NER` и **pymorphy3 против `ru_core_news_sm`** на русских ФИО. Отчёт `results/REPORT.md`. | **Берём методологию сравнения.** Идея «русские ФИО распознаём морфологией, а не NER» — дешёвая альтернатива ruBERT и прямо релевантна нашей цели по латентности. Лицензия неопределённая → только идеи. |
| k1ddy/kazpii-bench | https://github.com/k1ddy/kazpii-bench | 0 | 2026-10-12 | нет файла лицензии | Python | Открытый бенчмарк PII для **казахских и русских** текстов. Слои: A — программная генерация (`kz-identifiers` × контексты × OCR-шумы), **готов, 5000 примеров**; B — KazNERD (112 702 предложения, CC BY 4.0) с доразметкой, готов, 2390; C — перефразирование тремя семействами LLM, готов, 4913; D — золотое подмножество с двойной ручной разметкой, не начат. Тестовая часть (20 %) закрыта. Типы: KZ_IIN, KZ_BIN, KZ_IBAN, KZ_PASSPORT, CARD, PHONE, EMAIL, PERSON, ORG, LOC. Инварианты: точные офсеты, шум только 1:1-подстановками, детерминизм по seed, canary-строка. | **Берём идею canary-строки** (детекция утечки датасета в обучение) и структуру «слои A/B/C/D» — это грамотный протокол для нашего будущего бенчмарка. |
| k1ddy/kz-identifiers | https://github.com/k1ddy/kz-identifiers | 0 | 2026-10-11 | Apache-2.0 | Python | Генератор казахстанских идентификаторов с **валидными контрольными суммами** (используется как слой A в KazPII-Bench). | **Берём как образец** генератора «валидных синтетических реквизитов». Казахстанские ИИН/БИН у нас в [`../06-method/checksum-verification.md`](../06-method/checksum-verification.md) помечены как непроверенные — этот проект может дать алгоритмы. |
| blauffy/personal-data | https://github.com/blauffy/personal-data | 1 | 2026-05-11 | нет файла (README заявляет MIT) | Python | Курсовая работа (МЭИ, 2025). Гибрид: regex для структурированных форматов + эвристики для ФИО. **10 типов ПД**: ИНН, паспорт РФ, СНИЛС, свидетельство о браке, кадастровый номер, банковская карта, телефон РФ, URL, имя, фамилия (с определением рода). Идемпотентность замен. Генерация синтетического корпуса (150 текстов). Заявлено: **Weighted F1 = 0.934**. | **Заявка F1=0.934 не проверяема как сравнимая:** корпус из 150 текстов сгенерирован самим автором, протокол не опубликован. Фиксируем как «заявлено, протокол не опубликован» — именно так, как требует метод фазы 0. |
| feuerstrahll/ru-pii-detection-system | https://github.com/feuerstrahll/ru-pii-detection-system | 0 | 2026-09-22 | MIT | — | «Privacy-first, explainable system for detecting Russian personal data in unstructured organizational sources». | Смотрим (MIT, объяснимость). |
| SergeyK88i/pii-obfuscator | https://github.com/SergeyK88i/pii-obfuscator | 0 | 2026-09-24 | нет | Python | «PII detection and deterministic pseudonymization for Russian text». | Смотрим идею **детерминированной** псевдонимизации. |
| mynka999/legal-anonymizer | https://github.com/mynka999/legal-anonymizer | 0 | 2026-07-17 | MIT | Python (Natasha) | Локальный анонимайзер юридических документов перед отправкой в облачные LLM: **Natasha NER офлайн**, проверка гла… | Смотрим: Natasha + юрдомен. |
| kpshinnik/docs_masked | https://github.com/kpshinnik/docs_masked | 20 | 2026-08-18 | MIT | Python | «Локальное обезличивание документов перед отправкой в LLM и обратная подстановка после ответа». 20 звёзд — самый заметный русский проект такого рода вне redmadrobot. | Смотрим. |
| STRUKTOR/shirma | https://github.com/STRUKTOR/shirma | 0 | 2026-10-01 | MIT | Python | «Ширма» — обезличивание документов перед работой с Claude Code и обратная замена. | Смотрим. |
| cndfdv/docx-anonymizer | https://github.com/cndfdv/docx-anonymizer | 1 | 2026-08-21 | нет | Python (Docker) | Локальный DOCX-анонимайзер: **GLiNER мультиязычная PII-модель + русские правила идентификаторов**, полностью офлайн. | Смотрим: пример связки GLiNER + русские правила. |
| agisota/dataclenear | https://github.com/agisota/dataclenear | 0 | 2026-05-08 | MIT | — | «Russian-first document anonymization UI built on Microsoft Presidio». | Смотрим UI. |
| Perruer/gorget | https://github.com/Perruer/gorget | 7 | 2026-09-25 | MIT | Python | «Security scanners for LLM prompts and responses: prompt injection, **PII (incl. Russian docu…)**». | Смотрим: единственный русский проект, объединяющий injection + PII. |
| Pioneer113/mcp-152fz-anonymizer | https://github.com/Pioneer113/mcp-152fz-anonymizer | 4 | 2026-07-27 | MIT | Python | «Автоматическое обезличивание персональных данных (ПД) по **152-ФЗ**. Готовый **MCP-сервер** и API». | Смотрим: MCP-интеграция — актуальный способ встраивания. |
| dbashkirov/pii_detection | https://github.com/dbashkirov/pii_detection | 2 | 2026-06-18 | нет | Python (Presidio) | Гибридный детектор ПД для русского на базе Presidio; породил датасет `alrosait/pii-synthetic-ru` (4500 примеров). | Смотрим как пример вклада в открытые данные. |
| bekzod/yashirgich | https://github.com/bekzod/yashirgich | 3 | 2026-02-15 | нет | Python | Узбекско-русская детекция и маскирование PII на ML (**rubai**) + правила (Presidio). | Смотрим: модель `rubai` покрывает и русский (см. `islomov/rubai-NER-150K-Personal`). |
| geekpartnerslab-kz/llmguard | https://github.com/geekpartnerslab-kz/llmguard | 2 | 2026-04-24 | нет | Python | Открытый прокси анонимизации PII между приложениями и LLM (Claude, GPT, локальные), OpenAI-совместимый, обратимый. | Смотрим. |
| NeoflexDognauts/* | см. выше | | | | | | |
| levitation-opensource/DataAnonymiser | https://github.com/levitation-opensource/DataAnonymiser | 7 | 2026-01-15 | MPL-2.0 | Python | Анонимизация данных в текстовых файлах и таблицах. | Смотрим (MPL-2.0 — слабый копилефт, совместим с Apache-2.0 при аккуратности). |

### 1.4 Мёртвые и архивные проекты

| Имя | URL | Звёзды | Последний коммит | Статус | Комментарий |
|---|---|---:|---|---|---|
| protectai/llm-guard | https://github.com/protectai/llm-guard | 3 216 | 2026-07-08 | **АРХИВИРОВАН** | Ориентир из плана. Развитие прекращено. |
| intenthq/anon | https://github.com/intenthq/anon | 352 | 2018-05-29 | **АРХИВИРОВАН** | UNIX-утилита анонимизации. Мёртв 8 лет. |
| tokern/piicatcher | https://github.com/tokern/piicatcher | 348 | 2023-10-12 | **АРХИВИРОВАН** | Сканер БД/хранилищ. Мёртв 3 года. |
| LeapBeyond/scrubadub | https://github.com/LeapBeyond/scrubadub | 433 | 2023-09-01 | мёртв (не архивирован) | 3 года без коммитов; старый URL `scrubadub/scrubadub` — 404. |
| redhuntlabs/Octopii | https://github.com/redhuntlabs/Octopii | 747 | 2025-01-22 | заброшен | 21 месяц. |
| postralai/masquerade | https://github.com/postralai/masquerade | 77 | 2025-08-11 | заброшен | 14 месяцев. |
| amanvirparhar/elara | https://github.com/amanvirparhar/elara | 69 | 2025-01-26 | мёртв | 21 месяц. |
| takashiishida/cleanprompt | https://github.com/takashiishida/cleanprompt | 26 | 2024-03-24 | мёртв | 2.5 года. |
| gbrigandi/mcp-server-conceal | https://github.com/gbrigandi/mcp-server-conceal | 18 | 2025-07-24 | заброшен | 14 месяцев. |
| ArtLabss/open-data-anonymizer | https://github.com/ArtLabss/open-data-anonymizer | 303 | 2023-07-12 | мёртв | 3 года. |
| thoughtworks-datakind/anonymizer | https://github.com/thoughtworks-datakind/anonymizer | 22 | 2021-11-22 | мёртв | 5 лет. |
| deeppavlov/ner | https://github.com/deeppavlov/ner | 338 | 2018-10-23 | мёртв | 8 лет. |
| Guitaricet/ner | https://github.com/Guitaricet/ner | 0 | 2018-05-22 | мёртв | 8 лет. |
| deeppavlov/DeepPavlov | https://github.com/deeppavlov/DeepPavlov | 6 997 | 2024-11-26 | почти мёртв | Почти 2 года без коммитов. Как русский NER-стек больше не опора. |
| **natasha/razdel** | https://github.com/natasha/razdel | 288 | **2023-07-24** | заморожен | 3 года без коммитов. |
| **natasha/navec** | https://github.com/natasha/navec | 220 | **2023-07-24** | заморожен | 3 года. |
| **natasha/slovnet** | https://github.com/natasha/slovnet | 254 | **2023-07-24** | заморожен | 3 года. |
| **natasha/nerus** | https://github.com/natasha/nerus | 76 | **2023-07-24** | заморожен | 3 года. |
| **natasha/yargy** | https://github.com/natasha/yargy | 338 | **2023-07-24** | заморожен | 3 года. |
| natasha/natasha | https://github.com/natasha/natasha | 1 359 | 2026-10-14 | живой (обёртка) | Умбрелла-репозиторий обновлялся недавно, но **все компоненты (`razdel`, `navec`, `slovnet`, `nerus`, `yargy`) стоят на 2023-07-24**. То есть метрики Natasha, на которые ссылаются конкуренты, получены на моделях трёхлетней давности. |
| vdobrovolskii/rucoco | https://github.com/vdobrovolskii/rucoco | 30 | 2022-12-25 | мёртв | 4 года. |
| dialogue-evaluation/RuCoCo-2023 | https://github.com/dialogue-evaluation/RuCoCo-2023 | 11 | 2023-03-24 | мёртв | 3.5 года. |
| dialogue-evaluation/factRuEval-2016 | https://github.com/dialogue-evaluation/factRuEval-2016 | 59 | **2016-12-08** | мёртв | **10 лет без коммитов**, лицензия MIT. Данные всё ещё используются (в PII Arena), но репозиторий — археология. |
| dialogue-evaluation/RuNNE | https://github.com/dialogue-evaluation/RuNNE | 12 | 2024-07-17 | заброшен | 15 месяцев, лицензия не указана. |
| stefan-it/hmbench | https://github.com/stefan-it/hmbench | 6 | 2023-11-14 | заброшен | 3 года. |

---

## 2. Живые и мёртвые проекты: выводы

**Сколько живых.** Из выборки в **276 уникальных репозиториев** (полный список с метаданными: [`scripts/gh_search_all.json`](scripts/gh_search_all.json), сырой вывод поиска: [`scripts/gh_search_out.txt`](scripts/gh_search_out.txt)) отобрано и проверено поштучно **79 репозиториев** (файлы [`scripts/gh_results.json`](scripts/gh_results.json), [`scripts/gh_key_results.json`](scripts/gh_key_results.json), [`scripts/gh_extra_results.json`](scripts/gh_extra_results.json), [`scripts/gh_last_results.json`](scripts/gh_last_results.json)). Из них:

- **живых (коммит за последние 12 месяцев, не архивированы) — 58**;
- **мёртвых или архивных — 15** (таблица 1.4);
- **замороженных компонентов Natasha — 5** (все 2023-07-24).

**Вывод 1. Ниша «PII-шлюз перед LLM» перегрета и одновременно хрупка.** Только по запросу `llm privacy proxy` GitHub отдаёт **209 репозиториев**, по `pii guardrails` — **805**. При этом почти все — свежие (сентябрь–октябрь 2026), с 0–30 звёздами и одним автором. Репозитории создаются и бросаются: `cleanprompt` (26 звёзд), `elara` (69), `masquerade` (77) — все мертвы. Это значит, что «сделать шлюз» — не барьер; барьер — довести до продакшна и держать.

**Вывод 2. Три проекта мертвы именно там, где мы планировали брать основу.** `llm-guard` (3 216 звёзд) — **архивирован**. `scrubadub` (433) — мёртв 3 года. `DeepPavlov` (6 997) — не обновлялся почти 2 года. План опирался на инфраструктуру, которая уже не развивается. Это надо зафиксировать в [`../02-solutions/`](../02-solutions/).

**Вывод 3. Русский NER-стек заморожен.** Natasha — это де-факто стандарт русского NER, на нём стоят AGIMA, `zubarevdm/pii-gateway`, `mynka999/legal-anonymizer`. Но `razdel`, `navec`, `slovnet`, `nerus`, `yargy` — **все на 2023-07-24**. Три года без обновлений. Для проекта, который заявляет русский приоритетом №1, это одновременно угроза (нет живого апстрима) и возможность (мы можем стать живой альтернативой).

**Вывод 4. Переезды репозиториев — операционный риск.** `microsoft/presidio` → `data-privacy-stack/presidio`; `microsoft/presidio-research` → `data-privacy-stack/presidio-research`; `NVIDIA/NeMo-Guardrails` → `NVIDIA-NeMo/Guardrails`; `scrubadub/scrubadub` → `LeapBeyond/scrubadub`; `zricethezav/gitleaks` → `gitleaks/gitleaks`. Любая фиксация версии в плане по старому URL со временем ломается. В наших документах нужно хранить и старый, и канонический URL.

**Вывод 5. Продакшн-шлюзы отказываются от NER ради латентности.** `packyme/privacy-filter` прямо пишет, что распознавание имён удалено, потому что «NER стоит секунды CPU на длинном тексте». `localai-org/privacy-filter.cpp` измеряет 140 мс на 512 токенов на CPU. AGIMA держит бюджет «сотни миллисекунд, а не секунды». Это подтверждает наш выбор rule-first — и указывает, что **наша цель p95 25–50 мс на CPU вместе с именами была бы уникальной**.

**Вывод 6. Самое опасное — `redmadrobot-rnd/pii-guard`.** Это не «ещё один проект». Это открытый, Apache-2.0, русский, обратимый, с контрольными суммами и восстановлением падежей, с собственным обучающим датасетом (17 137 строк), бенчмарком (2 841 строка) и весами модели. От корпорации redmadrobot. **Ядро замысла `privacy_core` уже реализовано открыто.** Единственное, что у них не измерено и не заявлено, — латентность и вес. Это должно стать предметом отдельного технического сравнения, а не строчкой в отчёте.

---

## 3. Бенчмарки PII и приватности

| Бенчмарк | URL (проверен) | Лицензия | Объём | Языки | Метрика | Стоит ли брать |
|---|---|---|---|---|---|---|
| **PII Arena** | https://github.com/Quakeer444/pii-arena (200) · https://www.piiarena.com/leaderboard (200) | MIT | **41 датасет, 41 643 строки, 227 466 gold-спанов, 2 591 прогон** | en, ru, multi | **Fully Hidden** (все символы gold-спана покрыты после расширения по границам слов), Detected/Overlapped, Extra Masking, char F1, типы, **скорость: с/10k символов, символов/с, мс/строку p50 и p95, пиковый RSS** | **ДА — как основа нашего протокола.** Единственный найденный бенчмарк, который (а) включает русский срез, (б) измеряет p95 и RSS, (в) публикует методологию, инварианты и CSV, (г) даёт команду для оценки своего детектора. Лидеры: `gliner2-fastino` 79.86 % Fully Hidden, `pplx` 74.92 %, `nuner-zero` 71.63 %. |
| **TAB** (Text Anonymization Benchmark) | https://github.com/NorskRegnesentral/text-anonymization-benchmark (200) | MIT | корпус + метрики | en | отдельные метрики на утечку/сохранение смысла | **ДА — только метрики.** Язык английский (домен ECHR), но концепция раздельных метрик для «сокрытия» и «полезности» для нас правильная. |
| TAB-ECHR (HF-зеркало) | https://huggingface.co/datasets/ildpil/text-anonymization-benchmark (200) | MIT | 127 строк, 3 830 спанов (по каталогу PII Arena) | en | — | Берём как готовое зеркало. |
| TAB-зеркала | https://huggingface.co/datasets/mattmdjaga/text-anonymization-benchmark-train · …-val-test | — | — | en | — | Не нужны, есть оригинал. |
| **AI4Privacy PII-masking-300k** | https://huggingface.co/datasets/ai4privacy/pii-masking-300k (200) | other (cc-by-4.0) | 300k | en | token classification | **ДА — как источник англоязычных подмен и форматов.** 117 лайков, 5 595 загрузок, обновлён 2026-06-03. |
| AI4Privacy pii-masking-200k | https://huggingface.co/datasets/ai4privacy/pii-masking-200k (200) | не указана | 200k | en | — | Берём как дополнение. |
| **AI4Privacy OpenPII 1M / 1.5M** | https://huggingface.co/datasets/ai4privacy/pii-masking-openpii-1m (200) · https://huggingface.co/datasets/ai4privacy/pii-masking-openpii-1.5m (200) | other (cc-by-4.0) | 1M–10M | 1M: 23 языка; 1.5M: 30 языков | token + generation | **Внимание: русского НЕТ ни в 1M, ни в 1.5M.** Перечень языков 1.5M: en, fr, de, es, it, nl, pt, bg, cs, da, el, et, fi, hr, hu, lt, lv, pl, ro, sk, sl, sr, sv, id, ja, ko, ms, tl, vi, zh. Это важный факт: **крупнейший открытый мультиязычный PII-датасет не покрывает русский.** |
| AI4Privacy open-pii-masking-500k | https://huggingface.co/datasets/ai4privacy/open-pii-masking-500k-ai4privacy (200) | other | 500k | en | — | Не берём (дубль линейки). |
| **nvidia/Nemotron-PII** | https://huggingface.co/datasets/nvidia/Nemotron-PII (200) | cc-by-4.0 | 100K<n<1M | **en** | token classification | Берём как англоязычный эталон. 123 лайка, 5 282 загрузки. Русского нет. |
| gretelai/synthetic_pii_finance_multilingual | https://huggingface.co/datasets/gretelai/synthetic_pii_finance_multilingual (200) | apache-2.0 | — | multi | span | Берём как источник мультиязычных финансовых подмен (русского в списке нет). |
| gretelai/gretel-pii-masking-en-v1 | https://huggingface.co/datasets/gretelai/gretel-pii-masking-en-v1 (200) | apache-2.0 | — | en | — | Не критично. |
| **TonicAI/Privacy-Bench** | https://huggingface.co/datasets/TonicAI/Privacy-Bench (200) | cc-by-4.0 | — | en | — | Смотрим: 20 лайков, обновлён 2026-10-15, 3 098 загрузок — активно используется. |
| beki/privy | https://huggingface.co/datasets/beki/privy (200) | MIT | — | en | — | Смотрим. |
| **perplexity-ai/PII-TRACE** | https://huggingface.co/datasets/perplexity-ai/PII-TRACE (200) | MIT | 500 диалогов, **2 653 спана**, 9 меток | en | span detection, **многоходовый** | **Берём идею: многоходовой диалог.** Единственный найденный PII-бенчмарк, размеченный по ходам диалога с указанием роли (user/assistant) и точных offset в коде-поинтах. Это ровно наш «контекстный» слой. arXiv:2609.22200. |
| piimb/pii-masking-benchmark | https://huggingface.co/datasets/piimb/pii-masking-benchmark (200) | — | — | — | — | Смотрим. |
| eriktks/conll2003 | https://huggingface.co/datasets/eriktks/conll2003 (200) | other | — | en | BIO NER | **Только как база NER** (классика, 180 лайков). Не PII-датасет: PER/ORG/LOC/MISC, нет ИНН/СНИЛС. |
| unimelb-nlp/wikiann | https://huggingface.co/datasets/unimelb-nlp/wikiann (200) | unknown | **2 003 000 строк**, 142 МБ | 282 языка, включая ru | BIO NER (PER/ORG/LOC) | Берём как **дешёвый источник русских имён/организаций/локаций** для NER-слоя. Лицензия «unknown» — проверить. |
| Guardrails AI «SOTA PII» (блог) | https://guardrailsai.com/blog/advanced-pii-and-jailbreak | — | — | en | заявлено SOTA | **Не проверено.** Страница открывается (HTTP 200), но тело статьи отрендерилось пустым при выборке; цифры не подтверждены. Требуется ручная проверка. |
| Presidio Research evaluator | https://github.com/data-privacy-stack/presidio-research | MIT | — | — | precision/recall по спанам | **Берём как харнесс.** |
| `llm-shield-proxy` pii-leak-benchmark | https://github.com/ninadphalak/LLM-Shield-Proxy | Apache-2.0 | — | en | «does your gateway leak PII in streams?» | **Смотрим: единственный найденный бенчмарк протечки PII именно в потоковом режиме.** Для нас критично, т.к. ответы LLM — потоковые. |

---

## 4. Русские датасеты для NER и обезличивания

### 4.1 Основная таблица

| Датасет | URL (проверен, HTTP 200) | Язык | Строк | Типы разметки | Лицензия | Обновлён | ПД: реальные или синтетические |
|---|---|---|---|---|---|---|---|
| **redmadrobot-rnd/pii_benchmark** | https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark | ru | **2 841** (5 614 спанов) | **21 тип, BIO**: FIRST_NAME, LAST_NAME, MIDDLE_NAME, COUNTRY, REGION, DISTRICT, CITY, STREET, HOUSE, EMAIL, PHONE, URL, IP_ADDRESS, **PASSPORT, INN, SNILS, OMS, CREDIT_CARD, DRIVER_LICENSE, MILITARY_ID, BIRTH_CERTIFICATE** | **MIT** | 2026-06-09 | **Синтетические.** «Примеры из продакшн-логов, где **все реальные ПД заменены синтетическими эквивалентами**», + шаблонные документы, + hard negatives. 23 лайка, 597 загрузок. |
| **redmadrobot-rnd/pii_train** | https://huggingface.co/datasets/redmadrobot-rnd/pii_train | ru | **17 137** | те же 21 тип | **MIT** | 2026-09-08 | Синтетические (та же процедура). Обучающая половина, тест — `pii_benchmark`, строки не пересекаются. |
| **hivetrace/pii-bench** | https://huggingface.co/datasets/hivetrace/pii-bench | ru | **1 810** (1 667 спанов) | **символьные offset, 13 типов**: NAME, PHONE_NUMBER, EMAIL, ADDRESS, BANK_CARD_NUMBER, CVC, **INN, KPP, OGRN, OGRNIP, SNILS, PASSPORT_NUMBER**, TOKEN | **Apache-2.0** | 2026-08-25 | **Синтетические**, «expert-generated» разметка. Домены (L-CHAT и др.). Наличие offset вместо BIO делает бенчмарк **агностичным к токенизатору** — можно оценивать целый пайплайн. 11 лайков, 528 загрузок. |
| **just-ai/jayguard-ner-benchmark** | https://huggingface.co/datasets/just-ai/jayguard-ner-benchmark | ru (+en) | **850** (1 195 спанов) | токены+BIO; акцент **PERSON** и **STREET_ADDRESS** | **MIT** | 2025-09-03 | Источник — «реальные сложные разговорные тексты: рабочие чаты, логи поддержки, транскрипты речи». **Опубликован вендором Jay Guard (Just AI)** — то есть коммерческий конкурент выложил свой русский бенчмарк открыто. 4 лайка. |
| **Powpowpow23/ru-pii-ner-data** | https://huggingface.co/datasets/Powpowpow23/ru-pii-ner-data | ru | **115 409** (104 111 train + 11 298 val; 495 948 сущностей) | **25 меток, nested NER**: FULL_NAME, DATE_OF_BIRTH, BIRTH_PLACE, **PASSPORT, CITIZENSHIP, PASSPORT_ISSUER, DEPARTMENT_CODE, PASSPORT_ISSUE_DATE, DRIVER_LICENSE**, ADDRESS, COUNTRY, POSTCODE, CITY, STREET, HOUSE, APARTMENT, EMAIL, PHONE, **INN, CARD_NUMBER, CVV, PIN, CARDHOLDER_NAME**, REGION, DISTRICT | **Apache-2.0** | 2026-09-26 | **Полностью синтетический** (`statistics.json`: «All texts originate in generation»; телефон и email заменены; «no general privacy guarantee»). Есть `SHA256SUMS`, `labels.json`, `tools/validate_dataset.py`. |
| **scanpatch/pii-ner-corpus-synthetic-controlled** | https://huggingface.co/datasets/scanpatch/pii-ner-corpus-synthetic-controlled | ru + uk | **5 318** (4 786 train + 532 test) | **27 типов**: name, first/middle/last_name, name_initials, address*, mobile_phone, email, nickname, ip, **tin, snils**, vehicle_number, document_number, military_individual_number | **MIT** | 2025-12-27 | **Синтетический**, разметка Claude Sonnet 4 с «human-verified annotation guidelines». |
| **alrosait/pii-synthetic-ru** | https://huggingface.co/datasets/alrosait/pii-synthetic-ru | ru | **4 500** | только **NAME** (2 748) и **ADDRESS** (2 569), + 1 026 негативных | **MIT** | 2026-06-18 | **Синтетический.** Сделан в рамках `dbashkirov/pii_detection` (гибрид на Presidio). |
| alexen2/pii-ner-ru-benchmark | https://huggingface.co/datasets/alexen2/pii-ner-ru-benchmark | ru | **1 000** (911 в каталоге PII Arena; 1 261 спан) | tokens + ner_tags (типы в карточке не перечислены) | не указана | 2026-04-09 | Предположительно синтетический. **Лицензия не указана** → использовать нельзя без уточнения. |
| **Wismut/nym-pii-multilingual-data** | https://huggingface.co/datasets/Wismut/nym-pii-multilingual-data | **23 языка, включая ru и uk** | **805 000** | PII token classification, «exactly-labeled» | **MIT** | 2026-07-08 | **Синтетические**, ровно размеченные. Крупнейший мультиязычный синтетический PII-датасет с русским, который я нашёл. 491 загрузка. |
| **islomov/rubai-NER-150K-Personal** | https://huggingface.co/datasets/islomov/rubai-NER-150K-Personal | uz + **ru** | ~150 000 | персональные сущности | **Apache-2.0** | 2026-02-02 | Синтетический/собранный. Основа модели `rubai`, на которую опирается `bekzod/yashirgich`. |
| tomaarsen/MultiCoNER | https://huggingface.co/datasets/tomaarsen/MultiCoNER | 11 языков, включая ru | — | BIO NER (MultiCoNER) | cc-by-4.0 | 2023-10-01 | Не PII-датасет (сложные именованные сущности), но **русский срез полезен** для NER-слоя. |
| **guneeshv/REDACT-PII-Benchmark** | https://huggingface.co/datasets/guneeshv/REDACT-PII-Benchmark | multi, есть ru-срез | ru: 500 строк, 9 230 спанов | PII | other, **gated=auto** | 2026-08-23 | Доступ по принятию условий. Учесть как источник, требующий юридической проверки. |
| **wolframko/russian-pii-66k** | https://huggingface.co/datasets/wolframko/russian-pii-66k | ru | **65 652** (25 МБ) | GIVENNAME, SURNAME, STREET, BUILDINGNUM, CITY, ACCOUNTNUM, USERNAME, EMAIL, TELEPHONENUM, DATEOFBIRTH, **IDCARDNUM** | не указана | 2026-02-15 | **Внимание — ловушка.** Это машинный перевод датасета ai4privacy 66k на русский. Проверка первых строк показала **европейские форматы**: «счёт **RU45FOMI8350305641395**» — это IBAN, а не российский 20-значный счёт; «номер паспорта **29 37 900581**» — формат ID-карты, а не российского паспорта (серия 4 + номер 6); «д. Костомукша» в поле CITY — артефакт перевода. **Для обучения русскому детектору реквизитов не годится**, годится только для имён/адресов/контактов. |
| iluvvatar/NEREL | https://huggingface.co/datasets/iluvvatar/NEREL | ru | 933 (24 369 спанов в PII Arena) | NEREL: вложенные сущности, отношения, события | не указана | 2023-03-30 | **Не PII-датасет в юридическом смысле**: PER/ORG/LOC и отношения. Не содержит ИНН/СНИЛС. |
| iluvvatar/RuNNE | https://huggingface.co/datasets/iluvvatar/RuNNE | ru | — | NER (часть NEREL) | не указана | 2023-03-30 | То же: только имена/организации. 1 лайк, 73 загрузки. |
| dialogue-evaluation/factRuEval-2016 | https://github.com/dialogue-evaluation/factRuEval-2016 (200) | ru | 254 (7 966 спанов в PII Arena) | PER, ORG, LOC + факты | **MIT** | коммит **2016-12-08** | Только имена/организации/локации. Репозиторий мёртв 10 лет. |
| gusevski/factrueval2016 | https://huggingface.co/datasets/gusevski/factrueval2016 | ru | 12,7 МБ (3 конфигурации) | то же | не указана | 2022-04-29 | HF-зеркало FactRuEval. 0 лайков, 65 загрузок. |
| unimelb-nlp/wikiann | см. раздел 3 | ru + 281 | 2 003 000 | PER/ORG/LOC | unknown | 2024-02-22 | Только имена/организации/локации. |
| usmiva/bg_ner_bsnlp | https://huggingface.co/datasets/usmiva/bg_ner_bsnlp | bg | — | NER | apache-2.0 | 2023-06-27 | **BSNLP**: единственное найденное HF-зеркало BSNLP — только болгарский срез. Хост BSNLP-2019 живой: https://bsnlp.cs.helsinki.fi/ (200); домен `bsnlp-2019.github.io` — **404**. |

### 4.2 Есть ли среди русских датасетов данные с реальными типами ПД

**Да — но только синтетические.** Разбираю по типам, которые нас интересуют:

| Тип ПД | Есть ли в открытом русском датасете | Где именно |
|---|---|---|
| **ИНН** (физлица/организации) | **да** | redmadrobot `pii_benchmark`+`pii_train` (263 спана в тесте), hivetrace `pii-bench`, Powpowpow23, scanpatch (как `tin`), `ru-llm-proxy`-совместимые правила |
| **СНИЛС** | **да** | redmadrobot (223 спана), hivetrace, Powpowpow23, scanpatch (`snils`) |
| **Паспорт РФ** | **да** | redmadrobot (494 спана — самый частый тип после имён), hivetrace (`PASSPORT_NUMBER`), Powpowpow23 (`PASSPORT`, `PASSPORT_ISSUER`, `DEPARTMENT_CODE`, `PASSPORT_ISSUE_DATE`) |
| **Счёт / БИК / банк** | **частично** | redmadrobot: `BANK_ACCOUNT`, `BIK` (в тестовом наборе типов есть, но в таблице спанов теста не показаны). `ACCOUNTNUM` в `russian-pii-66k` — это **европейский IBAN**, не российский счёт. |
| **Банковская карта** | **да** | redmadrobot (`CREDIT_CARD`, 203 спана), hivetrace (`BANK_CARD_NUMBER`, `CVC`), Powpowpow23 (`CARD_NUMBER`, `CVV`, `PIN`) |
| **ОМС** (полис) | **да** | redmadrobot (`OMS`, 170 спанов) |
| **Водительское удостоверение** | **да** | redmadrobot (`DRIVER_LICENSE`, 371 спан), Powpowpow23 |
| **Военный билет** | **да** | redmadrobot (`MILITARY_ID`, 278 спанов), scanpatch (`military_individual_number`) |
| **Свидетельство о рождении** | **да** | redmadrobot (`BIRTH_CERTIFICATE`, 341 спан) |
| **ОГРН / ОГРНП / КПП** | **да** | hivetrace (`OGRN`, `OGRNIP`, `KPP`) |
| **Телефон, email, адрес** | **да, широко** | почти все |
| **Госномер автомобиля** | **да** | scanpatch (`vehicle_number`) |
| **Telegram-ник** | **да** | redmadrobot (`TELEGRAM`) |
| **Полис/медданные** | **да** | redmadrobot (`OMS`) |
| **РЕАЛЬНЫЕ персональные данные** | **НЕТ НИГДЕ** | Ни один найденный открытый датасет не содержит реальных ПД. Все либо синтетические, либо «реальные логи с заменёнными на синтетику ПД». |

---

## 5. Главный вывод про русские датасеты с ПД

**Исходная гипотеза плана («открытых русских датасетов с ПД нет, и это наш ключевой вывод») — ОПРОВЕРГНУТА по состоянию на 2026-10-16, и это надо зафиксировать как важное изменение.**

Что реально обнаружено:

1. **Открытые русские датасеты с русскими типами ПД существуют, и их не один, а около двадцати.** Каталог PII Arena (проверен, MIT, `docs/datasets.md`) перечисляет **20 русскоязычных наборов** в срезе `ru`: `redmadrobot`, `corrupt-redmadrobot`, `hivetrace`, `corrupt-hivetrace`, `alexen2`, `alrosait`, `nerel`, `factrueval`, `multiconer-ru`, `nym-ru`, `redact-ru`, `rubai-ru`, `russian-pii-66k`, `scanpatch`, `jayguard`, а также четыре собственных синтетических набора проекта: `synth-ru-tickets` (32 099 спанов!), `synth-wiki-tables` (13 094), `synth-jira-comments` (2 705), `synth-secrets-ru`.

2. **Самый сильный набор — `redmadrobot-rnd/pii_benchmark` + `pii_train`.** 2 841 тестовая строка с 5 614 спанами и 17 137 обучающих строк, **21 тип**, включая именно те, которых нет ни у кого на Западе: ИНН, СНИЛС, ОМС, паспорт РФ, военный билет, свидетельство о рождении, БИК, банковский счёт, водительское удостоверение, индекс Почты России, Telegram. Лицензия MIT, то есть можно использовать и в коммерческом Apache-2.0-продукте. Опубликованы: датасет, веса модели (`rubert-base-pii-ner`), код (`pii-guard`), лидерборд по «общей области» PERSON+LOCATION (лучший zero-shot: `gliner_multi` с русским промптом — 73.1 % micro-F1; `gliner-guard-uniencoder` — 70.1 %; `gliner_multi_pii-v1` — 69.9 %).

3. **НО: реальных персональных данных нет ни в одном открытом наборе.** Это не пробел рынка, а **юридическая невозможность**: публикация реальных ПД россиян нарушает 152-ФЗ, и ни один вменяемый публикатор этого не делает. Формулировка redmadrobot предельно честная: «реальные пользовательские запросы, вручную проверенные и размеченные, **где все реальные персональные данные заменены синтетическими эквивалентами**».

**Следствие для стратегии. Вывод нужно переформулировать:**

> ~~«Открытых русских датасетов с ПД нет — значит, мы сделаем свой и это будет преимущество».~~
>
> **«Открытые русские датасеты с типами ПД уже есть (redmadrobot, hivetrace, Powpowpow23 и ещё ~17), и их можно брать бесплатно. Реальных ПД нет и не будет по закону. Значит, нашим преимуществом не может быть „свой датасет“ — преимуществом может быть только качество решения на этих общих данных плюс то, что в них отсутствует: контекстные ссылки, многоходовость, потоковое восстановление, латентность».**

Отдельно: **самый крупный русский PII-датасет (`russian-pii-66k`, 65 652 строки) для реквизитов непригоден** — это машинный перевод с европейскими форматами (IBAN вместо счёта, ID-карта вместо паспорта РФ). Если мы возьмём его «как есть» и обучимся на нём, мы получим детектор, который на реальных российских реквизитах будет вести себя неверно. Это конкретная ловушка, которую стоит записать в протокол подготовки данных.

---

## 6. Генераторы синтетических персональных данных

### 6.1 Что есть

| Инструмент | URL (проверен) | Звёзды | Последний коммит | Лицензия | Что генерирует по России | Годно для подмен? |
|---|---|---:|---|---|---|---|
| **joke2k/faker** | https://github.com/joke2k/faker (200) | 19 447 | 2026-10-09 | MIT | Провайдеры `ru_RU`: `person` (ФИО с полом и падежными формами), `address` (регион/город/улица/индекс), `phone_number`, `company`, `bank` (**БИК**, **корреспондентский счёт**, **расчётный счёт**), `passport` (**номер паспорта** в форматах `## ## ######` и `#### ######`), `ssn` (12 случайных цифр) | **Частично.** ФИО/адрес/телефон — да. **Реквизиты — только формат, без контрольной суммы.** См. 6.2. |
| **lk-geimfari/mimesis** | https://github.com/lk-geimfari/mimesis (200) | 4 848 | 2026-09-29 | MIT | Локаль `ru` покрывает `address` (в т.ч. `federal_subject()`) и `person`. Провайдеры `finance` (только **названия** банков), `code` (ISSN/ISBN/EAN/IMEI/PIN) | **НЕТ для реквизитов.** В `mimesis` **нет** провайдеров ИНН, СНИЛС, ОГРН, БИК, банковских счетов, паспорта РФ. Для наших целей mimesis почти бесполезен. |
| **DooGer1/russian-data-generator** | https://github.com/DooGer1/russian-data-generator (200) | 9 | 2025-02-28 | MIT | По описанию: «Библиотека для генерации случайных российских данных: **ФИО, ИНН, СНИЛС, адресов и номеров телефонов**» | **Да, но Java** (100 % Java по данным shields). 9 звёзд, полтора года без коммитов. Можно прочитать алгоритмы. |
| **k1ddy/kz-identifiers** | https://github.com/k1ddy/kz-identifiers (200) | 0 | 2026-10-11 | Apache-2.0 | Генератор казахстанских идентификаторов с **валидными контрольными суммами** | **Да, как образец подхода** «генерировать валидные реквизиты, а не случайные цифры». |
| LilKirill00/GeneratorPersonalData | https://github.com/LilKirill00/GeneratorPersonalData | 0 | 2024-03-04 | нет | «Программа генерирующая случайные персональные данные с возможностью перегенерации и копирования» | Учебный проект, 0 звёзд, мёртв 2.5 года. Не берём. |
| levitation-opensource/DataAnonymiser | https://github.com/levitation-opensource/DataAnonymiser (200) | 7 | 2026-01-15 | MPL-2.0 | Анонимизация, а не генерация | Не генератор. |
| scanpatch, alrosait, Powpowpow23 (датасеты) | см. раздел 4 | — | — | MIT/Apache-2.0 | Готовые синтетические русские ПД в виде датасетов | **Да — как источник готовых синтетических значений**, а не как генератор. |

### 6.2 Ключевой отрицательный вывод: Faker не даёт валидных русских реквизитов

Это надо зафиксировать отдельно, потому что это ломает очевидное решение «взять Faker для подмен».

Чтение исходного кода `faker/providers/bank/ru_RU/__init__.py` (https://raw.githubusercontent.com/joke2k/faker/master/faker/providers/bank/ru_RU/__init__.py, проверено):

```python
def bic(self) -> str:
    region = self.random_element(self.region_codes)
    department_code = self.numerify(self.random_element(self.department_code_formats))
    credit_organization_code = self.numerify(self.random_element(self.credit_organization_code_formats))
    return "04" + region + department_code + credit_organization_code

def correspondent_account(self) -> str:
    credit_organization_code = self.numerify(self.random_element(self.credit_organization_code_formats))
    return "301" + self.numerify("#" * 14) + credit_organization_code

def checking_account(self) -> str:
    account = self.random_element(self.checking_account_codes)
    organization = self.random_element(self.organization_codes)
    currency = self.random_element(self.currency_codes)
    return account + organization + currency + self.numerify("#" * 12)
```

Что из этого следует:

1. **`bic()`** собирает 9 цифр из справочников. Проверка БИК по контрольному разряду (4-я и 5-я цифры = «04» для РФ, дальше условное число) **не выполняется**. То есть БИК получается правдоподобным по виду, но не обязательно валидным.
2. **`correspondent_account()`** = `"301"` + 14 **случайных** цифр + код кредитной организации. Контрольный ключ корсчёта **не вычисляется**.
3. **`checking_account()`** = 3 цифры (код счёта) + 2 (организация) + 3 (валюта) + **12 случайных цифр**. Контрольный разряд расчётного счёта — **тоже случайная цифра**, статистически независимая от остальных 19.

**Оценка.** Если контрольный разряд счёта формируется независимо и равновероятно, доля синтетических счетов, проходящих проверку по контрольному ключу, составит ≈ **10 % (1 из 10)**. Это согласуется с независимым наблюдением из [`../06-method/checksum-verification.md`](../06-method/checksum-verification.md): «счёт даёт ~10 % ложных прохождений».

**Статус проверки:** это **вывод из исходного кода**, а не измерение. Запустить Faker в песочнице не удалось: `pip install faker` заблокирован (PEP 668, `externally-managed-environment`), `pip install --break-system-packages --user faker` — отказ по правам (`Errno 13: Permission denied: '/home/timofey/.local/lib/python3.12'`). **Чтобы получить точную цифру, нужно запустить скрипт проверки на машине с установленным Faker.** Файл для запуска сохранён: [`scripts/faker_ru_checksum_probe.py`](scripts/faker_ru_checksum_probe.py).

**`Faker.ssn('ru_RU')`** — это `ssn_formats = ("############",)`, то есть буквально 12 случайных цифр. ИНН физлица тоже 12 цифр, но ИНН имеет две контрольные суммы, которые здесь, разумеется, не считаются. **Использовать `faker.ssn()` как подмену ИНН нельзя** — получится невалидный ИНН.

**`Faker.passport_number('ru_RU')`** — форматы `## ## ######` и `#### ######`. У российского паспорта контрольной суммы нет, поэтому **здесь Faker пригоден**.

### 6.3 Что из этого следует для генерации подмен

| Тип | Готовый генератор | Можно ли брать как есть |
|---|---|---|
| ФИО, пол, падежные формы | `Faker('ru_RU')`: `name_male()`, `name_female()`, `last_name_male()`, `middle_name_male()` | **Да.** У Faker русские ФИО с согласованием рода — это ровно то, что нужно для «похожих фейков». |
| Адрес (регион/город/улица/индекс) | `Faker('ru_RU')` `address()`, `postcode()`, `mimesis` локаль `ru` | **Да.** |
| Телефон РФ | `Faker('ru_RU')` `phone_number()` | **Да**, но проверить, что префикс не попадает в реальные диапазоны, если это важно. |
| **Паспорт РФ** | `Faker('ru_RU')` `passport_number()` | **Да** — контрольной суммы у паспорта нет. |
| **ИНН** | готового валидного генератора не найдено | **Нет.** Писать свой генератор: 10/12 цифр с двумя контрольными суммами. Прототип алгоритма — в [`scripts/gen_ru_checksums.py`](scripts/gen_ru_checksums.py). |
| **СНИЛС** | готового не найдено; `scanpatch` даёт синтетические значения | **Нет.** Свой генератор: 11 цифр, контрольное число = сумма произведений цифр на (9..1), mod 101. |
| **БИК / счёт** | `Faker('ru_RU')` `bic()`, `checking_account()`, `correspondent_account()` | **Нет — формат без контрольного ключа** (см. 6.2). Нужен свой генератор с вычислением ключа. |
| **ОГРН / ОГРНИП / КПП** | готового не найдено | **Нет.** Свой генератор. |
| Карта | `Faker` `credit_card_number()` (Luhn), `Powpowpow23` | **Да** — Luhn реализован. |
| Секреты (API-ключи) | `gitleaks/gitleaks` правила + `betterleaks` | **Да**, для негативных примеров. |

**Стратегический вывод по генераторам.** Готового решения «валидные российские реквизиты» **не существует** в открытом виде: Faker даёт формат без контрольных сумм, mimesis не покрывает реквизиты вовсе. Это **реальная ниша**, и она совпадает с тем, что уже сделано в [`../06-method/checksum-verification.md`](../06-method/checksum-verification.md): раз мы умеем **проверять** контрольные ключи, мы почти бесплатно получаем умение **генерировать** валидные значения. Генератор валидных подмен — это малый, но честный дифференциатор и одновременно обязательное условие, чтобы подменённый промпт не ломал downstream-валидацию.

**Прототип генератора написан и проверен запуском.** Файл [`scripts/gen_ru_checksums.py`](scripts/gen_ru_checksums.py) — чистый Python без зависимостей, детерминированный по seed. **Запущен, `EXIT=0`, все проверки пройдены** (проверено 2026-10-16):

- контрольные суммы на **известных публичных векторах** сошлись: ИНН юрлиц `7707083893`, `7736050003`, `7728168971`; ИНН физлица `500100732259`; СНИЛС `112-233-445 95`;
- сгенерированные на seed 1, 42 и 20261016 значения проходят **собственные** проверки (ИНН-10, ИНН-12, СНИЛС, формат БИК, контрольный ключ расчётного счёта);
- **отрицательный контроль** сработал: испорченная цифра ИНН ломает проверку;
- **детерминированность по seed** подтверждена (`generate(7) == generate(7)`).

Пример вывода на `seed=20261016`: ИНН юрлица `1988684720`, ИНН физлица `149838402087`, СНИЛС `097-596-596 57`, БИК `046116650`, расчётный счёт `40702810893073885224`, паспорт `42 76 665533`. Корреспондентский счёт намеренно **не** генерируется — см. [`../06-method/checksum-verification.md`](../06-method/checksum-verification.md).

**Что это значит.** Утверждение «готового генератора валидных российских реквизитов нет» проверено дважды: (а) чтением кода Faker и mimesis, (б) тем, что рабочий прототип пришлось писать самим. Это можно оформлять как малый технический актив проекта.

---

## 7. Коммерческие продукты

Вынесено в отдельный документ: [`../04-market/README.md`](../04-market/README.md). Там глобальные вендоры, российские вендоры, проверяемость заявлений, поддержка русского и рыночный контекст.

Кратко для связи разделов: ближайшие к нам коммерческие продукты — **StarGuard AI** (Orion soft, реестр Минцифры №35067 от 07.09.2026), **Jay Guard** (Just AI), **KodikRouter** (Architech AI), **CorpSoft24**. Все три ключевых российских вендора заявляют ровно наш сценарий: перехват запроса → детекция ПД → маскировка на токены → демаскировка ответа.

---

## 8. Что мы можем переиспользовать

Раздел делит находки на «берём код» (совместимая лицензия), «берём данные», «берём метод» и «только читаем».

### 8.1 Берём код (лицензия позволяет)

| Что | Откуда | Лицензия | Зачем |
|---|---|---|---|
| Харнесс оценки распознавателей | `data-privacy-stack/presidio-research` | MIT | Готовый `PresidioEvaluator`: precision/recall по спанам, сравнение конфигураций. Каркас для нашего бенчмарка. |
| Правила поиска секретов | `gitleaks/gitleaks` | MIT | ~222 правила, уже доказавшие переносимость в RE2 (`packyme/privacy-filter` скомпилировал все). Нужны для негативных примеров и для защиты от утечки ключей в промпте. |
| Русские распознаватели Presidio | `NeoflexDognauts/dognauts-guardrails` (`recognizers.yaml`, `sensitive_data/pii/ru_*.py`) | Apache-2.0 | Готовые паттерны: паспорт РФ, водительское удостоверение, КПП. Совместимо с Apache-2.0. |
| Утилита подсчёта метрик NER | `davidsbatista/NER-Evaluation` | MIT | Отчёт по типам, seqeval-подобный. Стабильный код, несмотря на 2 года без коммитов. |
| Структура пресетов политик и модель угроз | `Kirill-Murashev/mithril-veil` | Apache-2.0 | Пресеты `general_ru`/`legal_ru`/`banking_ru`/`court_case_ru` + `docs/threat_model.md` — образец честной документации границ. |
| Архитектура Rust-детектора | `censgate/redact`, `CertaMesh/gaze`, `colliery-io/muninn`, `runta-dev/clawshell` | Apache-2.0 | Четыре независимых Rust-реализации в нише. Изучить матчинг, аллокации, работу со стримингом. |

### 8.2 Берём данные (с оговорками)

| Что | Лицензия | Оговорка |
|---|---|---|
| `redmadrobot-rnd/pii_benchmark` + `pii_train` | MIT | **Главный актив.** 21 русский тип ПД, включая ИНН/СНИЛС/ОМС/паспорт/БИК/счёт. Готовые train/test без пересечения. |
| `hivetrace/pii-bench` | Apache-2.0 | Символьные offset → агностичен к токенизатору. 13 типов. |
| `just-ai/jayguard-ner-benchmark` | MIT | 850 строк реальных чатов и транскриптов. Вендорский, но открытый. |
| `Powpowpow23/ru-pii-ner-data` | Apache-2.0 | 115 409 строк, 25 меток, nested NER, есть `SHA256SUMS` и валидатор. |
| `scanpatch/pii-ner-corpus-synthetic-controlled` | MIT | ru+uk, 27 типов, включая `tin`/`snils`/`vehicle_number`. |
| `alrosait/pii-synthetic-ru` | MIT | Только NAME/ADDRESS, но 4 500 чистых примеров. |
| `Wismut/nym-pii-multilingual-data` | MIT | 805 000 примеров, 23 языка, включая ru и uk. |
| `islomov/rubai-NER-150K-Personal` | Apache-2.0 | uz+ru, 150k. |
| `unimelb-nlp/wikiann` | **unknown** | Лицензия неизвестна → **юридически рискованно**, сначала выяснить. |
| `ai4privacy/pii-masking-300k` | other (cc-by-4.0) | Английский; для подмен форматов и негативных примеров. |
| `eriktks/conll2003` | other | База NER, английский. |

### 8.3 Берём метод (не код)

| Метод | Откуда | Что именно |
|---|---|---|
| **Протокол бенчмарка целиком** | `Quakeer444/pii-arena` | 41 датасет, метрика **Fully Hidden** как первичная, отдельно Detected / Extra Masking / char F1, **отдельно скорость: p50 и p95 мс/строку и пиковый RSS**, отдельно качество и скорость как разные оси. «Do not collapse into one composite score». Плюс frozen snapshot с датой. |
| **Canary-строка в датасете** | `k1ddy/kazpii-bench` | Каждая запись несёт canary — детекция утечки датасета в обучение. Дёшево, ставим у себя. |
| **Слоистая сборка датасета A/B/C/D** | `k1ddy/kazpii-bench` | A — программная генерация, B — переиспользование внешнего корпуса, C — перефразирование LLM, D — двойная ручная разметка с адъюдикацией. |
| **Сравнение pymorphy3 против spaCy на русских ФИО** | `Exemtik/pii-bench-ru-en` | Готовый метод дешёвого морфологического распознавания имён без NER. |
| **Правило приоритета «regex важнее модели»** | AGIMA (`../04-market/README.md`), `redmadrobot/pii-guard` | При пересечении спанов побеждает формат с контрольной суммой. Промышленно проверено. |
| **Многоходовая разметка диалога** | `perplexity-ai/PII-TRACE` | Спаны с привязкой к номеру хода и роли (user/assistant), offset в коде-поинтах. Наш «контекстный» слой. |
| **Бенчмарк протечки в стриминге** | `ninadphalak/LLM-Shield-Proxy` (`pii-leak-benchmark`) | Проверка, не утекает ли PII при потоковом восстановлении. |
| **Stateless-маппинг** | `redmadrobot/pii-guard`, `CassiopeiaCode/CosyRedactGateway`, `mynka999/legal-anonymizer` (идея) | Mapping не хранится на сервере, уезжает вызывающему. Сильно упрощает соответствие 152-ФЗ. |
| **Дисклеймер «не гарантия соответствия 152-ФЗ»** | `zubarevdm/pii-gateway` (`docs/legal/ОТКАЗ.md`) | Готовая формулировка честного позиционирования (согласовать с потоком 5-legal). |

### 8.4 Только читаем (лицензия не позволяет переиспользовать)

| Что | Лицензия | Почему нельзя |
|---|---|---|
| `xiaYuTian11/maskit` | AGPL-3.0 | Сильный копилефт — несовместим с Apache-2.0-продуктом. |
| `yatuk/tamga` | AGPL-3.0 | То же. |
| `zubarevdm/pii-gateway`, `vovkins/ru-llm-proxy`, `SergeyK88i/pii-obfuscator`, `bekzod/yashirgich`, `geekpartnerslab-kz/llmguard`, `dbashkirov/pii_detection`, `blauffy/personal-data`, `k1ddy/kazpii-bench`, `alexen2/pii-ner-ru-benchmark`, `iluvvatar/NEREL`, `iluvvatar/RuNNE`, `gusevski/factrueval2016`, `wolframko/russian-pii-66k` | лицензия не указана | По умолчанию «все права защищены». **Идеи — можно, код и данные — нет.** |
| `RPBLC-hq/DAM` | нет файла лицензии | То же. |
| `guneeshv/REDACT-PII-Benchmark` | other, gated | Требует принятия условий — проверить юристом. |
| `unimelb-nlp/wikiann` | unknown | Неизвестна. |
| `NVIDIA-NeMo/Guardrails` | Other | Не OSI-лицензия. |
| `ArtLabss/open-data-anonymizer`, `thoughtworks-datakind/anonymizer` | нет | Мертвы и без лицензии. |

---

## 9. Библиография с URL

### 9.1 Репозитории (все URL проверены HTTP 200; «→» означает редирект)

**Инфраструктура PII:**
- https://github.com/microsoft/presidio → https://github.com/data-privacy-stack/presidio
- https://github.com/microsoft/presidio-research → https://github.com/data-privacy-stack/presidio-research
- https://github.com/openai/privacy-filter
- https://github.com/localai-org/privacy-filter.cpp
- https://github.com/urchade/GLiNER
- https://github.com/Knowledgator/GLiNER
- https://github.com/HiveTrace/gliner-guard
- https://github.com/guardrails-ai/guardrails
- https://github.com/guardrails-ai/detect_pii
- https://github.com/guardrails-ai/guardrails_pii
- https://github.com/NVIDIA/NeMo-Guardrails → https://github.com/NVIDIA-NeMo/Guardrails
- https://github.com/protectai/llm-guard (**архивирован**)
- https://github.com/maziyarpanahi/openmed
- https://github.com/Rizzo-AI-Academy/rizzo-pii
- https://github.com/zeroc00I/DontFeedTheAI
- https://github.com/redhuntlabs/Octopii
- https://github.com/arx-deidentifier/arx
- https://github.com/GreenmaskIO/greenmask
- https://github.com/censgate/redact
- https://github.com/hoophq/alcatraz
- https://github.com/thyrisAI/safe-zone
- https://github.com/anonde-io/anonde
- https://github.com/CertaMesh/gaze
- https://github.com/colliery-io/muninn
- https://github.com/gbrigandi/mcp-server-conceal
- https://github.com/RPBLC-hq/DAM
- https://github.com/awslabs/ferret-scan
- https://github.com/thoughtbot/top_secret
- https://github.com/zricethezav/gitleaks → https://github.com/gitleaks/gitleaks
- https://github.com/betterleaks/betterleaks
- https://github.com/EdyVision/pii-codex
- https://github.com/scrubadub/scrubadub → https://github.com/LeapBeyond/scrubadub
- https://github.com/davidsbatista/NER-Evaluation
- https://github.com/NorskRegnesentral/text-anonymization-benchmark
- https://github.com/stefan-it/hmbench
- https://github.com/intenthq/anon (**архивирован**)
- https://github.com/tokern/piicatcher (**архивирован**)
- https://github.com/ArtLabss/open-data-anonymizer
- https://github.com/thoughtworks-datakind/anonymizer

**Шлюзы приватности:**
- https://github.com/packyme/privacy-filter
- https://github.com/dataiku/kiji-proxy
- https://github.com/sgasser/pasteguard
- https://github.com/xiaYuTian11/maskit
- https://github.com/runta-dev/clawshell
- https://github.com/CassiopeiaCode/CosyRedactGateway
- https://github.com/voidmind-io/voidllm
- https://github.com/yatuk/tamga
- https://github.com/zolderio/token-proxy
- https://github.com/k13lucien/maskme
- https://github.com/DataFog/datafog-python
- https://github.com/crp4222/PrivAiTe
- https://github.com/postralai/masquerade
- https://github.com/takashiishida/cleanprompt
- https://github.com/amanvirparhar/elara
- https://github.com/sherwinvishesh/SurrogateShield
- https://github.com/ninadphalak/LLM-Shield-Proxy

**Русскоязычные:**
- https://github.com/redmadrobot-rnd/pii-guard
- https://github.com/dewil/pii-mask
- https://github.com/Kirill-Murashev/mithril-veil
- https://github.com/zubarevdm/pii-gateway
- https://github.com/vovkins/ru-llm-proxy
- https://github.com/NeoflexDognauts/dognauts-guardrails
- https://github.com/Quakeer444/pii-arena
- https://github.com/Exemtik/pii-bench-ru-en
- https://github.com/k1ddy/kazpii-bench
- https://github.com/k1ddy/kz-identifiers
- https://github.com/blauffy/personal-data
- https://github.com/feuerstrahll/ru-pii-detection-system
- https://github.com/SergeyK88i/pii-obfuscator
- https://github.com/mynka999/legal-anonymizer
- https://github.com/kpshinnik/docs_masked
- https://github.com/STRUKTOR/shirma
- https://github.com/cndfdv/docx-anonymizer
- https://github.com/agisota/dataclenear
- https://github.com/Perruer/gorget
- https://github.com/Pioneer113/mcp-152fz-anonymizer
- https://github.com/dbashkirov/pii_detection
- https://github.com/bekzod/yashirgich
- https://github.com/geekpartnerslab-kz/llmguard
- https://github.com/levitation-opensource/DataAnonymiser
- https://github.com/natasha/natasha
- https://github.com/natasha/razdel
- https://github.com/natasha/navec
- https://github.com/natasha/slovnet
- https://github.com/natasha/nerus
- https://github.com/natasha/yargy
- https://github.com/deeppavlov/DeepPavlov
- https://github.com/dialogue-evaluation/RuNNE
- https://github.com/dialogue-evaluation/factRuEval-2016
- https://github.com/vdobrovolskii/rucoco
- https://github.com/dialogue-evaluation/RuCoCo-2023

**Генераторы:**
- https://github.com/joke2k/faker
- https://raw.githubusercontent.com/joke2k/faker/master/faker/providers/bank/ru_RU/__init__.py
- https://raw.githubusercontent.com/joke2k/faker/master/faker/providers/ssn/ru_RU/__init__.py
- https://raw.githubusercontent.com/joke2k/faker/master/faker/providers/passport/ru_RU/__init__.py
- https://github.com/lk-geimfari/mimesis
- https://github.com/DooGer1/russian-data-generator
- https://github.com/LilKirill00/GeneratorPersonalData

### 9.2 Датасеты (все URL проверены HTTP 200 через HF API)

- https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark
- https://huggingface.co/datasets/redmadrobot-rnd/pii_train
- https://huggingface.co/models/redmadrobot-rnd/rubert-base-pii-ner
- https://huggingface.co/datasets/hivetrace/pii-bench
- https://huggingface.co/datasets/just-ai/jayguard-ner-benchmark
- https://huggingface.co/datasets/Powpowpow23/ru-pii-ner-data
- https://huggingface.co/datasets/scanpatch/pii-ner-corpus-synthetic-controlled
- https://huggingface.co/datasets/alrosait/pii-synthetic-ru
- https://huggingface.co/datasets/alexen2/pii-ner-ru-benchmark
- https://huggingface.co/datasets/wolframko/russian-pii-66k
- https://huggingface.co/datasets/Wismut/nym-pii-multilingual-data
- https://huggingface.co/datasets/islomov/rubai-NER-150K-Personal
- https://huggingface.co/datasets/tomaarsen/MultiCoNER
- https://huggingface.co/datasets/guneeshv/REDACT-PII-Benchmark
- https://huggingface.co/datasets/iluvvatar/NEREL
- https://huggingface.co/datasets/iluvvatar/RuNNE
- https://huggingface.co/datasets/gusevski/factrueval2016
- https://huggingface.co/datasets/unimelb-nlp/wikiann
- https://huggingface.co/datasets/ai4privacy/pii-masking-300k
- https://huggingface.co/datasets/ai4privacy/pii-masking-200k
- https://huggingface.co/datasets/ai4privacy/pii-masking-openpii-1m
- https://huggingface.co/datasets/ai4privacy/pii-masking-openpii-1.5m
- https://huggingface.co/datasets/ai4privacy/open-pii-masking-500k-ai4privacy
- https://huggingface.co/datasets/nvidia/Nemotron-PII
- https://huggingface.co/datasets/gretelai/synthetic_pii_finance_multilingual
- https://huggingface.co/datasets/gretelai/gretel-pii-masking-en-v1
- https://huggingface.co/datasets/TonicAI/Privacy-Bench
- https://huggingface.co/datasets/beki/privy
- https://huggingface.co/datasets/perplexity-ai/PII-TRACE
- https://huggingface.co/datasets/ildpil/text-anonymization-benchmark
- https://huggingface.co/datasets/mattmdjaga/text-anonymization-benchmark-train
- https://huggingface.co/datasets/mattmdjaga/text-anonymization-benchmark-val-test
- https://huggingface.co/datasets/eriktks/conll2003
- https://huggingface.co/datasets/usmiva/bg_ner_bsnlp
- https://huggingface.co/datasets/DataikuNLP/kiji-pii-training-data
- https://huggingface.co/datasets/piimb/pii-masking-benchmark
- https://huggingface.co/datasets/OpenMed/privacy-filter-multilingual
- https://huggingface.co/models/openai/privacy-filter

### 9.3 Бенчмарки и методология

- https://github.com/Quakeer444/pii-arena
- https://www.piiarena.com/leaderboard
- https://raw.githubusercontent.com/Quakeer444/pii-arena/main/docs/datasets.md
- https://github.com/NorskRegnesentral/text-anonymization-benchmark
- https://guardrailsai.com/blog/advanced-pii-and-jailbreak (страница открывается, содержимое не извлеклось)
- https://bsnlp.cs.helsinki.fi/ (хост BSNLP жив; `bsnlp-2019.github.io` → 404)
- https://arxiv.org/abs/2605.05277 (статья GLiNER Guard — ведёт другой поток)
- https://arxiv.org/abs/2609.22200 (PII-TRACE)

---

## 10. Что проверено / по описанию / не проверено

### 10.1 Проверено фактически (открыл страницу или API)

| Утверждение | Как проверено |
|---|---|
| Существование и метаданные **всех 79 репозиториев** в таблицах 1.1–1.4 | `https://github.com/<owner>/<repo>` → HTTP 200; звёзды из `"stargazerCount"`, лицензия из `"license":{"spdxId"}`, архивность из `"isArchived"` в HTML |
| Дата последнего коммита по каждому репозиторию | `https://github.com/<owner>/<repo>/commits.atom` → HTTP 200, поле `<updated>` |
| `microsoft/presidio` → `data-privacy-stack/presidio` | `curl -sIL` → `https://github.com/data-privacy-stack/presidio`, HTTP 200 |
| `NVIDIA/NeMo-Guardrails` → `NVIDIA-NeMo/Guardrails` | HTML-поле `final` после редиректа |
| `scrubadub/scrubadub` → 404, живёт как `LeapBeyond/scrubadub` | HTTP 404 на старом, 200 на новом |
| `protectai/llm-guard` архивирован | `"isArchived":true` |
| Natasha-компоненты заморожены на 2023-07-24 | atom-фиды `razdel`, `navec`, `slovnet`, `nerus`, `yargy` |
| Существование и метаданные **всех датасетов** в разделе 4 | `https://huggingface.co/api/datasets/<id>` → HTTP 200, поля `likes`, `downloads`, `lastModified`, `cardData.license`, `tags` |
| Размеры датасетов (число строк) | `https://datasets-server.huggingface.co/size?dataset=<id>` |
| Содержимое `russian-pii-66k` (европейские форматы) | `https://datasets-server.huggingface.co/first-rows?dataset=wolframko/russian-pii-66k&config=default&split=train` — прочитаны реальные строки: IBAN `RU45FOMI8350305641395` в поле ACCOUNTNUM, `29 37 900581` как IDCARDNUM |
| Содержимое `hivetrace/pii-bench` | `first-rows` → реальные примеры с offset-разметкой |
| Состав 20 русских датасетов в PII Arena | `https://raw.githubusercontent.com/Quakeer444/pii-arena/main/docs/datasets.md` — прочитана таблица каталога |
| Лидерборд PII Arena живой | `https://www.piiarena.com/leaderboard` → HTTP 200 |
| Содержимое README `redmadrobot-rnd/pii-guard` (20 типов, режимы, восстановление падежа) | `raw.githubusercontent.com/redmadrobot-rnd/pii-guard/main/README.md` — прочитан целиком |
| Замеры латентности `packyme/privacy-filter` и `privacy-filter.cpp` | Прочитаны README обоих репозиториев |
| Исходный код `Faker` ru_RU (bank/ssn/passport) | `raw.githubusercontent.com/joke2k/faker/master/faker/providers/...` — прочитан код |
| `calypsoai.com` → `f5.com/products/ai-guardrails` | `curl -sL -o /dev/null -w "%{url_effective}"` → редирект, финальный HTTP 200 |
| `private-ai.com/en/pricing` → `getlimina.ai/en/pricing` | То же |
| Статья AGIMA «Почему мы не нашли готового PII-детектора для русского языка» | `https://www.agima.ru/blog/architecture/pochemu-my-ne-nashli-...` → HTTP 200, прочитана; `https://habr.com/ru/companies/agima/articles/1072932/` → HTTP 200 |
| Статистика Solar Dozor (38 % / 41 % / 30 % / 18 % / 11 %) | `https://osp.ru/articles/2026/0811/13060958` → HTTP 200, прочитана |
| Цены Nightfall (медиана $25 000/год) | `https://www.vendr.com/marketplace/nightfall` → HTTP 200 |
| Lasso Security $30M Series A, LEAP <5 мс | `https://dealroom.co/news/148347-...` → HTTP 200 |
| StarGuard AI в реестре Минцифры №35067 | `https://www.orionsoft.ru/starguardai` → HTTP 200, прочитано |
| Jay Guard: функции, поставка, лицензирование | `https://www.robotx.ru/programmnoe-obespechenie/ofisnye-programmy/jay-guard/` → HTTP 200 |
| KodikRouter: NER-модуль сокрытия ПД | `https://tadviser.com/index.php/Product:AI_Architech:_KodikRouter:...` → HTTP 200 |
| Samsung запретила ChatGPT после утечки кода | `https://4cio.ru/news/view/8184` → HTTP 200 |
| `factRuEval-2016` MIT, последний коммит 2016-12-08 | HTML репозитория |

### 10.2 По описанию (вторичный источник, страницу не открывал или содержимое не извлеклось)

| Утверждение | Источник | Почему не «проверено» |
|---|---|---|
| Guardrails AI заявляет SOTA по PII-детекции | `https://guardrailsai.com/blog/advanced-pii-and-jailbreak` | HTTP 200, но тело статьи отрендерилось пустым (SPA). Цифры не подтверждены. |
| Protecto: Privacy Gateway for AI Chat, GPTGuard, Privacy Vault | `https://www.protecto.ai/pricing/` → 404, но навигация сайта видна | Страница цен отсутствует; состав продуктов взят из навигации 404-страницы. |
| `robustintelligence.com` — часть Cisco | Поисковая выдача: `https://www.cisco.com/site/us/en/products/security/ai-defense/robust-intelligence-is-part-of-cisco/index.html` | Страницу Cisco не открывал. |
| Prompt Security куплен SentinelOne | Поисковая выдача: `https://prompt.security/blog/prompt-security-sentinelone-a-new-chapter-begins`; сайт `prompt.security` — HTTP 000 | Сайт недоступен из среды. |
| Skyflow — «LLM privacy vault» | `https://www.skyflow.com/pricing` → редирект на корень (200), страницы цен нет | Состав продукта не подтверждён документацией. |
| BSNLP как источник русского NER | `usmiva/bg_ner_bsnlp` — только болгарский; хост `bsnlp.cs.helsinki.fi` (200) | Русского среза BSNLP на HF не нашёл. |
| Датасет Gareev | — | **Не найдено.** Ни на HF, ни в GitHub-поиске воспроизводимого репозитория/датасета «Gareev» для русского NER не обнаружено. Считать «не найдено», а не «есть». |

### 10.3 Не проверено (не нашёл или проверить нельзя)

| Что | Почему |
|---|---|
| **Замер контрольного ключа `Faker` ru_RU** | `pip install faker` заблокирован (PEP 668), `--break-system-packages --user` — отказ по правам (`Errno 13`). Вывод «≈10 %» сделан **из чтения кода**, не измерением. Скрипт для запуска: [`scripts/faker_ru_checksum_probe.py`](scripts/faker_ru_checksum_probe.py). |
| **Латентность `redmadrobot-rnd/pii-guard`** | В README замеров **нет**. Сравнить с нашей целью p95 25–50 мс нечем. Требуется собственный замер. |
| Ссылка на репозиторий `HiveTrace/gliner-guard` как источник кода | Репозиторий существует (HTTP 200, Apache-2.0), но содержимое не изучалось. |
| Точная лицензия `protectai/llm-guard` в актуальном состоянии | В HTML лицензия определена как MIT, но репозиторий архивирован — юридически фиксировать по факту архивации. |
| Наличие `Sber`/`Yandex`-продуктов класса PII-шлюз | Целенаправленно не искал: это было бы дублированием с рыночным потоком, см. [`../04-market/README.md`](../04-market/README.md). |
| Русский срез `REDACT-PII-Benchmark` | Доступ gated (`gated=auto`) — содержимое не читал. |
| Содержимое `guneeshv/REDACT-PII-Benchmark`, `TonicAI/Privacy-Bench`, `beki/privy`, `piimb/*` | Смотрел только метаданные, не данные. |
| Датасет RuCoCo | Детально ведёт другой поток (по указанию постановщика). Здесь только URL: `vdobrovolskii/rucoco` (30 звёзд, мёртв 2022-12-25), `dialogue-evaluation/RuCoCo-2023` (11 звёзд, мёртв 2023-03-24). |

### 10.4 Что изменилось по сравнению с исходным планом (важно для [`../01-audit.md`](../01-audit.md))

| Гипотеза плана | Факт на 2026-10-16 |
|---|---|
| «Открытых русских датасетов с ПД нет» | **Опровергнуто.** ~20 русских PII-датасетов, лучший — `redmadrobot-rnd/pii_benchmark` (MIT, 21 тип, 2 841 тест + 17 137 train). Реальных ПД нет ни в одном — по закону. |
| «llm-guard — ориентир» | **Архивирован.** |
| «scrubadub — ориентир» | **Мёртв 3 года**, старый URL 404. |
| «Presidio — ориентир» | Живой (11 345 звёзд), но **переехал** в `data-privacy-stack/`. |
| «Замена на похожие фейки не сделана в продуктах» | **Опровергнуто дважды:** `dataiku/kiji-proxy` («realistic dummy values») и `redmadrobot/pii-guard` (псевдонимы с полом и восстановлением падежа). |
| «Русский — наше отличие» | **Ослаблено.** Есть `redmadrobot-rnd/pii-guard` (Apache-2.0, 20 типов, обратимо), плюс 20+ русских репозиториев-шлюзов, плюс три российских коммерческих вендора. |
| Цель p95 25–50 мс на CPU | **Подтверждена как реальное отличие:** `privacy-filter.cpp` даёт 140 мс на 512 токенов; `packyme` уложился бы (0.46 мс на 2 КБ), но **выбросил имена**; AGIMA держит бюджет «сотни мс»; у redmadrobot замеров нет. |
