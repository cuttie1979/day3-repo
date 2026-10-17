# Research plan — session 3

Every question below is pre-run in the Qubes qube with `feynman deepresearch`.
The results are the day's material: we walk through them one by one in the session.

Run convention: output lands in `outputs/` (drafts in `papers/`); copy the markdown here as
`research/<slug>.md`, plus the provenance file when Feynman writes one.

| # | Question | Act | From | Status |
|---|---|---|---|---|
| 1 | What does Karpathy recommend for CLAUDE.md, and what reasoning or evidence supports it? | Opening · Material | Cuttie | todo |
| 2 | How does Elon Musk use First Principle Thinking? | Opening (stance) | Cuttie | todo |
| 3 | What are the best patterns to break down business requirements into technical requirements? | Contract | Cuttie | todo |
| 4 | What questions are required to write quality business requirements with acceptance criteria? | Contract | Cuttie | todo |
| 5 | What quality gates do technology companies use for professional software development, and which have evidence behind them? | Try · Close | Cuttie | todo |
| 6 | How do regulated industries trace requirements to tests and evidence (ASPICE, ISO 26262, DO-178C, IEC 62304), and what can teams outside them learn from it? | Contract · Try | Jarvis | todo |
| 7 | What documents does a professional engineering organisation maintain (ADR, design doc/RFC, runbook, threat model, changelog, PR description), and what is each for? | Contract · Close | Jarvis | todo |
| 8 | Does writing tests before code improve outcomes with LLM coding agents? | Try | Jarvis | todo |
| 9 | What test adequacy measures actually predict escaped defects: line coverage, mutation score, or seeded-fault detection? | Try · Close | Jarvis | todo |
| 10 | Is there evidence that automated code formatting and linting improve outcomes? | Code | Jarvis | todo |
| 11 | What does security-by-design look like in practice (threat modelling, OWASP ASVS/SAMM, least privilege, secrets management, dependency hygiene, SBOM)? | Code | Jarvis | todo |
| 12 | What makes an effective agent context file (AGENTS.md / CLAUDE.md), and what are its failure modes? | Material | Jarvis | todo |
| 13 | What is the empirical evidence on defects in AI-generated code: a taxonomy of where coding agents typically fail? | Whole day | Jarvis | todo |
| 14 | Is the claim that a defect costs 100× more to fix later supported by evidence? | Close | Jarvis | todo |

Acts: Understand (Feynman) → Contract (spec) → Try (test) → Material (context) → Close (loop).

## Commands

```bash
# 1
feynman deepresearch "What does Karpathy recommend for CLAUDE.md, and what reasoning or evidence supports it?"
# 2
feynman deepresearch "How does Elon Musk use First Principle Thinking?"
# 3
feynman deepresearch "What are the best patterns to break down business requirements into technical requirements?"
# 4
feynman deepresearch "What questions are required to write quality business requirements with acceptance criteria?"
# 5
feynman deepresearch "What quality gates do technology companies use for professional software development, and which have evidence behind them?"
# 6
feynman deepresearch "How do regulated industries trace requirements to tests and evidence (ASPICE, ISO 26262, DO-178C, IEC 62304), and what can teams outside them learn from it?"
# 7
feynman deepresearch "What documents does a professional engineering organisation maintain (ADR, design doc/RFC, runbook, threat model, changelog, PR description), and what is each for?"
# 8
feynman deepresearch "Does writing tests before code improve outcomes with LLM coding agents?"
# 9
feynman deepresearch "What test adequacy measures actually predict escaped defects: line coverage, mutation score, or seeded-fault detection?"
# 10
feynman deepresearch "Is there evidence that automated code formatting and linting improve outcomes?"
# 11
feynman deepresearch "What does security-by-design look like in practice (threat modelling, OWASP ASVS/SAMM, least privilege, secrets management, dependency hygiene, SBOM)?"
# 12
feynman deepresearch "What makes an effective agent context file (AGENTS.md / CLAUDE.md), and what are its failure modes?"
# 13
feynman deepresearch "What is the empirical evidence on defects in AI-generated code: a taxonomy of where coding agents typically fail?"
# 14
feynman deepresearch "Is the claim that a defect costs 100x more to fix later supported by evidence?"
```

## Slugs

```
1  karpathy-claude-md        8  test-first-agents
2  musk-first-principles     9  test-adequacy
3  requirements-breakdown   10  formatting-evidence
4  acceptance-criteria      11  security-by-design
5  quality-gates            12  context-file-effectiveness
6  regulated-traceability   13  ai-code-defects
7  engineering-documents    14  defect-cost-100x
```

## Notes

- **#14 is a second self-reveal.** The opening corrects a misattributed quote; this corrects the
  profession's most-repeated number. If the evidence is thin, say so on the slide.
- **#6 is the room's own language.** Automotive, medical and aerospace answered "guaranteed quality"
  decades ago with traceability and evidence, not with better programmers. The room has automotive
  engineers in it.
- **#1 and #12 are one thread:** how to write the agent's context, and how it goes wrong.
- Every claim that reaches a slide needs its source; a claim without one is marked as the day's own
  hypothesis, not as fact.
