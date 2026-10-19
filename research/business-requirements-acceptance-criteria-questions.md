# Questions Required to Write Quality Business Requirements with Acceptance Criteria

## Executive Summary

Writing quality business requirements with acceptance criteria is less about filling in a template and more about asking the right questions at the right time. Practitioner guides and standards converge on a four-stage question pattern:

1. **Elicit the need** — ask who, what, when, where, why, and how to understand the business problem and stakeholder context before proposing a solution (Watermark Learning; Business Analyst’s Toolkit; Bridging the Gap).
2. **Shape the requirement** — test the requirement against quality attributes such as clear/unambiguous, complete, consistent, feasible, testable, atomic, traceable, and prioritized (IIBA 2021; IEEE 29148:2018; Mannion & Keepence 1995).
3. **Define acceptance criteria** — turn each requirement into verifiable, pass/fail conditions using scenario-based (Given-When-Then), checklist, or rules-based formats, and explicitly cover happy paths, edge cases, failure behaviour, boundaries, and permissions (Atlassian; Scrum Alliance; ThoughtWorks; GitScrum).
4. **Review and confirm** — ask whether the criteria are observable, bounded, owned, and whether a tester or stakeholder could confirm them without further clarification (ReqLens; IIBA task card).

No single canonical checklist exists. The most useful question sets adapt the core frameworks — IIBA’s quality criteria, IEEE 29148, SMART requirements, INVEST, and Given-When-Then — to the project’s scale, regulatory context, and team maturity.

---

## 1. What makes a requirement "quality"?

Before asking questions about a specific requirement, it helps to know what "quality" means. Three references give overlapping but slightly different attribute lists.

### 1.1 IIBA’s nine quality criteria

The International Institute of Business Analysis lists nine criteria for requirement quality (IIBA 2021):

| Criterion | Meaning |
|-----------|---------|
| Understandable | Stakeholders can read and comprehend it. |
| Concise | No unnecessary content. |
| Feasible | Achievable within technical, schedule, budget, and risk constraints. |
| Atomic | One idea per requirement. |
| Complete | Nothing left to the imagination; actor, trigger, and outcome stated. |
| Testable | Acceptable levels of fulfillment can be verified. |
| Prioritized | Ranked by significance, importance, or value. |
| Consistent | No conflicts with other requirements or business needs. |
| Unambiguous | Clear whether the solution meets the need. |

### 1.2 IEEE 29148:2018 characteristics

The systems and software engineering standard ISO/IEC/IEEE 29148:2018 says every individual requirement should be necessary, appropriate, unambiguous, complete, singular, feasible, verifiable, correct, and conforming (TheArtOfService summary of IEEE 29148:2018).

### 1.3 SMART requirements

Mannion and Keepence adapted the SMART objective framework to software requirements. In this usage, SMART means Specific, Measurable, Attainable, Realisable, and Traceable — note that **T stands for Traceable**, not Time-bound, because delivery dates usually live in the project plan rather than individual requirements (Mannion & Keepence 1995).

### 1.4 INVEST (for user stories)

For teams working with user stories, the Agile Alliance summarizes INVEST as Independent, Negotiable, Valuable, Estimable, Small, and Testable (Agile Alliance).

---

## 2. Questions to ask while eliciting the need

Before a requirement is written, the analyst must understand the business context, stakeholders, problem, and constraints. Practitioner sources consistently structure these questions around the classic "W" dimensions plus pain-point probes.

### 2.1 The six dimensions (Who, What, How, When, Where, Why)

The Business Analyst’s Toolkit proposes preparing at least one strong question for each dimension (Business Analyst’s Toolkit):

- **Who:** Who are the roles, users, and stakeholders? Who provides inputs? Who receives outputs? Who has authority to decide?
- **What:** What services, products, or outputs must be produced? What is the end result? What are the pieces of this feature?
- **How:** How does the current process work? How will stakeholders use this feature? How will we know it is complete?
- **When:** When will the feature be used? When do processes occur? When will it fail? When does it need to be done?
- **Where:** Where does the process start? Where will the user access the feature? Where will results be visible?
- **Why:** Why was the change requested? Why is this level of service needed? Why does the current situation exist?

Bridging the Gap adds that these questions should be asked per feature or business requirement, not as a rigid script, and should include follow-ups such as: What assumptions am I making that I need to confirm? What must happen before this feature can work? What if alternative scenarios occur? What other questions should I be asking? (Bridging the Gap).

