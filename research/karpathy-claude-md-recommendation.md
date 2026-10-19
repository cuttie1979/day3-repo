# What Karpathy Recommends for CLAUDE.md

## Executive Summary

Andrej Karpathy has **not published a CLAUDE.md file or template that he recommends**. The widely circulated "Karpathy CLAUDE.md" (four principles: Think Before Coding, Simplicity First, Surgical Changes, Goal-Driven Execution) was written by developer Jiayuan Chang and is explicitly described as being **derived from Karpathy's public observations** about LLM coding-agent failure modes [[repo]][[mcp]]. Karpathy's own CLAUDE.md, found in his `karpathy/llm-council` repository, is ordinary project documentation—architecture notes, ports, and implementation gotchas—not a prescriptive behavioral guide [[llm-council]][[delv]].

So the accurate answer is twofold:
1. The famous four-principle file is a **distillation by Chang** of Karpathy's reasoning.
2. Karpathy's reasoning centers on four failure modes he observed while dogfooding Claude Code: silent wrong assumptions, overcomplication, drive-by orthogonal changes, and vague success criteria [[thread]].

## What Karpathy Actually Said

In a long X thread (status `2015883857489522876`) posted around January 2026 after weeks of using Claude Code, Karpathy described a rapid shift to agentic coding and listed recurring failure modes. A direct fetch of X.com returned HTTP 403, but the full text is preserved by a ThreadReaderApp mirror [[thread]].

Key excerpts from the thread:

> "The models make wrong assumptions on your behalf and just run along with them without checking. They don't manage their confusion, they don't seek clarifications, they don't surface inconsistencies, they don't present tradeoffs, they don't push back when they should." [[thread]]

> "They also really like to overcomplicate code and APIs, they bloat abstractions, they don't clean up dead code after themselves, etc. They will implement an inefficient, bloated, brittle construction over 1000 lines of code and it's up to you to be like 'umm couldn't you just do this instead?'" [[thread]]

> "They still sometimes change/remove comments and code they don't like or don't sufficiently understand as side effects, even if it is orthogonal to the task at hand." [[thread]]

> "LLMs are exceptionally good at looping until they meet specific goals... Don't tell it what to do, give it success criteria and watch it go." [[thread]]

Crucially, Karpathy noted that these problems persisted **"despite a few simple attempts to fix it via instructions in CLAUDE.md."** This shows he was already experimenting with CLAUDE.md as a mitigation, but he did not share the contents of those attempts [[thread]].

## The Derived File: What It Recommends

A few weeks after Karpathy's thread, Jiayuan Chang (GitHub: `forrestchang`) created `forrestchang/andrej-karpathy-skills`, a repository containing a single CLAUDE.md file [[repo]]. The README states it is "derived from Andrej Karpathy's observations on LLM coding pitfalls." The file has been widely forked and packaged as a Claude Code plugin [[repo]].

The four principles and the problems they address:

| Principle | Targets Karpathy's Failure Mode |
| --- | --- |
| **1. Think Before Coding** | Silent assumptions; hidden confusion; missing tradeoffs; failure to push back [[repo]][[mcp]] |
| **2. Simplicity First** | Overcomplication; bloated abstractions; speculative flexibility [[repo]][[mcp]] |
| **3. Surgical Changes** | Drive-by changes to orthogonal code; "improving" unasked code; style drift [[repo]][[mcp]] |
| **4. Goal-Driven Execution** | Vague prompts; lack of verifiable success criteria; wasted agent loops [[repo]][[mcp]] |

The file also includes a tradeoff note: the guidelines bias toward caution over speed, and judgment should be used for trivial tasks [[raw]].

## Reasoning and Evidence Supporting Each Principle

### 1. Think Before Coding
Karpathy's observation that models "make wrong assumptions on your behalf and just run along with them" is the direct rationale [[thread]]. The CLAUDE.md instructs Claude to state assumptions explicitly, present multiple interpretations, and stop to ask when confused [[raw]]. This converts a default-confident agent into one that surfaces uncertainty before writing code [[mcp]].

### 2. Simplicity First
Karpathy noted agents "like to overcomplicate code and APIs" and produce "a bloated construction over 1000 lines of code" that can be reduced to 100 [[thread]]. The file counters this with negative rules: no features beyond what was asked, no abstractions for single-use code, no configurability nobody requested, and a 200-to-50-line rewrite heuristic [[raw]].

### 3. Surgical Changes
Karpathy observed agents "sometimes change/remove comments and code they don't sufficiently understand as side effects, even if it is orthogonal to the task" [[thread]]. The file's "touch only what you must" rule, the prohibition on refactoring unbroken code, and the instruction to mention—but not delete—unrelated dead code all map to this failure mode [[raw]].

