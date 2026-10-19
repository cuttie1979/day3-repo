# Is the "a defect costs 100x more to fix later" claim supported by evidence?

**Verdict: As literally stated and universally applied, no.** The "100x" figure is a
citation-trail artifact whose provenance chain cannot be traced to a measurement, and the
largest empirical study of the delayed-issue effect finds no consistent escalation [4]. As a
*conditional, order-of-magnitude heuristic for severe defects in large release-based systems*, a
roughly 100x worst case is partially supported [3, 5] — but only under conditions the popular
claim omits, and with the central source (Boehm & Basili 2001) itself explicitly warning that
small non-critical systems are more like 5:1 [1].

Three separable things are routinely conflated:

1. **The provenance claim** — that a specific study measured 100x.
2. **The empirical claim** — that fix cost rises steeply and monotonically with phase.
3. **The practical claim** — that shifting defect detection earlier pays for itself.

Evidence supports (3) directionally [16], supports (2) only conditionally [3, 5, 6, 7], and
largely fails to support (1) [9, 11, 12].

---

## 1. What the claim actually is

The folk claim has several variants, and they are not equivalent:

| Variant | Typical statement | Origin claimed |
|---|---|---|
| A. Requirements vs. production | "Fixing a bug after delivery is 100x more expensive than at requirements" | Boehm & Basili 2001 [1] |
| B. The chart | 1 : 6.5 : 15 : 60–100 (design : pre-test : test : post-release) | "IBM Systems Sciences Institute" [11] |
| C. The ladder | 1 : 3 : 7 : 50 : 100 across five phases | Wolverton 1974 [12, 18] |
| D. The vague version | "Bugs get more expensive the later you find them" | Boehm's cost-of-change curve [19, 20] |

Only variant A is stated in a peer-reviewed source, and that source hedges it heavily [1].
Variants B and C cannot be traced to a supporting dataset [11, 12].

**Variant A, verbatim** (Boehm & Basili, *IEEE Computer* 34(1), 2001, Item ONE):
> "Finding and fixing a software problem after delivery is often 100 times more expensive than
> finding and fixing it during the requirements and design phase." [1]

The same paragraph narrows it:
> "For this updated list, we have added the word 'often' to reflect additional insights about
> this observation. One insight shows the cost-escalation factor for small, noncritical software
> systems to be more like 5:1 than 100:1. … Another insight reveals that good architectural
> practices can significantly reduce the cost-escalation factor even for large critical systems." [1]

The 2001 article has **no reference list and cites no study or dataset** for the 100:1 number
itself; it points back only to Boehm's 1987 "Industrial Metrics Top 10 List" column [1, 20].

---

## 2. Provenance: no surviving primary source *containing a measurement*

The citation chain, as reconstructed by Bossavit [11–14] and reported downstream by the press
[9, 10], runs:

```
IBM/GTE/TRW data (pre-1976) [2]
  └─ Boehm 1976, IEEE Trans. Computers, p. 1227 — illustrative: "1 man-hour … 100 man-hours" [2]
       └─ Boehm 1981, Software Engineering Economics, Fig. 4-2  [book not fetched; UNVERIFIED] [19]
            └─ Boehm 1987, "Industrial Metrics Top 10 List"       [not fetched; contents UNVERIFIED] [20]
                 └─ Boehm & Basili 2001 — "often 100 times"       [no references] [1]

Pressman, Software Engineering (1987), citing
  "[IBM81] Implementing Software Inspections, course notes, IBM Systems Sciences Institute, 1981"
       └─ "1 : 6.5 : 15 : 60-100" chart [11]
            └─ Code Complete 2e, vendor decks, blog posts [11, 15]
                 └─ primary study: NOT FOUND [9, 11]
```

> **Single-investigator dependency (explicit).** The provenance reconstruction below rests
> principally on one investigator, Laurent Bossavit [11–14]. *The Register* [9] and Hillel
> Wayne's newsletter [10] are derivative reporting on Bossavit's work, not independent
> investigations, and are cited here as such.

Key findings:

