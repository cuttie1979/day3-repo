# Requirements-to-Test-to-Evidence Traceability in Regulated Industries — and What Transfers

**Slug:** `regulated-requirements-traceability`
**Date:** 2026-10-18
**Scope:** Automotive SPICE (PAM 3.1/4.0/4.1) + ISO 26262:2018 · RTCA DO-178C / ED-12C · IEC 62304 + FDA device-software guidance + EU MDR · cross-industry empirical traceability evidence · transferable practice.
**Evidence classes used throughout:** `[primary]` = standard text, regulator document, or official publisher page read directly; `[guidance]` = official assessment guidance (VDA QMC) or regulator-recognised supporting document; `[secondary]` = vendor technical paper, consultancy summary, training material, encyclopedia; `[empirical]` = peer-reviewed study (abstract or full text as stated). Sources are listed with URLs at the end; inline numeric citations `[n]` reference that list (each entry also retains its short source tag, e.g. `[PAM40]`).

> **Revision note (2026-10-18).** This is the post-review revision. A verification-independent review pass (`outputs/.drafts/regulated-requirements-traceability-verification.md`) raised **two FATAL and eight MAJOR findings**; all are corrected in place in sections 1, 2, 4.4, 5.1, 5.3, 6, 8 and 9. A second independent pass (`...-verification2.md`) found **no FATAL remaining**, one residual MAJOR about evidence-class bookkeeping (fixed in this file) and several minor consistency defects (fixed). Residual single-source and unverified items are listed in §10; nothing was upgraded in confidence during revision.

---

## 1. Executive summary

1. **All four regimes trace the same spine; three state bidirectionality explicitly.** The chain is stakeholder/system requirement → (architecture/design) → item-level requirement → verification measure → verification result, plus change request ↔ affected artefacts. ASPICE mandates bidirectional traceability as a *base practice* in each engineering process [2 §4.3–4.9]; ISO 26262 anchors it in one cross-cutting clause, 26262-8 §6.4.3.2, which requires each safety requirement to reference its upper-level source, its lower-level realisation, and its verification specification [15 §6.4.3.2]; DO-178C requires six bidirectional trace pairs [22]. **IEC 62304 is the exception in the sources read:** it requires requirements to be *traceable to parent specifications or another source* (§5.2.6) and requires *recorded traceability between requirements and tests* (§5.7.4) but those sourced formulations do not state bidirectionality [47; 46]. FDA's 2023 guidance does state bidirectionality explicitly, for design-document pairs only (SRS↔SDS, source↔SDS) [41].

2. **The regimes disagree far more about *evidence* than about *traceability*.** ASPICE assesses process capability by sampling work products and testimony, with assessor judgement and explicit "shall not be downrated" protections for manual, cluster-level traceability [4 §2.1.6, §1.4.3]. DO-178C agrees the evidence plan *before* development starts (the PSAC) and then closes it with an accomplishment summary [21; 37]. ISO 26262 operates the trace through a safety case plus independence-graded confirmation reviews and a functional safety assessment (I0–I3 by ASIL) [14 Table 1]. FDA treats traceability as one element of a risk-tiered documentation set submitted to a reviewer [41].

3. **Rigor is graded in every regime — by different scales but the same logic.** ASIL (QM–D) in ISO 26262, software level A–E in DO-178C, safety class A/B/C in IEC 62304, and Basic vs Enhanced documentation level in FDA's 2023 guidance. The FDA formulation is the most portable: Enhanced documentation applies where a software failure "could present a hazardous situation with a probable risk of death or serious injury" [41 §V].

4. **Coverage metrics beyond the trace appear only in the safety regimes — and the ISO 26262 version is weakly sourced here.** DO-178C requires structural coverage scaled by level (statement, decision, and MC/DC at Level A) in addition to requirements coverage [24; 28] `[S]`. For ISO 26262, structural coverage (statement/branch/MC-DC) selected by ASIL for unit verification is stated by a **single secondary clause-paraphrase source whose own page disclaims holding the standard's text** [20] — treat it as provisional, not established, and note that no research file independently corroborates it. ASPICE requires traceability between *verification measures and requirements* — i.e. a coverage argument — but prescribes no coverage metric [2]. IEC 62304 requires tests covering **all** software requirements, with no structural-coverage requirement [46] `[S]`.

5. **Verification independence is a first-class, graded obligation only in the safety regimes.** ISO 26262 defines four independence levels (I0–I3) and grades the functional safety assessment by ASIL: **I0 at ASIL B** (I0 = the measure "should be performed; if performed, by a different person" — i.e. not an independence requirement), **I2 at ASIL C** and **I3 at ASIL D**; none at QM or ASIL A [14 §6.4.12.1, Table 1]. DO-178C requires independence for a subset of verification objectives, concentrated at Levels A and B [22; 28] `[S]`. ASPICE does not assess independence of *verification* at all — it assesses whether the process is performed and managed — although it does require independent quality assurance (SUP.1.BP1) [2].

6. **Tool qualification is most explicit in aviation, but it is not unique to it.** DO-178C/DO-330 is the most fully specified regime, with graded tool qualification levels (TQL-1…TQL-5) tied to how much the tool substitutes for human verification [31; 30] `[S]`. **ISO 26262 also carries a tool-confidence clause** — Part 8 Clause 11, "Confidence in the use of software tools", including §11.4.6 qualification of a software tool (official 2018 table of contents) — so the correct statement is *degree*, not *existence*: aviation grades tool qualification most heavily, automotive addresses it in one clause, and ASPICE explicitly does not mandate tooling for traceability at all [2 §2.1.6.3]. What a team outside both industries should refuse is the *formal qualification data set*, not the idea of trusting a tool's output only as far as it has been shown to deserve.

7. **The empirical literature says traceability helps, weakly and with caveats.** A controlled experiment with >50 subjects found trace navigation improved performance, quality and workflow on change tasks [56]; an observational study over 24 open-source projects found traceability completeness for three implementation-support activities significantly associated with lower expected defect rate [57]. Neither is a field trial, and no study measures traceability ROI in currency or measures stale-link rates. Treat "traceability pays for itself" as a hypothesis with moderate support, not a fact.

8. **The best-documented practice failure is not missing links — it is unused links and unmaintained links.** The closest well-sourced phenomena are Ramesh & Jarke's "low-end" users (who trace because mandated) [55], Ruiz et al.'s finding that cost/benefit ratio and specific use cases — not headcount — drive adoption [58], and Wohlrab et al.'s 15-project case study showing maintenance and cross-team/cross-tool breakdowns [59]. The terms "traceability theater" and "trace explosion" could **not** be traced to reputable literature in this run and should not be quoted as established concepts.

9. **What transfers, in priority order:** typed links with unique IDs; automatic "suspect link" invalidation on upstream change; requirement-level (not document-level) trace from requirement to test case to test result; requirement-coverage measurement *in addition to* code coverage; risk-tiered test depth; graded verification independence; evidence emitted as artifacts at the time of work; a first-class unresolved-anomaly register; and a scoping rule that permits traceable clusters rather than atomic everything.

10. **What should not transfer:** MC/DC at Level A, the full DO-178C lifecycle data set, ASPICE capability-level assessment, formal tool qualification, and exhaustive review-record sets. Each is defensible only when the software can kill someone or a customer contract mandates it.

---

## 2. How to read this report (evidence and limits)

**Five limits shape every clause-level claim below.**

1. **Three of the four standards are paywalled.** RTCA/DO-178C, ISO 26262 and IEC 62304 are sold by their publishers. Automotive SPICE is the exception: VDA QMC publishes the PAM and Guidelines openly, and this report reads the PAM text directly [2, 3, 4].
2. **Where clause text was read from a publicly posted copy rather than a licensed copy, it is labelled.** ISO 26262 Part 2 (2018), Parts 3/4/5/6/7/8 (2011) clause text was read from public mirrors; the 2018 front matter/table of contents was read from the publisher's official preview pages [11; 12; 13]. **The 2018 edition is authoritative and its clause numbering differs from 2011 in Parts 3 and 4** — every clause number below is tagged with the edition it came from, and the deltas are listed in §7.
3. **Vendor material is used only for tool *mechanism*, never for effectiveness.** No vendor publishes how often "suspect links" are eventually cleared [62; 63].
4. **A lead-side primary-source check was performed** on the four documents that could be obtained officially: Automotive SPICE PAM v4.0 (153 pp) and v4.1 (159 pp), FAA AC 20-115D, the FDA 2023 device-software guidance (46 pp), and the NASA/AIAA DO-178C paper. Verified quotations from that check are in `outputs/.drafts/regulated-requirements-traceability-research-lead.md`.
5. **Evidence-class labels in the comparison matrix (§6) are graded, not binary:** `[P]` = read from a source that publishes the text in question (PAM/Guidelines, FDA guidance, FAA AC, NASA paper); `[P*]` = read from a **publicly posted copy** of a standard, or from a source that does not publish the normative text in full — specifically: ISO 26262 **Part 2** clause text was read from a 2018 public copy, **Parts 3–8** clause text from 2011 public copies, and the 2018 official table of contents only for Parts 3, 4, 6 and 8, so the 2018 normative sentences of Parts 3–8 were **not** read; `[S]` = clause-level claim resting on vendor/consultancy/training secondary sources because the standard is paywalled (DO-178C Annex A detail, IEC 62304 clause applicability). A cell may carry two labels when it mixes elements; the matrix marks the weaker element.

**Currency note (verified, narrowly).** Automotive SPICE PAM **v4.1 was released 2026-08-24** (VDA QMC), superseding v4.0; its stated change list is "Removal of MLE.2 BP5; editorial changes based on feedback from community" [3 rev. history; 6]. A text comparison performed for this report found that the **three** traceability base practices sampled (`SYS.2.BP5`, `SWE.6.BP4`, `SUP.10.BP4`) carry identical requirements in 4.0 and 4.1 apart from articles/plurals (artifact: `notes/regulated-trace/aspice-pam-4{0,1}.txt`). **This is a sample, not an exhaustive BP-by-BP diff**, and in MLE.2 the traceability base practice was renumbered BP6→BP5 as part of the MLE.2 BP5 removal — so the correct claim is that the sampled engineering traceability BPs are unchanged, not that the 4.0→4.1 delta is purely cosmetic.

