#!/usr/bin/env python3
"""Batch verifier for GitHub repos: exact stars, license, archived, last commit, top language.
Uses github.com HTML (no rate limit) + commits.atom + shields.io for language.
"""
import json
import re
import sys
import urllib.error
import urllib.request
from concurrent.futures import ThreadPoolExecutor

UA = {"User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 Chrome/120 Safari/537.36"}


def get(url, timeout=25):
    req = urllib.request.Request(url, headers=UA)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return r.status, r.read().decode("utf-8", "ignore"), r.geturl()
    except urllib.error.HTTPError as e:
        return e.code, "", url
    except Exception as e:  # noqa
        return 0, "ERR:%s" % e, url


def check(repo):
    out = {"repo": repo, "status": 0, "stars": None, "license": None,
           "archived": None, "last_commit": None, "lang": None, "desc": None,
           "canonical": None, "error": None}
    st, html, final = get("https://github.com/" + repo)
    out["status"] = st
    if st != 200:
        out["error"] = "HTTP %s" % st
        return out
    out["canonical"] = final.replace("https://github.com/", "")
    m = re.search(r'"stargazerCount":(\d+)', html)
    if m:
        out["stars"] = int(m.group(1))
    m = re.search(r'"license":\{"spdxId":"([^"]*)","name":"([^"]*)"\}', html)
    if m:
        out["license"] = m.group(2) if m.group(1) in ("NOASSERTION", "") else m.group(1)
    m = re.search(r'"repo":\{[^}]*?"isArchived":(true|false)', html)
    if not m:
        m = re.search(r'"isArchived":(true|false)', html)
    if m:
        out["archived"] = m.group(1) == "true"
    m = re.search(r'"description":"(.*?)(?<!\\)"', html, re.S)
    if m:
        d = m.group(1).encode().decode("unicode_escape", "ignore")
        out["desc"] = re.sub(r"\s+", " ", d)[:400]
    # forks
    m = re.search(r'"forkCount":(\d+)', html)
    if m:
        out["forks"] = int(m.group(1))
    # last commit via atom feed
    st2, atom, _ = get("https://github.com/%s/commits.atom" % out["canonical"])
    if st2 == 200:
        m = re.search(r"<updated>([^<]+)</updated>", atom)
        if m:
            out["last_commit"] = m.group(1)
    # language via shields
    st3, sh, _ = get("https://img.shields.io/github/languages/top/%s.json" % out["canonical"])
    if st3 == 200:
        try:
            j = json.loads(sh)
            out["lang"] = "%s %s" % (j.get("label"), j.get("message"))
        except Exception:
            pass
    return out


def main():
    repos = [l.strip() for l in open(sys.argv[1]) if l.strip() and not l.startswith("#")]
    with ThreadPoolExecutor(max_workers=8) as ex:
        res = list(ex.map(check, repos))
    for r in res:
        print("\t".join(str(r.get(k)) for k in
                        ("repo", "status", "stars", "forks", "last_commit", "license",
                         "archived", "lang", "canonical", "desc")))
    with open(sys.argv[2], "w") as f:
        json.dump(res, f, ensure_ascii=False, indent=1)


if __name__ == "__main__":
    main()