- **Boehm's own 1976 statement is an illustrative sentence**, not a measurement: "say, 1
  man-hour rather than waiting to find the error during operations and having to spend 100
  man-hours correcting it." Units are man-hours. No sample size or method is stated in the
  accessible text [2]. The widely repeated "1x / 4x / 25x / 100x" ladder attributed to Boehm's 1981
  book is **UNVERIFIED** — no source found prints that exact ladder, and the book itself was
  not accessible [19].
- **The "IBM Systems Sciences Institute" chart does not trace to a study.** Bossavit's
  investigation found the institute was "an internal training program for employees at IBM" in
  Los Angeles (documented 1967 to before 1982), not a research body [11]. The citation is to
  *course notes* dated 1981 [11]. Hillel Wayne's blunt summary — reproduced in *The Register*
  (2021-07-22) — is "the study might not even exist" [9, 10]. Bossavit's own framing is narrower
  and fairer: "The original project data, if any exist, are not more recent than 1981, and
  probably older; and could be as old as 1967" [11]. He explicitly does **not** allege fraud,
  saying of Pressman he is "at worst mildly" embarrassed [13].
- **The Wolverton ladder is misattributed.** The 139:455:977:7136:14102 dollar figures normalize
  to 1:3:7:50:100 and appear to originate in **Safeguard** missile-program data (credited in a
  1977 Wolverton tutorial to W. E. Stephenson and R. O. Lewis), in **1975 dollars** — not from a
  multi-project defect study, and not from Boehm & Basili 2001, which does not contain those
  numbers [1, 12]. Wolverton's 1974 paper is itself a cost-forecasting methodology built on a
  25 × 7 structural forecast matrix, not a defect study [18]. **UNVERIFIED caveat:** this
  reconstruction rests on Bossavit's reading of a photographed 1977 tutorial page [12]; the
  Safeguard dataset size was not independently inspected and the exact project count is
  **UNVERIFIED**.
