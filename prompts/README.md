# Prompts — session 3

One file per prompt. The file name is the slide it belongs to:

```
<slide>.<order on that slide>-<slug>.md
```

`15.2` is the second prompt on slide 15. On the day, when slide 15 is up, the prompt is `15.1`.

⚠️ **The slide numbers are provisional.** They come from the deck, and the deck is not written yet.
The numbers below assume the opening has 14 slides before the first prompt. When the deck exists,
they are renumbered in one pass — see `renumber.sh`.

## The prompts

| File | Slide | What it creates |
|---|---|---|
| `15.1-jarvis-orchestrator.md` | 15 | the global master orchestrator (Jarvis) |
| `16.1-workflow-agents.md` | 16 | the project's `wf-*` lens agents |
| `17.1-workflow-md.md` | 17 | `WORKFLOW.md` — the measure |
| `48.1-review-calibration.md` | ~48 | runs the review on the known-broken module |

## Why the prompts are shaped this way

Every prompt carries the same four things, and they are the day's own discipline:

1. **the contract** — what must be true of the result
2. **what NOT to build** — the orchestrator gets no measure; a lens is a question, not a persona
3. **"ask, do not guess"** — the same rule the day 2 `CLAUDE.md` used
4. **evidence** — print the path, and say how it can be checked

Two things are stated explicitly because the model would otherwise get them wrong:

- **OpenCode V2 syntax** (`permissions`, `shell`, `subagent`). The existing agent files in
  `~/.config/opencode/agents/` use V1 syntax (`permission`, `bash`, `task`), and a model that reads
  them will copy them.
- **The agents may not edit the measure.** In V2 that is a permission rule, not a promise:

```yaml
permissions:
  - action: edit
    resource: ".opencode/**"
    effect: deny
  - action: edit
    resource: "WORKFLOW.md"
    effect: deny
```

## Acceptance criteria for the generated agents

Check these with `grep` at the end, not by opinion:

| AC | Check |
|---|---|
| every lens has **one question**, not a role | its body states `finding = …` and `evidence = …` |
| none of them can edit the measure | the `permissions` list denies `.opencode/**` and `WORKFLOW.md` |
| a review may only report a **sourced** deviation | the body requires a memory reference and its source |
| **V2 syntax** | `permissions`, `shell`, `subagent` appear; `permission:`, `bash:`, `task:` do not |
| the orchestrator does not implement | `edit` is denied |

**One level up, this is the day's own recursion:** the tool gets acceptance criteria too, and the
result is checked against them. The same discipline the agents apply to the code.