---

## 3. Regime 1 — Automotive SPICE + ISO 26262

### 3.1 Automotive SPICE: traceability as a base practice, in two editions

Automotive SPICE addresses traceability and consistency through dedicated base practices (BPs) **inside the engineering processes**, plus change-request management and project management:

> "In the Automotive SPICE consistency and traceability are addressed by a BP in the engineering processes and in the Change Request Management process. Furthermore, consistency is addressed in the Project Management process." [4 §2.1.6]

**PAM v3.1 (2017)** used **two** BPs per engineering process — "Establish bidirectional traceability" and "Ensure consistency" — in SYS.2–SYS.5, SWE.1–SWE.6, and SUP.10 (traceability), and MAN.3 (consistency only). **SYS.1, SUP.1, SUP.8 and ACQ.4 carry no traceability BP** [1 §4.3–4.6].

**PAM v4.0 (2023-11-29)** merges each pair into one BP, *"Ensure consistency and establish bidirectional traceability"*, and extends traceability to the new process groups HWE.1–HWE.4 (hardware), MLE.1–MLE.4 (machine learning) and VAL.1 (validation) [2 §4.3–4.9; 7]. Verbatim example:

> "SWE.1.BP5: Ensure consistency and establish bidirectional traceability. Ensure consistency and establish bidirectional traceability between the software requirements and system architecture. Ensure consistency and establish bidirectional traceability between the software requirements and the system requirements." [2 §4.4.1]

Traceability obligations by process area (v4.0 numbering; v3.1 BP numbers in the research file): SYS.2.BP5, SYS.3.BP4, SYS.4.BP4, SYS.5.BP4, SWE.1.BP5, SWE.2.BP4, SWE.3.BP4, SWE.4.BP4, SWE.5.BP6, SWE.6.BP4, VAL.1.BP4, HWE.1–4.BP5, MLE.1–4.BP5/6, SUP.10.BP4 [2].

**Three details inside the model matter more than the BP list:**

- **Traceability ≠ consistency.** The PAM states the distinction explicitly: consistency "addresses content and semantics and ensures that work products are not in contradiction to each other", and the note repeated across process areas says *"Traceability alone, e.g., the existence of links, does not necessarily mean that the information is consistent with each other."* [2; 4 glossary]. A link is evidence of a relationship, not evidence that the relationship is true.
- **Redundant traceability is not intended**, and traceability to non-functional requirements may legitimately be absent (they remain subject to verification) [2 SWE.1.BP5 Notes 9–10].
- **Unidirectionality is allowed in one place**: in VAL.1, where the source is a legal or homologation requirement, "direct bidirectional traceability from those sources to the validation measures [is] not possible. In such a case, unidirectional traceability is sufficient." [2 §4.5]

### 3.2 ASPICE assessment: what an assessor actually does

Traceability is not audited by checking that a matrix exists; it is **rated as a base practice** on the ISO/IEC 33020 scale, and protected from unfair downrating by official guidance.

**Rating.** N/P/L/F = Not/Partially/Largely/Fully achieved, with percentage anchors of ≤15%, >15–50%, >50–85%, >85–100%, refinable to P−/P+/L−/L+ [2 Tables 16–19]. Capability level rules (v3.1): CL1 = PA 1.1 *Largely*; CL2 = PA 1.1 *Fully* + PA 2.1, PA 2.2 *Largely*; CL3 = PA 1.1/2.1/2.2 *Fully* + PA 3.1, PA 3.2 *Largely* [1 Table 16]. PAM 4.0 restates the level table using "largely or fully" for the level-defining attributes [2 Table 20] — a genuine relaxation between editions.

**Objective evidence.** "All such evidence comes either from the examination of work products related to a specific output of the processes assessed, or from statements made by the performers and managers of the processes." Information items are "what to look for", and the PAM is explicit that there is **no 1:1 relationship** between an information item and the sampled work product [2 §3.3.2].

**Sampling (VDA Guidelines, not the PAM).** Samples must be representative and must cover the most important functions, new/adapted/reused/platform software, **"the whole spectrum of ASIL levels applied within the assessment scope"**, and both manual and model-based development. The assessor follows "a clear path through the engineering life cycle". Critically, assessors perform **plausibility checks**: documents whose last change timestamp or change history looks back-filled "should not be used unless there is a plausible reason for the late documentation" [4 §1.4.3]. That single rule is the strongest anti-theater control found in any of the four regimes.

**Rating rules that protect cheap traceability (verbatim intent) [4 §2.1.6.2–2.1.6.4; 5]:**

| Rule | Content | Consequence |
|---|---|---|
| TAC.RL.1 | Traceability at **cluster** level (clusters of requirements/architectural elements/units/verification measures/results) instead of atomic elements | shall **not** be downrated |
| TAC.RL.2 | Manual / snapshot-based traceability instead of automated tooling | shall **not** be downrated |
| TAC.RL.3 | No documented review/analysis record, where alternative evidence exists (pair working, peer spot checks, revision history, change commenting) | shall **not** be downrated |
| TAC.RL.4 | Traceability between information not part of baselines | shall **not** be downrated |

Plus: "A PAM does not predefine any methodology/approach or tools. The same applies for the realization of traceability." [4 §2.1.6.3]

### 3.3 ISO 26262: one cross-cutting traceability clause plus part-specific allocation rules

ASPICE says *how well you run a process*; ISO 26262 says *what the safety argument must connect*. The traceability backbone is a single clause:

> "6.4.3.2 Safety requirements shall be traceable with a reference being made to: a) each source of a safety requirement at the upper hierarchical level, b) each derived safety requirement at a lower hierarchical level, or to its realisation in the design, and c) the specification of verification in accordance with 9.4.2. NOTE Additionally, traceability supports: an impact analysis if changes are made to particular safety requirements, and the functional safety assessment." [15 §6.4.3.2 — 2011 wording; the 2018 table of contents confirms clause 6.4.3 "Management of safety requirements" is retained, but the 2018 normative sentence was not available for direct reading]

Each safety requirement carries "a unique identification remaining unchanged throughout the safety lifecycle; a status; an ASIL" [15 §6.4.2.5], and the requirement *set* must be hierarchical, complete, externally consistent, non-duplicated within a level and maintainable [15 §6.4.3.1].

The hierarchy the standard itself draws: HARA → safety goals → functional safety requirements (FSC) → technical safety requirements (TSC) → hardware safety requirements / software safety requirements [15 Figure 2]. Part-specific obligations:

| Level | Obligation | Edition/source |
|---|---|---|
| Concept | "At least one functional safety requirement shall be specified for each safety goal"; FSRs allocated to preliminary architectural elements; ASIL inherited or decomposed | 2011 §8.4.2.2 (2018: FSC = Part 3 clause 7) [18; 14 Table 1] |
| System | Technical safety requirements allocated to system design elements; design shall implement them; TSRs allocated "directly or by further refinement to hardware, software or both" | 2011 §7.4.1.2–.3, §7.4.5.1 (2018: TSC = Part 4 clause 6) [17; 13] |
| Software | Software safety requirements derived from the TSC and system design; verified for compliance/consistency with the TSR; **NOTE: "This implies bi-directional traceability between the software architectural design and the software safety requirements."** Clauses 6–11 stable across editions | 2011 §6.4.2, §6.4.8, §7 [16; 12] |
| Hardware | "The traceability between the hardware safety requirements and their implementation shall be [ensured]… NOTE The traceability is not required down to hardware detailed design" | 2011 §7.4.1.5, §7.4.5.3 [19] |
| Assessment | Functional safety assessment judges whether "requirements management (see ISO 26262-8:2018, Clause 6), **including bidirectional traceability**, is adequately implemented" | 2018 §6.4.12.7 Note 2 [14] |

**Audit mechanics.** Unlike ASPICE, ISO 26262 puts an *independence* requirement on the people who verify the trace and the safety argument: I0 (different person) → I1 (different person) → I2 (independent of the creating team, i.e. not reporting to the same manager) → I3 (independent in management, resources and release authority). Confirmation reviews and the functional safety assessment are graded on that scale, e.g. functional safety assessment at I0 for ASIL B, I2 for ASIL C, I3 for ASIL D, with **no requirement at QM or ASIL A** [14 §6.4.12.1, Table 1]. The assessment must be "performed progressively" and "finalized before the release for production", and its report recommends acceptance, conditional acceptance, or rejection [14 §6.4.12.3, §6.4.12.9–11].

**Evidence artefacts** (from the part texts read): safety plan; HARA and safety goals; FSC; TSC; HSI specification; hardware safety requirements specification + verification report; software safety requirements specification + verification plan/report; safety case; confirmation-measure reports; DIA (development interface agreement for distributed development) [14 §6.4.6–6.4.12; 15 §§6–9, §5].

### 3.4 How the two interact

They are complementary and both are usually required by OEMs: ASPICE is a **process capability** assessment, ISO 26262 is a **product safety** lifecycle with its own evidence and independence structure. The VDA Guidelines map clause-by-clause where the two touch (verification terminology ↔ ISO 26262-6 clause 9; HWE.3/HWE.4 ↔ ISO 26262-5 clause 10; requirement characteristics ↔ ISO 26262-8:2018; prioritisation/aggregation weighting ↔ ISO 26262 risk class) [4]. There is **no single normative mapping table** in either document; any "ASPICE ⇒ ISO 26262" mapping seen in vendor material is an interpretation [8] (secondary) — flagged as inference.

---

## 4. Regime 2 — DO-178C / ED-12C

### 4.1 Level sets the workload; the PSAC sets the contract

DO-178C is objectives-based: the software level (A–E), derived from the failure-condition classification, selects which of the Annex A objectives apply and which must be satisfied "with independence". Objective totals reported by public sources: 71/69/62/26/0 objectives at Levels A/B/C/D/E, with 30/18/5/2/0 requiring independence [38; 28]. **These per-level counts are secondary-sourced; the authoritative tables (A-1…A-10) are paywalled [23].** Lane: treat counts as indicative, not normative.

