# Prompts — session 3

One file per prompt. The file name is the slide it belongs to:

```
<slide>.<order on that slide>-<slug>.md
```

`15.1` is the first prompt on slide 15. On the day, when slide 15 is up, the prompt is `15.1`.

⚠️ **The numbers come from the written deck (74 slides).** If the deck is reordered, `renumber.sh`
moves the files in one pass.

## Two kinds

| | Slides | What |
|---|---|---|
| **the setup** | 13–17 | builds the demo environment: the toolchain, the skill, the two MCPs |
| **the day** | 19, 20, 21, 64 | shown on the projector: the three creation prompts and the calibration |

## The setup, in order

Four prompts at the MCP beats. **The order is a dependency chain, not a preference:**

```
13 rust toolchain  →  needed to build the datetime MCP, and to warm the demo's build cache
14 opencode skill  →  needed by 15 and 17: both configure through it
15 datetime MCP    →  needs 13 (a Rust server) and 14
17 mempalace MCP   →  needs 14
```

| File | Slide | What it creates | Needs |
|---|---|---|---|
| `13.1-install-rust-toolchain.md` | 13 | rustup, cargo, clippy, rustfmt — and a warmed `target/` | — |
| `14.1-opencode-skill.md` | 14 | the global `opencode-configuration` skill | — |
| `15.1-install-datetime-mcp.md` | 15 | the datetime MCP, built from source | 13, 14 |
| `17.1-install-mempalace-mcp.md` | 17 | the mempalace MCP in `~/AIHome/mempalace` | 14 |

The gap is real: 16 is the memory slide (what mempalace is *for*); its install prompt follows it.

## The prompts shown on the day

| File | Slide | What it creates |
|---|---|---|
| `19.1-jarvis-orchestrator.md` | 19 | the global master orchestrator (Jarvis) |
| `20.1-workflow-agents.md` | 20 | the project's `wf-*` lens agents |
| `21.1-workflow-md.md` | 21 | `WORKFLOW.md` — the measure |
| `64.1-review-calibration.md` | 64 | runs the review on the known-broken module |

## One thing to know about the language

The setup prompts (13–17) are in **English**: they run in a terminal, and the room does not read
them off a projector. The prompts shown on the day are in **Hungarian**, because the room reads
them, and the deck keeps product names, commands and code in English.

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
