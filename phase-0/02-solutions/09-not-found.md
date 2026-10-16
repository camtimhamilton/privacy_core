# Что не найдено и что требует уточнения

Приложение к [README.md](README.md), раздел 11.

## НЕ НАЙДЕНО (искал, артефакта нет)

- **Код Portcullis** — GitHub Search `portcullis LLM privacy` = 0, `portcullis bytedance` = 0;
  arXiv API по `all:Portcullis AND all:LLM` пуст; на странице статьи OJS только PDF и Video/Poster.
- **`anonymize-it`** — на PyPI **404**; GitHub Search выдал только посторонние проекты
  (`nucleuscloud/neosync`, `arx-deidentifier/arx` и др.).
- **`Oban` как инструмент обезличивания** — не найден в этом качестве. В экосистеме есть
  Elixir-библиотека `Oban` — очередь фоновых задач, к приватности отношения не имеет.
- **`OpenAnonymiser` как отдельный проект** — GitHub Search `openanonymiser OR openanonymizer` = 0.
  Существует только `ConductionNL/anonymiq-light` («OpenAnonymiser Light»), нидерландский.
- **Русские рекогнайзеры в Presidio** — ни одного при ≈130 predefined-типах.
- **Русские PII-типы в AWS Bedrock Guardrails** — 0 из 32.
- **Русский в AWS Comprehend** — отсутствует в списке поддерживаемых языков Comprehend вообще.
- **Открытые обезличиватели для Беларуси (УНП), Узбекистана, Украины** — не найдены; есть только
  валидаторы внутри мультиязычных пакетов (напр. `python-stdnum` в зависимостях scrubadub `[ДОК]`).
- **Открытый PII-детектор от Sberbank** — не найден. Модели `sberbank-ai/*` на HF существуют и
  используются как base-модели (`ai-forever/ruBert-base` — база для `rubert-base-pii-ner`).
- **Studio Ousia в контексте PII/анонимизации русского** — не найдено.
- **RuCoCo на HuggingFace** — поиск `rucoco` пуст. Наш 00-charter §2.6 ссылается на RuCoCo
  (arXiv 2206.04925) для кореференции, но датасета под этим именем в HF нет → **открытый риск для
  ворот ContextGuard**.
- **Открытый датасет с разметкой кореференции для русского** — не подтверждён (см. выше).
- **Замеры задержки Presidio от авторов** — не публикуются.
- **Русские контекстные слова в Presidio** — ни одного (только en/es/it/pl/de/ko/sv/th/tr/fi и др.).
- **Русские локали `surrogate_ahds`** — только `en-US` в дефолтах.

## ТРЕБУЕТ УТОЧНЕНИЯ ПОСТАНОВЩИКОМ

- **«SLIM»** — не установил, о каком проекте речь. Возможные кандидаты: SPARQL-связанные проекты,
  `slim-ai` entity linking, «SLIM» как метод дистилляции. В контексте PII/русского ничего не нашёл.
- **«SurrogateShield»** — если имелось в виду что-то другое, кроме `arXiv:2606.29567` +
  `github.com/sherwinvishesh/SurrogateShield`: других артефактов под этим именем поиск не дал
  (GitHub Search `SurrogateShield` → 1 результат).
- **«Yandex Tomita / Yandex NER»** — Tomita найден и разобран (MPL-2.0, мёртв с 2020-11-16).
  **Отдельного** публичного «Yandex NER» нет. Если имелся в виду Yandex Cloud NLP API — это
  проприетарный облачный сервис, и в список проприетарных ориентиров он не входил.
- **«Studio Ousia»** — в списке был, но связи с PII/русским не нашёл. Уточнить, что имелось в виду.
- **«SLIM / NEREL»** — NEREL найден (датасет русского NER, несколько реплик на HF с разными
  лицензиями; схема богаче PER/LOC/ORG, но категорий нашего реестра там нет). Что такое SLIM в
  постановке — неясно.
