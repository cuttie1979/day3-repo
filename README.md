# day3-repo — ZEL Agentic Coding, session 3 (2026-10-20)

Materials for session 3: research, sources, and everything prepared for the day.

- **Session 3 · Garantált minőség? (Guaranteed quality?) · 20 October 2026 · remote**
- Topic: test-first and spec-driven development, context engineering
- Presenter: Popovics „Popi" László (alone)
- Deck: the site's own repository (aicode.page), `content/day3/{hu,en}.js`

## Layout

```
research/    feynman.is /deepresearch outputs (markdown) + provenance files
```

## research/

Holds the output of `feynman deepresearch "<question>"`, run in a Qubes qube.
`qvm-copy` moves it out of the qube into `QubesIncoming/`; from there it is copied here.

Naming: `<topic-slug>.md` plus `<topic-slug>.provenance.md` when Feynman writes one.

Published on the day's page as `aicode.page/research/<file>.md`
(the site server serves the site repo's `content/files/research/` as-is).

## Research questions

| # | Question | Act | Mode |
|---|---|---|---|
| 1 | Does writing tests before code improve code quality with LLM coding agents? | Try | **live** |
| 2 | Do structured requirements (acceptance criteria, Gherkin, formal specs) improve LLM code generation? | Contract | pre-run |
| 3 | How does context (structure vs prose, ordering, window) affect LLM code generation, and what degrades over multi-turn? | Material | pre-run |
| 4 | Mutation testing and test adequacy: what does coverage miss? | Try / Close | pre-run |
| 5 | What is not guaranteed: LLM-as-reviewer reliability, self-preference bias, plausible-but-wrong code | Close | pre-run |

The day's acts: Understand (Feynman) → Contract (spec) → Try (test) → Material (context) → Close (loop).

## Running it (in the Qubes qube)

```bash
export FEYNMAN_TELEMETRY=off
export DEEPSEEK_API_KEY=…
export OPENALEX_API_KEY=…            # own daily budget instead of the shared one
export SEMANTIC_SCHOLAR_API_KEY=…    # own rate limit
export CROSSREF_MAILTO=<email>       # polite pool: 3/s instead of 1/s

feynman deepresearch "<question>"
```

Model: DeepSeek V4.1 Flash (`feynman model set deepseek/<id>`).
Output: `outputs/` (drafts to `papers/`); `/deepresearch`, `/lit` and `/recipe` also write a provenance file.

**Check after every run:** did a file land in `outputs/`? A flash-tier model tends to
answer in chat without writing files, and then there is no markdown to use.
