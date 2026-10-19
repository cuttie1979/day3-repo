# day3-repo — ZEL Agentic Coding, session 3 (2026-10-20)

Materials for session 3: research, sources, and everything prepared for the day.

- **Session 3 · Garantált minőség? (Guaranteed quality?) · 20 October 2026 · remote**
- Topic: test-first and spec-driven development, context engineering
- Presenter: Popovics „Popi" László (alone)
- Deck: the site's own repository (aicode.page), `content/day3/{hu,en}.js`

## Layout

```
research/        feynman.is /deepresearch outputs (markdown) + provenance files
rust-aes-demo/   the Rust demonstration: the same AES module, naive and hardened
prompts/         one file per prompt, named by the slide it belongs to
```

`itinerary.md` — what to make, test and measure, in order, with acceptance criteria.
`research-plan.md` holds the 14 research questions and their run commands.
`prompts/README.md` explains the numbering and the acceptance criteria for the generated agents.

## research/

Holds the output of `feynman deepresearch "<question>"`, run in a Qubes qube.
`qvm-copy` moves it out of the qube into `QubesIncoming/`; from there it is copied here.

Naming: `<topic-slug>.md` plus `<topic-slug>.provenance.md` when Feynman writes one.

Downloaded from this repository: everyone takes their own copy from `research/`.
The day's page does not serve them.

## Research questions

See **[research-plan.md](research-plan.md)** — the 14 questions, the ready-to-paste
`feynman deepresearch` command for each, the slugs, and the status.

All of them are pre-run in the qube; the session walks through the results one by one.

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
