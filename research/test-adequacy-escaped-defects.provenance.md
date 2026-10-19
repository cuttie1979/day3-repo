# Provenance: What test adequacy measures actually predict escaped defects — line coverage, mutation score, or seeded-fault detection?

- **Date:** 2026-10-18
- **Rounds:** 3 research/verification rounds — (1) four parallel `researcher` subagents over four evidence clusters; (2) lead synthesis plus independent lead verification of the load-bearing primary PDFs; (3) mandatory `verifier` citation pass, then three sequential `reviewer` passes with lead-applied corrections between each.
- **Slug:** `test-adequacy-escaped-defects`
- **Final artifact:** `outputs/test-adequacy-escaped-defects.md` (md5 `1ee2dcc874841247db9273c5487d775c`, 52,502 bytes, ~7,300 words)
- **Final candidate it was copied from:** `outputs/.drafts/test-adequacy-escaped-defects-revised.md` (identical md5)

## Verification

**Verification: PASS WITH NOTES**

- Reviewer pass 1 (`-verification.md`) raised 1 FATAL + 5 MAJOR + 8 MINOR. The FATAL was a bottom-line claim that
  escaped-defect rates are "driven more by age and churn than by any adequacy metric", attributed to Cotroneo et al.
  2026, which measured no adequacy metric. Fixed.
- Reviewer pass 2 (`-verification2.md`) **confirmed the FATAL resolved** and found 3 new MAJORs introduced by the fix
  (misattributed convergence between two studies; an unsearched question presented as a negative finding; an
  unrecorded lead-query list) plus 5 MINORs. All fixed; the lead query log was written to
  `-lead-checks.md` §Lead search log to make the enumeration auditable.
- Reviewer pass 3 (`-verification3.md`) confirmed all fixes, raised one scoping MAJOR (a §7 sentence that still fused
  the searched question with the unsearched gap) and 5 MINORs. All fixed.
- **Notes / limits:**
  - The report's central negative — that no study measures mutation score or seeded-fault detection against
    escaped/post-release defects — rests on an **uneven** search record, and the report says so. Only the
    escaped-defect strand (T4) enumerated its queries; the coverage, mutation and seeded-fault strands did not. The
    lead's own query log, including the two failed provider attempts, is recorded in `-lead-checks.md`.
  - One question is deliberately left as an explicit **gap rather than a negative**: whether a capture–recapture
    estimate of remaining defects has ever been validated against realised field defects. No query targeted it.
  - Four load-bearing numbers are single-source and are flagged in place: Vista block coverage `t = −3.5, p = 0.001`
    (Mockus et al. 2009); the Kochhar et al. 2017 null (`ρ = −0.059`); Google's 70% fault-coupling rate (Petrović
    et al. 2021); the Zhao et al. 2026 divergence (a preprint on LLM-generated suites only).
  - Several load-bearing industrial sources are abstract-only in this work: Antinyan & Staron 2019 (paywalled),
    Tengeri et al. 2016 (abstract only; effect sizes not obtained), Bowes et al. 2016 (abstract/indexed text).
    Rwemalika et al. 2019 is green open access; the researcher's parse failed and the figures follow the research
    file, checked against the open-access copy during review.
  - Publisher DOI landing pages for ACM, Wiley and IEEE return HTTP 403/202 to automated clients. Each cited source
    has at least one 200-status direct URL (author copy, arXiv, or institutional repository); the publisher links are
    cited for identification and marked gated.

## Blocked checks

- **`feynman alpha` (alphaXiv) paper tools: BLOCKED.** `feynman alpha status` returns "alphaXiv not logged in", so
  `alpha_search` / `alpha_get_paper` / `alpha_ask_paper` were unavailable to the lead and to all four researchers. No
  alpha-tool source is cited; the equivalent work was done with OpenAlex, Semantic Scholar (intermittent HTTP 429),
  DuckDuckGo search and direct PDF retrieval.
- **Default web-search provider: BLOCKED.** The default (Exa) returned HTTP 429 rate-limit errors; the `brave`
  provider had no API key. DuckDuckGo was used for URL discovery.
- **Coverage-threshold-as-policy (did mandating a threshold reduce escaped defects?): NOT FOUND.** No study located.
- **ISO/IEC/IEEE 29119-4 verbatim test-adequacy definition: BLOCKED** (iso.org and standards.ieee.org return 403).
- **IEEE 1044-2009 full text: snippet-level only** (scope text obtained; standard inactivated 2020-03-05).

## Sources consulted

- Four researcher evidence files, each with its own numbered source table and URL list:
  `outputs/.drafts/test-adequacy-escaped-defects-research-T1.md` (coverage; 15 sources),
  `-T2.md` (mutation score; 20), `-T3.md` (seeded faults; 21), `-T4.md` (escaped defects, head-to-head, confounds; 28).
- Lead primary-source verification: 5 PDFs downloaded with `curl` and text-extracted locally
  (`/tmp/tad/*.pdf`, `.txt`) — Papadakis et al. 2018 (ICSE), Chekam et al. 2017 (ICSE), Inozemtseva & Holmes 2014
  (ICSE), Just et al. 2014 (FSE), Petrović et al. 2021 (ICSE/arXiv), plus Mockus et al. 2009 (ESEM), the Google 2018
  and Google 2019 PDFs, and Facebook 2021 (arXiv:2010.13464).
