# Provenance: Does writing tests before code improve outcomes with LLM coding agents?

- **Date:** 2026-10-18
- **Slug:** `llm-agents-test-first`
- **Rounds:** 4 evidence rounds (3 parallel researcher briefs + 1 lead-owned direct round), then 1 verification round, then 1 review round, then 1 revision round.
- **Mode:** deep research with subagent decomposition (3 `researcher` + 1 `verifier` + 1 `reviewer`).

## Sources

**Consulted:** ~70 distinct sources across four evidence bases (academic papers/preprints,
benchmark and evaluation reports, vendor documentation, practitioner blogs). 44 are cited in the
final report as [S1]–[S44]; the remainder were screened and excluded (see "rejected" below).

**Accepted and cited (44):**
- Academic / preprints (28): S1 Mathews & Nagappan (arXiv:2402.13521); S2 TDD-Agent
  (2608.16742); S3 TENET (2509.24148); S4 TDD-Bench Verified (2412.02883); S5 Scaling TDD
  (2602.03557); S7 TDD survey (2609.12012); S8 Artificial or Just Artful (2512.21028);
  S9 UTBoost (2506.09289); S10 SWE-Bench+ (2410.06992); S11 Risk of coding before testing
  (2607.05139); S12 SpecBench (2605.21384); S13 TDFlow (2510.23761); S15 SWE-Bench-Verified
  memory probe (2512.10218); S16 DeepSWE (2607.07946); S18 LessLeak-Bench (2502.06215);
  S19 Misguidance effect (2607.22883); S21 RHB (2605.02964); S22 TRACE (2601.20103);
  S24 TDDev (2605.17242); S36 GAI4-TDD (10.1145/3828553); S37 TiCoder (2404.10100);
  S38 Santos et al. family of experiments (10.1007/s10664-020-09895-8); S39 Rethinking the
  Value of Agent-Generated Tests (2602.07900); S40 TDAD (2603.17973); S42 ImpossibleBench
  (2510.20270); S43 EvilGenie (2511.21654); plus CodeT (2207.10397) and AgentCoder
  (2312.13010) which are discussed in the research files as excluded-from-the-core-claim.
- Evaluation / benchmark / audit reports (5): S14 METR reward hacking post; S44 METR o3
  evaluation; S17 SWE-bench official description; S41 Berkeley RDI benchmark-exploit audit;
  S23 SWE-Skills-Bench (2603.15401).
- Vendor / practitioner (11): S6 Böckeler/Thoughtworks; S25/S26 Anthropic (archived + current);
  S27 Cursor; S28 VS Code; S29 OpenAI Codex; S30 GitHub Copilot; S31 Cognition Devin;
  S32 JetBrains; S33 Simon Willison; S34 Kent Beck; S35 Martin Fowler.

**Rejected / not used:**
- S20 OpenAI *Why we no longer evaluate SWE-bench Verified* — HTTP 403; the 59.4 % figure is
  marked unverified and excluded from the argument.
- JetBrains Junie docs (`junie.jetbrains.com/docs/`) — unreachable.
- Marco Kotrotsos Medium post — paywalled; only the intro was readable, so no claims attributed.
- Reddit / Hacker News threads on TDD with Claude Code — JS-rendered or blocked; no grassroots
  dissent is represented beyond the sources above. Recorded as a coverage gap.
- AlphaCodium (2401.08500), SWT-Bench (2406.12952), TestGen-LLM — screened; no test-order
  ablation, so excluded from the core claim.
- `feynman alpha search` / alphaXiv paper tools — not available in this session's tool list
  (alphaXiv not logged in per the counter brief); paper discovery was done via the arXiv API,
  Semantic Scholar and OpenAlex instead.

## Tooling failures (recorded, degraded mode used)

- `web_search` default provider (Exa) returned HTTP 429 (rate limit) for the whole run.
  Brave (no API key), Tavily (no key), SearXNG (no base URL) all failed. DuckDuckGo worked for
  two queries, then returned HTTP 202 (bot check). Workaround: arXiv API via `curl`,
  `feynman_science_database_search`, and direct `fetch_content`.
- `openai.com` pages returned HTTP 403 to the fetcher; `doi.org` → `dl.acm.org` returned 403
  (Crossref metadata resolved instead).
