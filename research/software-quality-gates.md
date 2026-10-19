# Software Quality Gates: What Technology Companies Use and What the Evidence Shows

## Executive Summary

A **software quality gate** is a predefined checkpoint in a software project or delivery pipeline that evaluates work products against agreed quality criteria and produces an enforceable go/no-go decision before the next phase, environment, or release stage [1][2][3]. Quality gates therefore combine checks (data), criteria (policy), and decision authority (blocking power); they are not merely tests or reviews [3][4].

Technology companies report a layered set of gates:

- **Code-level gates**: required code review, automated build/test checks in CI, static analysis, linting and type checking, coverage thresholds, pre-commit hooks, and branch-protection rules [17][18][19][20][24][25][26][27][28][29][30][31][38][42][43].
- **Release/operations gates**: reproducible/signed builds, security scans (SAST, DAST, dependency scanning), performance/load tests, progressive rollouts (canary, one-box, rolling, blue/green, feature flags), SLO/error-budget and observability-based gates, manual/ITSM approvals, and post-incident learning gates [44][45][46][47][48][49][50][51][52][53][54][55][56][57][58][59][60][61].

The evidence base is uneven:

- **Strongest evidence**: code review improves code understandability, knowledge transfer, and team awareness [21][22][68]; reviewer coverage, participation, and expertise are associated with software quality [22]. Continuous integration and automated testing correlate with higher delivery frequency and faster feedback [31][32][33][66][67], and DORA research links these practices to elite software-delivery performance [10][62].
- **Mixed evidence**: static analysis and type checkers detect real bugs, but the measured recall depends on the metric: Flow and TypeScript each detected about 15% of *public* JavaScript bugs [26], while mypy could have prevented about 15% of *corrective defects* in 210 Python projects [27]. Recall on real-world defects is low and false positives are a major barrier [24][29][30]. Code coverage thresholds are widely mandated [38], yet large-scale studies disagree on whether coverage correlates with post-release defects [39][40][41].
- **Weakest evidence**: many release/operations gates—canaries, error-budget policies, feature flags, post-incident reviews—are supported mainly by conceptual reasoning, practitioner case studies, and correlational reports rather than controlled experiments [46][47][48][49][54][56][59][71][72]. Their value is plausible and widely asserted, but causal evidence isolating individual gates is scarce [67][72].

A central caveat runs through the literature: most studies are observational, use self-reported or convenience samples, and do not isolate the effect of a single gate from confounding factors such as team skill, codebase maturity, and organizational culture [67][68][69].

---

## 1. What Is a Quality Gate?

### 1.1 Core definition
A quality gate is a **predefined checkpoint** that evaluates a work product against **predefined criteria** and produces a **pass/warn/fail decision** that can block progression until issues are resolved [1][2][3]. TechTarget defines it as "a milestone in an IT project that requires that predefined criteria be met before the project can proceed to the next phase" [1]. Sonar describes it as "a configurable checkpoint in the software development lifecycle that enforces predefined standards — such as code coverage or security thresholds — before code can advance to the next stage," where failing code "is rejected and prevented from continuing" [2]. Uzunova, Pavlič, and Beranič (2024) similarly define quality gates as "predefined checkpoints within the software development life-cycle, positioned at various stages of the project" where criteria must be satisfied before advancement [3].

Key elements:
- Checkpoint or milestone in the lifecycle or pipeline [1][3][4].
- Predefined, measurable criteria (e.g., test pass rate, vulnerability count, coverage threshold, reviewer sign-off) [2][3].
- Enforceable decision (pass, warn, fail) that can block progression [2][3].
- Stakeholder visibility into the decision [4].

### 1.2 Relationship to related concepts
- **CI/CD checks** produce data; **quality gates** make decisions based on that data [2][3]. A gate may combine multiple checks into one go/no-go outcome.
- **Definition of Done** sets the shared quality standard for an increment; **quality gates** verify whether the standard is met [16].
- **Entry/exit criteria** are the conditions a gate evaluates at a phase boundary [4].
- **Milestones** become quality gates only when they include enforced quality criteria and go/no-go authority [1][4].

### 1.3 Frameworks that shape quality gates
In the frameworks reviewed here, none names "quality gate" as a first-class practice in its core standard (ISO/IEC normative text was not inspected directly), but several describe functionally equivalent constructs:

- **ISO/IEC 25010** provides the quality-attribute model (functional suitability, performance, security, maintainability, etc.) that gate criteria often operationalize (source [5] is a vendor-hosted summary, not the standard text) [5].
- **ISO/IEC 330xx** assesses the capability and maturity of the processes that implement gates [6].
- **CMMI V2.0** "Ensuring Quality" capability area (Requirements Development & Management, Process Quality Assurance, Verification and Validation, Peer Review) contains the practices that gates enforce [7].
- **Microsoft Security Development Lifecycle (SDL)** embeds security checkpoints (design review, threat modeling, code review, security testing) across the lifecycle [11][12].
- **OWASP SAMM** provides maturity-level security activities that can become gate criteria [13].
- **Google SRE** documents a Launch Coordination Checklist used before production launch [14]; Cusick (2013) separately describes a Production Readiness Review that he calls "a gating process to ensure true application readiness for production" [15].
- **SAFe** explicitly treats traditional "phase gates" as legacy governance milestones and emphasizes Built-In Quality and continuous flow instead [8].
- **DevOps / DORA / Continuous Delivery** embed gates in deployment pipelines as automated checks [9][10].

