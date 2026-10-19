# Provenance: What quality gates do technology companies use for professional software development, and which have evidence behind them?

- **Date:** 2026-10-18
- **Rounds:** 4 (three parallel researcher passes over four sub-domains; then verifier citation pass, reviewer pass 1, reviewer pass 2, reviewer pass 3, with lead-applied corrections between passes)
- **Slug:** software-quality-gates
- **Final artifact:** outputs/software-quality-gates.md (md5 `1c7c6058325a176db459d885f8be3dcc`, 44,191 bytes)
- **Final candidate it was copied from:** outputs/.drafts/software-quality-gates-revised.md

## Verification

- **Verification: PASS WITH NOTES**
- All FATAL and MAJOR findings from reviewer passes 1 and 2 were fixed and confirmed fixed by reviewer pass 3 (802/803 items confirmed; pass 3 found no FATAL or MAJOR defect).
- Residual MINOR findings from pass 3 were also applied: the §4.1 table note now names the specific client-rendered page; the 68.6% row states its cohort label ("All Respondents"); the Elder et al. counts carry a source-internal-inconsistency caveat; source 69's non-resolving `10.5555` DOI was replaced with the ACM DOI plus a verified author-hosted PDF; "satellite decommission" corrected to "rack (satellite) decommission".
- **Notes / limits:**
  - The HTTP reachability sweep covered 34 URLs from an early candidate list, **not** a systematic check of all 75 final sources. Several sampled URLs were later dropped. Most final entries were instead checked by direct page/PDF fetch during the verification passes. This is disclosed in the artifact's Verification Notes.
  - Three cited hosts return HTTP 403 to automated clients (iso.org, Medium-hosted Netflix Tech Blog, ACM Digital Library). Each was separately confirmed to exist; they are bot-blocks, not dead links.
  - Client-rendered statistics were a recurring failure mode: the Stack Overflow 68.6% figure was initially reported as absent by text-only extraction and wrongly re-attributed; it was restored to the primary page after inspecting the raw chart markup. Four JetBrains testing-type percentages remain **excluded** because they could not be confirmed from any retrievable source.
  - Source [59] (Elder et al. 2022) reports internally inconsistent vulnerability counts (824/823 for SAST; 185/229 for EMPT); the artifact states the figures and flags the inconsistency.
  - The Stack Overflow chart series used for the 68.6% figure is labeled "All Respondents"; whether it is also the professional-developer subset could not be resolved. Disclosed in the artifact.

## Sources consulted

- 4 researcher evidence files (T1 frameworks/definitions; T2 code-level gates; T3 release/ops gates; T4 empirical evidence and adoption), each carrying its own numbered source table with URLs.
- Direct primary-source fetches by the lead for fact-checking: TechTarget; Sonar; IBM Cloud docs (failed); Microsoft SDL Practices and Resources; Microsoft DevSecOps SAST blog; MSR-TR-2016-27 PDF; Google SRE Book launch checklist and release engineering; SRE Workbook (Ch. 2, Ch. 10, Ch. 16, Appendix B, Appendix C); AWS Builders' Library; Microsoft Learn Azure DevOps gate docs; Uber Engineering; JetBrains developer-ecosystem page (raw HTML); JetBrains Qodana page; Stack Overflow 2024 survey (landing, professional-developers, methodology); CD Foundation; DORA; Gao et al. PDF; Habib & Pradel PDF; Elder et al. PDF; arXiv 2103.05451 (ar5iv HTML); NIST IR 8397; NIST SP 500-341 publication page.
- Crossref REST API: 4 DOI lookups (Khan et al.; Heričko & Šumak; NIST SP 500-341; one candidate source later dropped).

## Sources accepted

- **75 numbered references** in the final artifact. Sample of the load-bearing ones:
  - Uzunova et al. 2024 (CEUR-WS Vol. 3845) — definition of quality gates.
  - Bacchelli & Bird 2013 (ICSE); Sadowski et al. 2018 (ICSE-SEIP); McIntosh et al. 2015 (ESE) — code review evidence.
  - Gao et al. 2017 (ECOOP); Khan et al. 2022 (IEEE TSE); Habib & Pradel 2018 (ASE); NIST SATE VI (SP 500-341) — static analysis and type checking.
  - Soares et al. 2022 (ESE); Davila & Nunes 2021 (JSS); Rafi et al. 2012 (AST) — systematic reviews.
  - Google SRE Book / Workbook; AWS Builders' Library; Microsoft Learn; Uber Engineering — release and operations gates.
  - Stack Overflow 2024; JetBrains 2024; JetBrains Qodana 2024; CD Foundation/SlashData 2024; DORA 2024 — adoption data.

## Sources rejected

- **IBM Cloud Docs "Using gate policies to ensure quality"** — initially reachable (HTTP 200) but failed content extraction (HTTP 404 on re-fetch); quoted text unverifiable and the product is discontinued. Source slot 2 replaced with a verbatim-verified Sonar definition.
- **Netflix Tech Blog, "Deploying the Netflix API"** — HTTP 403 to automated fetching; no alternative source verified in this pass. Netflix was consequently dropped from the list of companies cited for progressive-rollout practice (Google, AWS, Microsoft, Uber, Spotify remain).
- **Clark, T. (2025), "The Future of CI/CD" (Apress)** — added by an intermediate revision as the supposed origin of the 68.6% figure, then removed once the figure was confirmed present on the primary Stack Overflow page. Only its Crossref metadata had ever been inspected (which cannot establish page content).
- **Four JetBrains figures (unit 78%, integration 63%, end-to-end 48%, performance 36%)** — excluded: not present in the report page's retrievable content and not confirmed by secondary coverage, which states only direction of change.
- **Reintroduced and then removed again during review:** the "SDL-derived standards call them security gates" clause; the 2015/2018 postmortem years; the "39% consider manual testing effective" relabel; the "outperform any single gate" phrasing (downgraded to a labelled inference).

## Research files

- outputs/.plans/software-quality-gates.md — plan, task ledger, verification log, decision log
- outputs/.plans/software-quality-gates-T1.md … -T4.md — researcher briefs
- outputs/.plans/software-quality-gates-workflow.js — parallel researcher workflow
- outputs/.drafts/software-quality-gates-research-T1.md … -T4.md — researcher evidence files
- outputs/.drafts/software-quality-gates-draft.md — first synthesis draft
- outputs/.drafts/software-quality-gates-cited.md — verifier citation pass (later found to contain a regressed revision)
- outputs/.drafts/software-quality-gates-verification.md — reviewer pass 1
- outputs/.drafts/software-quality-gates-verification2.md — reviewer pass 2
- outputs/.drafts/software-quality-gates-verification3.md — reviewer pass 3 (final)
- outputs/.drafts/software-quality-gates-linkcheck.md — link/metadata/quotation check log
- notes/software-quality-gates-linkcheck-cmds.txt — reproducible commands
- outputs/.drafts/software-quality-gates-revised.md — final candidate (source of the delivered file)