### 2.2 Stakeholder-specific questions

Modern Analyst divides generic stakeholder questions into four threads with distinct targets (Modern Analyst):

**Key stakeholder / sponsor (business requirements and change)**
- How does your organization contribute to the strategic plan?
- How do you measure success of the project and its business impact?
- What are the project risks and chances of success vs. failure?
- What could be discarded from the project plan without anyone noticing?

**User (user requirements)**
- Describe your role and major responsibilities.
- What information and systems do you use?
- How do you measure success in your job?
- What is helping or inhibiting you from doing your job?
- What would you change about how you carry out your responsibilities?

**Conditions / non-functional requirements**
- How quickly does the system need to respond?
- How many users will use it at the same time?
- When does the system need to be available?
- What level of security and audit is needed?
- What disaster backup and recovery are needed?

**Legacy / transition requirements**
- Does a legacy system need to continue after the new system is live?
- Do data files or business rules need to be converted?

### 2.3 Powerful probes

Watermark Learning’s 101+ questions include cross-cutting probes that surface hidden requirements (Watermark Learning):

- Ask for pairs: why/why not/why now; what/what not/what now; who/who not/who now.
- Ask for extremes: best/worst; longest/shortest; easiest/hardest.
- Ask volumetrics: how many, how often, when.
- Ask about exception processing: under what conditions might the product not perform as planned?
- Ask about dependencies: what must be completed before another function starts?
- Ask about physical environments, deployment requirements, safety, availability, accessibility, and regulatory bodies.

The Business Analyst’s Toolkit emphasizes that the most important word in elicitation is **why**, used curiously rather than confrontationally, often via the Five Whys technique (Business Analyst’s Toolkit).

---

## 3. Questions to shape a requirement

Once information has been elicited, the analyst turns it into a requirement statement. The following questions map the quality attributes into a practical checklist.

| Quality attribute | Question to ask |
|-------------------|-----------------|
| Specific / Atomic | Does this requirement contain exactly one idea? Can it be understood independently of other requirements? |
| Clear / Unambiguous | Will every stakeholder interpret it the same way? Are vague words like "fast," "easy," "user-friendly," "several," or "appropriate" removed? |
| Complete | Does it state actor, trigger, and outcome? Are limits, exceptions, and business rules included? |
| Consistent | Does it conflict with other requirements, policies, or current system behaviour? Is terminology used uniformly? |
| Feasible / Realisable | Can it be delivered within known technical, time, budget, and resource constraints? Have builders raised concerns? |
| Testable / Verifiable / Measurable | Can a pass/fail test be defined? Is there an observable outcome or a quantified threshold? |
| Traceable | Can we trace this back to a business need, forward to design/implementation/test, and identify its originator and rationale? |
| Prioritized / Necessary | Is this essential, or merely desirable? If omitted, would a gap remain? |
| Bounded / In scope | Are in-scope and out-of-scope items written down? Are dependencies named? |
| Owned | Does the requirement have an owner? Are decisions recorded with who made them and why? |

ReqLens’s one-page requirement-quality checklist distills this further into a reviewer gate (ReqLens):

- Clear: one reading only; no vague words; every number has a unit.
- Consistent terms: meanings match the glossary or policy wording.
- Complete: actor, trigger, and outcome stated; business rules include limits/exceptions and owners; failure behaviour stated; permissions stated.
- Testable: each acceptance criterion has an observable pass or fail without a clarifying question.
- Boundaries explicit: values either side of each limit can be tested.
- Consistent: conflicts resolved; intended changes to current behaviour stated.
- Feasible: delivery achievable; assumptions recorded; builder concerns raised.
- Bounded: scope and out-of-scope written; dependencies named.
- Owned: requirement has an owner; open questions have an owner and due date.

---

## 4. Questions to define acceptance criteria

Acceptance criteria convert a requirement into pass/fail conditions that determine when work is accepted. Sources agree they should be written before development begins and refined collaboratively (Atlassian; Scrum Alliance).

### 4.1 Core acceptance-criteria questions

Inside Product offers ten powerful questions for discovering acceptance criteria (Inside Product):