ITIL 4 and COBIT 2019 were not found to define "quality gate" directly; their relevance is through change-enablement and control-objective concepts.

---

## 2. Code-Level Quality Gates

### 2.1 Code review
**What companies do.** Code review is the most commonly reported pre-integration gate. At Google, all changes are expected to be reviewed, with reviewers inspecting design, functionality, complexity, tests, naming, comments, style, and documentation [17]. At Microsoft, a 2015 survey of 911 developers (reported in MSR-TR-2016-27) found that 94% said their teams require code review before check-in, though only 54% had explicit written policies [19]. At Meta, every diff is reviewed without exception [20].

**Evidence.** Bacchelli and Bird (2013) found that defect finding was the top motivation for code review, yet only 14% of comments addressed defects; the largest category was code improvements (29%), followed by understanding/knowledge transfer [21]. Sadowski et al. (2018) report that Google's primary motivation for code review is improving code understandability and maintainability, not bug finding [18]. McIntosh et al. (2015) found that review coverage, review participation, and reviewer expertise are significantly associated with software quality in open-source systems [22].

**Caveats.** Code reviews are costly and variable; they cannot replace other verification techniques [23]. Large changes are harder to review, and social dynamics can influence outcomes [19].

### 2.2 Static analysis, linting, and type checking
**What companies do.** Static analysis, linters, and type checkers are commonly integrated into CI or pre-commit hooks. Microsoft uses static analyzers such as Code Analysis for C/C++ and TSLint security rules in its SDL [12].

**Evidence.** Gao et al. (2017) found that Flow 0.30 and TypeScript 2.0 each detected 15% of *public* bugs in JavaScript projects, a conservative lower bound because those bugs had already survived testing and review [26]. Khan et al. (2022) found that mypy could have prevented 15% of *corrective defects* in 210 Python projects — a different outcome measure from Gao et al.'s public-bug count [27]. Heričko and Šumak (2023) found a negative correlation between ESLint warning density and maintainability; projects with linters had higher maintainability, though the effect size was small [24]. Ernst et al. (2011) found hundreds of bugs, including over 40 causing incorrect user-visible behavior, in 2 million lines of open-source code using pluggable type checkers [28].

**Caveats.** Static analysis recall on real-world bugs is low. Habib and Pradel (2018) found that three Java static analyzers together detected only 27 of 594 real-world bugs (4.5%), with 78% of candidate warnings spurious without manual validation [30]. Vassallo et al. (2020) found that many projects adopt static analysis tools superficially and do not enforce results; false positives remain a major barrier [29].

### 2.3 Automated testing and continuous integration build gates
**What companies do.** CI pipelines typically run build, unit tests, integration tests, and static/security scans on every pull request; a failing build or test suite blocks merge [31][43].

**Evidence.** Vasilescu et al. (2015) found that CI improves productivity (more outside contributions integrated) without observable diminishment in code quality, and that more bugs are reported, suggesting better detection [31]. Hilton et al. (2016) found that projects using CI release more frequently [32]. Wang et al. (2022) found that higher test automation maturity in CI is associated with improved product quality, shorter release time, and reduced test automation effort [33].

**Caveats.** Flaky tests, long-running suites, and CI noise undermine gate effectiveness [34][35][36][37]. Most CI evidence is correlational [67].

### 2.4 Code coverage thresholds
**What companies do.** Many organizations set minimum coverage thresholds (commonly 80% statement coverage) as merge requirements. NIST IR 8397 recommends at least 80% statement coverage while noting that "very high code coverage guarantees little" [38].

**Evidence.** Kochhar et al. (2015) found a statistically significant correlation between coverage and bug-kill effectiveness in Apache HttpClient and Mozilla Rhino [39]. Bach et al. (2017) found that covered code in SAP HANA had lower future bug density [40]. However, Kochhar et al. (2017) analyzed 100 large open-source Java projects and found that coverage had an insignificant correlation with post-release bugs at the project level and no correlation at the file level [41].

**Caveats.** Coverage thresholds can be gamed by tests that exercise code without asserting behavior [38]. The relationship between coverage and field defects is context-dependent and mixed [39][40][41].

### 2.5 Pre-commit hooks and branch-protection policies
**What companies do.** Pre-commit hooks enforce formatting, linting, type checking, secret scanning, and unit tests locally [42]. Branch protection rules require pull-request reviews, passing status checks, signed commits, and block direct pushes to default branches [43].

**Evidence.** Direct peer-reviewed empirical studies isolating pre-commit hooks or commit-message policies as independent variables are scarce. Their value is mostly indirect: they enforce checks whose effectiveness is documented elsewhere [24][26][27][28][29][30].

**Caveats.** Local hooks can be bypassed and must be paired with CI-side enforcement [42]. Overly rigid policies create friction [19].

---

## 3. Release and Operations-Level Quality Gates

