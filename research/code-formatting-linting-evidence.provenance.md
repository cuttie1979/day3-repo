# Provenance: Is there evidence that automated code formatting and linting improve outcomes?

- **Date:** 2026-10-18
- **Rounds:** 3 — (1) lead direct sweep for definitions/history/vendor claims; (2) 3 parallel
  researcher lanes (formatting, linting, costs); (3) verifier citation pass + reviewer
  verification pass + one consolidated revision.
- **Sources consulted:** 59 numbered works survive into the final report. The three researcher
  lanes proposed 84 source entries in total (T1 25, T2 28, T3 31) with substantial overlap
  (e.g. Sadowski et al. 2018 was read by both T2 and T3; Sadowski et al. 2015 by T2 and T3).
  Five seed records were additionally re-resolved by the lead through OpenAlex (Miara 1983,
  Sadowski 2018, Tricorder 2015, Gao 2017, Vassallo 2019) and seven key numbers were re-derived
  from primary PDFs (see `outputs/.drafts/code-formatting-linting-evidence-lead-checks.md`).
- **Sources accepted:** the 59 numbered sources in `outputs/code-formatting-linting-evidence.md`.
  Full-text read: ~25. Abstract/TLDR-only and flagged in place: Miara 1983, Kesler 1984,
  Morzeck 2023, Hanenberg 2024, Ribeiro 2023, Moser 2007, and the ASE 2024 Python type-error
  paper. Peer-reviewed vs preprint is stated per source.
- **Sources rejected:**
  - **Vendor ROI claims** — SonarSource ("strong ROI") and Testomat.io ("production bugs
    decrease 60% to 80%") were read but excluded as evidence because no method, sample, or
    dataset is offered. They appear only in the "asserted, not measured" discussion.
  - **Metadata-only, not used for claims** — Aloraini et al. 2019 (JSS) and Rahman et al. 2022
    (EMSE): records verified, abstracts/full texts not retrievable; no claim rests on them.
  - **Inaccessible** — Zampetti et al. MSR 2017 (closed access); the Meta Infer NFM 2015 paper's
    yield figures do not exist in the accessible text and were therefore not used.
  - **Secondary/practitioner only** — the Wikipedia "Lint (software)" page was used solely for
    the naming anecdote and is not cited in the final report; the ESLint issue #17814 thread
    (practitioner claim about blame pollution) was dropped in favour of the raw
    `.git-blame-ignore-revs` artifacts.
  - **Preprints** — two arXiv preprints are used (CodeQL false-positive refinement 2026;
    TypeScript bug taxonomy 2026) but are explicitly labelled "not peer-reviewed" rather than
    rejected, because no peer-reviewed equivalent was found.
- **Verification:** **PASS WITH NOTES**
  - Reviewer verdict: **0 FATAL, 5 MAJOR, 14 MINOR**. All 5 MAJOR and all 14 MINOR findings
    were applied in `outputs/.drafts/code-formatting-linting-evidence-revised.md`, which is the
    delivered file.
  - Notes / residual limits:
    1. Miara et al. 1983 full text was bot-blocked (HTTP 403). Its direction is reported from
       two independent secondary reports, and the report says so. This is the weakest
       supported load-bearing citation.
    2. The SAST 12.7% / 70.9% detection figures come from one ESEC/FSE 2023 study; no
       independent replication was located. Disclosed at first use and in the caveats.
    3. No measured study of formatter-induced churn, blame degradation, or bisect breakage
       exists in the retrieved literature; the report treats this as a gap, not a result.
    4. No study with a proper denominator measures production-incident reduction from linting.
    5. ACM (`10.1145/…`) and IEEE (`10.1109/…`) DOI targets return HTTP 403/202 to automated
       clients. Records were resolved via Crossref metadata and, where available, open mirrors.
    6. Semantic Scholar's anonymous pool returned HTTP 429; OpenAlex's anonymous daily budget
       was exhausted mid-run; alphaXiv was not authenticated. The default web provider (Exa)
       was rate-limited for the entire session.
- **Plan:** `outputs/.plans/code-formatting-linting-evidence.md`
- **Research files:**
  - `outputs/.drafts/code-formatting-linting-evidence-research-formatting.md` (T1)
  - `outputs/.drafts/code-formatting-linting-evidence-research-linting.md` (T2)
  - `outputs/.drafts/code-formatting-linting-evidence-research-costs.md` (T3)
  - `outputs/.drafts/code-formatting-linting-evidence-research-direct.md` (lead)
  - `outputs/.drafts/code-formatting-linting-evidence-lead-checks.md` (independent re-checks)
  - `outputs/.drafts/code-formatting-linting-evidence-verification.md` (reviewer report)

## Corrections applied after verification

| # | Correction | Where |
|---|---|---|
| 1 | Trautsch p-value `7.3e-08` → exact `7.2559e-08`; effect size `0.269` → `0.26943` | Q3.4 |
| 2 | SAST 12.7%/70.9% reattributed from "EASE 2024" to Li et al., ESEC/FSE 2023; Semgrep numbers to Bennett et al., EASE 2024 | Q3.1 table, Q4, Sources [51]-[53] |
| 3 | `.git-blame-ignore-revs` units harmonised to "excluded entries (non-comment hash lines)" for scipy, django and gecko-dev | Exec summary, Q2, verdict table |
| 4 | Kesler 1984 design marked "three groups (inferred from TLDR)" | Q1 table |
| 5 | Internal "Verifier corrections" changelog removed from the reader-facing report | (moved here) |
| 6 | "6–12×" → "6–17×", with explicit statement that it is the inverse of the cited fix rates and that fixed-warning ≠ prevented-defect | Q3.2 |
| 7 | O4 "Decisively shaped by workflow integration" → "Plausibly dominated by workflow integration", confidence Medium | Verdict table |
| 8 | Global access assurance replaced with a precise abstract/TLDR statement | How to read this |
| 9 | "75% of finds" → "535 of 709 true defects (75%)" | Q3.1 |
| 10 | "1,740 merges" → "1,740 real-world file merges" | Q2 |
| 11 | O6 "unquantified" → "vendor-reported, not independently measured" (30% to >300%) | Verdict table |
| 12 | Mermaid edge split: 51% inert suppressions vs 61–68% unactionable in sampled suppressions | Diagram |
| 13 | Null results no longer cited to topical papers; reworded as search results | Q4 |
| 14 | "independent aggregation" → "odds-ratio aggregation of three further experiments" | Exec summary |
| 15 | Citation cluster for the SLR quote trimmed from [9, 11, 14, 15] to [11, 14, 15] | Exec summary |
| 16 | Miara direction noted as "2-space per SLR; a second lane reads 2–4 spaces"; quote marked as abstract-level | Q1 |

## Verification checks performed

- Every URL in the final report resolved or was recorded as a publisher bot-block with DOI
  metadata verification (verifier pass).
- Seven load-bearing quantities re-derived by the lead from primary PDFs (Habib & Pradel
  27/594; Marcilio 421,976/36,974/8.77%/18.99; Trautsch 0.895/1.677/p/effect size; Hu 7,357/
  50.8%; Meta 70%/0%; Google 10%/3,954/16%; ESLint 30–300%).
- `.git-blame-ignore-revs` counts reproduced by `grep -cE '^[0-9a-fA-F]{7,40}'` on the raw files.
- Seven arXiv IDs resolved with matching titles via `arxiv.org/abs/<id>`.
- Verdict table (11 rows) checked against body text for strength-of-language mismatches; one
  (O4) was corrected.
