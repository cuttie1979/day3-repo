# Research plan — session 3

All 14 questions are **pre-run** in the Qubes qube with `feynman deepresearch`. This file now
carries what the runs actually produced: the real slug, the measured duration, and the state.

Results land in `outputs/` (drafts in `papers/`); the markdown goes into `research/<slug>.md`, with
the provenance file beside it.

| # | Question | Slug | Ran | Duration |
|---|---|---|---|---|
| 1 | What does Karpathy recommend for CLAUDE.md, and what reasoning or evidence supports it? | `karpathy-claude-md-recommendation` | 18:51–18:54 | **3 min** |
| 2 | How does Elon Musk use First Principle Thinking? | `elon-musk-first-principles-thinking` | 19:00–19:05 | 5 min |
| 3 | What are the best patterns to break down business requirements into technical requirements? | `requirements-decomposition-patterns` | 19:07–19:12 | 5 min |
| 4 | What questions are required to write quality business requirements with acceptance criteria? | `business-requirements-acceptance-criteria-questions` | 19:13–19:18 | 5 min |
| 5 | What quality gates do technology companies use for professional software development, and which have evidence behind them? | `software-quality-gates` | 19:39–20:18 | 39 min |
| 6 | How do regulated industries trace requirements to tests and evidence (ASPICE, ISO 26262, DO-178C, IEC 62304), and what can teams outside them learn from it? | `regulated-requirements-traceability` | 20:22–21:15 | 53 min |
| 7 | What documents does a professional engineering organisation maintain (ADR, design doc/RFC, runbook, threat model, changelog, PR description), and what is each for? | `engineering-document-types` | 21:16–21:21 | 5 min |
| 8 | Does writing tests before code improve outcomes with LLM coding agents? | `llm-agents-test-first` | 21:22–21:48 | 26 min |
| 9 | What test adequacy measures actually predict escaped defects: line coverage, mutation score, or seeded-fault detection? | `test-adequacy-escaped-defects` | 21:49–22:28 | 39 min |
| 10 | Is there evidence that automated code formatting and linting improve outcomes? | `code-formatting-linting-evidence` | 22:28–22:52 | 24 min |
| 11 | What does security-by-design look like in practice (threat modelling, OWASP ASVS/SAMM, least privilege, secrets management, dependency hygiene, SBOM)? | `security-by-design-practice` | 07:41–08:39 | 58 min |
| 12 | What makes an effective agent context file (AGENTS.md / CLAUDE.md), and what are its failure modes? | `agent-context-files` | 09:24–10:08 | 44 min |
| 13 | What is the empirical evidence on defects in AI-generated code: a taxonomy of where coding agents typically fail? | `ai-code-defect-taxonomy` | 10:12–11:17 | **65 min** |
| 14 | Is the claim that a defect costs 100x more to fix later supported by evidence? | `defect-cost-100x-claim` | 11:17–11:58 | 41 min |

**Total: 412 minutes — 6 h 52 min** of research. Range: 3 minutes to 65.

## What the durations mean

| Band | Runs | Use |
|---|---|---|
| **≤ 5 min** | 1, 2, 3, 4, 7 | short enough to **run live** in a session |
| 24–26 min | 8, 10 | a long block, or a pre-run |
| 39–65 min | 5, 6, 9, 11, 12, 13, 14 | pre-run only; never wait for one on stage |

For a live demonstration, **#1 at 3 minutes** is the safest: it is the tool-sharpening run, and the
result (how to write the context file) is used by the day itself.

## The two metadata anomalies, resolved

1. **#1's question text** in the message was #2's (Musk); the delivered file is
   `karpathy-claude-md-recommendation.md`, so the Karpathy question above is the one that ran.
2. **#5's "slug"** was the provenance file's name; the run's own markdown is `software-quality-gates.md`,
   and it is present.

## What the runs produced

28 files, 704 KB: **14 markdowns and 14 provenance files.** Every run wrote its markdown, so the
flash-tier risk (a run that answers in chat and writes nothing) did not materialise.

Every provenance file carries a verification verdict, and all 14 are **PASS WITH NOTES** — with the
reviewer passes recorded: FATAL and MAJOR findings were raised and fixed before delivery. Two runs
needed three reviewer passes (`test-adequacy-escaped-defects`, `software-quality-gates`).

### The first run is a second self-reveal

`karpathy-claude-md-recommendation` reports:

> *No evidence found that Karpathy wrote, published, or endorsed the derived CLAUDE.md file.*
> *Unverified anecdotal performance claims (e.g. "mistakes cut to 11%") excluded from findings.*

The CLAUDE.md that travels the world under Karpathy's name is **not his**, and the performance
number attached to it is folklore. This is the same shape as the opening quotation — and it is the
day's first research topic. It also lands beside a fact already in memory (the "41% → 3%" figure),
which the research does not support.

Three self-reveals now run through the day: the quotation, this file, and — to be read — the 100×
claim (#14).


## Commands (for a re-run)

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

## Notes

- **#14 is a second self-reveal.** The opening corrects a misattributed quotation; this corrects the
  profession's most-repeated number. If the evidence is thin, say so on the slide.
- **#6 is the room's own language.** Automotive, medical and aerospace answered "guaranteed quality"
  decades ago with traceability and evidence, not with better programmers.
- **#1 and #12 are one thread:** how to write the agent's context, and how it goes wrong.
- Every claim that reaches a slide needs its source; a claim without one is marked as the day's own
  hypothesis, not as fact.