### 3.1 Build and release-engineering gates
**What companies do.** Hermetic, reproducible builds; immutable, signed artifacts; branch and tag filters restricting what can reach production; role-based access control over approvals, releases, cherry-picks, and deployments [44][45].

**Evidence.** Google states that reproducible builds and strict access control allow SREs to trust that binaries are built in a repeatable way [44]. AWS notes that code review is the last manual approval before fully automated production deployment [45].

**Caveats.** Requires version-control discipline, build-system investment, and artifact repository. Teams with many unique pipeline templates impede company-wide safety improvements [56].

### 3.2 Security scanning gates
**What companies do.** SAST, DAST, dependency scanning, and artifact scanning are commonly integrated into CI/CD or deployment pipelines [12][45][51].

**Evidence.** Elder et al. (2022) applied SAST, DAST, manual penetration testing, and security review to OpenMRS. SAST surfaced the most total vulnerabilities (824); exploratory manual penetration testing surfaced 185, of which 165 were "more severe," and the authors report that EMPT found more severe vulnerabilities than the automated techniques. Each technique found vulnerabilities the others missed [59]. The paper's own counts are internally inconsistent — its prose also reports 229 for EMPT and 823 for SAST — so these figures should be read as approximate. Croft et al. (2021) found that SAST line-level warnings were extremely inaccurate (only ~5% valid for open-source datasets), while learning-based models had better precision [58].

**Caveats.** No single technique finds everything. SAST precision and recall vary heavily by tool, bug class, and code complexity. DAST can produce very high alert-to-vulnerability ratios [59]. Dependency-scanning effectiveness was not covered by high-citation empirical studies in this research pass.

### 3.3 Performance and load-test gates
**What companies do.** Some pipelines run short load tests in pre-production to detect latency or throughput regressions [45]. Google SRE notes that performance defects often manifest only under heavy load, so canary population and duration must be sized to representative traffic [46].

**Evidence.** No direct controlled experiment was located; benefit is asserted as part of safe-deployment practice [45][46].

### 3.4 Progressive rollout gates
**What companies do.** Canary, one-box, rolling, blue/green, and feature-flag-based rollouts are standard at Google, AWS, Microsoft, Uber, and Spotify [45][46][50][54][55][56][57].

**Evidence.** Google SRE Workbook provides a worked example showing that canarying limits blast radius and conserves error budget [46]. Uber reports that between early 2022 and March 2024 the share of services fully automated to production rose from 7% to 65%, and that over a 12-month window in which CD adoption climbed, reported incidents per 1,000 code changes fell by more than 50%; Uber states it "cannot claim causation" [56]. AWS states that staged waves and one-box deployments limit the scope of failed production deployments [45].

**Caveats.** Requires traffic splitting, metrics attribution, and isolation. Running multiple simultaneous canaries contaminates signals [46]. Feature flags add code-path complexity and cleanup debt [57].

### 3.5 SLO, error-budget, and observability gates
**What companies do.** Google uses error budgets as the control mechanism for release velocity; if the budget is exhausted, releases halt except P0/security fixes [47][48]. AWS deployment systems monitor high-severity alarms and automatically roll back [45]. Azure DevOps gates can query Azure Monitor alerts and check for reported incidents [51][52].

**Evidence.** An analysis of Google postmortems shows binary push (37%) and configuration push (31%) as the top two outage triggers in the 2010–2017 sample, justifying push-time observability and rollback [75]. AWS says automated rollback is often already in progress by the time the on-call engineer is paged [45].

**Caveats.** Requires well-defined SLIs/SLOs, monitoring coverage, and organizational authority to enforce freezes. Miscategorized errors can mask reliability problems [48].

### 3.6 Approval, change-management, and compliance gates
**What companies do.** Manual pre/post-deployment approvals, ServiceNow change-management gates, role segregation, and evidence collection for SOC 2 and ISO 27001 [51][52][53][54].

**Evidence.** ISO 27001 control 8.32 requires changes to information systems to be properly controlled and authorized [60]. SOC 2 CC8.1 expects changes to be authorized, designed, tested, approved, and implemented in a controlled manner [61]. Both of these descriptions come from consultancy summaries of the standards ([60][61]), not from the standards' own text; the same applies to the ISO/IEC 25010 summary in §1.3 [5].

**Caveats.** Change gates can become bottlenecks if change-advisory boards are not aligned with continuous-delivery cadence. Approver-submitter role segregation must be enforced technically [54].

### 3.7 Post-incident learning gates
**What companies do.** Blameless postmortems after significant incidents, action-item tracking, postmortem reading clubs, FixIt weeks, and release freezes after major incidents [49].

**Evidence.** Google cites a case where action items from a rack (satellite) decommission incident dramatically reduced the blast radius of a similar incident three years later [49]. Google also analyzes thousands of stored postmortems to target systemic improvements [49].

**Caveats.** Requires blameless culture, executive support, and action-item ownership. Postmortems can become performative if action items are not closed [49].

---

## 4. Adoption and Evidence Strength