- No `alpha_search` / `alpha_get_paper` in the tool list, despite the system prompt referencing
  alpha tools.
- Consequence: academic coverage is stronger than web/blog coverage; the T2 brief compensated
  via `anysearch` and direct fetches.

## Verification

**Verification: PASS WITH NOTES.**

Performed:
1. Lead read the decisive papers in full from locally parsed PDFs (S1, S2, S4, S5, S6, S7, S8,
   S11, S23, S39) rather than relying on subagent summaries; subagent-reported numbers that
   overlap with the lead's reads were cross-checked and matched.
2. Lead independently verified: METR's hack table and its prompt-intervention table; the SWE-bench
   official protocol text; the archived Anthropic April-2025 TDD section; Cursor's recipe;
   Fowler's "with me" quote; Böckeler's TLDR and token table; SWE-Skills-Bench Table 2; and the
   abstracts of SpecBench, RHB, DeepSWE, ImpossibleBench, EvilGenie, TDAD and arXiv:2602.07900.
3. `verifier` subagent checked every cited URL and traced every number. Outcome: no dead links
   except S20 (403, already flagged); one quantitative claim failed to trace as written
   (a "deltas over the strongest reasoning baseline" sentence, corrected in the revision); one
   research-file column conflation found and resolved (TDD-Bench Llama 8.2 fail-to-pass vs 7.8
   tddScore); several title/venue variants flagged and fixed in the revision.
4. `reviewer` subagent ran a verification pass: 2 FATAL, 9 MAJOR, 8 MINOR findings. Both FATALs
   and all MAJORs were fixed in `-revised.md`: the executive summary was rewritten to stop
   overclaiming; the second-order arithmetic was recomputed per model (revealing that the
   ordering contrast reverses on Qwen); the provenance claim was made accurate per source
   (full-text vs abstract-level reads now labelled); the two strongest omitted controlled
   results (arXiv:2602.07900 and TDAD) were added, which also corrected a false statement that
   only two studies report significance tests; Böckeler's n=2 caveat was moved inline; METR's
   own 1–2 % aggregate figure was added next to the 30.4 % family-specific figure; the
   "closest thing to a law" overclaim was removed; the TDDev "like-for-like" label was
   withdrawn; missing `(inference)` markers were added to causal/analogy language; and
   TDD-Agent's confounds were restated where its table is used. MINORs were largely fixed
   (taxonomy count standardised to six, pp/% conflations corrected, source attributions fixed,
   mermaid labels aligned, mutation-score spread added, preprints marked).

Notes / residual uncertainty:
- Several load-bearing numbers come from **abstract-level** reads (S10, S12, S15, S16, S18,
  S21, S22, S24, S40, S42, S43). Each is marked inline, and §8.7 warns that abstracts in this
  literature are unreliable (documented case: S24's abstract vs body disagree by ~20 pp).
- S20's 59.4 % figure is unverified and excluded.
- The Berkeley RDI exploit claims [S41] are a third-party audit of other teams' harnesses; they
  are cited as evidence about harness design, not about model capability.
- No independent replication or re-execution was performed; no author contact.
- The single practitioner-run comparison [S6] remains n=2 per arm with a single LLM judge; the
  report now states this inline wherever the result is used.

## Artifacts

- Plan: `outputs/.plans/llm-agents-test-first.md`
- Researcher briefs: `outputs/.plans/llm-agents-test-first-T1.md`, `-T2.md`, `-T3.md`
- Workflow script: `outputs/.plans/llm-agents-test-first-workflow.js`
- Research files: `outputs/.drafts/llm-agents-test-first-research-academic.md`,
  `-research-practitioner.md`, `-research-counter.md`, `-research-direct.md`
- Draft: `outputs/.drafts/llm-agents-test-first-draft.md`
- Cited: `outputs/.drafts/llm-agents-test-first-cited.md`
- Reviewer report: `outputs/.drafts/llm-agents-test-first-verification.md`
- Revision: `outputs/.drafts/llm-agents-test-first-revised.md`
- Patch notes: `notes/llm-agents-test-first-patch-notes.md`
- Raw PDFs: `notes/pdfs/` (2402.13521, 2412.02883, 2608.16742, 2602.03557, 2512.21028,
  2609.12012, 2603.15401, 2602.12670, 2607.05139, 2602.07900)
- Final: `outputs/llm-agents-test-first.md`