- **NIST's own 2002 report does not use 100x.** Its illustrative cost model (explicitly labelled
  "(Example Only)") tops out at 30x; its citation of Boehm 1976 is 0.2Y–15Y [8]. The US national
  cost estimate in that report therefore does not rest on the 100x figure (inference from the
  report's own tables) [8].

**Inference (flagged):** the "100x" is best understood as a *normative heuristic* that acquired
the appearance of an empirical constant through citation copying. This is an inference from the
provenance evidence, not a statement any single source makes.

---

## 3. What the empirical evidence shows

The single most important study is the direct test:

**Menzies, Nichols, Shull & Layman (2017), *Empirical Software Engineering* 22(4):1903–1935**
(arXiv:1609.04886) — 171 Team Software Process projects drawn from an SEI database of 212
projects, 47 organizations, 843 people, 2006–2014, 47,376 defect logs [4]. Abstract, verbatim:
> "We found no evidence for the delayed issue effect; i.e. the effort to resolve issues in a
> later phase was not consistently or substantially greater than when issues were resolved soon
> after their introduction." [4]

Most reported median ratios were between roughly **0.7x and 2.5x** across phase pairs, with one
exception at 3x (Planning→UnitTest), which the authors note is "not the exponential increase
promised by classic reports" [4]. The authors describe DIE as "an historical relic that occurs
intermittently only in certain kinds of projects" [4]. This is a load-bearing negative result.
It is also scoped: **all projects used TSP; the data do not extend into post-delivery
deployment; and it measures time-to-fix, not business cost** [4]. It therefore tests
pre-delivery phase delay, not the field-vs-requirements 100x claim head-on [4]. Crucially,
Menzies et al. state: *"We do not consider the severity of defects in this analysis"* [4] — so
this study cannot corroborate the severity-conditional version of the claim either.

| Study | Dataset | Measured escalation | vs. 100x |
|---|---|---|---|
| Menzies et al. 2017 (EMSE) [4] | 171 TSP projects, 47,376 defects, 2006–2014 | Mostly 0.7x–2.5x median, one 3x exception (pre-delivery only; severity not tested) | Rejects |
| Stecklein et al. 2004 (NASA/INCOSE) [5] | Measured: 5 spacecraft modifications; 1 aircraft program. **Model:** 1 hypothetical satellite (Method 3) | Measured/model: 3–8x design; 7–16x build; 21–78x test; operations 29x–>1500x — the >1500x endpoint is **model-derived** from the hypothetical-satellite method | Supports, wide band, partly modelled |
| Hamill & Goseva-Popstojanova 2017 (IST) [6] | 1 NASA flight-software mission, ~2.2 M LOC, ~10 yr, 37 post-release failures | Post-release fix effort: mean 50.9 h vs 14.0 h pre-release (~3.6x); median 7 h vs 3 h (~2.3x); p = 0.0175 | Weak support |
| Langenfeld, Post & Podelski 2016 (REFSQ, Bosch) [7] | 1 automotive project, 588 requirements defects, 4.5 yr | ~2.7–3.7x for the least-costly defect classes (8–11 h vs 3 h); 22.7 h average overall at vehicle test | Weak support |
| Baziuk 1995 (Nortel), via NIST 2002 [8] | Nortel repair histories | System testing 90x; installation testing 90x–440x; acceptance 440x; operation/maintenance 470x–880x (footnote: "possibly as high as 2,900x" with an engineering change order) | Supports large |
| NIST/RTI 2002 (Tassey) [8] | Literature synthesis, no new data | 1x–30x model, labelled "(Example Only)" | ~30x, illustrative |
| Shull et al. 2002 (IEEE Metrics) [3] | Expert e-workshop (Toshiba, IBM Rochester) | Toshiba 1:137; IBM Rochester 13:1 then 9:1 (≈117:1 code→field) — **UNVERIFIED: raw data were never published** | Supports ~100x, unverifiable data |

**Where 100x is roughly supported:** severe, architectural, or requirements-level defects in
large systems with a defined release point, especially aerospace/defense. The NASA/INCOSE
comparison table lists software operations factors of **100x–1000x**, with a system-level
composite of 29x–1615x; the top end (>1500x) is produced by a *hypothetical*-satellite model
that assumes any undetected error causes total mission loss, not by a measurement [5].

**Where it is not:** ordinary defects in mature or iterative processes, where measured escalation
is roughly 2x–5x [3, 4, 6]; small non-critical systems, where Boehm & Basili themselves say ~5:1
[1]; and the pre-delivery phase, where the largest multi-project study finds nothing consistent
[4].

### The severity condition is the crux — and it is single-sourced
The best supporting synthesis is not the folk chart but Shull et al. (2002), which after
re-examining the claim reformulated it:
> "Finding and fixing a **severe** software problem after delivery is often 100 times more
> expensive…" and "Finding and fixing **non-severe** software defects after delivery is about
> **twice** as expensive as finding these defects pre-delivery." [3]

This is the defensible version. The unqualified claim in circulation drops the severity
qualifier — and the severity qualifier is exactly where the evidence is strongest [3].
**However, this crux rests on a single source whose raw data were never published** [3], and the
other candidate corroboration (Menzies et al.) explicitly does *not* test severity [4]. The
severity-conditional form of the claim should therefore be treated as *plausible but
single-sourced and not raw-data-verifiable*, not as independently confirmed.

---

## 4. Critiques — and where the critics overreach

- **Overreach to discount:** Bossavit's conclusion that the curve has "100% ideological and 0%
  empirical content" is polemic [14]; the same chapter concedes Boehm had *some* data points
  (from two teams of first-year students, two GTE projects, and Fagan's IBM study — all small or
  unclear) [14]. Treat that line as rhetoric, not finding. (Quotation source: a secondary
  full-text copy of ch. 10 [14]; pagination may differ from the published edition.)
- **Corrected internal misattribution (this brief's own drafting):** an earlier draft of this
  brief attributed the retraction of the reproduced "HARD DATA" table to "Paul Wilson." That was
  an internal error; no source in the literature makes that attribution. The retraction is by
  **Graham Lee** (sicpers.info, Sept 2012), who found the table's survey provenance was something
  he had himself invented: "I seem to have made up the bit about it being the result of a
  survey… the table is completely false" [15].
- **Agile counterevidence is also weak.** Kent Beck's flat-curve claim is explicitly conditional
  ("Under certain circumstances… can be flattened") [21]. Secondary analyses describe it as
  backed by Beck's own anecdotal evidence and initially presented as a metaphor [22]. Larman &
  Basili 2003, often cited as the evidence, is a *history* of iterative development — its
  abstract frames it as a historical survey, and the text contains no defect-cost comparison
  [23]. So the "flat curve" is not a refutation of the 100x; it is a competing assertion.
- **Incentives cut both ways.** The claim has been used in vendor tooling arguments, and
  Bossavit identifies Cigital (a security consultancy) as a propagation node for the
  Wolverton-derived chart [12]. But the principal defenders (Boehm, Basili) are academics [1],
  and CISA — a government body with a secure-by-design agenda — questions the number's
  evidentiary basis, noting "there is no evidence to show that the IBM Systems Science Institute
  was an official research body" and that the subcommittee found "often no empirical evidence to
  substantiate some of its long-held security beliefs" [17]. Separately, security practitioners
  quoted in *Dark Reading* argue the *direction* is still correct even if the magnitude is not
  [16]. Those are different claims and should not be merged.
- **Methodological problems are real, in both directions.** Only defects observed in two or more
  phases can be compared, so late-found defects may be intrinsically harder (design bugs), not
  expensive *because* late [4]. "Defect" is defined differently across studies, and phase
  injection/removal is self-reported [4]. Menzies et al. also disclose that TSP was developed and
  promulgated by the employer of two co-authors [4].

---

## 5. Moderators: when escalation is steeper or flatter

| Condition | Effect on the ratio | Basis |
|---|---|---|
| Large critical system vs small non-critical | ~100:1 vs ~5:1 | Boehm & Basili 2001 [1] |
| Severe/architectural vs ordinary defect | ~100:1 (code→field) vs ~2:1 | Shull et al. 2002 [3] (single-sourced, raw data unpublished) |
| Requirements/architecture vs local code | Steeper vs flatter | Boehm & Basili 2001 [1]; Menzies et al. 2017 [4] |
| Aerospace/defense vs commercial IT | Steeper (up to >1000x measured/model band) vs flat in TSP sample | NASA/INCOSE 2004 [5]; Menzies et al. 2017 [4] |
| Defined release point vs continuous deployment | Steeper vs flatter — **inference**: Shull establishes the heuristic as untested for models without an early/late distinction (XP), not that continuous deployment flattens the ratio | *inference from* Shull et al. 2002 [3] |
| Fix effort only vs downstream cost (outage, data loss, legal) | Ratio rises; tail unbounded | Inference — not directly measured in the cited studies [4] |
| Modern iterative/TSP processes | ~1x–4x | Menzies et al. 2017 [4]; Hamill 2017 [6]; Langenfeld 2016 [7] |

---

## 6. What is and is not defensible

**Defensible:**
- Fix cost tends to increase with phase delay, but the increase is *conditional, not
  universal* — and in the largest multi-project study it was not consistent or substantial [4].
- For severe defects in large systems, a 100x order-of-magnitude worst case is plausible; it
  sits at the **low edge** of NASA's 100x–1000x software operations band, while the ~1500x
  figure is model-derived (hypothetical satellite), not measured [5].
- For typical defects in modern or small systems, measured escalation is closer to 2x–5x
  [1, 3, 4, 6].
- Early detection has real value; the direction of the effect is more robust than its
  magnitude [3, 16].

**Not defensible:**
- "A defect costs 100x more to fix later" as a universal constant [4].
- Citing the IBM Systems Sciences Institute chart, or Boehm & Basili 2001, as if either provides
  a dataset for the 100x number [1, 9, 11].
- The unqualified ladder 1:3:7:50:100 attributed to Wolverton 1974 (it is Safeguard data, 1975
  dollars, different attribution) [12, 18].

**Practical restatement:** "Fixing a severe defect after release commonly costs one to two
orders of magnitude more than fixing it at requirements — but ordinary defects in modern
iterative processes often cost only 2–5x more, and the widely cited '100x' lacks a traceable
primary source."

---

## 7. Open questions

1. **Why does the largest study find no effect while NASA/INCOSE and Nortel data find large
   effects?** (MAJOR, from review.) Plausible explanations — process maturity, severity mix,
   pre- vs post-delivery measurement — are not tested head-to-head by any source found
   [4, 5, 8]. Menzies et al. explicitly exclude severity [4] and exclude post-delivery data [4],
   so the two literatures may not be measuring the same quantity.
2. **The severity-conditional claim is single-sourced.** (MAJOR, from review.) It rests on
   Shull et al. 2002 [3], an expert e-workshop whose raw ratios Menzies reports as
   unconfirmable; no independent severity-stratified cost measurement was located.
3. **The provenance conclusion is single-investigator.** (MAJOR, from review.) It rests on
   Bossavit [11–14]; the press coverage [9, 10] is derivative. The Wolverton/Safeguard
   reconstruction was not independently inspected [12].
4. Menzies et al. have an internal inconsistency: §6.3 cites a "2.16 to 4.37" scale-up while
   §5.6/Figure 13 give 1.11. Both passages are present in the published text; the discrepancy is
   **UNVERIFIED** and unresolved [4].
5. No peer-reviewed study was located that directly tests the 100x figure against
   post-deployment repair cost in large open-source projects (scoped to the literature searched
   for this brief).
6. The AI-generated-code literature reports defect/vulnerability *rates and profiles*, not phase
   cost ratios, so it neither supports nor refutes the multiplier — e.g., Cotroneo et al. (2025)
   compare defect and vulnerability profiles across >500k code samples and report no fixed
   early-vs-late cost ratio [24].

---

## Removed Unsupported Claims

Material removed, corrected, or softened during verification and review:

1. **Beck "n=1" and the 30-minute-after-two-years anecdote.** The specific anecdote could not be
   verified against an accessible copy of *Extreme Programming Explained* (publisher page
   returns HTTP 403; archived copy did not contain the passage). Replaced with the verified
   conditional sentence [21] and a secondary characterization of the evidence base [22].
2. **Langenfeld "8–11 h vs 3 h" presented as the study's headline ratio.** In the paper, 8 and 11
   man-hours are the vehicle-test figures for the two *least-costly* defect classes; the overall
   vehicle-test average is 22.7 man-hours [7]. The table cell now states both.
3. **"CISA … while still endorsing the direction."** CISA's report questions the evidence base
   [17]; the "direction is still correct" position is attributed to practitioners quoted in
   *Dark Reading* [16], not to CISA. Split into two attributions.
4. **"Conflict of interest"** for Menzies et al. — the paper makes a disclosure statement, not a
   formal COI declaration. Reworded to match the text [4].
5. **Menzies "1.1x–2.5x" range.** Figure 13 values run from 0.69 to 2.54 with a 3.00 exception;
   the narrower range understated the low end. Corrected [4].
6. **Review fix — severity crux citation.** The draft cited the severity claim to "[3, 4]". Source
   [4] states "We do not consider the severity of defects in this analysis." Now cited to [3]
   only, with the single-source limitation flagged explicitly.
7. **Review fix — provenance sourcing.** The verdict cited "[9, 11, 14]" as if independent. Now
   attributes the reconstruction to Bossavit [11–14] and labels [9]/[10] as derivative.
8. **Review fix — NASA operations endpoint.** "can genuinely reach 100x–1500x" replaced with the
   correct split: 100x–1000x software band, >1500x model-derived from the hypothetical-satellite
   method [5].
9. **Review fix — Cigital over-citation.** Now cited to [12] (Bossavit) only; [16] (*Dark
   Reading*) mentions Cigital solely as a practitioner's former employer.
10. **Review fix — "mid-range" overstated confidence.** 100x re-described as the low edge of the
    100x–1000x band [5].
11. **Review fix — Hamill cell ambiguity.** Both the mean pair (50.9 h vs 14.0 h) and the median
    pair (7 h vs 3 h) are now stated [6].
12. **Review fix — Baziuk cell compression.** Phase breakdown corrected per NIST Table 1-5,
    including the "possibly as high as 2,900x" footnote [8].
13. **Review fix — release-point moderator.** Marked as an inference, not a Shull finding [3].
14. **Review fix — "Paul Wilson" artifact.** Recast as an internal drafting correction (see §4),
    not a live misattribution in the field.

---

## Sources

1. Boehm, B. & Basili, V. R. "Software Defect Reduction Top 10 List," *IEEE Computer* 34(1):135–137,
   2001. https://doi.org/10.1109/2.962984 · https://www.cs.umd.edu/~basili/publications/journals/J81.pdf
2. Boehm, B. W. "Software Engineering," *IEEE Trans. Computers* C-25(12):1226–1241, 1976.
   https://selab.netlab.uky.edu/homepage/boehm-sw-eng-paper.pdf
3. Shull, F., Basili, V., Boehm, B. et al. "What We Have Learned About Fighting Defects," *IEEE
   Metrics Symp.* 2002, pp. 249–258. https://doi.org/10.1109/METRIC.2002.1011343 ·
   https://kevin.burke.dev/docs/shull_defects.pdf
4. Menzies, T., Nichols, W., Shull, F., Layman, L. "Are Delayed Issues Harder to Resolve?
   Revisiting Cost-to-Fix of Defects throughout the Lifecycle," *Empirical Software Engineering*
   22(4):1903–1935, 2017 (online 2016). https://arxiv.org/abs/1609.04886 ·
   https://doi.org/10.1007/s10664-016-9469-x
5. Stecklein, J., Dabney, J., Dick, B., Haskins, B., Lovell, R., Moroney, G. "Error Cost
   Escalation Through the Project Life Cycle," INCOSE 2004; NASA JSC-CN-8435.
   https://ntrs.nasa.gov/citations/20100036670 ·
   https://ntrs.nasa.gov/api/citations/20100036670/downloads/20100036670.pdf
6. Hamill, M. & Goseva-Popstojanova, K. "Analyzing and predicting effort associated with finding
   & fixing software faults," *Information and Software Technology* 87:1–18, 2017; NASA NTRS
   20160008693. https://doi.org/10.1016/j.infsof.2017.01.002 ·
   https://ntrs.nasa.gov/api/citations/20160008693/downloads/20160008693.pdf
7. Langenfeld, V., Post, A., Podelski, A. "Requirements Defects over a Project Lifetime," REFSQ
   2016, LNCS 9619, pp. 145–160. https://doi.org/10.1007/978-3-319-30282-9_10 ·
   https://swt.informatik.uni-freiburg.de/staff/langenfeld/resources/Requirements%20Defects%20over%20a%20Project%20Lifetime
8. Tassey, G. / RTI for NIST. "The Economic Impacts of Inadequate Infrastructure for Software
   Testing," NIST Planning Report 02-3, May 2002.
   https://www.nist.gov/system/files/documents/director/planning/report02-3.pdf
9. Anderson, T. "Everyone cites that 'bugs are 100x more expensive to fix in production' research,
   but the study might not even exist," *The Register*, 22 Jul 2021.
   https://www.theregister.com/software/2021/07/22/everyone_cites_that_bugs_are_100x_more_expensive_to_fix_in_production_research_but_the_study_might_not_even_exist/1371906
10. Wayne, H. "I ****ing hate Science," 2021.
    https://buttondown.com/hillelwayne/archive/i-ing-hate-science
11. Bossavit, L. "The IBM Systems Science Institute" (gist).
    https://gist.github.com/Morendil/ebfa32d10528af04e2ccb8995e3cb4a7
12. Bossavit, L. "The citations game: Wolverton Ratios" (gist).
    https://gist.github.com/Morendil/f9c2e9f3f450d3a76de8aeee7cf2bd00
13. Bossavit, L. "Degrees of intellectual dishonesty" (gist).
    https://gist.github.com/Morendil/258a523726f187334168f11fc8331569
14. Bossavit, L. *The Leprechauns of Software Engineering*, ch. 10, Leanpub 2015.
    https://leanpub.com/leprechauns (full text consulted at
    https://dokumen.pub/the-leprechauns-of-software-engineering-how-folklore-turns-into-fact-and-what-to-do-about-it-1nbsped.html
    — a secondary copy; pagination may differ from the published edition)
15. Lee, G. "An apology to readers of Test-Driven iOS Development," Sept 2012.
    https://www.sicpers.info/2012/09/an-apology-to-readers-of-test-driven-ios-development/
16. Lemos, R. "'Shift Left' Gets Pushback, Triggers Security Soul Searching," *Dark Reading*, 2024.
    https://www.darkreading.com/application-security/shift-left-pushback-triggers-security-soul-searching
    (returns HTTP 403 to automated fetches; live in browsers)
17. CISA CSAC. *Report to the CISA Director: Secure By Design*, 11 Oct 2024.
    https://www.cisa.gov/sites/default/files/2024-10/CSAC_October-Quarterly-Meeting_SBD-Recommendations_20241011_508.pdf
18. Wolverton, R. W. "The Cost of Developing Large-Scale Software," *IEEE Trans. Computers*
    23(6):615–636, 1974. https://doi.org/10.1109/T-C.1974.224002
19. Boehm, B. W. *Software Engineering Economics*, Prentice-Hall, 1981 (metadata only; text not
    accessed). https://books.google.com/books?id=mpZQAAAAMAAJ
20. Boehm, B. "Industrial software metrics top 10 list," *IEEE Software*, Sept 1987, pp. 84–85
    (metadata only; text not accessed).
    https://www.semanticscholar.org/paper/acf378af999c5c9b2475d000ee92aed7dc11540b
21. Beck, K. *Extreme Programming Explained*, ch. 5, Addison-Wesley 1999.
    https://www.oreilly.com/library/view/extreme-programming-explained/0201616416/ch05.html
    (returns HTTP 403 to automated fetches; quoted text verified via archived copies)
22. Ambler, S. "Examining the Agile Cost of Change Curve."
    https://agilemodeling.com/essays/costOfChange.htm
23. Larman, C. & Basili, V. R. "Iterative and Incremental Development: A Brief History,"
    *Computer* 36(6):47–56, 2003. https://doi.org/10.1109/MC.2003.1204375 ·
    https://www.cs.umd.edu/~basili/publications/journals/J90.pdf
24. Cotroneo, D., Improta, G., Liguori, P. "Human-Written vs. AI-Generated Code: A Large-Scale
    Study of Defects, Vulnerabilities, and Complexity," ISSRE 2025.
    https://doi.org/10.1109/ISSRE66568.2025.00035

---

## Verification notes

**URL checks (corrected after review).** Sources 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
15, 17, 18, 22, 23, 24 were re-checked and resolve (HTTP 200, or 202 at IEEE/ACM publisher
fronts that serve bot challenges). **Two exceptions:** source [21] (O'Reilly Beck chapter)
returns **HTTP 403** to automated fetches, and source [16] (Dark Reading) returns **HTTP 403**;
both are live in browsers, and the passages quoted from [21] were verified via archived copies.
An earlier version of this note wrongly listed [21] as resolved and omitted [4]; both are
corrected here. Sources 19 and 20 are cited as **metadata-only** and are not used to support any
number.

**DOI/title cross-checks via Crossref:** all six DOIs resolve to records whose title, year, first
author, and page range match the citation, with three record-level notes:

- **Boehm & Basili 2001** — the Crossref record title is "Top 10 list [software development]",
  not the article's printed title "Software Defect Reduction Top 10 List." Same DOI, authors,
  volume, pages. Citation kept with the printed title.
- **Stecklein et al. 2004** — the NASA NTRS record lists **Stecklein** first; the INCOSE/Crossref
  record (10.1002/j.2334-5837.2004.tb00608.x) lists **Haskins** first and titles the paper
  "8.4.2 Error Cost Escalation Through the Project Life Cycle." Both are the same paper; this
  brief cites the NTRS record and the "Stecklein et al." form conventional in the literature.
- **Menzies et al. 2017** — Crossref gives an online date of 2016; the issue is 22(4), 2017,
  pp. 1903–1935. Both dates appear above.

**Quotation checks:** the Boehm & Basili Item ONE text, the 5:1 caveat, Boehm's 1976 "1 man-hour …
100 man-hours" sentence, the Shull et al. Item 1′/1.1 reformulation and the 13:1/9:1/117:1
passage, the Menzies abstract sentence and its "We do not consider the severity of defects"
limitation, the NASA operations range, the NIST "(Example Only)" labels and 1X–30X ladder, the
Baziuk phase cells, the Bossavit IBM-institute and Wolverton/Safeguard passages, Lee's
retraction wording, and Wayne's "it doesn't exist" line were all checked against retrieved full
texts, not against abstracts alone.

**UNVERIFIED markers carried forward:** Boehm 1981 Figure 4-2 contents; the "1x/4x/25x/100x"
ladder; Boehm 1987 column contents; the original IBM SSI dataset; the Wolverton/Safeguard
dataset size; the Menzies §6.3 vs §5.6/Figure 13 discrepancy; and the Shull et al. 2002
1:137 / 1:117 ratios (raw data never published).