### 4.1 Adoption rates from industry surveys
| Practice | Survey | Finding |
|----------|--------|---------|
| CI/CD availability | Stack Overflow 2024 | 68.6% of chart respondents report their company has CI/CD available (chart "My company has:"; the series is labeled "All Respondents"; DevOps function 58.3%, automated testing 56.3%) [63] |
| DevOps involvement | CD Foundation / SlashData 2024 | 83% of developers report involvement in DevOps-related activities [66] |
| Any testing activity | JetBrains 2024 | 95% of developers test (up from 85% in 2023) [64][74] |
| Automated quality gates for every merge request | JetBrains Qodana 2024 | 27% lack such gates [65] |
| Manual testing | JetBrains Qodana 2024 | 39% identify manual testing as the most resource-intensive quality practice [65] |

*Table note:* The developer-ecosystem report page [64] is client-rendered, and its testing-type percentages were not retrievable from static page content; the 95%/85% figures are stated explicitly by secondary coverage [74]. Percentages for unit, integration, end-to-end, and performance testing are not included because they could not be confirmed from a retrievable source. By contrast, the Qodana figures in this table (27%, 39%) are present in that page's static markup [65].

### 4.2 Evidence strength by gate family
| Gate family | Evidence strength | Summary |
|-------------|-------------------|---------|
| Code review | Strong for process benefits; mixed for defect detection as primary outcome | Improves understandability, knowledge transfer, and team awareness; associated with quality when coverage/participation/expertise are high [21][22][68] |
| CI / automated testing | Strong for correlation with delivery performance; weak for causal isolation | Associated with higher deployment frequency, faster feedback, and elite DORA performance [10][31][32][33][62][66][67] |
| Static analysis / type checking | Mixed / weak for real-bug recall; strong for specific bug classes | Type systems catch ~15% of public JavaScript bugs [26]; mypy could prevent ~15% of Python corrective defects [27]; static analysis recall on real bugs is low [30]; false positives are a major barrier [29] |
| Coverage thresholds | Mixed | Some studies find correlation with bug density [39][40]; others find no correlation with post-release defects [41] |
| Pre-commit hooks / branch policies | Weak / indirect | Widely used [42][43]; little peer-reviewed empirical evidence isolating their effect |
| Security scanning (SAST/DAST) | Mixed / method-dependent | Techniques are complementary [59]; SAST precision varies widely [58]; no single technique finds everything [59] |
| Progressive rollout (canary, feature flags) | Moderate / conceptual and correlational | Strong blast-radius reasoning [46]; Uber reports correlational incident reduction [56] |
| SLO / error-budget / observability gates | Moderate / practice-based | Supported by postmortem data and operational reasoning [45][46][47][48][75]; few controlled experiments |
| Approval / compliance gates | Weak / standards-based | Required by ISO 27001 and SOC 2 [60][61]; little empirical evidence on throughput impact |
| Post-incident learning gates | Weak / case-study-based | Google case studies show learning value [49]; generalizability unclear |

### 4.3 Systematic reviews: the broader evidence base
- Soares et al. (2022) reviewed 101 empirical CI studies and found predominantly positive associations with productivity, faster releases, and defect reduction, but only 5 studies used control/comparison groups [67].
- Davila and Nunes (2021) reviewed 139 modern code-review papers and concluded that code review improves quality and enables knowledge sharing, but most evaluations are offline and lack human-subject experiments [68].
- Rafi et al. (2012) combined an SLR of 25 test-automation papers with a 115-practitioner survey: benefits include repeatability, reusability, and coverage, but practitioners reject the idea that automation fully replaces manual testing [69].
- No rigorous meta-analysis of static analysis was found; the NIST SATE VI benchmark report and per-tool empirical studies show high variability [70].
- Release gates remain under-studied empirically; much of the literature is conceptual or descriptive [71][72].

---

## 5. Disagreements, Caveats, and Gaps

### 5.1 Methodological limitations
Most evidence is observational or self-reported. Controlled experiments isolating the effect of a single gate are rare. Samples are often convenience samples from large tech firms or open-source projects, limiting generalizability to smaller organizations or regulated industries [67][68][69].

### 5.2 Confounding factors
Organizations that adopt quality gates also tend to have strong engineering culture, skilled teams, mature tooling, and management support. It is difficult to separate the gate's effect from these confounders [67][68].

### 5.3 Disagreements in the literature
- **Coverage thresholds**: Kochhar et al. (2015) and Bach et al. (2017) report positive correlations between coverage and bug density, while Kochhar et al. (2017) find no significant correlation with post-release defects [39][40][41].
- **Code review's primary value**: Bacchelli and Bird (2013) emphasize that defect finding is the stated motivation but a minority of comments address defects [21]; Sadowski et al. (2018) report that Google values understandability and maintainability more than bug finding [18].
- **Static analysis effectiveness**: Tools detect real bugs, but Habib and Pradel (2018) report low recall on real bugs (27 of 594) [30], while a Microsoft DevSecOps exercise reports that static analysis "tripled the number of total security findings (adding twice as many new results to those located by manual review)" [73].

### 5.4 Evidence gaps
- **Dependency scanning**: little high-citation empirical evidence on effectiveness.
- **Performance/load-test gates**: mostly vendor documentation and conceptual reasoning [45][46].
- **Compliance/approval gates**: standards mandate them, but empirical studies on their cost, throughput impact, and effectiveness are scarce [60][61].
- **Small-team applicability**: most sources describe hyperscaler practices; evidence for which gates are feasible and beneficial for smaller teams is limited.

