# Provenance: engineering document types (ADR, design doc/RFC, runbook, threat model, changelog, PR description)

- **Date:** 2026-10-18
- **Rounds:** 1 (direct-mode research run; 15 web/database query attempts, 33 source fetches, 1 URL status sweep)
- **Scale decision:** direct search, lead-owned. No researcher subagents. Rationale: the question is a
  definitional "what is each for" taxonomy, and the scale rules forbid inflating "what is X" explainers into
  multi-agent surveys. Recorded in `outputs/.plans/engineering-document-types.md` before any evidence work.
- **Sources consulted:** 33 fetch attempts (24 distinct cited sources plus 9 that failed or proved
  unnecessary). Full list and per-source extraction notes in
  `outputs/.drafts/engineering-document-types-research-direct.md`.
- **Sources accepted (24):** Nygard 2011; adr.github.io; Zimmermann 2023 (ozimmer.ch); Ubl, *Design Docs at
  Google*; IETF *About RFCs*; RFC 2026; rust-lang/rfcs; PEP 1; PagerDuty *What is a Runbook?*; Google SRE
  Workbook *On-Call*; AWS Systems Manager runbooks; Azure Automation runbook types; OWASP *Threat Modeling*;
  Threat Modeling Manifesto 2020; Microsoft SDL Threat Modeling Tool; W3C *Threat Modeling Guide*;
  Shostack et al. arXiv:2511.08295; Keep a Changelog v1.1.0; GitHub *Pull requests*; Google *Writing good CL
  descriptions*; Google *The Standard of Code Review*; Aghajani et al. ICSE 2019 (author preprint + OpenAlex
  metadata); Rost et al. 2013 (metadata only); Google SRE *Postmortem Culture*.
- **Sources rejected:** `gnu.org/prep/standards/html_node/Change-Logs.html` (connection failure — so Keep a
  Changelog's characterisation of GNU's standard is reported second-hand);
  `rfc-editor.org/rfc/rfc2026.html` (HTML extraction failure — plain-text `.txt` used instead);
  `docs.github.com/.../about-pull-request-reviews` (HTTP 404 — replaced by the GitHub PR overview page);
  `atlassian.com/incident-management/on-call/runbooks` (HTTP 404);
  `martinfowler.com/articles/agile-threat-modelling.html` and `sre.google/sre-book/emergency-response/`
  and `sre.google/sre-book/being-on-call/` (fetched, not used — no unique claim needed from them);
  `learn.microsoft.com/.../threat-modeling-tool-threats` (fetched, superseded by the SDL overview page);
  six DuckDuckGo results on runbook staleness and threat-model theatre
  (devopsaitoolkit.com, dbdeployer.com, thegrayhaven.org, sync-o.io, upstat.io, beyond.minimumcd.org,
  lorikeetsecurity.com, security-threat.com) — **rejected as low-quality/SEO content with no identifiable
  authority or method**.
- **Verification:** PASS WITH NOTES
  - Performed: URL reachability sweep (`outputs/.drafts/engineering-document-types-urlcheck.txt`); source
    cross-check against the fetch log; verbatim re-check of every block quote; numeric re-check of the
    Aghajani figures against the saved preprint text; negative-claim re-check via text search on the Google
    SRE page; full self-review with 6 findings raised and all 6 fixed
    (`outputs/.drafts/engineering-document-types-verification.md`).
  - **Not performed: independent citation-to-claim verification.** No `verifier` or `reviewer` subagent was
    run, because direct-mode scale rules prohibit them for this topic. A human or a future larger-scale run
    should re-check the mapping of claims to sources before relying on this document for anything consequential.
  - Note: `https://www.w3.org/TR/threat-modeling-guide/` returns HTTP 403 to `curl` (bot protection) but was
    successfully retrieved and read by `fetch_content` (113,018 chars) on 2026-10-18.
- **Plan:** `outputs/.plans/engineering-document-types.md`
- **Research files:**
  - `outputs/.drafts/engineering-document-types-research-direct.md` (query log + extracted claims)
  - `outputs/.drafts/engineering-document-types-urlcheck.sh`, `...-urlcheck.txt` (URL sweep)
  - `outputs/.drafts/engineering-document-types-draft.md`
  - `outputs/.drafts/engineering-document-types-cited.md`
  - `outputs/.drafts/engineering-document-types-verification.md`
  - `outputs/.drafts/engineering-document-types-revised.md` (final candidate)
- **Final artifact:** `outputs/engineering-document-types.md`
- **Known limitations carried into the final artifact:** the §1 artifact map is a synthesis, not a quoted
  standard; the quantitative documentation-quality figures come from one ICSE 2019 study of software
  documentation generally, not of these six artifact types; two sources are vendor/expert-opinion content
  and are labelled as such inline.
