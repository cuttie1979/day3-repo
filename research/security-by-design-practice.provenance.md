# Provenance: Security by Design in Practice

- **Date:** 19 October 2026
- **Topic:** What security-by-design looks like in practice — threat modelling, OWASP ASVS/SAMM, least privilege, secrets management, dependency hygiene, SBOM
- **Slug:** `security-by-design-practice`
- **Rounds:** 5 (plan; parallel evidence gathering; lead synthesis/draft; citation + URL verification; adversarial review; lead revision)
- **Deliverable:** `outputs/security-by-design-practice.md` (8,363 words, 99 sources)

## Plan and workflow artifacts

- Plan: `outputs/.plans/security-by-design-practice.md`
- Researcher briefs: `outputs/.plans/security-by-design-practice-T1.md` … `-T4.md`
- Workflow script: `outputs/.plans/security-by-design-practice-workflow.js`

## Research files used

- `outputs/.drafts/security-by-design-practice-research-tm.md` (threat modelling; researcher, completed)
- `outputs/.drafts/security-by-design-practice-research-identity.md` (least privilege + secrets; researcher, completed)
- `outputs/.drafts/security-by-design-practice-research-supplychain.md` (dependency hygiene + SBOM; researcher, completed)
- `outputs/.drafts/security-by-design-practice-research-standards.md` (ASVS/SAMM/SSDF/CRA; researcher child failed on search-provider rate limits, completed by the lead from direct official fetches — see plan decision log)
- `outputs/.drafts/security-by-design-practice-lead-checks.md` (independent URL sweep and version spot-checks)
- `outputs/.drafts/security-by-design-practice-urlcheck.txt` (raw 93-URL sweep)
- `outputs/.drafts/security-by-design-practice-draft.md` (lead draft, pre-citation)
- `outputs/.drafts/security-by-design-practice-cited.md` (final candidate; equals the delivered file)
- `outputs/.drafts/security-by-design-practice-verification.md` (adversarial verification report + lead dispositions)

## Sources consulted

- Four research files plus lead fetches, covering roughly 120+ distinct URLs across: OWASP ASVS 5.0.0 (release, model docs, CSV), OWASP SAMM 2.0, NIST SP 800-218 and SP 800-207/800-53, CISA Secure by Design and pledge, OWASP Top 10:2025, EU CRA (Commission pages, EUR-Lex Articles 13/64 and Annex I), threat-modelling methodology and empirical literature, cloud identity/secrets documentation (AWS, Microsoft, Google, Kubernetes, SPIFFE, HashiCorp, OWASP), documented incident post-mortems, SBOM standards/regulators (NTIA 2021, CISA 2026, SPDX, CycloneDX, VEX, SLSA, Sigstore), US policy (EO 14028, OMB M-22-18, M-26-05), and empirical SBOM/dependency studies.

## Sources accepted

- **99** numbered sources in the delivered report, all cited at least once (checked by citation/source consistency scan; no orphan sources, no undefined citations).
- Verified primary documents include: ASVS 5.0.0 release (GitHub API: 2025-05-30), SAMM v2.0 (GitHub API: 2020-02-03), CISA 2026 SBOM Minimum Elements (PDF: published 29 July 2026), OMB M-26-05 (rescission text extracted from the official PDF), SLSA v1.2, SPDX 3.0.1, CycloneDX 1.7.2 (2026-09-17), CRA Articles 13(8)/(9)/(24), 64(2), Recital 77, Annex I Part II(1).

## Sources rejected, blocked, or downgraded

- **Dead link replaced:** a news link for the 2024 Snowflake/Mandiant campaign returned HTTP 404; replaced by Mandiant/Google Cloud's primary report.
- **Automated retrieval blocked:** EUR-Lex OJ PDF route returned HTTP 202 with an empty body (ELI/CELEX HTML routes were used); 403 bot-blocks for Microsoft SDL (two pages), Microsoft MSRC, LastPass, Medium (Birsan), ACM Digital Library, and DTIC. Publisher-blocked metadata was cross-checked via Crossref/arXiv where possible.
- **Not read in full:** Fucci et al. 2024 (SAMM assessment study) — metadata only, so no efficacy claim drawn; several threat-modelling papers were used via abstracts/metadata as noted in the research file.
- **Single-source / directional and labelled as such:** 47.7% over-privilege figure (arXiv preprint); 92.0% SBOM false-positive rate (four-repository case study on one pipeline); SEI STRIDE "great variability" claim (DTIC-hosted source blocked); GitGuardian secret-sprawl statistics (vendor-reported); CISA pledge progress claims (self-reported).
- **Not independently verified:** no official ASVS↔SAMM↔SSDF mapping document was located; no peer-reviewed evidence linking threat modelling, SBOM, SLSA, or SAMM maturity to reduced breaches was located.

## Verification

- **Verification: PASS WITH NOTES**
- Adversarial verification pass outcome: **no FATAL findings**; 3 MAJOR evidence-integrity findings (URL-sweep misstatement, CRA citation-chain misdescription, over-scoped Zhou et al. figure) and 9 MINOR findings. All MAJOR and low-risk MINOR findings were corrected before delivery; dispositions are logged in `security-by-design-practice-verification.md`.
- Residual notes: two cited claims remain single-source and could not be re-read directly (SEI/DTIC STRIDE variability; the 2018 STRIDE-recall result, confirmed only indirectly through the 2022 replication). The CRA Annex I excerpt is cross-referenced from EUR-Lex HTML and corroborating reproductions because the OJ PDF route blocks automated retrieval. Published CRA dates and the US rescission were confirmed first-party.
- Checks run: citation/source consistency (99/99); URL sweep (84×200, 2×202, 6×403, 1×404 resolved by replacement, plus one DTIC 403); version/date spot-checks via GitHub API and official pages; numeric spot-checks against primary PDFs/CSVs (ASVS counts, CISA SBOM restructuring, ENISA percentages, Zhou/Xia figures).

## Residual risk

- Standards, cloud guidance, and regulations in this space change; every version/date claim is tied to the access date 2026-10-19.
- Vendor documentation is used for prescribed configuration; it is labelled where relevant and is not treated as outcome evidence.
- The report's proposed gates, 90-day plan, and prioritization logic are the author's synthesis, not findings from a single source.
