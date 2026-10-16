# Журнал проверок (поток «репозитории/датасеты/рынок»)

**Дата:** 2026-10-16
**Среда:** текущая дата по `date -u` — `Fri Oct 16 02:35:16 AM UTC 2026`.
**Ограничения среды:**
- `api.github.com` — анонимный лимит 60 запросов/час, **исчерпан** (`remaining: 0`). Массовые проверки выполнены через HTML `github.com` и фиды `commits.atom`, у которых лимита нет.
- `pip install` — запрещён (PEP 668), `--break-system-packages --user` — отказ по правам (`Errno 13`). Поэтому `Faker` не запускался.
- Часть корпоративных сайтов недоступна: `HTTP=000` (соединение не устанавливается).

---

## 1. Методы проверки

| Что проверялось | Команда / эндпоинт | Что даёт |
|---|---|---|
| Существование репозитория | `curl -sL -o /dev/null -w "%{http_code}" https://github.com/<owner>/<repo>` | HTTP 200 = существует; 404 = нет |
| Канонический URL после переименования | тот же запрос с `-w "%{url_effective}"` | выявляет переезды репозиториев |
| Звёзды, лицензия, архивность | HTML `github.com/<owner>/<repo>`, регулярки по встроенному JSON: `"stargazerCount":(\d+)`, `"license":\{"spdxId":"([^"]*)"`, `"isArchived":(true\|false)` | точные значения |
| Дата последнего коммита | `https://github.com/<owner>/<repo>/commits.atom`, тег `<updated>` | ISO-дата последнего коммита |
| Топ-язык | `https://img.shields.io/github/languages/top/<owner>/<repo>.json` | язык и его доля |
| Метаданные датасета | `https://huggingface.co/api/datasets/<id>` | `id`, `likes`, `downloads`, `lastModified`, `cardData.license`, `tags`, `gated` |
| Размер датасета | `https://datasets-server.huggingface.co/size?dataset=<urlencoded-id>` | число строк |
| Содержимое датасета | `https://datasets-server.huggingface.co/first-rows?dataset=<id>&config=<c>&split=<s>` | реальные строки |
| Сырые README | `https://raw.githubusercontent.com/<owner>/<repo>/<branch>/README.md` (перебор `main`/`master`) | содержимое |
| Массовый поиск репозиториев | `https://api.github.com/search/repositories?q=...` (пока был лимит) | до 20 результатов на запрос |

## 2. Скрипты в этом каталоге

| Файл | Назначение | Запускался? |
|---|---|---|
| `gh_check.py` | Точечная проверка списка репозиториев: звёзды, лицензия, архивность, дата последнего коммита, топ-язык. Читает файл со списком, пишет TSV в stdout и JSON. | да |
| `gh_search.py` | 20 поисковых запросов к GitHub с паузами 7 с (уважение лимита 10/мин). Пишет `gh_search_all.json`. | да |
| `repos.txt`, `repos_key.txt`, `repos_extra.txt`, `repos_last.txt` | Списки репозиториев по группам. | да |
| `faker_ru_checksum_probe.py` | **Измеряет**, какую долю реквизитов `Faker('ru_RU')` проходит проверку контрольного ключа. | **нет** — `faker` не устанавливается в песочнице |
| `gen_ru_checksums.py` | Генератор **валидных** российских реквизитов (ИНН-10, ИНН-12, СНИЛС, БИК, расчётный счёт, паспорт) + самопроверка на публичных векторах. | **да, `EXIT=0`, все проверки пройдены** |

## 3. Артефакты данных

