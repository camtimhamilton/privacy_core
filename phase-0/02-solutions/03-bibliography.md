# Библиография с URL (полная)

Приложение к [README.md](README.md), раздел 10. 129 источников; каждое утверждение отчёта опирается
на один из них. Все страницы открывались напрямую 2026-10-16.

## 10.1 Presidio

1. Project transition (переход к community ownership) — <https://presidio.dataprivacystack.org/project_transition/>
2. Документация, поддерживаемые сущности — <https://presidio.dataprivacystack.org/supported_entities/>
3. Raw-источник списка сущностей — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/supported_entities.md>
4. Языки и NLP-движки — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/analyzer/languages.md>
5. spaCy/Stanza NLP engine — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/analyzer/nlp_engines/spacy_stanza.md>
6. Anonymizer: операторы и разрешение перекрытий — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/docs/anonymizer/index.md>
7. Дефолтный NLP-конфиг — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default.yaml>
8. Дефолтный реестр рекогнайзеров — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/presidio_analyzer/conf/default_recognizers.yaml>
9. CHANGELOG (случайная соль в hash, ORGANIZATION в игнор-листе, чистка CLI, AHDS) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/CHANGELOG.md>
10. `presidio-analyzer` pyproject (зависимости, 2.2.364, MIT) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/presidio-analyzer/pyproject.toml>
11. LICENSE (MIT, Presidio Contributors) — <https://raw.githubusercontent.com/data-privacy-stack/presidio/main/LICENSE>
12. Репозиторий — <https://github.com/data-privacy-stack/presidio>
13. PyPI `presidio-analyzer` 2.2.364 — <https://pypi.org/pypi/presidio-analyzer/json>
14. Presidio на PyPI (мета-пакет) — <https://pypi.org/project/presidio/>

## 10.2 LLM Guard

15. Репозиторий (архивирован) — <https://github.com/protectai/llm-guard>
16. README с предупреждением об архивации — <https://raw.githubusercontent.com/protectai/llm-guard/main/README.md>
17. Документация Anonymize-сканера (сущности, модели, `use_faker`, бенчмарки p95) — <https://protectai.github.io/llm-guard/input_scanners/anonymize/>
18. PyPI `llm-guard` 0.3.16 — <https://pypi.org/pypi/llm-guard/json>
19. Модели Protect AI на HF (не поддерживаются) — <https://huggingface.co/protectai>
20. Список сущностей Anonymize в коде — <https://github.com/protectai/llm-guard/blob/main/llm_guard/input_scanners/anonymize.py>
21. Кастомные regex-паттерны — <https://github.com/protectai/llm-guard/blob/main/llm_guard/resources/sensisitive_patterns.json>

## 10.3 Portcullis и SurrogateShield

22. Portcullis, AAAI-25 (страница статьи, DOI, авторы, абстракт, «96x») — <https://ojs.aaai.org/index.php/AAAI/article/view/32088>
23. Portcullis PDF — <https://ojs.aaai.org/index.php/AAAI/article/view/32088/34243>
24. Portcullis DOI — <https://doi.org/10.1609/aaai.v39i1.32088>
25. SurrogateShield, arXiv:2606.29567 — <https://arxiv.org/abs/2606.29567>
26. SurrogateShield PDF — <https://arxiv.org/pdf/2606.29567>
27. SurrogateShield HTML — <https://arxiv.org/html/2606.29567v1>
28. SurrogateShield, репозиторий — <https://github.com/sherwinvishesh/SurrogateShield>
29. SurrogateShield README (архитектура, бенчмарки, ограничения) — <https://raw.githubusercontent.com/sherwinvishesh/SurrogateShield/main/README.md>
30. SurrogateShield, PyPI — <https://pypi.org/project/surrogateshield/>
31. SurrogateShield, сайт — <https://sherwinvishesh.github.io/SurrogateShield/>
32. SurrogateShield в EmergentMind (аннотация) — <https://www.emergentmind.com/papers/2606.29567>

## 10.4 Русскоязычные инструменты

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
49. Corus (датасеты factRuEval, ne5, bsnlp, gareev) — <https://github.com/natasha/corus>
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

## 10.5 Остальные open-source

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

## 10.6 LLM-прокси и privacy firewall

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

## 10.7 СНГ

91. **llm-pii-proxy-showcase** (Казахстан, MIT) — <https://github.com/MihaRS2/llm-pii-proxy-showcase>
92. llm-pii-proxy-showcase README (KZ-рекогнайзеры, солёные плейсхолдеры, SSE, fail-closed) — <https://raw.githubusercontent.com/MihaRS2/llm-pii-proxy-showcase/main/README.md>
93. **kazpii-bench** (бенчмарк KZ/RU) — <https://github.com/k1ddy/kazpii-bench>
94. **yashirgich** (узбекский/русский) — <https://github.com/bekzod/yashirgich>
95. **pii-obfuscator** (ru, без лицензии) — <https://github.com/SergeyK88i/pii-obfuscator>
96. **ru-pii-detection-system** (ru, MIT) — <https://github.com/feuerstrahll/ru-pii-detection-system>

## 10.8 Модели PII на HuggingFace (проверенные метаданные)

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
107. `nvidia/Nemotron-PII` (датасет, внешний замер SurrogateShield) — <https://huggingface.co/datasets/nvidia/Nemotron-PII>
108. `redmadrobot-rnd/pii_benchmark` (MIT) — <https://huggingface.co/datasets/redmadrobot-rnd/pii_benchmark>
109. `Powpowpow23/ru-pii-ner-data` — <https://huggingface.co/datasets/Powpowpow23/ru-pii-ner-data>

## 10.9 Проприетарные

110. Purview, список всех SIT-определений (вкл. 4 русских) — <https://learn.microsoft.com/en-us/purview/sit-sensitive-information-type-entity-definitions>
111. Purview, Russian taxpayer identification number — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-taxpayer-identification-number>
112. Purview, Russian Passport Number (Domestic) — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-domestic>
113. Purview, Russian Passport Number (International) — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-passport-number-international>
114. Purview, Russia physical addresses — <https://learn.microsoft.com/en-us/purview/sit-defn-russia-physical-addresses>
115. Google SDP, де-идентификация (infoTypes, кастомные детекторы) — <https://docs.cloud.google.com/sensitive-data-protection/docs/concepts-de-identification>
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

## 10.10 Внутренние документы проекта

128. `docs/00-charter.md` — рабочий документ (инварианты I1–I10, профили задержки, модель данных)
129. `docs/20-entity-registry.md` — реестр типов сущностей и контрольные суммы
