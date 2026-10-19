# Provenance: empirical evidence on defects in AI-generated code (failure taxonomy)

- **Date:** 2026-10-19
- **Rounds:** 3 evidence rounds (4 parallel researcher streams + lead direct search), then 3 review passes (citation → reviewer → re-review → closure check)
- **Slug:** `ai-code-defect-taxonomy`

## Sources

- **Consulted:** ~70 distinct records (papers, preprints, official docs, vendor reports, one GitHub issue tracker).
- **Accepted:** 56, listed in the deliverable's `## Sources` section with title, year, venue, and direct URL. All 56 URLs were HTTP-checked by the verification pass: **53 × 200, 2 × 403** (`openai.com`, JS-rendered — content verified by browser-style fetch), **0 × 404**. The lead independently confirmed 25 key arXiv abs URLs return 200.
- **Peer-reviewed:** Tambon et al. (EMSE 2025); Spracklen et al. (USENIX Sec 2025); Pearce et al. (IEEE S&P 2022); Fu et al. (TOSEM 2025); Zhong et al. (ICLR 2026); CodeHalu (AAAI 2025); MAST (NeurIPS 2025); BigCodeBench (ICLR 2025); Perry et al. (CCS 2023); Paradis et al. (ICSE-SEIP 2025); Wang et al. (ACL 2026); SecureVibeBench (ACL 2026); OctoBench (ACL 2026); Majgaonkar et al. (ICSE 2026); Coppola et al. (ESEM 2026); Chen et al. (ICSE 2026); Gao & Peng (ICSE 2026); HAL (ICLR 2026); SWE-Bench Illusion (ICSE-SEIP 2026); Popescu et al. (MSR 2026); Hora & Robbes (MSR 2026).
- **Preprints:** 20, flagged as `preprint` in every table row that uses them.
- **Industry telemetry / vendor:** GitClear (2025, 2026), DORA 2024/2025, Stack Overflow 2025, Uplevel 2024, CodeRabbit 2025, METR blog, Terminal-Bench leaderboard. All labelled; none used as the sole basis for a prevalence claim after revision (revised per reviewer finding M7).
- **Rejected / not used as evidence:**
  - ACM DL full text for `10.1145/3716848` returned a 257-char stub → used the arXiv version instead.
  - GitClear 2024 report PDF (S3 URL returned HTML, not a parseable PDF) → 2024 per-year churn table is **not confirmed by direct read**; only the 2025 report's revised series is used.
  - Three Exa queries returned zero results; superseded by direct arXiv/OpenAI fetches.
  - Semantic Scholar returned HTTP 429 throughout (anonymous pool rate-limited); OpenAlex used as fallback.
  - A low-quality "HonestCode-Bench" blob-hosted PDF and a vendor "SILT-RP-008" self-report surfaced in search and were **excluded** as unverifiable.

## Verification

**Verification: PASS WITH NOTES**

- Citation pass: 56 sources, 0 orphan citations, 0 duplicate source lines; 6 claims marked `[UNVERIFIED]`.
- Review pass 1 (FATAL/MAJOR/MINOR): 1 FATAL + 9 MAJOR + 19 MINOR. All addressed. The FATAL was a ~10× denominator inversion in the "Debt Behind the AI Boom" cell (89.3% is the share of *issues* that are smells, not the share of commits). Fixed and re-verified against arXiv:2603.28592 §5.1–5.2.
- Review pass 2: confirmed F1 fixed; found **3 new MAJORs introduced by my own fixes** (N1 magnitude descriptor, N2 Veracode denominator double-count, N3 an unsourced `$0.08–$32` cost range) + 4 MINORs. All fixed.
- Review pass 3 (closure): 6 of 7 fixed, 1 cosmetic count mismatch remained; fixed after the pass.
- The citation pass itself **timed out at 30 min after writing the file**; the 48 KB artifact was intact and complete (ends with source [56]), so no work was lost.

### Notes and residual limits

1. **The `[UNVERIFIED]` apparatus was unreliable.** Four of six markers were false positives: the citation pass used the criterion "absent from the internal research notes," which is not the same as "unsupported by the literature." All six were re-resolved against primary sources and the apparatus replaced with a resolution log.
2. **The functional-correctness stream (T1) failed twice** (`Stream ended without finish_reason`) and was not retried; §2.1 rests on lead direct search only, which is narrower than the brief. Its uncovered scope is listed in the deliverable's §6 rather than dropped.
3. **The security stream failed once** and was re-run as a bounded single-researcher task, which succeeded. Lead-owned security verification was completed independently and does not depend on it.
4. **Version sensitivity:** arXiv:2507.02976 and arXiv:2509.22097 changed headline numbers between versions. The report's figures match the **current** versions; the S8/S9 pair is footnoted because it draws on v1 and v5 of the same record with different best-agent results.
5. **Not independently re-derived:** coverage claims in §7 ("30+ and 20+ sources"), and several 2026 venue attributions that support evidence grades rather than numbers.

## Artifacts

- **Deliverable:** `outputs/ai-code-defect-taxonomy.md` (332 lines, 54,243 bytes, 5 tables, 56 sources)
- **Plan:** `outputs/.plans/ai-code-defect-taxonomy.md`
- **Briefs:** `outputs/.plans/ai-code-defect-taxonomy-T1.md` … `-T4.md`
- **Workflow script:** `outputs/.plans/ai-code-defect-taxonomy-workflow.js`
- **Research files:**
  - `outputs/.drafts/ai-code-defect-taxonomy-research-agents.md` (67 KB; T3, completed)
  - `outputs/.drafts/ai-code-defect-taxonomy-research-field.md` (33 KB; T4, completed)
  - `outputs/.drafts/ai-code-defect-taxonomy-research-security.md` (27 KB; T2, completed on relaunch)
  - `outputs/.drafts/ai-code-defect-taxonomy-research-direct.md` (19 KB; lead, incl. the T1 gap-fill)
  - `outputs/.drafts/ai-code-defect-taxonomy-sources.md` (lead fallback source list)
- **Drafts:** `-draft.md` → `-cited.md` → `-revised.md` (= final candidate)
- **Verification:** `-verification.md` (pass 1), `-verification-2.md` (pass 2), `-verification-3.md` (pass 3)
- **Failed children:** T1 `95acc8c4-4803-4030-8915-452003692e04` and T2 `cad39e7c-d385-4e45-a816-8896c44920eb`, both `Stream ended without finish_reason`