1. If I deliver this user story, what things should I account for?
2. If this user story has already been implemented, what would I try out?
3. How do we know when we have successfully delivered this user story?
4. How should we verify that this story is implemented completely and correctly?
5. Are there cases with this user story that we have not identified how the system should behave?
6. What about…? (followed by "Is that within scope?")
7. Anything that has not been considered so far?
8. What happens when…?
9. What scenarios are relevant?
10. Are all scenarios necessary (right now)?

GitScrum’s refinement questions cover similar ground (GitScrum):

- What if the user does X?
- What happens when Y fails?
- Is this testable as written?
- What’s the boundary here?
- How do we know this works?

### 4.2 Format-specific guidance

Three formats dominate practitioner guidance (GitScrum):

**Bullet / checklist format** — best for simple features, CRUD operations, form requirements, or when scenarios are overkill. Example: "Display name field (max 50 chars)", "Success message on save", "Cancel discards changes."

**Given-When-Then (scenario) format** — best for behaviour flows, login flows, and anything with cause-and-effect. ThoughtWorks stresses the strict structure (ThoughtWorks):

- **Given** = precondition(s) / state / parameters relevant to the scenario.
- **When** = a single trigger or state change.
- **Then** = expected outcome(s).

Anti-patterns include mixing conditions into the When clause, using multiple Whens, or over-sequencing preconditions that do not matter to the scenario.

**Rules-based format** — best for business constraints, validation logic, calculations, and regulatory requirements. Example: "Rule 1: Orders > $100 get free shipping. Rule 2: Premium members get 10% discount."

### 4.3 Coverage questions for acceptance criteria

Scrum Alliance and Atlassian add that strong acceptance criteria should explicitly cover (Scrum Alliance; Atlassian):

- **Happy path** — the normal successful flow.
- **Sad / error paths** — invalid input, empty states, network errors, permission denied, concurrent access.
- **Boundaries** — minimum/maximum values, thresholds, limits.
- **Roles and permissions** — who may and who must not perform the action.
- **Failure behaviour** — what the system does when a dependency fails.
- **Outcome, not implementation** — what the user experiences, not how it is built.

### 4.4 Acceptance-criteria quality checks

Atlassian summarizes good acceptance criteria as clear and concise, testable/objectively verifiable, outcome-focused (not recipe-focused), measurable/quantified where possible, and independent — each criterion stands on its own (Atlassian). Scrum Alliance adds that acceptance criteria are pass/fail; they are never partially fulfilled (Scrum Alliance).

---

## 5. Questions to review before approval

Before a requirement and its acceptance criteria are approved for delivery, a final review should confirm that both are ready.

### 5.1 Requirement review questions

Based on ReqLens and IIBA guidance, ask (ReqLens; IIBA 2021):

- Is the requirement clear after one reading?
- Are all vague terms removed and every number given a unit?
- Are actor, trigger, and outcome stated?
- Are business rules, limits, exceptions, and their owners documented?
- Is failure behaviour stated (invalid input, repeated attempts, dependency failure)?
- Are permissions stated (who may do this, who must not)?
- Are conflicts with other requirements or policies resolved?
- Are assumptions recorded?
- Is the requirement feasible within known constraints?
- Are scope, out-of-scope, and dependencies written down?
- Does the requirement have an owner?
- Does every open question have an owner and due date?

### 5.2 Acceptance-criteria review questions

Ask whether (ReqLens):

- QA can create test cases from these criteria without asking clarifying questions.
- Each criterion is observable pass/fail.
- All business rules are covered.
- Negative scenarios are included.
- Boundary cases are included.
- Role-based scenarios are included, if relevant.
- Assumptions are clearly marked.
- Open questions are listed.
- Stakeholders would understand the criteria.
- Each criterion is independent of the others.
- Criteria describe "what" rather than "how".

### 5.3 Verification vs. validation

IIBA distinguishes two review tasks:

- **Verify Requirements** — check that requirements and designs are defined correctly and meet quality standards (internal correctness, consistency, completeness).
- **Validate Requirements** — check that they align with stakeholder needs and business value.

Both are needed: a perfectly written requirement can still solve the wrong problem (IIBA task card).

---

## 6. Caveats and disagreements

### 6.1 Acceptance criteria are optional in Scrum

Scrum Alliance and Atlassian note that acceptance criteria are **not part of the Scrum Guide**. They are a complementary practice that many Scrum and agile teams choose to use. Therefore, mandating them should be a team decision, not a framework requirement (Scrum Alliance; Atlassian).