The **PSAC (Plan for Software Aspects of Certification)** is the agreed evidence plan — "Once approved, this PSAC represents an agreement between the applicant and the customer and/or certification authority" [37] — and the **SAS (Software Accomplishment Summary)** closes the loop by showing each planned objective was achieved. FAA AC 20-115D recognises DO-178C and its supplements (DO-330 tool qualification, DO-331 model-based, DO-332 OO, DO-333 formal methods) [21], as does the RTCA document set published for DO-178C [26], and the AC requires: "You should satisfy all of the objectives associated with the software level assigned to the software and develop all of the associated life cycle data demonstrating satisfaction of the applicable objectives, as listed in the ED-12C/DO-178C Annex A tables" [21, read directly].

### 4.2 Six bidirectional trace pairs — and the reason for bidirectionality

> "DO-178C emphasizes that two-way or bi-directional traceability is required between 1) system requirements (allocated to software) and high-level requirements, 2) high-level requirements and low-level requirements, 3) low-level requirements and source code, 4) software requirements and test cases, 5) test cases and procedures, and 6) test procedures and test results." [22, read directly]

The stated purpose of the backward direction is concrete: it "assures that orphan source code and dead source code are not inadvertently produced" [22]. In DO-178C the overarching statements sit in **§5.5** (development traceability) and **§6.5** (verification traceability, described as new in DO-178C), with objective-level rows at §6.3.1.f (HLR→system requirements), §6.3.2.f (LLR→HLR) and §6.3.4.e (source code→LLR) [29; 39; 28]. **Verbatim clause text is not publicly available; the section numbers are secondary-sourced.**

### 4.3 Traceability is not coverage

This is the single most useful structural fact in DO-178C for teams elsewhere. The regime separates:

- **Traceability** — the artifact-to-artifact association exists, in both directions (§5.5, §6.5).
- **Requirements coverage** — every requirement has test cases and every test procedure ran (Table A-7 objectives 3–4, §6.4.4.a/b).
- **Structural coverage** — the tests exercised the code adequately: MC/DC at Level A, decision coverage at A/B, statement coverage at A/B/C, plus data and control coupling, all *with independence* [28; 24; 25].
- **Object-code-to-source traceability** — at Level A, if a compiler/linker generates code not directly traceable to source, that code must be verified additionally; the planning process must provide a means to detect it [22; 31].

A complete traceability matrix therefore satisfies **part of one** of these obligations. MC/DC's presence in DO-178C at Level A is verified by an FAA research report [24] and a NASA tutorial [25]; the per-level pairing is secondary-sourced.

### 4.4 Independence and tool qualification: the two cost drivers

