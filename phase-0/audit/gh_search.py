#!/usr/bin/env python3
"""Broad GitHub repo search across many queries, with rate-limit friendly sleeps."""
import json
import time
import urllib.error
import urllib.parse
import urllib.request

UA = {"User-Agent": "Mozilla/5.0", "Accept": "application/vnd.github+json"}

QUERIES = [
    "obezlichivanie",
    "обезличивание",
    "персональные данные",
    "privacy gateway LLM",
    "anonymize prompts LLM",
    "pii proxy LLM",
    "pii masking gateway",
    "redact prompts before sending to LLM",
    "pseudonymization LLM",
    "topic:pii-detection",
    "topic:anonymization",
    "topic:data-anonymization",
    "topic:pii",
    "topic:de-identification",
    "pii detection russian",
    "russian ner pii",
    "llm privacy proxy",
    "masking personal data llm",
    "reversible anonymization llm",
    "pii guardrails",
]

seen = {}
for q in QUERIES:
    url = ("https://api.github.com/search/repositories?q="
           + urllib.parse.quote(q) + "&sort=stars&order=desc&per_page=20")
    for attempt in range(3):
        try:
            req = urllib.request.Request(url, headers=UA)
            with urllib.request.urlopen(req, timeout=30) as r:
                d = json.load(r)
            break
        except urllib.error.HTTPError as e:
            if e.code in (403, 429):
                time.sleep(20)
                d = None
            else:
                d = None
                break
        except Exception:
            d = None
            break
    if not d:
        print("## QUERY FAILED: %s" % q)
        time.sleep(7)
        continue
    print("## QUERY: %s (total=%s)" % (q, d.get("total_count")))
    for x in d.get("items", []):
        fn = x["full_name"]
        rec = dict(full_name=fn, stars=x["stargazers_count"], pushed=x["pushed_at"],
                   license=(x.get("license") or {}).get("spdx_id"),
                   lang=x.get("language"), archived=x.get("archived"),
                   desc=(x.get("description") or "")[:220],
                   created=x.get("created_at", "")[:10])
        if fn not in seen:
            seen[fn] = rec
        print("   %-52s %6s %s %-12s %s | %s" % (
            fn, rec["stars"], rec["pushed"][:10], rec["lang"], rec["license"], rec["desc"][:110]))
    time.sleep(7)

with open("gh_search_all.json", "w") as f:
    json.dump(list(seen.values()), f, ensure_ascii=False, indent=1)
print("\nTOTAL UNIQUE: %d" % len(seen))