---

## 6. Open Questions

1. Which gates provide causal improvement in defect density, change-failure rate, or time-to-recovery when isolated from confounding factors?
2. What is the minimum viable set of quality gates for small and medium-sized teams, and how does it differ from hyperscaler practice?
3. How should organizations calibrate static-analysis and coverage-threshold gates to maximize true positives and minimize noise and gaming?
4. What is the empirical effectiveness of dependency-scanning gates in reducing CVE exposure and incidents?
5. How do AI-assisted coding and review tools change the value and failure modes of existing quality gates?

---

## 7. Bottom Line

Quality gates are a defining feature of professional software development at major technology companies. The most widely adopted and best-evidenced gates are **code review**, **CI build/test gates**, and **automated testing**. These are associated with better code quality, faster feedback, knowledge sharing, and elite delivery performance, though causal evidence remains limited [18][21][22][31][32][33][62][66][67][68].

**Static analysis, type checking, linting, and coverage thresholds** are also common, but their evidence is mixed: they catch specific bug classes and support maintainability, yet real-bug recall is low, false positives are a barrier, and coverage thresholds do not consistently predict post-release defects [24][26][27][29][30][38][39][40][41].

**Release and operations gates**—progressive rollouts, SLO/error-budget policies, security scanning, manual approvals, and post-incident learning—are standard practice at large firms and supported by strong operational reasoning and some correlational data [45][46][47][48][49][54][56][59]. However, controlled empirical evidence isolating their individual impact is still sparse [71][72].

Organizations should treat quality gates as **risk-reduction and feedback mechanisms**, not as guarantees. The pattern most consistent with the sources reviewed — though it is an inference from them rather than a tested finding — is that combining several complementary gates and adapting them to context is better supported than reliance on any single gate [3][67].

---

## Corrections Log

This brief went through three verification passes. Corrections made, with evidence in `software-quality-gates-linkcheck.md`:

**Corrections that stand:**
- **Microsoft SAST claim reworded and re-sourced.** The source says static analysis "tripled the number of *total* security findings (adding twice as many new results to those located by manual review)" — not "compared with manual review alone." It was also cited to the SDL Resources page, which does not contain it; the claim is now attributed to the DevSecOps exercise blog [73].
- **Removed the "SDL-derived standards explicitly call them 'security gates'" clause.** The phrase does not appear in the cited Microsoft SDL pages [11][12].
- **Removed the invented years 2015/2018** from the Google postmortem case; the source says only "three years after" [49].
- **Fixed the Google postmortem statistics citation.** The 37%/31% figures are in the SRE Workbook **Appendix C** ("Top eight outage triggers, 2010–2017"), not in Chapter 10; Appendix C is now cited separately [75].
- **Fixed the Elder et al. numbers.** 824 is SAST's total vulnerability count; 185 is exploratory manual penetration testing's total, of which 165 were "more severe." The earlier text called 185 a count of severe findings [59].
- **Separated the Uber time windows.** The 7%→65% automation rise spans early 2022 to March 2024; the >50% incident decline covers a 12-month window. The earlier text implied both happened in 12 months [56].
- **Fixed bibliographic fields:** Khan et al. (2021 → 2022) [27]; Heričko & Šumak venue (ICPC → ICSIM 2023) [24]; NIST SP 500-341 (2024 → 2023) [70].
- **Softened the Microsoft survey year** to a 2015 survey, which is when the 911-respondent survey was fielded (the report is MSR-TR-2016-27) [19].
- **Replaced source 2.** The IBM DevOps Insights quotation could not be verified (the cited page now returns HTTP 404; the product is discontinued). Sonar's definition, verified verbatim, is used instead [2].
- **Scoped two overstated claims:** the universal negative about frameworks (ISO/IEC normative text was not inspected), and the "outperform any single gate" conclusion, which the cited sources do not test.
- **Labelled consultancy/blog summaries as secondary** in §3.6 and §1.3 [5][60][61].
- **Dropped "Netflix"** from the list of firms cited for progressive-rollout practice: the available Netflix Tech Blog URL returns HTTP 403 to automated fetching and no alternative source was verified in this pass. The broader claim remains supported by verified sources for Google, AWS, Microsoft, Uber, and Spotify.

**Corrections to earlier-verification errors (recorded for transparency):**
- **The 68.6% CI/CD figure is on the cited primary page.** An intermediate revision re-attributed it to a secondary book chapter on the mistaken belief that the Stack Overflow page lacked it. Direct inspection of the page's chart markup shows the "My company has:" chart contains *Continuous integration and c… 68.6%*, *DevOps function 58.3%*, and *Automated testing 56.3%*. The figure is restored to [63] and the secondary attribution was removed.
- **The Qodana 39% row was briefly relabelled incorrectly.** The source states "39% identify manual testing as the most resource-intensive quality practice"; an intermediate revision changed this to "consider manual testing effective," which the source does not say. Restored to the source's wording [65].
- **The four JetBrains testing-type percentages** (unit, integration, end-to-end, performance) remain excluded: they were not confirmable from the report page's retrievable content, and the direction of change is all that secondary coverage states [74]. A secondary summary suggests the values are plausible, but no citable, retrievable source was located.

