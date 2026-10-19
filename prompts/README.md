# Prompts — session 3

One file per prompt. Two kinds, and the file name says which:

```
00.N-<slug>.md          the setup, run in order before the day (N is the run order)
<slide>.<order>-<slug>.md   a prompt shown on that slide during the session
```

`15.2` is the second prompt on slide 15. On the day, when slide 15 is up, the prompt is `15.1`.
`00.3` is the third setup prompt: the name itself carries the run order.

⚠️ **The slide numbers come from the written deck.** The three creation prompts sit on slides 15, 16
and 17; the calibration prompt on slide 60. If the deck is reordered, `renumber.sh` moves them in one pass.

## The setup, in order

Four prompts that build the demo environment. **The order is a dependency chain, not a preference:**

```
00.1 rust toolchain   →  needed to build the datetime MCP, and to warm the demo's build cache
00.2 opencode skill   →  needed by 00.3 and 00.4: both configure through it
00.3 datetime MCP     →  needs 1 (a Rust server) and 2
00.4 mempalace MCP    →  needs 2
```

| File | What it creates | Needs |
|---|---|---|
| `00.1-install-rust-toolchain.md` | rustup, cargo, clippy, rustfmt — and a warmed `target/` | — |
| `00.2-opencode-skill.md` | the global `opencode-configuration` skill | — |
| `00.3-install-datetime-mcp.md` | the datetime MCP, built from source | 00.1, 00.2 |
| `00.4-install-mempalace-mcp.md` | the mempalace MCP in `~/AIHome/mempalace` | 00.2 |

## The prompts shown on the day

| File | Slide | What it creates |
|---|---|---|
| `15.1-jarvis-orchestrator.md` | 15 | the global master orchestrator (Jarvis) |
| `16.1-workflow-agents.md` | 16 | the project's `wf-*` lens agents |
| `17.1-workflow-md.md` | 17 | `WORKFLOW.md` — the measure |
| `60.1-review-calibration.md` | 60 | runs the review on the known-broken module |

## One thing to know about the language

The setup prompts (00.x) are in **English**: they run in a terminal, before the session, and nobody
reads them off a projector. The prompts shown on the day are in **Hungarian**, because the room reads
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
