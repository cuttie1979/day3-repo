# Provenance: Is the claim that a defect costs 100x more to fix later supported by evidence?

- **Date:** 2026-10-19
- **Rounds:** 1 research round, 3 parallel researcher subagents (origin / empirical / critique) +
  1 verifier pass + 1 reviewer pass + 1 lead revision pass
- **Sources consulted:** ~35 across the three research lanes (papers, primary PDFs, official
  reports, investigative gists/journalism, vendor and standards documents)
- **Sources accepted (24, cited in the final artifact):**
  1. Boehm & Basili 2001, *IEEE Computer* — doi:10.1109/2.962984
  2. Boehm 1976, *IEEE Trans. Computers*
  3. Shull et al. 2002, IEEE Metrics — doi:10.1109/METRIC.2002.1011343
  4. Menzies, Nichols, Shull & Layman 2017, *EMSE* 22(4) — arXiv:1609.04886 / doi:10.1007/s10664-016-9469-x
  5. Stecklein et al. 2004 (NASA/INCOSE), NTRS 20100036670
  6. Hamill & Goseva-Popstojanova 2017, *IST* — doi:10.1016/j.infsof.2017.01.002
  7. Langenfeld, Post & Podelski 2016, REFSQ — doi:10.1007/978-3-319-30282-9_10
  8. Tassey / RTI for NIST, Planning Report 02-3, 2002
  9. Anderson, *The Register*, 2021-07-22
  10. Wayne, "I ****ing hate Science," 2021
  11. Bossavit, IBM Systems Science Institute gist
  12. Bossavit, Wolverton Ratios gist
  13. Bossavit, "Degrees of intellectual dishonesty" gist
  14. Bossavit, *Leprechauns of Software Engineering*, ch. 10
  15. Lee, sicpers.info retraction, 2012
  16. Lemos, *Dark Reading*, 2024
  17. CISA CSAC, *Secure By Design*, 2024-10-11
  18. Wolverton 1974, *IEEE Trans. Computers* — doi:10.1109/T-C.1974.224002
  19. Boehm 1981, *Software Engineering Economics* (metadata only)
  20. Boehm 1987, "Industrial software metrics top 10 list" (metadata only)
  21. Beck, *Extreme Programming Explained*, ch. 5
  22. Ambler, "Examining the Agile Cost of Change Curve"
  23. Larman & Basili 2003, *Computer* — doi:10.1109/MC.2003.1204375
  24. Cotroneo, Improta & Liguori 2025, ISSRE — doi:10.1109/ISSRE66568.2025.00035
- **Sources rejected / not usable:**
  - Radatz 1981, DTIC ADA104249 — `Blocked` (apps.dtic.mil maintenance page)
  - iSixSigma "Defect Prevention" page — search snippet no longer present on live page
  - Original "IBM Systems Sciences Institute" study — **not found**; no surviving primary
    research document (the institute existed as an IBM training body)
  - IBM "Santa Teresa / FSD, OT1/OT2" cost ratios — no matching primary source located
  - Boehm 1981 book and Boehm 1987 column — paywalled/not fetched; cited metadata-only, used to
    support no number
  - Menzies et al. §6.3 "2.16–4.37" passage — retained as UNVERIFIED (internally inconsistent
    with §5.6 / Figure 13)
- **Verification:** PASS WITH NOTES
  - Mandatory verifier pass completed: added inline citations, corrected five unsupported
    claims, checked all DOIs via Crossref, and re-read load-bearing primary texts.
  - Reviewer pass completed: **no FATAL findings**; 4 MAJOR and 13 MINOR findings. All MAJOR
    findings were fixed in `outputs/.drafts/defect-cost-100x-claim-revised.md`, and all MINOR
    findings were addressed or explicitly recast (see "Removed Unsupported Claims" §6–14 in the
    final artifact). MAJOR findings are also carried into Open Questions §7.1–7.3.
  - Notes: (a) two source URLs (O'Reilly Beck ch. 5; Dark Reading) return HTTP 403 to automated
    fetches but are live in browsers — flagged in the artifact; (b) Bossavit ch. 10 was read from
    a secondary full-text copy (pagination may differ) — flagged; (c) the provenance conclusion
    is single-investigator (Bossavit) and the severity-conditional claim is single-sourced
    (Shull et al. 2002, raw data unpublished) — both flagged explicitly in the artifact.
- **Plan:** outputs/.plans/defect-cost-100x-claim.md
- **Research files:**
  - outputs/.plans/defect-cost-100x-claim-T1.md, -T2.md, -T3.md (per-researcher briefs)
  - outputs/.plans/defect-cost-100x-claim-workflow.js (workflow)
  - outputs/.drafts/defect-cost-100x-claim-research-origin.md
  - outputs/.drafts/defect-cost-100x-claim-research-empirical.md
  - outputs/.drafts/defect-cost-100x-claim-research-critique.md
  - outputs/.drafts/defect-cost-100x-claim-draft.md (lead synthesis)
  - outputs/.drafts/defect-cost-100x-claim-cited.md (verifier output)
  - outputs/.drafts/defect-cost-100x-claim-verification.md (reviewer output)
  - outputs/.drafts/defect-cost-100x-claim-revised.md (final candidate, post-review fixes)