### 6.2 Format wars are context-dependent

There is no universal best format. Checklists work for simple features; Given-When-Then works for behaviour flows; rules work for constraints. The right choice depends on complexity, team skill, and whether the criteria will be automated (GitScrum).

### 6.3 Over-formalization is a risk

Scrum Alliance warns against covering every detail, which can produce cumbersome and ineffective acceptance criteria. GitScrum similarly flags criteria that are "too detailed" (e.g., specifying button colours or implementation technologies) as anti-patterns (Scrum Alliance; GitScrum).

### 6.4 SMART has domain-specific interpretations

In generic goal-setting, SMART often means Time-bound. In software requirements, Mannion and Keepence argue T should mean Traceable because individual requirements rarely carry their own dates. Teams should clarify which version they are using (Mannion & Keepence 1995).

### 6.5 "Why" requires finesse

Both the Business Analyst’s Toolkit and Bridging the Gap note that asking "why" directly can annoy stakeholders. Rephrasing as "What is the reason…?" or "What would happen if…?" is often more productive (Business Analyst’s Toolkit; Bridging the Gap).

---

## 7. Open questions

- Empirical research comparing acceptance-criteria formats (checklist vs. Given-When-Then vs. rules) against defect or rework rates is scarce; most guidance is experience-based.
- The impact of AI-assisted requirements-generation tools on requirement quality, traceability, and acceptance-criteria completeness has not yet stabilized into widely accepted practices.
- There is no agreed-upon minimal viable set of acceptance criteria per requirement type; teams must calibrate by domain risk and regulatory demands.

---

## Sources

- Agile Alliance. "What does INVEST Stand For?" https://agilealliance.org/glossary/invest/
- Atlassian. "What is Acceptance Criteria? Definition, Examples, & Tips." https://www.atlassian.com/work-management/project-management/acceptance-criteria
- Bridging the Gap. "What Questions Do I Ask During Requirements Elicitation?" https://www.bridging-the-gap.com/what-questions-do-i-ask-during-requirements-elicitation/
- Business Analyst’s Toolkit. "Requirements Elicitation Questions for Business Analysts." https://businessanalyststoolkit.com/requirements-elicitation-questions/
- GitScrum. "Writing Clear Acceptance Criteria Best Practices." https://docs.gitscrum.com/en/best-practices/writing-clear-acceptance-criteria
- Inside Product. "Ten Powerful Questions for Discovering Acceptance Criteria." https://insideproduct.co/questionsforacceptancecriteria/
- International Institute of Business Analysis. "Quality Criteria for Requirements." PDF, 2021. https://www.iiba.org/contentassets/1dc6a35dff8c46ff85c6cb6b916b7ea3/9-quality-criteria-for-requirements.pdf
- International Institute of Business Analysis. "Verify Requirements and Designs" task card. PDF. https://www.iiba.org/contentassets/b31f8cd50b4e44399d72cf2374d18360/task-card-verify-req-and-designs.pdf
- ISO/IEC/IEEE 29148:2018 characteristics summary via TheArtOfService. https://compliance.theartofservice.com/controls/iso-iec-ieee-29148-2018-systems-and-software-engineering-requirements-engineering/5-2-5
- Mannion, M. and Keepence, B. "SMART Requirements." ACM SIGSOFT Software Engineering Notes, vol. 20 no. 2, April 1995. https://wstomv.win.tue.nl/edu/2ip30/references/smart-requirements.pdf
- Modern Analyst. "Generic Questions for Interviewing Stakeholders." https://www.modernanalyst.com/Resources/Articles/tabid/115/ID/1087/Generic-Questions-for-Interviewing-Stakeholders.aspx
- ReqLens. "Requirement quality checklist." PDF. https://reqlens.com/academy/downloads/requirement-quality-checklist.pdf
- Scrum Alliance. "Everything You Need to Know About Acceptance Criteria." https://resources.scrumalliance.org/Article/need-know-acceptance-criteria
- ThoughtWorks. "Applying BDD acceptance criteria in user stories." https://www.thoughtworks.com/en-us/insights/blog/applying-bdd-acceptance-criteria-user-stories
- Watermark Learning. "101+ Questions you can ask to Elicit Requirements and Uncover Expectations." PDF. https://www.watermarklearning.com/downloads/101-questions.pdf