| Файл | Содержимое |
|---|---|
| `gh_search_all.json` | **276 уникальных репозиториев** из 20 поисковых запросов, с полями `full_name`, `stars`, `pushed`, `license`, `lang`, `archived`, `desc`, `created` |
| `gh_search_out.txt` | Сырой вывод 20 поисковых запросов (сгруппировано по запросам) |
| `gh_results.json` | Первая точечная проверка (30 репозиториев) |
| `gh_key_results.json` | Ключевая выборка (79 репозиториев): Presidio, OpenAI privacy filter, Kiji, Packyme, LLM Guard, SurrogateShield, русские конкуренты, Natasha, RuCoCo, FactRuEval |
| `gh_extra_results.json` | Дополнительно: GLiNER Guard, GLiNER, Faker, mimesis, kz-identifiers, Natasha-компоненты |
| `gh_last_results.json` | FactRuEval-2016, NER-Evaluation, gitleaks, betterleaks, hmbench |

## 4. Ключевые проверки, изменившие выводы

| Проверка | Результат | Как |
|---|---|---|
| `microsoft/presidio` существует? | **Переехал** → `data-privacy-stack/presidio` | `curl -sIL -w "%{url_effective}"` |
| `scrubadub/scrubadub` существует? | **404.** Живёт как `LeapBeyond/scrubadub`, последний коммит **2023-09-01** (мёртв 3 года) | HTTP 404 на старом, 200 на новом |
| `protectai/llm-guard` жив? | **Архивирован** (`"isArchived":true`), 3 216 звёзд | HTML-флаг |
| `RussianNLP/RuCoCo` существует? | **404.** Реальные репозитории: `vdobrovolskii/rucoco` (2022-12-25), `dialogue-evaluation/RuCoCo-2023` (2023-03-24) — оба мёртвы | HTTP 404 + поиск |
| Natasha жива? | Обёртка `natasha/natasha` обновлялась 2026-10-14, но **все компоненты** (`razdel`, `navec`, `slovnet`, `nerus`, `yargy`) — **2023-07-24** | atom-фиды пяти репозиториев |
| Есть ли русские датасеты с типами ПД? | **Да, ~20.** Лучший — `redmadrobot-rnd/pii_benchmark`: 2 841 строка, 21 тип, MIT | HF API + каталог `pii-arena/docs/datasets.md` |
| `wolframko/russian-pii-66k` годится для реквизитов? | **Нет.** Машинный перевод: IBAN `RU45FOMI8350305641395` в поле ACCOUNTNUM, `29 37 900581` как IDCARDNUM | `datasets-server/first-rows` |
| `piiarena.com` живой? | Да, **HTTP 200** на `/leaderboard` | `curl -sL` |
| Цену Nightfall можно узнать? | Медиана **$25 000/год** (41 сделка) | `vendr.com/marketplace/nightfall`, HTTP 200 |
| `calypsoai.com` самостоятелен? | **Нет** — редирект на `f5.com/products/ai-guardrails` | `curl -sL -w "%{url_effective}"` |
| `private-ai.com` самостоятелен? | **Нет** — 301 на `getlimina.ai` (ребрендинг в Limina) | то же |
| Кто ещё делает PII-шлюзы по-русски? | **StarGuard AI, Jay Guard, KodikRouter, CorpSoft24** + открытый `redmadrobot-rnd/pii-guard` | поиск + страницы продуктов |

## 5. Что не удалось проверить (и почему)

| Что | Причина | Как проверить |
|---|---|---|
| Долю валидных реквизитов у `Faker('ru_RU')` | `faker` не устанавливается в песочнице | `python3 faker_ru_checksum_probe.py` на машине с Faker |
| Латентность `redmadrobot-rnd/pii-guard` | Не опубликована | Собрать Docker-образ, замерить p50/p95 на своём корпусе |
| Цены российских вендоров | Не публикуются | Запрос коммерческого предложения |
| Метрики российских вендоров | Не публикуются | — |
| Содержимое `aigov-redact`, `piiguardrails` (PyPI) | Не разбиралось | `pip download` + чтение |
| Список языков Lakera Guard | Сайт `HTTP=000` | Ручная проверка из другой сети |
| Русский срез `REDACT-PII-Benchmark` | Доступ gated (`gated=auto`) | Принять условия на HF |
| Датасет «Gareev» | **Не найден** ни на HF, ни в GitHub-поиске | Уточнить у постановщика, что имелось в виду |