---

## Verification Notes

Checks were run on 18 October 2026 and are logged in `software-quality-gates-linkcheck.md` (with reproducible commands in `notes/software-quality-gates-linkcheck-cmds.txt`).

- **Reachability:** an initial active sweep covered 34 candidate URLs; 31 returned HTTP 200 and 3 returned HTTP 403 because the publisher blocks non-browser clients (iso.org, Medium-hosted Netflix Tech Blog, ACM Digital Library). Each 403 URL was separately confirmed to exist via search or metadata. This sweep was built from an early candidate list and is **not** a systematic check of the final 75-entry reference list: several sampled URLs are no longer cited in this brief (e.g. docs.gitlab.com, github.blog, arXiv 1911.08816, arXiv 1703.07019), one sampled URL that returned 200 (IBM Cloud docs) was subsequently dropped because it failed content extraction, and most reference entries were instead checked by direct fetch of the page or PDF during the verification passes.
- **Bibliographic metadata:** four DOIs were resolved against the Crossref REST API (Khan et al., Heričko & Šumak, NIST SP 500-341, and one candidate source later dropped) and corrected where the draft was wrong. Crossref metadata confirms authorship, container, and year only; it is not evidence about page *content*, and it cannot establish that a source states a particular statistic.
- **Quotation checks:** TechTarget, Sonar, and NIST IR 8397 sentences were verified verbatim; the Microsoft DevSecOps "tripled" sentence, the SRE Workbook "three years after" and Appendix C table, the Elder et al. counts, and the Uber figures were verified against the primary pages/PDFs.
- **Client-rendered pages:** the JetBrains developer-ecosystem page and the Stack Overflow professional-developer page render some figures via client-side markup. The Stack Overflow figure was confirmed by inspecting the page's raw chart markup; the JetBrains percentages could not be, and are either excluded or attributed to secondary coverage.

---

## Sources