**Independence.** "[T]he phrase 'with independence' refers to a separation of responsibilities where the objectivity of the verification … processes is ensured by virtue of their 'independence' from the software development team. For objectives that must be satisfied with independence, the person verifying the item … may not be the person who authored the item and this separation must be clearly documented." [38 — the Wikipedia DO-178C article's quotation of the definition; the standard itself is paywalled, so this is `[S]` provenance for a nominally verbatim definition]. Independence rows live in Tables A-3…A-7 and A-9, concentrated at Levels A and B [40; 28] `[S]`.

**Tool qualification.** Tools whose output removes the need for human verification or development activity must themselves be qualified under DO-330. The three qualification criteria distinguish: Criterion 1 — the tool's output is part of the airborne software (e.g. a code generator), so the *tool* must be qualified; Criterion 2 — a verification tool whose output is used without the software being independently verified, i.e. a tool that could fail to detect an error; Criterion 3 — a tool whose output is independently verified. Qualification levels run TQL-1 (most demanding, essentially development-level rigour) down to TQL-5; the DO-178C research file records **Criterion 2 → TQL-4 at Levels A/B and TQL-5 at C/D, and Criterion 3 → TQL-5** [30; 31] `[S]`. The directional point survives the detail: *the more a team wants a tool to substitute for human verification, the more evidence the tool itself must carry* — the inverse of the usual commercial incentive. **Whether that trade pays for itself is not measured anywhere in the sources read** (see §8.2 item 4).

**Cost.** Reported figures: DO-178B raised development costs 20–40% and DO-178C can exceed it [35, conference abstract, snippet-level]; vendor case studies report 18–36 months and several $M for Level A/B civil programmes [36, vendor]. Both are single-source and should be treated as order-of-magnitude anecdotes, not benchmarks.

---

## 5. Regime 3 — IEC 62304, ISO 14971 and FDA

### 5.1 IEC 62304 is a process standard with a class-scaled V-model

Current edition: **IEC 62304:2006 + A1:2015 (Edition 1.1 consolidated)**; FDA recognises this edition as consensus standard 13-79 [44; 45]. Software safety class is assigned from the possible harm *before* risk controls: **A** = no injury possible, **B** = non-serious injury possible, **C** = death or serious injury possible; an item that can contribute to a hazardous situation is at least B, and an item that is the sole control preventing serious injury or death is C [47; 51].

The class decides how far down the V the manufacturer must go [46; 50]:

| Activity | Clause | A | B | C |
|---|---|---|---|---|
| Development planning | 5.1 | ● | ● | ● |
| Requirements analysis | 5.2 | ● | ● | ● |
| Architectural design | 5.3 | — | ● | ● |
| Detailed design | 5.4 | — | (5.4.1) | ● |
| Unit implementation / verification | 5.5 | ● / — | ● / ● | ● / ● |
| Integration & integration testing | 5.6 | — | ● | ● |
| System testing | 5.7 | ● | ● | ● |
| Release | 5.8 | ● | ● | ● |

*(Class applicability read from training material that reproduces the standard, corroborated by a second tertiary source (the Wikipedia summary); the normative Annex A.1 table is paywalled, so this is not an independent verification of the normative table. Flagged as secondary.)*

### 5.2 Where the trace actually lives in IEC 62304

Four clause locations matter, and they are easy to confuse:

1. **§5.1.1(c) — the plan must name the traceability.** The software development plan shall address traceability between system requirements, software requirements, software-system test, and risk-control measures implemented in software [46 §5.1].
2. **§5.2.6 — requirements verified, including traceable to source.** Documented verification that software requirements (a) implement the parents *including risk-control measures allocated to software*, (b) are non-contradictory, (c) unambiguous, (d) **verifiable**, (e) **uniquely identified**, and (f) **traceable to parent specifications or another source** [46; 48].
3. **§5.7.1 — tests covering all requirements.** "Establish tests for software requirements": tests with input stimuli, expected outcomes, pass/fail criteria and procedures, covering **all** software requirements [46].
4. **§5.7.4 — the recorded trace.** "Evaluate software system testing" produces *recorded traceability between requirements and tests* plus evidence that results met the pass/fail criteria; §5.7.5 fixes the test-record contents (case reference, result and anomalies, software version, hardware/software configuration, tools, date, tester) [46].

**The chain from risk to test** runs: ISO 14971 hazard/hazardous situation → risk-control option → if implemented in software, becomes a software safety requirement (§7.2.2, §5.2.3 — B/C only) → verified traceable to source (§5.2.6) → covered by tests (§5.7.1) → traceability recorded (§5.7.4) → risk file documents hazardous situation → software item → cause → control → verification (§7.3) [49; 47]. **The exact §7.3 sub-clause number carrying the traceability sentence is unconfirmed (reported variously as 7.3.3 and 7.3.6) — see §10.**

Release adds two evidence items non-safety teams should steal: §5.8.2 requires known **residual anomalies** to be documented (all classes), and §5.8.3 requires them to be **evaluated for contribution to unacceptable risk** (B/C) [46].

### 5.3 FDA: traceability as a documentation tier, not a certificate

FDA does not mandate IEC 62304 by law; it recognises it as a consensus standard and accepts a **Declaration of Conformity to specific IEC 62304 clauses** (§5.1.1–5.1.3, 5.1.6–5.1.9, Clause 6, Clause 8) in place of narrative process documentation [41 §VI.G].

The 2023 guidance "Content of Premarket Submissions for Device Software Functions" (final, 14 June 2023) replaces the 2005 three-tier "Level of Concern" with a **two-tier documentation level**:

> "Enhanced Documentation should be provided for any premarket submission that includes device software function(s) where a failure or flaw of any device software function(s) could present a hazardous situation with a probable risk of death or serious injury… Basic Documentation should be provided … where Enhanced Documentation does not apply." [41 §V]

Traceability appears in four distinct places in that guidance [41]:

| Location | Requirement |
|---|---|
| §VI.D SRS | "Each requirement identified in the software requirements specification should be traceable"; the submission should include "a description of the software requirement identification and tracking methodology used to support the traceability of the requirements" |
| §VI.C Risk management file | Verify implemented risk control measures by tracing "the identified hazard to the verified specific risk control measures (e.g., a requirement ID in the SRS and SDS, a test name and identifier in the testing documentation that shows pass/fail results, a user manual name and identifier, a training manual name and identifier)" — with a worked example `HAZ-XXX → SRS-XXX → SDS-XXX → UT-XXX → INT-XXX → SYS-XXX`. FDA explicitly permits "a separate document linking together software requirements specifications, software design specifications, testing and identified hazards" when inline presentation is cumbersome |
| §VI.G Development/CM/maintenance | Enhanced level requires "Processes, procedures and tools used to link user needs, system requirements, software requirements, software design specifications, software testing and implemented risk control measures (i.e., traceability)" |
| Verification definition | software verification includes "traceability analyses (e.g., software requirements specification to software design specification (**and vice versa**), source code to software design specification (**and vice versa**), and test cases to source code and to software design specification)" |

Two further FDA-era facts: the earlier *General Principles of Software Validation* (2002) already required a requirements traceability analysis "to (and from) system requirements and to risk analysis results" and a design traceability analysis [42]; and the **QMSR** (21 CFR 820, effective **2 February 2026**) incorporates **ISO 13485:2016 by reference** [43]. Note the layering: IEC 62304's *normative* reference is ISO 14971 (risk management) [49]; its relationship to ISO 13485 (quality management) is an **informative Annex C** relationship, not a normative reference [47]. The FDA QMSR is what puts ISO 13485 into US regulatory force for device manufacturers.

**Regulatory status of the standard itself:** IEC 62304 **Edition 2** was still in draft as of this writing (committee-draft stage; publication reported as delayed, with a 2028 date cited by a consultancy). Manufacturers should treat Edition 1.1 as current [52; 51].

---

## 6. Comparison matrix

Dimensions chosen because they are the ones that change a team's cost model. Labels per §2 item 5: `[P]` = read from a source that publishes the text; `[P*]` = read from a publicly posted copy of a paywalled standard (ISO 26262 — 2011 clause text read; 2018 official TOC read but 2018 normative sentences not); `[S]` = clause-level claim resting on vendor/consultancy/training secondary sources because the standard is paywalled.

| Dimension | ASPICE (PAM 4.0/4.1) | ISO 26262:2018 | DO-178C | IEC 62304 + FDA |
|---|---|---|---|---|
| **Why trace** | Rate whether the process ensures consistency and coverage [P] | Build the safety argument from goals to realisation; support impact analysis and the FSA [P*] | Demonstrate to the certification authority that requirements are implemented and code is intended; avoid orphan/dead code [P] (six bidirectional pairs, NASA paper) | Identify requirements, verify risk controls, support the submission and the risk file [P] (FDA) / [S] (IEC) |
| **Anchor artefact** | The work-product set of each process [P] | The safety-requirement hierarchy (SG→FSR→TSR→HW/SW req) [P*] | The requirement hierarchy + §11 lifecycle data [S] (the 11.1–11.20 list used here is DO-178B-derived) | The SRS + risk-management file + test records [P] (FDA) / [S] (IEC) |
| **Direction** | Bidirectional (one documented exception: legal/homologation sources in VAL.1) [P] | Bidirectional per §6.4.3.2; hardware traceability stops above detailed design [P*] | Bidirectional across six pairs [P] (NASA paper) | IEC: requirement→source and requirement↔test recorded, **direction not stated in the sources read** [S]. FDA: "software requirements specification to software design specification (**and vice versa**)" and "source code to software design specification (**and vice versa**)"; test cases to source code and to design are stated in one direction in that sentence [P] |
| **Granularity** | Cluster-level explicitly acceptable and protected from downrating [P] | Requirement-level; each requirement atomic, uniquely identified, with status and ASIL [P*] | Requirement-level (HLR/LLR) plus source-code level; object-code level at Level A [S] | Requirement-level (IEC §5.2.6 unique IDs, §5.7.1 covers all) [S] |
| **Coverage metric beyond the trace** | None prescribed; verification measures ↔ requirements is the coverage argument [P] | Structural coverage (statement/branch/MC-DC) by ASIL for unit verification — **single secondary source, uncorroborated** [S] | Requirements coverage + structural coverage (statement/decision/MC-DC) + coupling, with independence [S] | Tests covering all requirements; no structural-coverage requirement [S] |
| **Independence of verification** | Not a PAM concept **for verification** — process capability is rated, not the verifier's independence; QA independence *is* required (SUP.1.BP1) [P] | I0–I3 graded by ASIL; FSA at I0 (ASIL B: "should … by a different person"), I2 (ASIL C), I3 (ASIL D), none at QM/ASIL A [P*] | Required for a subset of objectives, concentrated at Levels A/B; §6.2-e definition [S] | Not established in the sources read; §5.5 unit verification is process-scaled and independence is not a class differentiator [S] — open question |
| **Tool qualification** | Explicitly none: "A PAM does not predefine any methodology/approach or tools" [P] | Part 8 **Clause 11 "Confidence in the use of software tools"**, incl. §11.4.6 qualification of a software tool [P*] | DO-330 with TQL-1…TQL-5; heavier for tools that substitute for verification [S] | No tool-qualification regime; SOUP items are specified and assessed (5.3.3/5.3.4, 8.1.2) [S] |
| **Evidence mechanism** | Assessment: sampling work products + testimony, with plausibility checks on document freshness [P] | Confirmation reviews + functional safety assessment + safety case [P*] | PSAC agreed before development; SOI reviews; SAS closes — mechanism [S], regulator recognition of DO-178C [P] | Premarket submission documentation set + QMS audits/inspections [P] (FDA) / [S] (IEC) |
| **Change impact** | SUP.10: CR ↔ affected work products, CR ↔ problem reports [P] | Part 8 clause 8 change management; affects the safety case [P*] | CM + problem reports + regression re-run of impacted tests [S] | IEC 8.2 change control, 5.6.6 regression, 5.8.3 residual-anomaly safety evaluation [S]; FDA lists change management among expected practices [P] |
| **Risk-tiering scale** | ASIL spectrum must appear in the assessment sample set [P] | QM / ASIL A–D [P*] | Software Levels A–E defined by failure condition [S]; the AC's requirement to satisfy the objectives "associated with the software level assigned" [P] | Class A/B/C [S]; FDA Basic vs Enhanced documentation level, Enhanced where failure could present "a hazardous situation with a probable risk of death or serious injury" [P] |

Where a cell carries two labels, the claim rests on the weaker element for the clause-level portion. **ISO 26262 cells carrying `[P*]` derive from publicly posted copies (Part 2: 2018 edition; Parts 3–8: 2011 edition) plus the 2018 official tables of contents; the 2018 normative sentences of Parts 3–8 were not read** (see §2 item 5 and §10). No ISO 26262 cell should be quoted as 2018 clause text without re-verification against a licensed copy.

**Reading of the matrix.** The regimes agree on the shape of the spine (requirement → verification measure → verification result, plus change request ↔ affected artefacts) and on risk-tiering, while **differing on the directionality of the requirement-side links** (IEC 62304's sourced formulations do not state bidirectionality). They also diverge on (i) whether coverage beyond the trace is measured, (ii) whether the verifier must be independent of the author, (iii) how far tools must themselves be proven, and (iv) whether the evidence is assessed, certified, or submitted. Those axes are exactly the ones a team outside these industries should choose deliberately rather than inherit.

---

## 7. Edition and numbering traps

Anyone reusing clause numbers from a supplier template or an old checklist will hit these [14; 13; 12; 11; 1; 2]:

| Trap | Detail |
|---|---|
| **ISO 26262 Part 3** | HARA was clause 7 in 2011 and is **clause 6** in 2018; the FSC was clause 8 and is **clause 7** in 2018 |
| **ISO 26262 Part 4** | Clause 6 in 2011 was "Specification of the technical safety requirements"; in 2018 clause 6 is the **Technical safety concept**, and the functional safety assessment moved to Part 2 §6.4.12 |
| **ISO 26262 Parts 6 and 8** | Clause numbering appears stable (Part 6: 5–11; Part 8: 5–16) |
| **ISO 26262 hardware** | Traceability is explicitly **not** required down to hardware detailed design — a deliberate scoping decision, not an omission |
| **ASPICE 3.1 → 4.0** | Two BPs (traceability, consistency) merged into one; "test" terminology replaced by "verification measures/results"; SWE.5/SWE.6 renamed; HWE/MLE/VAL process groups added; strategy BPs moved to GP 2.1.1 |
| **ASPICE 4.0 → 4.1** | Traceability BPs unchanged (own text comparison, artifact `notes/regulated-trace/aspice-pam-4{0,1}.txt`); change list limited to MLE.2 BP5 removal and editorial fixes [3] |
| **ASPICE capability level tables** | v3.1 requires "Largely" for level-defining attributes; v4.0/4.1 permit "largely or fully" |

---

## 8. What actually transfers — and what does not

### 8.1 Practices worth adopting, with mechanism, cost and evidence strength

| Practice | Mechanism | Cost | Evidence strength | Source basis |
|---|---|---|---|---|
| **Unique IDs + typed links** | Stable identifiers and link types that encode meaning (derives / verifies / satisfies) | Low upfront; ongoing discipline | Moderate — link-type catalogue derived from 26 organisations; traceability completeness statistically associated with lower defect rate in 24 OSS projects | [55; 57; 65] |
| **Bidirectional trace with suspect-link invalidation on change** | When an upstream item changes, downstream links are flagged suspect and must be explicitly revalidated | Tooling + the discipline to clear the flags | Strong as a *mechanism* (documented by every major ALM vendor); **no study was found that tests this mechanism's outcome**. The adjacent controlled experiment tested trace *navigation* on change tasks, not suspect-link invalidation [56], and effectiveness in practice is unmeasured (no vendor publishes how often suspect links are cleared) | [62; 63; 64; 56] |
| **Requirement → test case → test result trace, at requirement-or-cluster granularity** | Coverage measured against requirements, not only against code | Moderate; test-side discipline | Moderate — ASPICE SYS.5/SWE.6 base practices mandate a requirement↔verification-measure trace (cluster-level explicitly permitted and protected); IEC 62304 §5.7.4 mandates recorded requirement↔test traceability; FDA explicitly asks for hazard→SRS→SDS→test IDs | [2; 46; 41 §VI.C] |
| **Risk-tiered test depth** | Assign a tier per item (ASIL / level / class / Documentation Level) and scale technique and coverage accordingly | Higher cost concentrated on high-risk items | Moderate — every regime ties rigor to a risk tier, though the tiers differ and ASPICE's is an assessment-sampling rule rather than a testing-rigor rule; no study found comparing tiered vs uniform rigor outcome | [41 §V; 14; 2] |
| **Graded verification independence** | Independence level scales with blast radius: same person → different person → different team → different management chain | People/scheduling | Moderate (standards practice); weak empirically — no controlled comparison found | [14 Table 1; 28] |
| **Scope discipline: cluster-level traceability** | Trace clusters of requirements → clusters of design elements → clusters of tests; don't trace atomically when a cluster suffices | *Negative* cost | Moderate — ASPICE guidance protects cluster-level tracing from downrating, and the PAM states redundant traceability is not intended | [4 TAC.RL.1; 2 SWE.1.BP5 Note 9] |
| **Evidence emitted at the time of work** | Build/test/verify pipelines emit provenance and logs as artifacts instead of reconstructing documents pre-audit | Automation engineering | Anecdotal-to-moderate — SSDF frames artifacts as records of the process and SLSA defines build-provenance levels, but neither source measures adoption or ROI | [67; 68] |
| **Residual-anomaly register with impact evaluation** | Every known unresolved anomaly is listed with a risk-based rationale for shipping | Low | Anecdotal — mandated by IEC 62304 §5.8.2/5.8.3 and FDA element J; no outcome study found | [46; 41 §VI.J] |
| **Change-impact propagation from requirement to tests** | On change, enumerate affected tests and require re-run or explicit waiver | Tooling + process | Moderate mechanism; ASPICE SUP.10 and ISO 26262 Part 8 clause 8 require the CR↔work-product trace | [2 SUP.10.BP4; 15 §8] |
| **Lightweight assurance case (GSN)** | Structured goal→strategy→evidence argument with explicit defeaters, replacing a document pile | Skill cost; upkeep | Moderate as methodology — GSN Community Standard v3 is an open specification from the SCSC Assurance Case Working Group; no outcome study found and no peer-review status was verified | [69] |
| **Plausibility checks on evidence freshness** | Reject documents whose timestamps/change history look reconstructed; require a reason for late documentation | Low | Moderate — official assessment rule, i.e. evidence that auditors already treat back-filled documentation as a signal | [4 §1.4.3.2] |

### 8.2 What should not transfer — and why

1. **MC/DC at Level A.** It exists because a catastrophic failure condition justifies exhaustive condition-level testing of complex boolean logic [24; 25]. The cost grows superlinearly with condition count, and the fault class it targets is not the dominant one for a web service. *Inference, not measurement:* the "expensive" claim is supported directionally by the FAA/NASA material and by the existence of alternative formal-methods routes (DO-333) [34]; no cost curve was found.
2. **The full DO-178C lifecycle data set** (PSAC, development/verification/CM/QA plans, requirements and design data, verification cases/procedures/results, configuration indices, problem reports, accomplishment summary) [27]. Its justification is certification of software with catastrophic potential; reproduced for a SaaS product it produces documents nobody reads and slows delivery (inference).
3. **ASPICE capability-level assessment.** An assessment consumes assessor time and yields an organisational maturity rating used for OEM supplier qualification. The published empirical ASPICE work measures *process* weaknesses, not product defect reduction [9; 10].
4. **Formal tool qualification** (DO-330 data sets). The proportional alternative — independently verify the tool's output rather than qualifying the tool — is argued to capture most of the assurance at lower cost; the sources cited document tool-qualification *pitfalls* for COTS tools and the difficulty of qualification across standards, **not a measured assurance-per-unit-cost comparison** [32; 33]. Treat the cost claim as an inference.
5. **Exhaustive review records.** The Guidelines themselves list alternative evidence (pair working, peer spot checks, revision history, change commenting) as acceptable [4 TAC.RL.3] — *inference:* the objective a document trail optimises (audit survivability) is not the same as the objective a team usually wants (defect removal).

**Honest caveat.** "Should not transfer" is a *scoping* argument. Any of these becomes rational if the software can cause serious harm, if a regulator requires it, or if a customer contract mandates it.

---

## 9. What the evidence says about practice — and where the evidence is missing

### 9.1 Empirical findings (all with sample sizes)

| Finding | Study | Sample | Read depth |
|---|---|---|---|
| Trace navigation improved **performance, quality and workflow** on change tasks | Mäder & Egyed (journal issue 2015; online 2014), EMSE 20(2) | Controlled experiment, >50 subjects [56] | abstract + related-work summary (paywalled paper) |
| **More complete traceability → lower expected defect rate** for 3 of 4 studied implementation-support activities | Rempel & Mäder, IEEE TSE 2016 | 24 medium-to-large open-source projects, multi-level Poisson regression; replication data on Harvard Dataverse (DOI 10.7910/DVN/PLCBZV) [57] | abstract |
| The hardest traceability problems are **pre-requirements-specification**: origin, rationale, decisions | Gotel & Finkelstein (ICRE 1994; index record dates the IEEE reissue 2002) | Empirical studies with >100 practitioners [54] | abstract |
| Link types are empirically derived; organisations split into **"low-end"** (mandate-driven) and **"high-end"** (value-driven) users | Ramesh & Jarke, IEEE TSE 27(1) 2001 | Focus groups/interviews in 26 organisations [55] | abstract |
| Adoption is driven by **cost/benefit ratio and concrete use cases**, not headcount | Ruiz, Hu & Dalpiaz, RE 2023 | 55 survey respondents from 13 countries + 14 interviews [58] | full text (large portion) |
| Failures cluster in **maintenance** and **cross-team/cross-tool boundaries** | Wohlrab et al., RE 2018 | 15 industrial projects [59] | snippet (via a citing paper's summary) |
| In ASPICE assessments, requirements-management weaknesses recur; the lowest-rated requirement-engineering base practice was **"develop verification criteria"** (SWE.1.BP5, a v3.1-era BP) | Falcini & Lami, IEEE RE 2020 (assessor-authored) | Assessments across multiple organisations worldwide [9] | abstract; full text returned HTTP 403 |

**Methodological caution.** The traceability corpus is dominated by papers *proposing link-recovery techniques* rather than measuring industrial outcomes, and it lacks field trials; reporting quality is weak and industrial case studies scarce [60; 61].

### 9.2 Explicitly not found (do not cite these as facts)

- **No measured stale-link rate** in any named industrial corpus. "Links quickly become outdated" is asserted as a motivating premise [58] but has no measured magnitude.
- **No ROI figure** for traceability in currency or person-hours; no meta-analysis or randomised field trial.
- **No published frequency data** for ASPICE traceability nonconformities, ISO 26262 nonconformities, or FAA findings that specifically cite traceability.
- **No public dataset** isolating FDA citations for *software traceability* (as opposed to design-controls citations generally). FDA's 483 database and citation dataset exist and are queryable [53], but no such analysis was found.
- **The terms "traceability theater" and "trace explosion" are not established in the literature.** Searches returned non-authoritative content only. Use the sourced analogues instead: the "traceability benefit problem", "low-end" compliance behaviour [55; 58], and maintenance/cross-boundary breakdown [59].
- **No evidence** that "document-level traceability instead of requirement-level" or "tracing to a test plan rather than test cases" are documented failure modes — plausible, but unsourced.
- **No measurement** of how often suspect links are ever reviewed or cleared, despite every major ALM vendor implementing the mechanism [62; 63].

### 9.3 Tooling: what exists (mechanisms only)

| Tool family | Link model | Change propagation |
|---|---|---|
| Jama Connect | Directional upstream/downstream relationships covering horizontal and vertical V-model trace | Upstream change marks the relationship suspect; explicit "clear suspect links" workflow; coverage/trace view for gap analysis ("requirement lacking verification") [62] |
| PTC codebeamer | Tracker-item associations; Traceability Browser/report | "Trigger suspected links": a change marks dependents suspected; suspect badges propagated in the traceability report [63] |
| IBM DOORS Next / DOORS | Artifact relationships; traceability tree/grid views; object-history-driven suspect flags; baselines | Suspect-link invalidation and a "requirements covered by test cases" metric; documentation was reachable to a human browser but returned HTTP 403 to automated fetching in this run [64 — verification note] |
| Polarion | Work-item traceability model with derivation menus; Resource Traceability SDK linking work items to source-code entities | "Auto Suspect" propagation on save (vendor community documentation, weaker than the above) [secondary] |
| Open standards | **ReqIF** (OMG, v1.2, 2016) for requirement exchange with GUIDs; **OSLC** Core 3.0 / RM 2.1 / QM 2.1 / CM 3.0 (OASIS, 2020) for cross-tool linking | Exchange and cross-tool reference, not propagation [65] |
| Open source | Eclipse Capra (last release 0.8.2, 2021-05): adapters for UML/SysML/AADL/EAST-ADL, ReqIF, CI test executions, source code, issues; renders matrices "as requested by standards like ISO 26262" | Notifies on moved/deleted/renamed linked elements with semi-automatic fixes [66] |

**Note:** ReqIF was checked for an ISO number because the task brief asserted ISO/IEC 23396; **no evidence of that ISO designation was found** — ReqIF is documented only as an OMG specification [65].

### 9.4 Adjacent frameworks worth borrowing from

- **ISO/IEC/IEEE 29119** — internationally agreed test process and documentation templates ("test case ↔ requirement ↔ result" records without a safety standard) [71].
- **NIST SSDF (SP 800-218 v1.1)** — secure-SDLC practices in which artifacts "provide records of secure software development practices"; the closest non-certification evidence-as-artifact framework [67].
- **SLSA v1.2** — build provenance levels L1–L3 ("what entity built the artifact, what process, what inputs"), the supply-chain analogue of a trace link [68].
- **EU AI Act, Article 12** — high-risk AI systems must "technically allow for the automatic recording of events (logs) over the lifetime of the system… to ensure a level of traceability", plus Article 11 technical documentation. This is a live, legally binding traceability obligation for a new class of systems [70]. (ISO/IEC 42001 could not be verified from a primary page in this run.)
- **GSN Community Standard v3** — the open assurance-case notation, if a team wants the *argument* structure without the document set [69].

---

## 10. Open questions and unverified items

**Open questions a reader should treat as unresolved**

0. **Reviewer-flagged items that remain open (MAJOR, mitigated but not closed).** (a) ISO 26262 structural coverage by ASIL rests on one self-labelled paraphrase source and is now marked as such wherever used; a licensed reading of Part 6 unit verification is needed to close it. (b) IEC 62304's requirement→source and requirement↔test formulations do not state bidirectionality; treat IEC 62304 bidirectionality as *unestablished* pending a licensed reading of §5.2.6 and §5.7.4. (c) DO-178C objective counts, Annex A row mappings and the §5.5/§6.5 section numbers remain vendor/secondary-sourced. (d) The PAM 4.0→4.1 comparison is a three-BP sample, not an exhaustive diff.

1. Does any regime require *verification independence* under IEC 62304? The sources read scale verification activities by class but do not establish an independence requirement; this needs a licensed reading of §5.5 and Annex A.1.
2. What proportion of trace links in industrial projects survives a release cycle unchanged? No measured stale-link rate was found in any corpus.
3. Does suspect-link invalidation actually change behaviour? Every major vendor implements it; nobody publishes how often suspect links are cleared.
4. Do risk-tiered test depths (ASIL/DAL/class/Documentation Level) outperform uniform rigor on cost-adjusted defect removal? No comparative study found.
5. Which ISO 26262-8:2018 sentences correspond to the 2011 §6.4.3.2 quote? The 2018 text is authoritative and was not directly readable here.

**Unverified / could not confirm (carried forward from the research files)**

| Item | Status |
|---|---|
| ISO 26262-8:2018 §6.4.3.2 exact wording | 2011 wording read from a public copy; 2018 retains clause 6.4.3 per the official TOC, but the 2018 normative sentence was not read |
| ISO 26262-3:2018 clause numbering (HARA = 6, FSC = 7) | Inferred from ISO 26262-2:2018 Table 1, not read from the Part 3 TOC |
| ISO 26262-5/6/7:2018 normative sentences | 2011 text read; clause numbers corroborated as stable only for Part 6 |
| IEC 62304 §7.3 sub-clause number for risk traceability | Reported as 7.3.3 and 7.3.6 by different secondary sources; conflicting — unconfirmed |
| IEC 62304 Annex A.1 normative table | Paywalled; class applicability taken from training material and corroborated by a second tertiary source — not an independent verification of the normative table |
| DO-178C Annex A objective counts per table and per level (71/69/62/26/0; 30/18/5/2/0) | Secondary/vendor sources; the Annex A tables themselves are paywalled |
| DO-178C §11.x numbers for newer data items (Trace Data, PDIF) | 11.1–11.20 list is DO-178B-derived |
| IEC 62304 Edition 2 publication date | Draft stage; a 2028 date is cited by a consultancy, one vendor claims 2027 — not firm |
| Whether IEC 62304 is listed as an EU harmonised standard under MDR | Not verified against the Official Journal |
| ISO/IEC 42001 primary page | iso.org returned HTTP 403 during research |
| ReqIF = ISO/IEC 23396 | No evidence found; do not assert |
| Vendor e-signature / Part 11-style immutable audit trails | No vendor documentation retrieved; not asserted |
| IBM DOORS Next documentation URL | Reachable in a browser; HTTP 403 to automated fetch — the mechanism claim rests on vendor documentation that was read by the researcher |

**Blocked capabilities (recorded as required).** Web search providers were rate-limited or key-blocked at points (Exa 429, Brave/Jina missing keys, DuckDuckGo bot check); two paywalled-standard mirrors returned HTTP 403 (CNR full text of an ASPICE requirements-management paper, EASA CM-SWCEH-002); ISO and FAA PDFs rejected one fetcher with HTTP 403 and were retrieved by `curl` instead. Where a claim depended on a blocked source, it is marked secondary/snippet-level above.

---

## 11. Sources

Numbering matches the inline citations `[n]`. Each entry keeps its short source tag in brackets for traceability to the research files. URL status was checked on 2026-10-18; notes record only exceptions.

**Automotive SPICE**
1. [PAM31] Automotive SPICE PAM v3.1, VDA QMC (2017-11-01) — https://vda-qmc.de/wp-content/uploads/2023/02/Automotive_SPICE_PAM_31_EN.pdf — live.
2. [PAM40] Automotive SPICE PAM v4.0, VDA QMC WG13 (2023-11-29) — https://vda-qmc.de/wp-content/uploads/2023/12/Automotive-SPICE-PAM-v40.pdf — live.
3. [PAM41] Automotive SPICE PAM v4.1, VDA QMC WG13 (2026-08-24) — https://vda-qmc.de/wp-content/uploads/2026/09/Automotive-SPICE-PAM-v41.pdf — live; title, version "4.1", date 2026-08-24 and the revision list ("Removal of MLE.2 BP5; editorial changes…") confirmed by direct fetch.
4. [ASG20] Automotive SPICE Guidelines v2.0 (Blue-Gold Book), VDA QMC (2023) — https://vda-qmc.de/wp-content/uploads/2023/06/BGB_A-SPICE-Guidlines-2.0_V8.pdf — live.
5. [ASG21] Automotive SPICE Guidelines v2.1 (Yellow Volume), VDA QMC (2026) — https://vda-qmc.de/wp-content/uploads/2026/03/ASPICE_BGB_2.1_Yellow-Volume_final-2.pdf — live; v2.1 restates the consistency/traceability rules of 4.
6. [VDApub] VDA QMC Automotive SPICE publications page — https://vda-qmc.de/en/automotive-spice/automotive-spice-veroeffentlichungen/ — live; confirms PAM 4.1 (August 2026), PAM 4.0 (29.11.2023), PAM 3.1 (01.11.2017) and Guidelines 3rd ed. (July 2026).
7. [ULPG] UL Solutions, Automotive SPICE Pocket Guide, PAM v4.0 (2024) — https://www.ul.com/sites/default/files/2024-10/Automotive_Spice_Pocket_Guide.pdf — live; secondary-derived confirmation of the merged BP text.
8. [UL-ASPICE44] UL Solutions / Kugler Maag, "Automotive SPICE 4.0: What is new" (2023) — https://process-insights.org/wp-content/uploads/2023/09/Kugler%20Maag%20by%20UL%20Solutions%20-%20Automotive%20SPICE%204.0.%20What%20is%20new.pdf — live.
9. [Falcini-Lami-2020] Falcini & Lami, "Requirements Management in Automotive: an Empirical Study on Process Improvement Areas", IEEE RE 2020 — https://doi.org/10.1109/RE48521.2020.00037 — DOI resolves; OpenAlex record W3091892842 matches title, first author (Fabio Falcini) and year.
10. [ICST-2021] "System and Software Testing in Automotive: an Empirical Study on Process Improvement Areas", IEEE ICST 2021 — https://www.computer.org/csdl/proceedings-article/icst/2021/683600a253/1tRP7gQ1wgE — live.

**ISO 26262**
11. [26262-8-2018-toc] ISO 26262-8:2018 official preview (front matter/TOC) — https://cdn.standards.iteh.ai/samples/68390/e2540a4426a94057bb445f81876d6049/ISO-26262-8-2018.pdf — live.
12. [26262-6-2018-toc] ISO 26262-6:2018 official preview (front matter/TOC) — https://cdn.standards.iteh.ai/samples/68388/34205953cd2c4c5f947890009caa464e/ISO-26262-6-2018.pdf — live.
13. [26262-4-2018-toc] ISO 26262-4:2018 official preview (front matter/TOC) — https://cdn.standards.iteh.ai/samples/68386/7db689a782c8467b896a59b76920f037/ISO-26262-4-2018.pdf — live.
14. [26262-2-2018] ISO 26262-2:2018 — public copy used for clause 6 (safety management, safety case, confirmation measures, FSA, Table 1 independence) — https://files.infocentre.io/files/docs_clients/126_2008096317_4226750_docu_nt_doga_ISO_26262-2.pdf — live. Canonical record https://www.iso.org/standard/90027.html — unreachable to automated fetch (HTTP 403; verified by curl with browser user-agent).
15. [26262-8-2011] ISO 26262-8:2011 — public copy used for §6.4.2/§6.4.3 traceability text, clauses 7–9 — https://www.szilassy.eu/R8/content/referredDocs/standards/ISO_26262-8.pdf — live.
16. [26262-6-2011] ISO 26262-6:2011 — public copy used for Part 6 §6.4.2/§6.4.8 and software-architecture bidirectional note — https://www.szilassy.eu/R8/content/referredDocs/standards/ISO_26262-6.pdf — live.
17. [26262-4-2011] ISO 26262-4:2011 — public copy used for TSR allocation §7.4.1.2–.3, §7.4.5.1 — https://www.szilassy.eu/R8/content/referredDocs/standards/ISO_26262-4.pdf — live.
18. [26262-3-2011] ISO 26262-3:2011 — public copy used for HARA/FSC and "at least one FSR per safety goal" — https://www.szilassy.eu/R8/content/referredDocs/standards/ISO_26262-3.pdf — live.
19. [26262-5-2011] ISO 26262-5:2011 — public copy used for hardware traceability §7.4.1.5, §7.4.5.3 — https://www.szilassy.eu/R8/content/referredDocs/standards/ISO_26262-5.pdf — live.
20. [ArtOfService-26262] TheArtOfService, ISO 26262 clause pages (self-labelled paraphrase) — https://requirements-traceability-gap-finder.theartofservice.com/standards/iso-26262/ — live.

**DO-178C**
21. [FAA-AC-115D] FAA Advisory Circular 20-115D, "Airborne Software Development Assurance Using EUROCAE ED-12() and RTCA DO-178()" (2017-07-21) — https://www.faa.gov/documentLibrary/media/Advisory_Circular/AC_20-115D.pdf — live.
22. [NASA-Jacklin] Jacklin, "Certification of Safety-Critical Software Under DO-178C and DO-278A" (NASA NTRS / AIAA 2012) — https://ntrs.nasa.gov/api/citations/20120016835/downloads/20120016835.pdf — live.
23. [FAA-diff-tool] FAA, DO-178B/C Differences Tool (Annex A table list; §11 item numbering) — https://www.faa.gov/sites/faa.gov/files/aircraft/air_cert/design_approvals/air_software/differences_tool.pdf — live.
24. [FAA-MCDC] FAA, "An Investigation of Three Forms of the Modified Condition/Decision Coverage (MC/DC) Criterion" (AR-01-18) — https://www.faa.gov/sites/faa.gov/files/aircraft/air_cert/design_approvals/air_software/AR-01-18_MCDC.pdf — live.
25. [NASA-MCDC-tut] Hayhurst et al., "A Practical Tutorial on Modified Condition/Decision Coverage" (NASA/TM-2001-210876) — https://shemesh.larc.nasa.gov/fm/papers/Hayhurst-2001-tm210876-MCDC.pdf — live.
26. [RTCA] RTCA, DO-178() product page (document set and supplements) — https://www.rtca.org/do-178/ — live.
27. [DO178B-TOC] RTCA DO-178B SC-167 table of contents (public copy, §11 data items) — https://users.ece.cmu.edu/~koopman/depend/book/rtca92.htm — live.
28. [Rapita] Rapita Systems, "How to achieve DO-178C certification" (vendor objective/DAL tables) — https://www.rapitasystems.com/do178c-certification — live.
29. [AdaCore-reviews] Appéré & Nava Bello (AdaCore), "A Fresh Take on DO-178C Software Reviews" — https://www.adacore.com/blog/a-fresh-take-on-do-178c-software-reviews — live.
30. [AdaCore-DO330] AdaCore, "DO-330/ED-215: Benefits of the new Tool Qualification Document" — https://www.adacore.com/uploads/books/do-330-ed-215-tool-qualification-document.pdf — live.
31. [DO330-tool-qual] LDRA, "DO-330 Test tool qualification for aerospace applications" (vendor) — https://ldra.com/wp-content/uploads/ldra/DO-330-Test-Tool-Qualification-Technical-Briefing-v2.0.pdf — live.
32. [DO330-pitfalls] Marques & Cunha, "COTS tool qualification using RTCA DO-330: Common pitfalls" (DASC 2017) — https://doi.org/10.1109/DASC.2017.8102031 — DOI resolves; OpenAlex record W2767916198 matches title and first author (Johnny Marques).
33. [Krauss-2015] Krauss, Rejzek & Hilbes, "Tool Qualification Considerations for Tools Supporting STPA" (2015) — https://doi.org/10.1016/J.PROENG.2015.11.500 — DOI resolves; OpenAlex record W2194630141 matches title and first author.
34. [Moy-2013] Moy et al., "Testing or Formal Verification: DO-178C Alternatives and Industrial Experience", IEEE Software (2013) — https://openalex.org/W2049777185 — live; record matches title, first author (Yannick Moy), year and IEEE Software 30(3).
35. [Hilderman-2014] Hilderman, "Understanding DO-178C Software Certification: Benefits Versus Costs", ISSREW 2014 — https://www.computer.org/csdl/proceedings-article/issrew/2014/7377a114/12OmNxw5B8k — live (conference abstract; snippet-level).
36. [Wind-River] Wind River, "Accelerating Avionics Safety Certification" case study (vendor) — https://www.windriver.com/sites/default/files/2023-10/551003_PDF_Accelerating%20Avionics%20Safety%20Certification%20IP%20Blocks%20Case%20Study.pdf — live.
37. [PSAC-sample] AirworthinessCert, PSAC sample (2022) — https://airworthinesscert.com/wp-content/uploads/2022/11/PSAC-Sample.pdf — live.
38. [Wikipedia-DO178C] Wikipedia, "DO-178C" (secondary; level/objective totals and independence definition) — https://en.wikipedia.org/wiki/DO-178C — live.
39. [ArtOfService-DO178C] TheArtOfService, DO-178C clause pages (self-labelled paraphrase) — https://compliance.theartofservice.com/controls/do-178c-ed-12c-software-considerations-in-airborne-systems/a-5-5 — live.
40. [LDRA-independence] LDRA, "Safety Assurance Through Independent Verification" (vendor) — https://ldra.com/wp-content/uploads/ldra/Safety_Assurance_through_Independent_Verification_White_Paper_v3.3.pdf — live.

**IEC 62304 / ISO 14971 / FDA / MDR**
41. [FDA-2023] FDA, "Content of Premarket Submissions for Device Software Functions", final guidance (14 June 2023) — https://www.fda.gov/media/153781/download — live. Landing page: https://www.fda.gov/regulatory-information/search-fda-guidance-documents/content-premarket-submissions-device-software-functions — live.
42. [FDA-GPSV-2002] FDA, "General Principles of Software Validation", final guidance (2002) — https://www.fda.gov/media/73141/download — live.
43. [FDA-QMSR] FDA, Quality Management System Regulation (QMSR), effective 2 February 2026 — https://www.fda.gov/medical-devices/postmarket-requirements-devices/quality-management-system-regulation-qmsr — live.
44. [FDA-62304-recognition] FDA Recognized Consensus Standards, Recognition 13-79 (IEC 62304 Edition 1.1 2015-06 CONSOLIDATED) — https://www.accessdata.fda.gov/scripts/cdrh/cfdocs/cfStandards/detail.cfm?standard__identification_no=38829 — live; entry confirms FDA-recognised edition and recognition list 051. (URL replaced during verification: the draft's generic FDA standards-programme page was swapped for the specific recognition record.)
45. [IEC-preview] IEC 62304:2006+A1:2015 catalogue record — https://webstore.iec.ch/publication/3842 — live. (Alternative working URL substituted during verification for the ANSI preview used in research, https://webstore.ansi.org/preview-pages/IEC/preview_iec62304%7Bed1.1%7Den.pdf, which is unreachable to automated fetch — HTTP 403, verified by curl with browser user-agent.)
46. [STJOHNLINCH] St John Lynch, IEC 62304 training — software system testing and clause-by-clause tables (Clauses 4, 5.1–5.8, 7, 8) — https://www.stjohnlynch.com/training/iec-62304-ed1/docs/08-software-system-testing.html — live; 5.7.4 "recorded traceability between requirements and tests" confirmed by direct fetch.
47. [JOHNER] Johner Institute, IEC 62304 safety classes — https://blog.johner-institute.com/iec-62304-medical-software/safety-class-iec-62304/ — live; class A/B/C definitions and clause applicability confirmed by direct fetch.
48. [ArtOfService-IEC62304] TheArtOfService, IEC 62304 clause pages (self-labelled paraphrase) — https://requirements-traceability-gap-finder.theartofservice.com/standards/iec-62304/ — live.
49. [ISO14971-2019] ISO 14971:2019, medical device risk management (3rd ed.) — https://www.iso.org/standard/72704.html — unreachable to automated fetch (HTTP 403; verified by curl with browser user-agent). Working equivalent for the clause map: https://www.en-standard.eu/iso-14971-medical-devices-application-of-risk-management-to-medical-devices-4/ — live.
50. [Wikipedia-IEC62304] Wikipedia, "IEC 62304" (secondary; class/applicability summary) — https://en.wikipedia.org/wiki/IEC_62304 — live.
51. [ED2-draft] "IEC 62304 Edition 2: Draft changes for medical device software" (secondary) — https://intuitionlabs.ai/articles/iec-62304-edition-2-medical-software-changes — live.
52. [ED2-delay] QuickBird Medical, "IEC 62304 Edition 2 has been delayed until October 2028" (secondary) — https://quickbirdmedical.com/en/news/iec-62304-edition-2-august-2026/ — live; confirms three classes → two rigour levels (I/II).
53. [FDA-483-DB] FDA Inspections / 483 database search — https://www.accessdata.fda.gov/scripts/483inspsearch/index.cfm?action=search.search — live.

**Empirical traceability**
54. [Gotel-Finkelstein-1994] Gotel & Finkelstein, "An analysis of the requirements traceability problem", ICRE 1994 — https://ieeexplore.ieee.org/document/292398/ — resolves; OpenAlex record W2129559874 matches title and first author (O. C. Z. Gotel). Record lists publicationDate 2002-12-17 (IEEE reissue of the 1994 ICRE paper) — venue/year record differs from the 1994 conference citation; the substantive claim (>100 practitioners) rests on the paper abstract/snippet.
55. [Ramesh-Jarke-2001] Ramesh & Jarke, "Toward reference models for requirements traceability", IEEE TSE 27(1) — https://ieeexplore.ieee.org/document/895989 — resolves; OpenAlex record W2045895677 matches title, first author, year and TSE 27(1):58–93.
56. [Mader-Egyed-2015] Mäder & Egyed, "Do developers benefit from requirements traceability when evolving and maintaining a software system?", EMSE 20(2) — https://doi.org/10.1007/s10664-014-9314-z — DOI resolves; OpenAlex record W2031400820 matches title and first author. Record year 2014 (online), journal issue 20(2) 2015.
57. [Rempel-Mader-2016] Rempel & Mäder, "Preventing Defects: The Impact of Requirements Traceability Completeness on Software Quality", IEEE TSE 43(8) — https://doi.org/10.1109/TSE.2016.2622264 — DOI resolves; OpenAlex record W2540556128 matches title, first author (Patrick Rempel) and TSE 43(8):777–797.
58. [Ruiz-2023] Ruiz, Hu & Dalpiaz, "Why don't we trace? A study on the barriers to software traceability in practice", Requirements Engineering (2023) — https://doi.org/10.1007/s00766-023-00408-9 — DOI resolves; OpenAlex record W4389195372 matches title, first author (Marcela Ruiz); abstract confirms 55 survey participants and 14 interviews.
59. [Wohlrab-2018] Wohlrab et al., "Collaborative traceability management: a multiple case study from the perspectives of organization, process, and culture", Requirements Engineering (2018) — https://doi.org/10.1007/s00766-018-0306-1 — DOI resolves; OpenAlex record W2901437552 matches first author (Rebekka Wohlrab); abstract confirms 24 individuals from 15 industrial projects. Note: the draft's short title omits the record's subtitle.
60. [Borg-2013] Borg, Runeson & Ardö, "Recovering from a decade: a systematic mapping of IR approaches to software traceability", EMSE 19(6) — https://doi.org/10.1007/s10664-013-9255-y — DOI resolves; OpenAlex record W2109699558 matches title, first author and EMSE 19(6):1565–1616.
61. [Charalampidou-2020] Charalampidou et al., "Empirical studies on software traceability: A mapping study", JSEP 33(2) — https://doi.org/10.1002/smr.2294 — unreachable to automated fetch (HTTP 403 from DOI and Wiley; verified by curl with browser user-agent). Working equivalent: https://openalex.org/W3040257249 — live; record matches title, first author (Sofia Charalampidou) and JSEP 33(2).

**Tooling and standards**
62. [Jama] Jama Software — Relationships (coverage and traceability) — https://help.jamasoftware.com/ah/en/manage-content/coverage-and-traceability/relationships.html — live.
63. [PTC] PTC codebeamer — Suspected links — https://support.ptc.com/help/codebeamer/r2.1/en/codebeamer/user_guide/ug_suspected_links.html — live.
64. [IBM-DOORS-Next] IBM — DOORS Next overview — https://www.ibm.com/docs/en/engineering-lifecycle-management-suite/doors-next/7.1.0?topic=overview-doors-next — unreachable to automated fetch (HTTP 403; verified by curl with browser user-agent); reachable in a browser. The suspect-link/coverage mechanism claim rests on the vendor documentation read during research.
65. [OMG-ReqIF] OMG, Requirements Interchange Format (ReqIF) v1.2 — https://www.omg.org/spec/ReqIF/1.2/PDF — live.
66. [Capra] Eclipse Capra project page — https://projects.eclipse.org/projects/modeling.capra — live.
67. [NIST-SSDF] NIST SP 800-218, Secure Software Development Framework v1.1 — https://nvlpubs.nist.gov/nistpubs/specialpublications/NIST.SP.800-218.pdf — live.
68. [SLSA] SLSA v1.2, "About SLSA" — https://slsa.dev/spec/v1.2/about — live.
69. [GSN-v3] SCSC, Goal Structuring Notation (GSN) Community Standard v3 — https://scsc.uk/r1386.pdf — live.
70. [EU-AIA-12] EU AI Act, Article 12 — Record-keeping — https://artificialintelligenceact.eu/article/12/ — live.
71. [ISO-29119] ISO/IEC/IEEE 29119-1:2022 — https://committee.iso.org/cms/live/live/en/sites/isoorg/contents/data/standard/08/12/81291.html — live.

**Removed during verification (previously listed but uncited):** Rathod, Cebulla & Kugele, *Traceability Evaluation in Requirements Engineering According to Automotive SPICE* (IEEE RE 2023); LDRA, *Implementing ISO 26262 second edition with the LDRA tool suite*; Bouillon, Mäder & Philippow, *A survey on usage scenarios for requirements traceability in practice* (REFSQ 2013). Each appeared in the draft's source list but carried no inline citation and no draft claim depended on it, so it was removed to keep the Sources list free of orphan entries.

**Local artifacts produced and used by this report**
- `notes/regulated-trace/aspice-pam-40.pdf` / `.txt` — PAM v4.0 (153 pp) and extracted text
- `notes/regulated-trace/aspice-pam-41.pdf` / `.txt` — PAM v4.1 (159 pp) and extracted text (edition-delta check)
- `notes/regulated-trace/faa-ac20-115d.pdf` / `.txt`
- `notes/regulated-trace/fda-2023-device-software.pdf` / `.txt`
- `notes/regulated-trace/nasa-do178c.pdf` / `.txt`
- `notes/regulated-trace/iso26262-6-preview.txt`, `iso26262-8-iteh.txt` — official preview TOCs
- `outputs/.drafts/regulated-requirements-traceability-research-automotive.md`
- `outputs/.drafts/regulated-requirements-traceability-research-do178c.md`
- `outputs/.drafts/regulated-requirements-traceability-research-medical.md`
- `outputs/.drafts/regulated-requirements-traceability-research-transfer.md`
- `outputs/.drafts/regulated-requirements-traceability-research-lead.md` — lead's primary-source verification quotes

---

## Appendix V — Verification notes (citations, URLs, quantitative claims)

This pass added no factual claims. It (a) converted the draft's inline `[Tag]` citations into numbered `[n]` citations keyed to the Sources list, (b) resolved two unresolved tags, (c) checked every source URL, (d) removed three uncited sources, and (e) audited every quantitative claim below. The report's structure, tables and evidence-class labels are unchanged.

### V.1 Tag resolution

| Tag used inline in the draft | Resolution |
|---|---|
| `[JOHNER-classes]` (§5.1) | Not a defined source tag; resolved to `[47]` **JOHNER** (Johner Institute safety-class page, which states the A/B/C class definitions). |
| `[LDRA-DO330]` (§4.3) | Not a defined source tag; resolved to `[31]` **DO330-tool-qual** (LDRA DO-330 technical briefing, the LDRA item in the source list). |
| `[JOHNER/STJOHNLINCH]` (§1.1, §5.2) | Resolved to `[47; 46]`. |

All other inline tags resolved to a defined Sources entry. Five entries that were defined but never cited inline — ASG21, VDApub, ULPG, FAA-diff-tool, RTCA — were anchored to the report claims they support (numbers [5], [6], [7], [23], [26], per the research files); three uncited entries — Rathod-2023, LDRA-26262, Bouillon-2013 — were removed (see Sources note). No orphan citations and no orphan sources remain.

### V.2 URL verification (checked 2026-10-18)

Of 76 URLs in the draft's source list, 71 returned HTTP 200 or 202 (202s are DOI/IEEE bot-throttled redirects that resolve). Five failed automated fetch; each was re-checked and either replaced with a working equivalent or marked:

| Source | URL | Status / action |
|---|---|---|
| [45] IEC 62304 preview | `webstore.ansi.org/preview-pages/IEC/preview_iec62304{bed1.1}en.pdf` | HTTP 403 to automated fetch (verified by curl with browser user-agent). Replaced with the live IEC webstore catalogue record `webstore.iec.ch/publication/3842` (HTTP 200). |
| [61] Charalampidou-2020 | `https://doi.org/10.1002/smr.2294` (and Wiley) | HTTP 403 to automated fetch (verified by curl with browser user-agent). Working equivalent `https://openalex.org/W3040257249` (HTTP 200; title/authors/venue match). |
| [64] IBM DOORS Next | `ibm.com/docs/...doors-next/7.1.0...` | HTTP 403 to automated fetch (verified by curl with browser user-agent); reachable in a browser. Mechanism claim rests on vendor documentation read in research. |
| [49] ISO 14971:2019 | `https://www.iso.org/standard/72704.html` | HTTP 403 (verified by curl with browser user-agent). Clause map corroborated via working equivalent `en-standard.eu` (HTTP 200). |
| [14] ISO 26262-2:2018 canonical record | `https://www.iso.org/standard/90027.html` | HTTP 403 (verified by curl with browser user-agent). Primary mirror URL (infocentre.io) returns HTTP 200 and is the copy actually used. |

Content spot-checks by direct fetch confirmed the substantive quotes for [3] PAM v4.1 (version/date/revision list), [6] VDA publications (version list), [44] FDA recognition 13-79, [52] IEC 62304 Ed.2 delay (A/B/C → I/II), [47] Johner classes, and [46] St John Lynch §5.7.4. Regulatory/standard texts that are paywalled ([2], [11]–[20], [22], [41]) were already read by the lead/researchers as noted; their URLs resolve.

### V.3 DOI / index cross-check for cited papers

Each cited paper's DOI was matched against an OpenAlex work record (title, first author, year):

- **Matched as cited:** Falcini & Lami 2020 [9]; Rempel & Mäder 2016 [57] (TSE 43(8):777–797); Ruiz et al. 2023 [58]; Borg et al. 2013 [60]; Marques & Cunha 2017 [32]; Krauss et al. 2015 [33]; Moy et al. 2013 [34]; Ramesh & Jarke 2001 [55] (TSE 27(1):58–93).
- **Record matched but the draft's year/title differs from the index record — flagged, not silently kept:**
  - **[59] Wohlrab et al. 2018** — the index title is *"Collaborative traceability management: a multiple case study **from the perspectives of organization, process, and culture**"*. The draft's short title omits the subtitle. Same paper; subtitle restored in Sources.
  - **[56] Mäder & Egyed** — index publication year 2014 (online), journal issue EMSE 20(2) 2015. The draft's "EMSE 2015" is the issue year; both are recorded.
  - **[54] Gotel & Finkelstein** — cited as ICRE 1994; the OpenAlex record lists publicationDate 2002-12-17 (IEEE reissue). Same paper, year/venue record differs from the conference citation. The ">100 practitioners" figure is a snippet-level claim from the abstract.
- **Preprint/venue note:** none of the cited papers has a separate conflicting preprint record in the indexes consulted.

### V.4 Quantitative-claim audit (all match the research files and retain their evidence class)

| Claim in draft | Value | Source | Evidence class in draft |
|---|---|---|---|
| DO-178C objectives per level | 71/69/62/26/0 | [38]; [28] | secondary/vendor — explicitly labelled "secondary-sourced" |
| DO-178C objectives "with independence" | 30/18/5/2/0 | [38]; [28] | secondary/vendor |
| ASPICE NPLF anchors | N ≤15%; P >15–50%; L >50–85%; F >85–100% | [1] (PAM v3.1 Table 13); [2] | `[P]` primary |
| DO-178B cost increase | 20–40% (DO-178C can exceed) | [35] (Hilderman 2014, conference abstract) | labelled "conference abstract, snippet-level" |
| Mäder & Egyed experiment size | >50 subjects | [56] | empirical (study abstract/summary) |
| Rempel & Mäder corpus | 24 open-source projects | [57] | empirical (observational) |
| Gotel & Finkelstein study size | >100 practitioners | [54] | empirical (abstract/snippet) |
| Ramesh & Jarke study size | 26 organisations | [55] | empirical |
| Wohlrab et al. corpus | 15 industrial projects | [59] | empirical |
| Ruiz et al. study size | 55 respondents / 13 countries (+14 interviews) | [58] | empirical (full text read in research) |
| IEC 62304 release clause | §5.8 (5.8.2 residual anomalies; 5.8.3 class B/C evaluation) | [46] | secondary clause reproduction (normative table paywalled) |
| ISO 26262 independence levels | I0–I3; FSA at I0 (ASIL B), I2 (C), I3 (D); none at QM/ASIL A | [14] Table 1 | `[P*]` — 2018 public copy of Part 2 read, per §2 item 5 |
| ISO 26262 cross-cutting traceability clause | 26262-8 §6.4.3.2 | [15] (2011 wording); 2018 retention per [11] | `[P*]` — 2011 wording read; 2018 normative sentence not read, per §2 item 5 |

**Not present in the draft:** the numeral **"20.1.2"** does not appear anywhere in the draft, so there is no draft claim to verify against it. The nearest item in the research files is the VDA Guidelines mapping reference **ASG20 §3.20.1.2–3.20.1.3** (ISO 26262-5 clause 10 ↔ HWE.3/HWE.4) in the automotive research file; the draft's §3.4 states the same mapping without a clause number. No clause number was added.

### V.5 Unsupported-material sweep

- No numeric score, benchmark, figure/image reference, dataset size or superiority claim in the draft was found to lack a traceable source or artifact path.
- No figure, chart or visualisation is present to audit.
- The report's explicit hedges and "not found" statements (§8.2, §9.2, §10) are retained; no certainty language (`verified`, `confirmed`, `reproduced`) was added. Where the draft already used a verification result from the research files (e.g. the PAM 4.0↔4.1 text comparison, artifact `notes/regulated-trace/aspice-pam-4{0,1}.txt`), the existing wording is unchanged.
- Evidence-class markers `[P]`, `[S]`, `[P/S]`, `[primary]`, `[guidance]`, `[secondary]`, `[empirical]` are preserved verbatim.