### 4. Goal-Driven Execution
Karpathy's remark that "LLMs are exceptionally good at looping until they meet specific goals" is the foundation [[thread]]. The file tells the user to transform vague instructions into verifiable goals: "Add validation" becomes "Write tests for invalid inputs, then make them pass" [[raw]]. It also provides a plan template with explicit verification steps for multi-step tasks [[raw]]. Anthropic's own best-practices documentation independently recommends giving Claude verifiable checks, calling it "the single highest-leverage thing you can do" [[anthropic-best]].

## What Karpathy's Own CLAUDE.md Looks Like

Karpathy's actual CLAUDE.md is in the `karpathy/llm-council` repository [[llm-council]]. It contains:
- Project overview of LLM Council (a multi-model deliberation system)
- Backend and frontend architecture notes
- Port numbers (backend 8001, frontend 5173)
- Design decisions (Stage 2 prompt format, de-anonymization strategy)
- Common gotchas (module imports, CORS, ranking parse failures)
- Future enhancement ideas

It does **not** contain the four behavioral principles. Independent sources confirm this gap [[delv]][[rusin]].

## Caveats and Disagreements

### Attribution is often misleading
The repository is named `andrej-karpathy-skills`, but Karpathy did not write it, publish it, or publicly endorse it. Multiple sources explicitly warn against this misattribution. The wording of the principles is Chang's; the diagnosis is Karpathy's [[mcp]][[delv]][[rusin]].

### CLAUDE.md instructions are probabilistic, not deterministic
A file like this nudges behavior but does not enforce it. The root causes of agent failures (RLHF reward signals, agent-loop design) are not fixable by a markdown file [[delv]][[rusin]].

### The principles are conventional software engineering advice
Commentators note that rules like "match existing style" and "every changed line should trace to the user's request" restate long-standing engineering principles. The novelty is the imperative, commandment-tablet form tailored for an LLM context file [[delv]].

### It does not solve product-level problems
The file targets code-level mistakes. It does not address whether the agent is building the right feature, only whether it is building the feature right [[delv]].

### Anthropic's guidance on length and specificity
Anthropic recommends keeping CLAUDE.md under 200 lines, grouping instructions under headers, and including only facts Claude would otherwise need re-explained. The derived Karpathy file is roughly 60 lines, which aligns with that guidance [[anthropic-docs]][[anthropic-best]].

## Open Questions

1. Has Karpathy ever commented directly on the `andrej-karpathy-skills` repo or the derived CLAUDE.md? No public endorsement has been found, but a direct search of his X replies or podcast appearances could strengthen this.
2. What were the exact "simple attempts" at CLAUDE.md instructions Karpathy mentioned in his thread? He did not share them [[thread]].
3. How much of the reported improvement from using the file (e.g., "mistakes cut to 11%") is reproducible versus anecdotal? Quantitative claims in secondary sources lack controlled benchmarks.

## Sources

[[thread]] Andrej Karpathy X thread, preserved via ThreadReaderApp mirror. URL: https://threadreaderapp.com/thread/2015883857489522876.html (original X.com link: https://x.com/karpathy/status/2015883857489522876)

[[repo]] forrestchang/andrej-karpathy-skills GitHub repository README. URL: https://github.com/forrestchang/andrej-karpathy-skills

[[raw]] forrestchang/andrej-karpathy-skills CLAUDE.md (raw). URL: https://raw.githubusercontent.com/forrestchang/andrej-karpathy-skills/main/CLAUDE.md

[[mcp]] MCP.Directory, "Karpathy's CLAUDE.md, Annotated (2026)." URL: https://mcp.directory/blog/karpathy-claude-md-annotated-2026

[[anthropic-docs]] Anthropic, "How Claude remembers your project" (CLAUDE.md docs). URL: https://code.claude.com/docs/en/claude-md

[[anthropic-best]] Anthropic, "Best practices for Claude Code." URL: https://code.claude.com/docs/en/best-practices

[[llm-council]] karpathy/llm-council CLAUDE.md. URL: https://github.com/karpathy/llm-council/blob/master/CLAUDE.md

[[llm-council-raw]] karpathy/llm-council CLAUDE.md (raw). URL: https://raw.githubusercontent.com/karpathy/llm-council/master/CLAUDE.md

[[delv]] Delv, "Karpathy's CLAUDE.md: the actual story." URL: https://delv.tools/blog/karpathy-claudemd-isnt-karpathys-may-2026

[[rusin]] Alex Rusin Blog, "The Viral Karpathy Claude.md File: Here's the Honest Truth About What It Actually Is." URL: https://blog.alexrusin.com/the-viral-karpathy-claude-md-file-heres-the-honest-truth-about-what-it-actually-is/