1. TechTarget / WhatIs.com. "What is a quality gate?" https://www.techtarget.com/searchsoftwarequality/definition/quality-gate
2. Sonar. "What are quality gates in software development?" https://www.sonarsource.com/resources/library/quality-gate/
3. Uzunova, N., Pavlič, L., & Beranič, T. (2024). "Quality Gates in Software Development: Concepts, Definition and Tools." CEUR-WS Vol. 3845. https://ceur-ws.org/Vol-3845/paper06.pdf
4. Flohr, T. (2008). "Assessing the Quality of Quality Gate Reference Processes." IFIP CEESER 2008. https://inria.hal.science/hal-01572539v1/file/978-3-642-22386-0_16_Chapter.pdf
5. ISO 25000.com. "ISO/IEC 25010." https://iso25000.com/index.php/en/iso-25000-standards/iso-25010
6. ISO/IEC JTC 1/SC 7. "ISO/IEC 33000 Family — Process assessment." https://committee.iso.org/sites/jtc1sc7/home/projects/flagship-standards/isoiec-33000-family.html
7. TMMi Foundation. "Changes in CMMI 2.0 and how they can affect TMMi." https://www.tmmi.org/tm6/wp-content/uploads/2020/12/CMMI-2.0-whitepaper-V1.0.pdf
8. Scaled Agile Framework. SAFe Glossary entries for "Phase Gates" and "Built-In Quality." https://framework.scaledagile.com/glossary
9. Fowler, M. "Continuous Delivery." https://martinfowler.com/bliki/ContinuousDelivery.html
10. DORA / Google Cloud. "Deployment automation." https://dora.dev/capabilities/deployment-automation/
11. Microsoft Security Engineering. "Security Development Lifecycle (SDL) Practices." https://www.microsoft.com/en-us/securityengineering/sdl/practices
12. Microsoft. "Security Development Lifecycle Resources." https://www.microsoft.com/en-us/securityengineering/sdl/resources
13. OWASP SAMM. "About OWASP SAMM." https://owaspsamm.org/about/
14. Google SRE Book. "Launch Coordination Checklist." https://sre.google/sre-book/launch-checklist/
15. Cusick, J. (2013). "Architecture and Production Readiness Reviews in Practice." arXiv:1305.2402. https://arxiv.org/pdf/1305.2402
16. Atlassian. "What is the Definition of Done (DoD) in Agile?" https://www.atlassian.com/agile/project-management/definition-of-done
17. Google Engineering Practices. "Code Review — Overview." https://github.com/google/eng-practices/blob/master/review/index.md
18. Sadowski, C., Söderberg, E., Church, L., Sipko, M., & Bacchelli, A. (2018). "Modern Code Review: A Case Study at Google." ICSE-SEIP. https://doi.org/10.1145/3183519.3183525
19. Greiler, M., Bird, C., Storey, M.-A., MacLeod, L., & Czerwonka, J. (2016). "Code Reviewing in the Trenches" (MSR-TR-2016-27; survey fielded March 2015). Microsoft Research. https://www.microsoft.com/en-us/research/wp-content/uploads/2016/05/MS-Code-Review-Tech-Report-MSR-TR-2016-27.pdf
20. Meta Engineering. (2022). "Move faster, wait less: Improving code review time at Meta." https://engineering.fb.com/2022/11/16/culture/meta-code-review-time-improving/
21. Bacchelli, A., & Bird, C. (2013). "Expectations, Outcomes, and Challenges of Modern Code Review." ICSE. https://doi.org/10.1109/icse.2013.6606617
22. McIntosh, S., Kamei, Y., Adams, B., & Hassan, A. E. (2015). "An empirical study of the impact of modern code review practices on software quality." Empirical Software Engineering. https://doi.org/10.1007/s10664-015-9381-9
23. Greiler, M., & Bird, C. (2015). "Code Reviews Do Not Find Bugs." ICSE-SEIP talk. https://www.microsoft.com/en-us/research/wp-content/uploads/2015/05/codereviewsdonotfindbugs-icse2015-seiptalk.pdf
24. Heričko, T., & Šumak, B. (2023). "On the Relationship Between Linter Warning Density and Software Maintainability: An Empirical Study of JavaScript Projects." ICSIM 2023. https://doi.org/10.1145/3584871.3584884
25. Oliveira, N. D. C., Ribeiro, M., Bonifácio, R., Gheyi, R., Wiese, I., & Fonseca, B. (2022). "Lint-Based Warnings in Python Code." SCAM. https://doi.org/10.1109/scam55253.2022.00030
26. Gao, Z., Bird, C., & Barr, E. T. (2017). "To Type or Not to Type: Quantifying Detectable Bugs in JavaScript." ECOOP. https://discovery.ucl.ac.uk/id/eprint/10064729/1/typestudy.pdf
27. Khan, F. A., Chen, B., Varró, D., & McIntosh, S. (2022). "An Empirical Study of Type-Related Defects in Python Projects." IEEE TSE. https://doi.org/10.1109/TSE.2021.3082068
28. Ernst, M. D., et al. (2011). "Building and Using Pluggable Type-Checkers." ICSE. https://homes.cs.washington.edu/~mernst/pubs/pluggable-checkers-icse2011.pdf
29. Vassallo, C., Panichella, S., Palomba, F., Proksch, S., Gall, H. C., & Zaidman, A. (2020). "How developers engage with static analysis tools in different contexts." Empirical Software Engineering. https://doi.org/10.1007/s10664-019-09750-5
30. Habib, A., & Pradel, M. (2018). "How Many of All Bugs Do We Find? A Study of Static Bug Detectors." ASE. https://andrewhabib.org/publications/ase18-StaticCheckers.pdf
31. Vasilescu, B., Yu, Y., Wang, H., Devanbu, P., & Filkov, V. (2015). "Quality and Productivity Outcomes Relating to Continuous Integration in GitHub." FSE. https://cs.uwaterloo.ca/~m2nagapp/courses/CS846/1179/papers/vasilescu_fse15.pdf
32. Hilton, M., Tunnell, T., Huang, K., Marinov, D., & Dig, D. (2016). "Usage, Costs, and Benefits of Continuous Integration in Open-Source Projects." ASE. https://doi.org/10.1145/2970276.2970358
33. Wang, Y., Mäntylä, M. V., Liu, Z., & Markkula, J. (2022). "Test automation maturity improves product quality." Journal of Systems and Software. https://doi.org/10.1016/j.jss.2022.111259
34. Hilton, M., et al. (2018). "Trade-Offs in Continuous Integration: Assurance, Security, and Flexibility." ESEC/FSE. http://dig.cs.illinois.edu/papers/Hilton_CI_Tradeoffs.pdf
35. Meta Engineering. (2020). "Probabilistic Flakiness." https://engineering.fb.com/2020/12/10/developer-tools/probabilistic-flakiness/
36. Gallaba, K., Macho, C., Pinzger, M., & McIntosh, S. (2018). "Noise and Heterogeneity in Historical Build Data." ASE. https://doi.org/10.1145/3238147.3238171
37. Beller, M., Gousios, G., & Zaidman, A. (2017). "Oops, My Tests Broke the Build." MSR. https://doi.org/10.1109/msr.2017.62
38. NIST. (2021). "Guidelines on Minimum Standards for Developer Verification of Software" (NIST IR 8397). https://nvlpubs.nist.gov/nistpubs/ir/2021/NIST.IR.8397.pdf
39. Kochhar, P. S., Thung, F., & Lo, D. (2015). "Code coverage and test suite effectiveness." SANER. https://doi.org/10.1109/SANER.2015.7081877
40. Bach, T., Andrzejak, A., Pannemans, R., & Lo, D. (2017). "The Impact of Coverage on Bug Density in a Large Industrial Software Project." ESEM. https://doi.org/10.1109/ESEM.2017.44
41. Kochhar, P. S., et al. (2017). "Code Coverage and Post-release Defects: A Large-Scale Study on Open Source Projects." Microsoft Research. https://www.microsoft.com/en-us/research/publication/code-coverage-and-post-release-defects-a-large-scale-study-on-open-source-projects/
42. pre-commit. https://pre-commit.com/
43. GitHub Docs. "About protected branches." https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
44. Google. *Site Reliability Engineering*, Chapter 8: Release Engineering. https://sre.google/sre-book/release-engineering/
45. AWS Builders Library. "Automating safe, hands-off deployments." https://aws.amazon.com/builders-library/automating-safe-hands-off-deployments
46. Google. *The Site Reliability Workbook*, Chapter 16: Canarying Releases. https://sre.google/workbook/canarying-releases/
47. Google. *The Site Reliability Workbook*, Appendix B: Example Error Budget Policy. https://sre.google/workbook/error-budget-policy/
48. Google. *The Site Reliability Workbook*, Chapter 2: Implementing SLOs. https://sre.google/workbook/implementing-slos/
49. Google. *The Site Reliability Workbook*, Chapter 10: Postmortem Culture. https://sre.google/workbook/postmortem-culture/
50. AWS Well-Architected Framework. OPS06-BP03 Employ safe deployment strategies. https://docs.aws.amazon.com/wellarchitected/latest/operational-excellence-pillar/ops_mit_deploy_risks_deploy_mgmt_sys.html
51. Microsoft Learn. "Deployment gates concepts." https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/gates?view=azure-devops
52. Microsoft Learn. "Release gates and approvals overview." https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/
53. Microsoft Learn. "Integrate with ServiceNow change management." https://learn.microsoft.com/en-us/azure/devops/pipelines/release/approvals/servicenow?view=azure-devops
54. Microsoft DevOps Blog. "Configuring your release pipelines for safe deployments." https://devblogs.microsoft.com/devops/configuring-your-release-pipelines-for-safe-deployments/
55. Spinnaker. "Canary Overview." https://spinnaker.io/docs/guides/user/canary/canary-overview
56. Uber Engineering. "Continuous deployment for large monorepos." https://www.uber.com/us/en/blog/continuous-deployment/
57. Spotify. "Experiment like Spotify: Feature Flags." https://confidence.spotify.com/blog/feature-flags
58. Croft, R., Newlands, D., Chen, Z., & Babar, M. A. (2021). "An Empirical Study of Rule-Based and Learning-Based Approaches for Static Application Security Testing." ESEM. https://arxiv.org/pdf/2107.01921
59. Elder, S., Zahan, N., Shu, R., Metro, M., Kozarev, V., Menzies, T., & Williams, L. (2022). "Do I really need all this work to find vulnerabilities?" Empirical Software Engineering. https://arxiv.org/pdf/2208.01595
60. Advisera. "ISO 27001 Control 8.32 – Change management." https://advisera.com/iso27001/control-8-32-change-management
61. SecureSlate. "SOC 2 Evidence for Change Management." https://getsecureslate.com/blog/soc-2-evidence-for-change-management
62. DORA / Google Cloud. "Accelerate State of DevOps Report 2024." https://dora.dev/research/2024/dora-report/
63. Stack Overflow. "2024 Developer Survey — Professional Developers." https://survey.stackoverflow.co/2024/professional-developers/
64. JetBrains. "State of Developer Ecosystem Report 2024." https://www.jetbrains.com/lp/devecosystem-2024/
65. JetBrains Qodana. "State of Software Quality Report 2024." https://lp.jetbrains.com/software-quality-report-2024-by-qodana/
66. CD Foundation / SlashData. "State of CI/CD Report 2024." https://cd.foundation/state-of-cicd-2024/
67. Soares, E., Sizílio, G., Santos, J., Costa, D. A., & Kulesza, U. (2022). "The effects of continuous integration on software development: a systematic literature review." Empirical Software Engineering. https://doi.org/10.1007/s10664-021-10114-1
68. Davila, N., & Nunes, I. (2021). "A Systematic Literature Review and Taxonomy of Modern Code Review." Journal of Systems and Software. https://doi.org/10.1016/j.jss.2021.110951
69. Rafi, D. M., Kiran Moses, K. R., Petersen, K., & Mäntylä, M. V. (2012). "Benefits and Limitations of Automated Software Testing." AST. https://dl.acm.org/doi/10.1145/2663608.2663616 (author-hosted PDF: https://mmantyla.github.io/ast_2012_bare_conf.pdf)
70. NIST. (2023). "SATE VI Report: Bug Injection and Collection" (SP 500-341). https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.500-341.pdf
71. Schermann, G., Cito, J., Leitner, P., & Gall, H. C. (2016). "Towards Quality Gates in Continuous Delivery and Deployment." ICPC. https://doi.org/10.1109/icpc.2016.7503737
72. Schermann, G., Cito, J., Leitner, P., Zdun, U., & Gall, H. C. (2018). "We're doing it live: A multi-method empirical study on continuous experimentation." Information and Software Technology. https://doi.org/10.1016/j.infsof.2018.02.010
73. Microsoft DevOps Blog. "A Microsoft DevSecOps Static Application Security Testing (SAST) Exercise." https://devblogs.microsoft.com/devops/microsoft-devsecops-static-application-security-testing-sast-exercise
74. The New Stack. "Developers Testing More, JetBrains Study Finds." https://thenewstack.io/developers-testing-more-jetbrains-study-finds/ — secondary report stating the 95%/85% testing figures.
75. Google. *The Site Reliability Workbook*, Appendix C: Results of Postmortem Analysis. https://sre.google/workbook/postmortem-analysis/