- One document-screenshot check to confirm a sign that text extraction had dropped: Inozemtseva & Holmes Table 3,
  HSQLDB `τ = −0.35`.
- Index/metadata APIs: OpenAlex REST (used for Pecorelli et al. 2019's abstract and several DOI cross-checks);
  Semantic Scholar Graph API (rate-limited); arXiv abstract pages.

## Sources accepted

- **36 numbered references** in the final artifact. The load-bearing ones:
  - Inozemtseva & Holmes 2014 (ICSE) — coverage vs effectiveness, size control.
  - Papadakis et al. 2018 (ICSE) — mutation score vs real-fault detection, size confound.
  - Just et al. 2014 (FSE) — mutants as real-fault substitute; 17% uncoupled.
  - Chekam et al. 2017 (ICSE) — statement/branch/weak/strong mutation fault revelation.
  - Kochhar, Thung & Lo 2015 (SANER); Kochhar et al. 2017 (IEEE T-Rel) — coverage vs real bugs and vs post-release
    defects.
  - Mockus, Nagappan & Dinh-Trong 2009 (ESEM) — coverage vs post-verification defects, Avaya + Windows Vista.
  - Pecorelli et al. 2019 (ESEC/FSE); Gren & Antinyan 2019; Chioteli et al. 2019; Antinyan & Staron 2019 —
    null/negative coverage results.
  - Andrews et al. 2005 (ICSE) and Andrews et al. 2006 (IEEE TSE) — seeded vs real fault validity.
  - Schuler & Zeller 2013; Gopinath et al. 2014; Papadakis et al. 2016 (ISSTA) — equivalence, fault realism, validity
    threats.
  - Petrović & Ivanković 2018 and Petrović et al. 2021 (Google); Beller et al. 2021 (Facebook) — industrial scale.
  - NIST SATE V (SP 500-326) — synthetic vs real defect detection.
  - Cotroneo et al. 2026 (preprint); Rwemalika et al. 2019 (ICSME) — escaped-defect characterisations.
  - Sobreira et al. 2018 (SANER) — Defects4J patch anatomy.
  - Zhao, Zhou & Cohen 2026 (arXiv:2607.22880, preprint) — divergent replication on LLM-generated suites.
  - Jain et al. 2023 (ISSRE) — coverage–mutation gap.
  - ISTQB glossary (escaped defect, fault seeding, DDP); IEEE 1044; DORA.

## Sources rejected

- **"83,100" as a count of Chekam et al.'s test suites** — a researcher claim; verified against the primary to be the
  **line count of Coreutils**, their largest subject (Table I column "Largest Subject"; Table II row
  `Coreutils 83,100 …`). Excluded from the report.
- **"Joda Time → essentially zero when size is controlled"** — a researcher claim whose only source is Inozemtseva &
  Holmes Figure 3, which could not be text-extracted and was not read from a screenshot. Excluded as unverified.
- **Vista `t = −3.5, p = 0.001` attributed to the Avaya logistic model** — a researcher misattribution; the statistic
  belongs to the Microsoft/Vista linear model (Table 6). Corrected.
- **Go mutant survival rate (14.0%)** — the counts table is garbled by text extraction in the Google 2018 PDF
  ("1'050.7"). Not quoted.
- **A derived "4–15×" fall in the mutation–fault correlation** — no source reports a ratio; replaced with the source's
  own ranges and a correctly bounded 1.75×–15× comparison.
- **"age and churn beat any adequacy metric" for escaped defects** — asserted in the first draft; removed because no
  cited study measured an adequacy metric. This was the reviewer's FATAL finding.

## Research files

- `outputs/.plans/test-adequacy-escaped-defects.md` — plan, task ledger, verification log, decision log
- `outputs/.plans/test-adequacy-escaped-defects-T1.md` … `-T4.md` — researcher briefs
- `outputs/.plans/test-adequacy-escaped-defects-workflow.js` — parallel researcher workflow
- `outputs/.drafts/test-adequacy-escaped-defects-research-T1.md` … `-T4.md` — researcher evidence files
- `outputs/.drafts/test-adequacy-escaped-defects-lead-checks.md` — lead verification of primary PDFs, the wave-1
  conflict-resolution table, and the lead search log
- `outputs/.drafts/test-adequacy-escaped-defects-linkcheck.md` — URL reachability sweep and the primary-source
  re-verification of the industrial numbers
- `outputs/.drafts/test-adequacy-escaped-defects-draft.md` — pre-citation synthesis
- `outputs/.drafts/test-adequacy-escaped-defects-cited.md` — verifier citation pass
- `outputs/.drafts/test-adequacy-escaped-defects-verification.md` — reviewer pass 1
- `outputs/.drafts/test-adequacy-escaped-defects-verification2.md` — reviewer pass 2
- `outputs/.drafts/test-adequacy-escaped-defects-verification3.md` — reviewer pass 3 (final)
- `outputs/.drafts/test-adequacy-escaped-defects-revised.md` — final candidate (source of the delivered file)
