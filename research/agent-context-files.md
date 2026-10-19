# Effective agent context files (AGENTS.md / CLAUDE.md) and their failure modes

**Status:** research synthesis, 2026-10-19
**Scope:** repository-level instruction files for coding agents: `AGENTS.md`, `CLAUDE.md`, and closely related rule formats (Codex `AGENTS.override.md`, GitHub Copilot `.github/copilot-instructions.md` / `.instructions.md`, Cursor `.cursor/rules/*.mdc`).
**Thesis:** These files can change agent behavior and are poor at guaranteeing better outcomes. They are best treated as a small, maintained, security-reviewed configuration layer whose hard constraints are enforced by deterministic tooling, not by prose.

---

## Executive summary

1. **They can change behavior, not necessarily success.** Controlled studies find that agents follow instructions in context files — tool names, commands, testing behavior — but do not find a general improvement in task success. The largest controlled evaluation reviewed here reports null-to-marginal accuracy effects and a cost increase above 20% [S8]. A second, independent two-agent ablation (17 tasks, 288 evaluated runs) found no detectable correctness effect; the authors describe the result as an unpowered descriptive bound (≤10–15 pp), not equivalence, and note that a 10 pp effect would need roughly 120–200 tasks [S9].
2. **The defensible content is non-inferable operational knowledge.** Custom commands, build/test quirks, domain rules, dangerous operations, and conventions that differ from tool defaults are the material an agent cannot derive from the code. Repository overviews, architecture tours, style rules already enforced by linters, and generic "be careful" advice are the weakest content and a common source of bloat [S8][S13][S27].
3. **Human-written beats auto-generated, but only marginally.** In the ETH Zurich evaluation, LLM-generated files averaged −0.5% (SWE-bench) and −2% (CTXbench), both not significant, while developer-committed files on CTXbench (138 instances, 12 repositories) averaged +2.4% (p=.21, n.s.) and were significantly better than LLM-generated files (p=.038) [S8]. A later tuning study suggests production method matters: on one model and benchmark, probe-and-refine guidance reached 33.0% resolve versus 28.3% for a static knowledge base and 25.5% unguided [S42].
4. **Failure modes are common and detectable.** In 100 popular repositories, lint leakage affected 62% of files, context bloat 42%, skill leakage 35%, init fossilization 24%, and conflicting instructions produced 28 flagged instances (16 confirmed after manual review, 57% precision) [S13]. Across 247,694 instruction lifetimes, prompts grew +226% and gained +4.9 net instructions per commit; older instructions became *less* likely to be deleted [S14].
5. **Precedence is not portable.** The AGENTS.md site says the nearest file wins [S1]; Claude Code concatenates files and says conflicting instructions may be resolved arbitrarily [S2]; Codex concatenates root-to-CWD and treats later text as overriding [S4]; Copilot and VS Code document different or no general precedence [S5][S38]. A file that depends on nesting semantics can behave differently across tools.
6. **Prose is not enforcement.** Claude Code's own documentation says the file is "context, not enforced configuration" [S2]. Executable checks achieve substantially higher constraint compliance than prompt-only instructions (88.3% vs 67.0% in one evaluation) [S20]. Hard rules belong in hooks, permissions, linters, CI, and tests.
7. **Security failures can be the most damaging.** Context files are automatically ingested, instruction-bearing repository content. Attacks demonstrated in researcher and vendor proof-of-concepts include hidden-Unicode rules files [S23], credential exfiltration through a malicious `AGENTS.md` in Codex `exec` mode [S32], goal hijacking in VS Code Chat [S33a], and a poisoned take-home repository that led an agent to read cloud credentials and exfiltrate them [S34]. Review these files like executable code.

---

## 1. What the files are and how they load

| File / convention | Owner | Load semantics | Precedence / nesting | Documented size guidance |
|---|---|---|---|---|
| `AGENTS.md` | Agentic AI Foundation (Linux Foundation); emerged from a multi-vendor effort including OpenAI [S1] | Read by adopting agents at session/task start [S1][S4] | Format says "closest ... wins"; implementations vary [S1] | None in the format [S1] |
| `CLAUDE.md`, `CLAUDE.local.md` | Anthropic | Loaded at session start; subdirectory files load on demand; concatenated root→CWD [S2] | Additive, not overriding; managed → user → project → local load order [S2] | Target under 200 lines; file >4 MiB skipped [S2] |
| Codex `AGENTS.md` / `AGENTS.override.md` | OpenAI | Instruction chain built once per run [S4] | Global → repo root→CWD; one file per directory; later text overrides [S4] | 32 KiB combined default (`project_doc_max_bytes`); the config reference describes the limit as per-file [S4][S35] |
| `.github/copilot-instructions.md`, `.github/instructions/*.instructions.md` | GitHub | Repo-wide or path-scoped (`applyTo`) [S5] | Personal → repo → organization on GitHub.com; many surfaces define no general order [S5][S6] | Generator prompt: no more than 2 pages [S5]; review guide: about 1,000 lines/file [S36] |
| Cursor `.cursor/rules/*.mdc` | Cursor | Included at start of model context when applied [S7] | Team → Project → User; earlier sources win on conflict [S7] | Under 500 lines [S7] |
| `AGENTS.md` in Copilot / Cursor / VS Code | GitHub / Cursor / Microsoft | Agent instructions; nested support varies [S5][S7][S38] | Copilot: nearest file wins; Cursor: more specific wins; VS Code nested support off by default [S5][S7][S38] | As above |

Three facts matter for practice.

**First, "AGENTS.md" is a convention, not a specification with enforceable semantics.** The site defines a name and a Markdown format, not a loading algorithm [S1]. Each vendor implements discovery and precedence independently. Claude Code, for example, reads `AGENTS.md` only when no `CLAUDE.md` or `CLAUDE.local.md` exists on the path by default [S2]. Cursor ignores a plain `.md` file inside `.cursor/rules`; project rules must be `.mdc` files with frontmatter [S7].

**Second, the documented precedence rules genuinely differ.** The AGENTS.md FAQ says "the closest AGENTS.md to the edited file wins" [S1]. Claude Code says discovered files are "concatenated into context rather than overriding each other" and that contradictory instructions may be followed arbitrarily [S2]. Codex says files are concatenated from root to CWD and "files closer to your current directory override earlier guidance because they appear later in the combined prompt" [S4]. Cursor says nested files are combined "with more specific instructions taking precedence" [S7]. Copilot's GitHub.com surface documents a priority order, while its CLI and VS Code surfaces say no general order is guaranteed [S5][S6][S38].

**Third, the load behavior is versioned and sometimes surprising.** Claude Code's direct `AGENTS.md` support requires v2.1.277 or later [S2]. VS Code's nested `AGENTS.md` support sits behind a setting that is disabled by default [S38]. Codex truncates the combined instruction chain at 32 KiB by default [S4]. An instruction file that is silently truncated or not loaded is worse than no file, because the team believes a rule is active.

**Documentation inconsistency worth knowing:** Anthropic's blog says a `CLAUDE.md` file "becomes part of Claude's system prompt" [S28]. The current official docs say the content "is delivered as a user message after the system prompt, not as part of the system prompt itself" [S2]. For reasoning about authority and conflict, the docs are the better source.

---

## 2. What the evidence actually shows

### 2.1 Evidence table

| Study | Design | Headline result | What it supports | Main limitation |
|---|---|---|---|---|
| Gloaguen et al., *Evaluating AGENTS.md* (ETH Zurich), arXiv:2602.11988 [S8] | Two designs: SWE-bench Lite 2-arm (no file vs LLM-generated, 300 tasks); CTXbench 3-arm (no file / LLM-generated / developer-committed, 138 tasks); 4 agent-model pairs | LLM-generated: −0.5% (SWE-bench, p=.87) and −2% (CTXbench, p=.37); cost +20%/+23%. Developer files (CTXbench only): +2.4% (p=.21), better than generated (p=.038) | No general accuracy gain; instructions are followed; cost rises; human files are less bad than generated ones | Python only; task resolution only; one sample per task |
| Khatri, *Two-Agent Ablation*, arXiv:2607.27250 [S9] | 2 agents, 17 tasks, 3 repos, 288 runs, gold tests, equivalence testing | No measurable correctness movement (unpowered descriptive bound ≤10–15 pp, not equivalence); failures are implementation-skill gaps, not missing repo facts | Correctness null is robust across two frontier harnesses | Small task set; short horizon |
| Lulla et al., *Impact of AGENTS.md on Efficiency*, arXiv:2601.20404 [S10] | Paired A/B, 10 repos, 124 PRs, Codex | Median runtime −28.64%, output tokens −16.58%, "comparable task completion behavior" | Context files can improve operational efficiency | Correctness not rigorously evaluated; completion behavior is a weak proxy |
| Shepard & Albrecht, *Probe-and-Refine Tuning*, arXiv:2606.20512 [S42] | SWE-bench Verified, 4 trials, Qwen3.5-35B-A3B, 200 steps | 33.0% resolve vs 28.3% static guidance vs 25.5% unguided (p<.001); gain from coverage, not patch precision | How guidance is produced can matter; tuned guidance can help | Single model family; author-developed tuning loop; guidance-length confound; cross-model transfer degraded on Nemotron |
| Chatlatanagulchai et al., *Agent READMEs*, arXiv:2511.12884 [S11] | 2,303 files from 1,925 repos; content and commit analysis | Median 485 words (Claude), 535 (Copilot), 335.5 (Codex); 59–67% edited in multiple commits; security in 14.8%, performance in 14.5% | Files are living config, skewed to functional content, weak on non-functional guardrails | Descriptive, not causal |
| Mohsenimofidi et al., *Context Engineering*, arXiv:2510.21413 [S12] | 466 repos; structure and commit-level evolution | No established structure; five styles (descriptive, prescriptive, prohibitive, explanatory, conditional); 50% of AGENTS.md never modified | Convention is immature and heterogeneous | Preliminary; 466 repos |
| dos Santos et al., *Configuration Smells*, arXiv:2606.15828 [S13] | 100 popular repos; six-smell heuristics with manual review | Lint leakage 62%, context bloat 42%, skill leakage 35%, init fossilization 24%, blind references 16, conflicting instructions 28 flagged (57% precision) | Failure modes are common and detectable | Top-starred sample; LLM detector noise |
| Chakrabarti, *Catastrophic Remembering*, arXiv:2608.11095 [S14] | 247,694 instruction lifetimes in 1,867 repos | Prompts grow +226%; +4.9 net instructions/commit; older instructions less likely to be deleted (log-hazard −0.032/commit) | Bloat is a ratchet, not a one-time mistake | Preprint; observational |
| Jaroslawicz et al., *IFScale*, arXiv:2507.11538 [S15] | 20 models, 500 simultaneous instructions | Best model 68% accuracy at 500 instructions; bias toward earlier instructions | Instruction density has real costs; ordering matters | Synthetic task, not repo work |
| Liu et al., *Lost in the Middle*, arXiv:2307.03172 [S16] | Positional ablation | Best recall at start/end; significant degradation in the middle | Placement and dilution matter | QA/retrieval tasks, not agents |
| Chroma, *Context Rot* [S17] | 18 models; length and distractor sweeps | Performance degrades with input length even on simple tasks; distractors amplify it | Every always-loaded token has an attention cost | Vendor report, directional |
| Sharma, *ContextCov*, arXiv:2603.00822 [S20] | 300 SWE-bench Lite tasks; prompt vs reflection vs compiled checks | 88.3% constraint compliance vs 67.0% prompt-only and 50.3% reflection; 3.4× lower feedback cost | Prose is weaker than executable guardrails | Author-developed framework; specific constraints |
| He et al., *RepoNorm*, arXiv:2610.07757 [S21] | 121 tasks, 3 coding models | Norm compliance +7.42–10.77%; functional success +5.79–9.09 pp, not significant | Guidance can improve process compliance without proven outcome gains | Preprint; benchmark-specific |

### 2.2 How to read the contradiction

The apparent contradiction between "context files help" and "context files do nothing" is mostly a difference in outcome variable and sample.

- **Accuracy studies** (ETH, Khatri) find null-to-marginal effects on task success [S8][S9]. They measure whether the final patch passes tests.
- **Efficiency studies** (Lulla) find lower runtime and fewer output tokens [S10]. They do not measure whether the patch is correct.
- **Process studies** (ContextCov, RepoNorm) find higher compliance with documented constraints, often without a significant functional gain [S20][S21].

All three can be true simultaneously. A file can make an agent test more, explore more, follow conventions more, finish faster or cheaper per token, and still produce the same pass rate. The ETH trace analysis supports exactly this: instructions are followed, tool use and reasoning increase, cost rises, and accuracy does not [S8].

A second reconciliation is **redundancy**. ETH reports an appendix ablation in which removing all other repository documentation (`.md`, `docs/`, examples) lets LLM-generated context files improve performance by 2.7% and beat developer files [S8]. The inference is that context files are most useful when they are the only source of operational knowledge, and least useful when they duplicate a well-documented repository. That inference is consistent with the observation that repository overviews do not meaningfully reduce the number of steps before the agent first touches a relevant file [S8].

A third is **production method**, though the evidence is narrow. ETH's generated files came from each agent's `/init`-style command and did not help [S8]. Probe-and-refine tuning, which diagnoses failures with synthetic bug-fix probes and patches the guidance, improved resolve rate by 4.7 pp over a static knowledge base on one model and benchmark; the authors report a guidance-length confound and that the loop degraded on a smaller model [S42]. "Generated vs human" is too coarse; "generated once and never tested" vs "iteratively tuned against observed failures" is a plausible distinction, not yet a general reconciliation.

---

## 3. What makes a context file effective

Effectiveness here means: the file changes behavior in the intended direction, at acceptable cost, without creating new failure modes. The evidence supports the following design rules.

### 3.1 Include only what the agent cannot infer

The strongest consistent finding across vendor guidance and empirical work is that context files should contain **non-inferable operational knowledge**, not a description of the repository.

Good candidates:

- Exact build, test, lint, and run commands, including flags and the single-test invocation [S1][S3][S13].
- Non-obvious environment setup, service dependencies, and known failure modes [S3][S26].
- Conventions that differ from language or framework defaults [S3].
- Domain rules and invariants that are not visible in code [S4], such as "all money amounts are integer cents" or "never rotate keys without notifying security".
- Boundaries: files, directories, commands, and data the agent must not touch [S4][S29].
- Pointers to canonical examples, with a reason to read them [S7][S13].

Poor candidates:

- Directory tours and architecture overviews that the agent can derive by listing files [S8][S2].
- Style rules already enforced by formatters and linters [S7][S13].
- Generic exhortations ("write clean code", "be careful", "follow best practices") [S2][S3].
- Long tutorials, pasted documentation, or copies of the README [S2][S7].
- One-off incident rules and hardcoded paths that will be stale next quarter [S26].

Anthropic's own `/doctor` checkup now proposes cutting "directory layouts, dependency lists, and architecture overviews" and keeping "pitfalls, rationale, and conventions that differ from tool defaults" [S2]. That is a notable convergence with the ETH finding that overviews do not help.

### 3.2 Write at the right altitude: specific, checkable, and short

Anthropic's context-engineering guidance describes the target as the "right altitude" between brittle hardcoded logic and vague guidance that assumes shared context [S27]. For a context file, that means instructions should be specific enough that a reviewer can check compliance.

- "Use 2-space indentation" beats "format code properly" [S2].
- "Run `npm test` before committing" beats "test your changes" [S2].
- "API handlers live in `src/api/handlers/`" beats "keep files organized" [S2].
- "Never modify `vendor/` or `dist/`" beats "be careful with generated code" [S4][S29].

The IFScale result explains why this matters mechanically: at 500 simultaneous instructions, the best of 20 models followed 68%, and performance degraded non-linearly with a bias toward earlier instructions [S15]. Context Rot found performance degradation with input length even when task complexity was held constant [S17]. These are not direct studies of `AGENTS.md`, but they establish the attention budget that a bloated file spends.

### 3.3 Scope aggressively: always-on, path-scoped, or on-demand

A context file should not be the only place for instructions. The tools now provide three tiers:

- **Always-on**: a small root file with commands, global conventions, and hard boundaries.
- **Path-scoped**: rules that load only when matching files are touched, such as Claude `.claude/rules/*.md` with `paths:` frontmatter [S2], Copilot `.instructions.md` with `applyTo` [S5], or Cursor `.mdc` rules with `globs` [S7].
- **On-demand**: skills, commands, or linked documents loaded when the task requires them [S2][S7].

"Skill leakage" — task-specific instructions loaded into every session — affected 35% of files in the configuration-smells study and co-occurred with bloat [S13]. The fix is structural, not editorial: move the content to a scoped rule or a skill.

### 3.4 Treat size as a budget, not a target

Vendor guidance mostly clusters at the short end — Claude Code targets under 200 lines [S2], Cursor under 500 [S7], and Copilot's generator prompt says no more than two pages [S5] — though Copilot's code-review guide allows about 1,000 lines for a different artifact [S36]. The AGENTS.md format itself specifies no limit [S1].

The empirical picture is more nuanced. ETH found no clear relationship between file length and either resolution rate or per-instance cost (Appendix B, Fig. 13, binned descriptively; no significance test reported) [S8]. So "under 200 lines" is a useful default, not a law. The defensible claim is: **every line should earn its place**, because each line consumes context and attention, and because low-value lines can hide high-value ones. The growth evidence makes this urgent: prompts grew +226% over their lifetimes and gained +4.9 net instructions per commit, while deletions were rare [S14].

A practical rule: when adding an instruction, ask what instruction it replaces, or what evidence shows it is needed. If the answer is "nothing" and "a real failure", add it. Otherwise, fix the underlying cause (linter, test, script, documentation) instead.

### 3.5 Structure for scanning

The mining studies find a consistent shallow structure in the wild: one H1, six or seven H2 sections, and a handful of H3s [S11]. That structure is reasonable. Put the highest-value material first:

1. Commands (install, build, test, lint, single-test).
2. Non-obvious setup and environment.
3. Conventions that differ from defaults.
4. Boundaries and dangerous operations.
5. Verification steps.
6. Links to deeper docs, each with a reason to follow them.

GitHub's practitioner analysis recommends commands early, code examples over prose, explicit boundaries, and specific stack versions [S29]. That is practitioner advice, not a controlled result, and its "2,500 repositories" analysis is not documented in enough detail to verify. Its worked examples are `.github/agents/*.md` persona files with YAML frontmatter, not root-level context files, so it concerns a related artifact rather than `AGENTS.md` itself.

### 3.6 Keep one source of truth and make precedence explicit

Duplicated instructions drift. The configuration-smells study flagged 28 conflicting-instruction instances across 100 files, with 16 confirmed after manual review (57% precision) [S13]. Claude Code's docs warn that contradictory instructions may be followed arbitrarily [S2]. Control Illusion found that system/user role separation does not establish a reliable instruction hierarchy and that models have inherent biases toward certain constraint types [S18]. ConInstruct found strong conflict *detection* in proprietary models but that models rarely notify the user or ask for clarification [S19].

Practical consequences:

- Do not copy the same rule into root and nested files. Put it in one place and reference it.
- If a nested file must override a root file, state the override explicitly and test it in the target tool.
- If `AGENTS.md` and `CLAUDE.md` both exist, decide which is canonical. Claude Code defaults to `CLAUDE.md` and ignores `AGENTS.md` unless configured otherwise [S2]. A common pattern is a one-line `CLAUDE.md` that imports `AGENTS.md` [S2].
- Run the tool's audit. Claude Code has `/doctor prompt-audit` for outdated references and contradictions [S2]. Codex rebuilds the chain every run and can log loaded files [S4].

### 3.7 Maintain it like configuration code

The "write once" model is wrong for many repositories, but so is "it maintains itself." The evidence is bimodal:

- In a sample of heavy agent users, 59–67% of files were modified in multiple commits, with median update intervals of about one day (Claude Code, Codex) to three days (Copilot); median additions were about 57 words per commit for Claude Code (the highest of the three) and deletions under 15 [S11].
- In a sample of mature popular repositories, 50% of `AGENTS.md` files were never modified, 23% once, and 21% two to seven times [S12].

Both can be true: some teams actively co-evolve the file with the code; others generate it once and forget it. The failure mode is the second, and the ratchet evidence shows why it is hard to fix later: old instructions become less likely to be deleted, and most instruction "deaths" happen only through wholesale rewrites or file migrations [S14].

Maintenance practices supported by the sources:

- Review context-file changes in pull requests, like code [S11][S26].
- When a build system, path, or workflow changes, update the file in the same PR [S11].
- Periodically audit for dead commands, dead paths, and contradictions [S2].
- Pair additions with deletions; do not let the file become an append-only log [S14][S26].
- Treat `/init` output as a draft, not a finished product [S2][S28].

### 3.8 Enforce hard rules outside the file

This is the most important design principle in the report.

Claude Code's documentation states that `CLAUDE.md` is "context, not enforced configuration" and recommends a `PreToolUse` hook to block an action regardless of what the model decides [S2]. Cursor says AI guidance "should not be your only security control" [S7]. The configuration-smells authors explicitly discarded "exposing secret keys" as a context-file-specific smell, because it is a broader configuration problem [S13].

The empirical support for executable enforcement is strong. ContextCov compiled documented constraints into AST queries, shell shims, and architectural validators; it reached 88.3% constraint compliance versus 67.0% for prompt-only and 50.3% for reflection, at 3.4× lower feedback cost [S20]. That 88.3% is a measured ceiling in one author-developed evaluation, not a guarantee. RepoNorm improved norm compliance by 7.42–10.77% with generated norm packages, but functional success gains were not significant [S21]. The lesson: a prose rule can shape behavior; a deterministic check can make a violation detectable and blockable, which prose cannot.

Use the file to explain *why* a rule exists and *when* it applies. Use hooks, permissions, linters, CI, tests, and scripts to make the rule true.

### 3.9 Measure the file, not the intention

The evaluation gap is large. ETH's paper describes itself as the first rigorous investigation of whether context files are effective [S8]. Most teams add rules based on a single bad experience and never test whether the rule changed anything.

A defensible local evaluation:

1. **Pick real tasks** with objective pass criteria (existing tests or a gold patch).
2. **Run paired conditions**: file present vs absent, same model, same harness, same repository state.
3. **Repeat** enough times to detect the effect you care about. Khatri's power analysis found that 17 tasks × 3 repeats detects a 30 pp effect only 57% of the time; detecting 10 pp at 80% power needs roughly 120–200 tasks [S9].
4. **Measure more than accuracy**: success, steps, input/output tokens, wall-clock, cost, constraint violations, and human review time.
5. **Use equivalence testing**, not just a non-significant p-value, when the goal is to show no harm [S9].
6. **Check the traces**: did the agent actually read the file, follow the command, and run the test? ETH's trace analysis is the model here [S8].

A file that cannot be shown to change behavior is documentation of an intention, not a control.

### 3.10 Treat it as a security boundary

Context files are automatically ingested instruction-bearing content. That makes them an indirect prompt-injection surface and a supply-chain artifact. The security section below covers the demonstrated attacks and mitigations.

---

## 4. Failure modes

| Failure mode | Mechanism | Evidence | Warning sign | Mitigation |
|---|---|---|---|---|
| **Context bloat** | Every token loads every session; low-value rules dilute attention and raise cost | 42% of 100 files ≥200 lines [S13]; +226% lifetime growth [S14]; cost +20%+ in ETH [S8] | File keeps growing; nobody can name the last deletion; agents ignore late rules | Cut inferable content; move scoped material to rules/skills; pair additions with deletions |
| **Lint leakage** | Restating rules already enforced by formatters/linters | 62% of 100 files [S13] | Lines like "use camelCase", "max line length 100" | Delete; let the linter enforce it; point to the config |
| **Skill leakage** | Rarely needed, task-specific instructions loaded every session | 35% of 100 files [S13] | Deployment runbooks, migration procedures, framework tutorials in the root file | Move to skills, commands, or path-scoped rules |
| **Staleness / init fossilization** | Generated once, never updated; dead commands and paths persist | 24% of 100 files had a single commit [S13]; 50% of AGENTS.md never modified in another sample [S12]; deletions rare [S14] | References to removed scripts, old package managers, retired services | Review in PRs; audit dead references; regenerate from current repo, then prune |
| **Conflicting instructions** | Contradictory rules at different scopes or in the same file; models may pick arbitrarily | 28 flagged instances, 16 confirmed (57% precision) [S13]; docs warn of arbitrary choice [S2]; Control Illusion [S18] | "Always X" in root, "never X" in a nested file; user/project rules disagree | One source of truth; explicit overrides; run prompt audit |
| **Precedence / nesting mismatch** | Different tools implement discovery and precedence differently | Agents.md vs Claude vs Codex vs Copilot vs VS Code [S1][S2][S4][S5][S38]; open Codex issue [S30]; historical Copilot issue [S31] | File works in one tool, silently ignored in another; nested rules never load | Verify with the tool's context inspector; avoid relying on nesting for critical rules |
| **Redundant overview** | Architecture tours and directory lists are inferable and do not speed navigation | ETH: overviews did not meaningfully reduce steps to a relevant file; accuracy evidence is mixed [S8]; Claude `/doctor` cuts them [S2] | Long "Project Structure" section; agents still grep for files | Replace with pitfalls, invariants, and "where to look for X" heuristics |
| **Vague / aspirational rules** | No observable check; model cannot self-verify | Vendor docs [S2][S3]; no controlled isolation found | "Be careful", "write clean code", "follow best practices" | Rewrite as a checkable command, pattern, or boundary |
| **Prose as policy** | Model may ignore or forget; no guarantee of compliance | Docs: "not enforced configuration" [S2]; ContextCov 88.3% vs 67.0% [S20] | Critical security/architecture rule exists only as a sentence | Hooks, permissions, CI, linters, tests |
| **Secret leakage** | Credentials pasted into a committed file | Anthropic warns against secrets in CLAUDE.md [S28] | API keys, tokens, connection strings in the file | Never commit secrets; use env vars/secret managers; scan history |
| **Prompt injection / poisoned rules** | Repository content is read as instructions; hidden or malicious text redirects the agent | Pillar hidden Unicode demo [S23]; Codex exec exfiltration [S32]; VS Code goal hijack [S33a]; malicious dependency [S33b]; poisoned take-home [S34] | Untrusted repo; unexpected network calls, file reads, or script tags; invisible characters | Treat as executable code; review; scan hidden Unicode; sandbox; least privilege; HITL |
| **Evaluation gap** | No test of whether the file changes behavior | ETH first rigorous study [S8]; most rules added from anecdote | "It feels better"; no baseline | Paired A/B with objective tasks and trace checks |
| **Format / portability mismatch** | Tool-specific syntax silently ignored | Cursor `.mdc` only [S7]; Claude imports [S2]; Codex fallback names [S4] | Copy-pasted file across tools; rules that never fire | Use plain Markdown for shared content; test per tool |

### 4.1 The bloat paradox

The common claim "longer files reduce adherence" is vendor guidance [S2] and is mechanistically plausible given IFScale and Context Rot [S15][S17]. But the ETH analysis found no clear relationship between length and resolution rate or cost (descriptive binning, no significance test) [S8]. How should a practitioner reconcile this?

- **Length is a proxy for signal density, not a causal variable.** A 300-line file of precise commands may outperform a 100-line file of vague advice. The ETH result does not license bloat; it says line count alone is not the right metric.
- **Bloat has a cost even when accuracy is unchanged.** The ETH files increased inference cost by over 20% and steps by 2.45–3.92 [S8]. If the file is not improving success, that cost is pure overhead.
- **Bloat is a ratchet.** The +226% growth and declining deletion hazard mean that without an explicit pruning process, the file converges on an append-only log [S14].

The practical target is not "under 200 lines" as a moral rule. It is "every line is high-signal, non-inferable, and either checkable or tied to a real failure."

### 4.2 Conflicts and precedence are a system-level defect

A conflict is not just two contradictory sentences. It is a failure of the whole instruction stack:

- **Within a file**: "always use pnpm" and "use npm for this package".
- **Across scopes**: root says "run the full test suite"; a package says "run only the package tests".
- **Across tools**: `CLAUDE.md` says one thing; `AGENTS.md` says another; the tool loads only one.
- **Across authority levels**: repository file vs user prompt vs system/developer message.

Control Illusion found that the system/user separation "fails to establish a reliable instruction hierarchy" and that models exhibit strong inherent biases toward certain constraint types [S18]. ConInstruct found that models detect conflicts reasonably well but "rarely explicitly notify users about the conflicts or request clarification" [S19]. So the agent may silently choose, and the developer may never know.

The engineering response is boring and effective: one canonical file, explicit scoping, no duplicated rules, an audit step, and a test that loads the file in each tool you support.

### 4.3 The overview question

The AGENTS.md site lists project overview as a popular section [S1], and Anthropic's blog recommends a project summary and directory tree [S28]. The ETH evaluation found that repository overviews do not meaningfully reduce the number of steps before the agent touches a relevant file, and that agents sometimes re-read the context file multiple times instead of using it as a map [S8]. Its category ablation is more equivocal on accuracy: CTXbench scored 62.32% without the overview versus 68.12% with it (p=.15), a non-significant difference in the pro-overview direction [S8]. Claude Code's `/doctor` now proposes cutting directory layouts and architecture overviews [S2]. The defensible conclusion is narrower than "overviews are useless": overviews do not consistently speed navigation, and their accuracy effect is unresolved.

This is a useful case study in evidence overriding intuition. A human onboarding document needs an architecture tour because a human cannot grep the whole repository instantly. An agent can. What the agent cannot infer is *why* the architecture is unusual, which invariants must hold, which directories are dangerous, and which command actually works. Those are the things worth writing down.

### 4.4 Prose is not a control

The enforcement gap is a widespread source of false confidence. A team writes "never commit secrets" or "always run the migration in dry-run mode" and believes the rule is active. In reality:

- The file is context, not configuration [S2].
- The model may follow it most of the time, but not deterministically.
- Under context saturation or conflicting local context, compliance drops [S20].
- A malicious file can instruct the agent to conceal its actions [S23][S33b].

Executable guardrails change the economics. ContextCov's compiled checks reached 88.3% compliance, versus 67.0% for prompt-only and 50.3% for reflection, at lower feedback cost [S20]. RepoNorm's generated norms improved compliance but not functional success significantly [S21], which is itself a warning: compliance with a norm is not the same as solving the task.

The design rule: **if violating the rule is unacceptable, make it impossible or detectable outside the model.** Use hooks, permission denies, sandboxing, linters, CI checks, tests, and secret scanning. Keep the prose to explain rationale and exceptions.

### 4.5 Security: the file is executable-adjacent

Security failures can have the highest impact when an attack succeeds. Context files are read automatically at session start and treated as instructions. That is the definition of an indirect prompt-injection surface.

Attack classes demonstrated in researcher and vendor proof-of-concepts (none independently reproduced here):

1. **Hidden Unicode in rules files.** Pillar Security embedded invisible instructions in Cursor/Copilot rules files; the model generated an attacker-controlled script tag and did not mention it in chat. The payload survived human review because the characters were invisible [S23]. GitHub later added a hidden-Unicode warning, but the class remains [S23].
2. **`AGENTS.md` command execution in Codex `exec` mode.** A malicious `AGENTS.md` instructed the agent to copy `~/.aws/credentials` to a temp path; the command ran before the user's request with no approval prompt. The specific payload was patched, but the trust-boundary issue is mode-dependent [S32].
3. **Goal hijacking in VS Code Chat.** Prompt Security reported that a benign-looking `AGENTS.md` drove the agent to email internal data; in their 2026 test, VS Code injected the file into every chat request with the feature enabled by default [S33a]. The payload was redacted and the vendor's docs describe the feature as configurable, so treat this as a demo, not a field observation.
4. **Malicious dependency rewrites `AGENTS.md`.** NVIDIA's red team showed a compromised Go library writing an untracked `AGENTS.md` that instructed Codex to insert a sleep and hide it from PR summaries [S33b].
5. **Poisoned take-home repository.** Hidden instructions in `CLAUDE.md`, `.cursor/rules`, README comments, and MCP config led an agent to read AWS credentials, Kubernetes config, and Terraform state, then exfiltrate via an MCP tool call in under two minutes [S34]. This is a vendor incident report about a synthetic take-home repository, not an independently reproduced field incident.
6. **The broader IDE attack surface.** IDEsaster catalogued 30+ vulnerabilities, 24 CVEs, and 10+ products; 100% of tested applications were vulnerable to at least one attack chain combining prompt injection with IDE features [S24]. Not all involve context files, but the trust boundary is the same.

Mitigations that follow directly:

- **Treat context files as executable code.** Review them in PRs; require approval for changes from external contributors; do not merge a file you have not read [S25].
- **Never put secrets in the file.** Use environment variables and secret managers; scan history [S28].
- **Scan for hidden Unicode** and reject zero-width/bidi/tag characters in instruction files [S23][S25].
- **Do not auto-approve tool calls in untrusted repositories.** Use sandboxing, least-privilege tools, egress controls, and human-in-the-loop for sensitive actions [S24][S33a][S33b].
- **Separate trusted from untrusted content.** OpenAI's agent-safety guidance warns that injecting untrusted input into developer messages gives attackers the highest degree of control; pass untrusted input as user messages [S39].
- **Assume injection is possible.** Design for detection and blast-radius reduction, not perfect prevention [S24].

Across these cases, the common failure is not a bug in one product; it is the decision to treat repository-resident text as trusted behavioral instructions. OpenAI declined to treat the NVIDIA dependency case as a new vulnerability class, noting that a compromised dependency already implies code execution [S33b]; that disagreement is itself part of the risk picture.

### 4.6 The evaluation gap

Most teams cannot answer "did this file help?" They add a rule after a bad experience, observe an improvement (often on a different task), and keep it. The evidence base is thin because rigorous evaluation is expensive: ETH needed 300 SWE-bench tasks plus 138 new CTXbench instances and four agent-model pairs [S8]; Khatri needed 288 runs to describe a null [S9].

The gap has two consequences:

- **False positives**: a rule that feels helpful may be neutral or harmful. LLM-generated files are the clearest case: they are plausible, cheap to produce, and did not help in the ETH evaluation [S8].
- **False confidence**: a rule that is followed in a demo may fail under context pressure or conflict [S20].

The fix is cultural as much as technical: treat context-file changes as experiments with a baseline, a metric, and a rollback condition.

### 4.7 Portability and format mismatch

A file written for one tool may be silently ignored by another:

- Claude Code reads `AGENTS.md` only when no `CLAUDE.md` or `CLAUDE.local.md` exists on the path by default [S2].
- Cursor ignores plain `.md` files in `.cursor/rules` [S7].
- Codex has its own `AGENTS.override.md` and fallback filename list [S4].
- VS Code nested `AGENTS.md` support is off by default [S38].
- Copilot has repo-wide and path-specific formats with `applyTo`, plus `AGENTS.md` support [S5].

If a team supports multiple tools, the portable core should be plain Markdown in `AGENTS.md`, with tool-specific files importing or pointing to it. Then verify loading in each tool: Claude `/context`, Codex instruction listing or logs, Cursor rule panel, Copilot references. Do not assume that "the file exists" means "the file loaded."

---

## 5. A practical template

The following is a synthesis of the evidence, not a validated template. Treat it as a starting point to prune, not a checklist to fill.

```markdown
# AGENTS.md

## Commands
- Install: `pnpm install`
- Dev: `pnpm dev`
- Test (all): `pnpm test`
- Test (single): `pnpm vitest run -t "<name>"`
- Lint/typecheck: `pnpm lint && pnpm typecheck`

## Environment
- Requires Node 22 and a local Redis on port 6379.
- If `pnpm install` fails on a clean checkout, run `pnpm store prune` first.

## Conventions that differ from defaults
- API handlers live in `src/api/handlers/`; business logic lives in `src/domain/`.
- Money is stored as integer cents; never use floats.
- All external calls go through `src/lib/http.ts` (retries, tracing).

## Boundaries
- Never edit `vendor/`, `dist/`, or generated migrations.
- Ask before changing database schemas or CI configuration.
- Never commit secrets or real customer data.

## Verification
- Run the single relevant test first, then the full suite before finishing.
- For schema changes, run `pnpm db:migrate --dry-run` and include the output.

## Where to look
- Architecture rationale: `docs/adr/` (read the ADR for the area you touch).
- Canonical examples: `src/api/handlers/users.ts`, `src/domain/billing.ts`.
```

What this template deliberately omits: directory tours, framework tutorials, style rules enforced by Prettier/ESLint, generic quality advice, and duplicated README content.

---

## 6. Review checklist

Use this before merging a context file.

**Content**
- [ ] Every instruction is non-inferable from the code, README, or tool defaults.
- [ ] Commands are exact, current, and tested on a clean checkout.
- [ ] Rules are specific enough to verify; no "be careful" or "write clean code."
- [ ] No duplication of linter/formatter rules.
- [ ] No task-specific procedures that belong in a skill or scoped rule.
- [ ] No architecture tour that the agent can derive.

**Structure**
- [ ] Commands appear early.
- [ ] Sections are short and scannable.
- [ ] Links explain why and when to follow them.
- [ ] The file is under the tool's recommended budget, or every excess line is justified.

**Consistency**
- [ ] One canonical source; no contradictory root/nested/user rules.
- [ ] Overrides are explicit and tested in each supported tool.
- [ ] `AGENTS.md` vs `CLAUDE.md` precedence is decided and documented.

**Maintenance**
- [ ] The change is reviewed like code.
- [ ] Dead commands and paths are removed.
- [ ] Additions are paired with deletions where possible.
- [ ] A periodic audit is scheduled.

**Enforcement**
- [ ] Hard security/architecture rules are enforced by hooks, permissions, CI, or tests.
- [ ] The file explains rationale and exceptions, not just the rule.

**Security**
- [ ] No secrets, tokens, or credentials.
- [ ] Untrusted repositories are reviewed before the agent runs.
- [ ] Hidden Unicode is scanned.
- [ ] Auto-approval is disabled in untrusted contexts.
- [ ] Tool permissions follow least privilege.

**Measurement**
- [ ] There is a baseline (file absent or previous version).
- [ ] Success, steps, tokens, cost, and constraint violations are measured.
- [ ] The sample is large enough for the effect you care about.
- [ ] There is a rollback condition.

---

## 7. Open questions and disagreements

1. **Does staleness cause measurable performance loss?** No study found that correlates context-file staleness with agent outcomes. The closest evidence is init fossilization (24% single-commit files) and dead-reference smells [S13], plus documentation-drift research outside agents. This is the largest gap in the evidence base.
2. **Does length matter?** Vendor guidance says yes [S2][S7]; the ETH analysis found no clear relationship [S8]. A dose-response study on real tasks is missing.
3. **Do "good" files work?** Khatri's files were rated Good/Excellent on a rubric and still produced a null [S9]. No rubric has demonstrated predictive validity.
4. **Which content categories help?** ETH's category ablations found no significant accuracy change [S8]; RepoNorm found compliance gains without significant functional gains [S21]. The mapping from content type to outcome is unresolved.
5. **Non-Python and niche stacks.** ETH is Python-only and notes that heavily pretrained languages may mask context-file effects [S8]. Effects may be larger where the model knows less.
6. **Long-horizon and multi-session work.** Most studies use single tasks. Interaction with auto-memory, compaction, and subagents is largely unstudied [S2].
7. **Optimal granularity.** Path-scoped rules, skills, and nested files are plausible, but there is little comparative evidence on which decomposition works best.
8. **Security outcomes.** ETH explicitly defers security and efficiency outcomes [S8]. The injection literature measures attack success, not whether context files increase breach rates in practice.
9. **Standard precedence.** The AGENTS.md convention would be more useful with a defined precedence algorithm. Current implementations differ [S1][S2][S4][S5][S7].
10. **Evaluation methodology.** Small-n studies are underpowered [S9]; vendor benchmarks are not independent; secondary coverage should be checked against primary sources.

**A note on primary sources.** Quantitative claims in this brief are taken from the primary documents cited. The ETH paper's own numbers are −0.5%/−2% for LLM-generated files and +2.4% for developer files, with Claude Sonnet-4.5 [S8]. Where secondary write-ups give different figures, treat the primary source as authoritative.

---

## 8. Bottom line

An effective `AGENTS.md` or `CLAUDE.md` is not a comprehensive project handbook. It is a small, maintained, security-reviewed configuration layer that contains the operational facts an agent cannot infer, scopes everything else to where it is needed, and hands hard constraints to deterministic enforcement.

The evidence does not support the promise that a context file will make an agent solve more tasks. It does support a more modest and more useful promise: a good file can make an agent follow your conventions, use your tools, avoid your dangerous paths, and, in some setups, reduce runtime and token use [S10] — while a bad file can add cost, conflicts, stale instructions, and a new attack surface.

---

## Sources

- [S1] AGENTS.md — "A simple, open format for guiding coding agents." https://agents.md/
- [S2] Anthropic, "How Claude remembers your project" (Claude Code memory docs). https://code.claude.com/docs/en/memory
- [S3] Anthropic, "Best practices for Claude Code." https://code.claude.com/docs/en/best-practices
- [S4] OpenAI, "Custom instructions with AGENTS.md" (Codex). https://developers.openai.com/codex/guides/agents-md
- [S5] GitHub, "Adding repository custom instructions for GitHub Copilot." https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions
- [S6] GitHub, "Custom instructions for GitHub Copilot" (concepts). https://docs.github.com/en/copilot/concepts/prompting-and-context/custom-instructions
- [S7] Cursor, "Rules." https://cursor.com/docs/context/rules
- [S8] Gloaguen, Mündler-Sasahara, Müller, Raychev, Vechev, "Evaluating AGENTS.md: Are Repository-Level Context Files Helpful for Coding Agents?" arXiv:2602.11988 (v3, 2026). https://arxiv.org/abs/2602.11988
- [S9] Khatri, "Do Context Files Help Coding Agents? A Two-Agent Ablation Study on Real Repositories." arXiv:2607.27250 (2026). https://arxiv.org/abs/2607.27250
- [S10] Lulla et al., "On the Impact of AGENTS.md Files on the Efficiency of AI Coding Agents." arXiv:2601.20404 (2026). https://arxiv.org/abs/2601.20404
- [S11] Chatlatanagulchai et al., "Agent READMEs: An Empirical Study of Context Files for Agentic Coding." arXiv:2511.12884 (v2, 2026); ACM TOSEM 10.1145/3840295. https://arxiv.org/abs/2511.12884
- [S12] Mohsenimofidi, Galster, Treude, Baltes, "Context Engineering for AI Agents in Open-Source Software." arXiv:2510.21413 (v4, 2026); MSR 2026. https://arxiv.org/abs/2510.21413
- [S13] dos Santos, Costa, Montandon, Silva, Valente, "Configuration Smells in AGENTS.md Files: Common Mistakes in Configuring Coding Agents." arXiv:2606.15828 (v5, 2026); SCAM 2026. https://arxiv.org/abs/2606.15828
- [S14] Chakrabarti, "Why Does CLAUDE.md Keep Growing? Catastrophic Remembering in Agentic Coding." arXiv:2608.11095 (2026). https://arxiv.org/abs/2608.11095
- [S15] Jaroslawicz et al., "How Many Instructions Can LLMs Follow at Once?" arXiv:2507.11538 (2025). https://arxiv.org/abs/2507.11538
- [S16] Liu et al., "Lost in the Middle: How Language Models Use Long Contexts." TACL 2024; arXiv:2307.03172. https://arxiv.org/abs/2307.03172
- [S17] Hong, Troynikov, Huber, "Context Rot: How Increasing Input Tokens Impacts LLM Performance." Chroma, 2025. https://www.trychroma.com/research/context-rot
- [S18] Geng et al., "Control Illusion: The Failure of Instruction Hierarchies in Large Language Models." AAAI-26; arXiv:2502.15851. https://arxiv.org/abs/2502.15851
- [S19] He et al., "ConInstruct: Evaluating Large Language Models on Conflict Detection and Resolution in Instructions." AAAI-26. https://ojs.aaai.org/index.php/AAAI/article/view/40356
- [S20] Sharma, "ContextCov: Deriving and Enforcing Executable Constraints from Agent Instruction Files." arXiv:2603.00822 (2026). https://arxiv.org/abs/2603.00822
- [S21] He et al., "Acquiring and Verifying Repository Norms" (RepoNorm). arXiv:2610.07757 (2026). https://arxiv.org/abs/2610.07757
- [S23] Pillar Security, "New Vulnerability in GitHub Copilot and Cursor: How Hackers Can Weaponize Code Agents" (Rules File Backdoor), 2025. https://www.pillar.security/blog/new-vulnerability-in-github-copilot-and-cursor-how-hackers-can-weaponize-code-agents
- [S24] Marzouk (MaccariTA), "IDEsaster: A Novel Vulnerability Class in AI IDEs," 2025. https://maccarita.com/posts/idesaster/
- [S25] Cloud Security Alliance, "README Injection: Repository Files Hijacking AI Coding Assistants," 2026. https://labs.cloudsecurityalliance.org/research/csa-research-note-readme-instruction-injection-ai-coding-age/
- [S26] vLLM, "Editing Agent Instructions" (contributor guide). https://docs.vllm.ai/en/v0.20.2/contributing/editing-agent-instructions
- [S27] Anthropic, "Effective context engineering for AI agents." https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents
- [S28] Anthropic, "Using CLAUDE.md files: Customizing Claude Code for your codebase." https://claude.com/blog/using-claude-md-files
- [S29] Nigh, "How to write a great agents.md: Lessons from over 2,500 repositories." GitHub Blog, 2025. https://github.blog/ai-and-ml/github-copilot/how-to-write-a-great-agents-md-lessons-from-over-2500-repositories/
- [S30] OpenAI Codex issue #12115, "Dynamically loading nested AGENTS.md" (open). https://github.com/openai/codex/issues/12115
- [S31] Microsoft VS Code issue #271489, "GitHub Copilot ignores nested AGENTS.md files but respects root-level ones" (closed). https://github.com/microsoft/vscode/issues/271489
- [S32] Backslash Security, "AGENTS.md Injection in OpenAI Codex CLI." https://www.backslash.security/blog/openai-codex-injection-in-agents-md-exfiltrating-credentials
- [S33a] Prompt Security, "When Your Repo Starts Talking: AGENTS.MD and Agent Goal Hijack in VS Code Chat." https://prompt.security/blog/when-your-repo-starts-talking-agents-md-and-agent-goal-hijack-in-vs-code-chat
- [S33b] NVIDIA AI Red Team, "Mitigating Indirect AGENTS.md Injection Attacks in Agentic Environments." https://developer.nvidia.com/blog/mitigating-indirect-agents-md-injection-attacks-in-agentic-environments/
- [S34] Mitiga, "How a Poisoned Coding Test Turned an AI Agent Into an Attacker." https://www.mitiga.io/blog/poisoned-coding-test-ai-agent-attack
- [S35] OpenAI, "Advanced Configuration" (Codex `project_doc_max_bytes`). https://developers.openai.com/codex/config-file/config-advanced
- [S36] GitHub, "Using custom instructions to unlock the power of Copilot code review." https://docs.github.com/en/copilot/tutorials/customize-code-review
- [S38] Microsoft, "Use custom instructions in VS Code." https://code.visualstudio.com/docs/copilot/customization/custom-instructions
- [S39] OpenAI, "Safety in building agents." https://developers.openai.com/api/docs/guides/agent-builder-safety.md
- [S42] Shepard and Albrecht, "Probe-and-Refine Tuning of Repository Guidance for Coding Agents." arXiv:2606.20512 (2026). https://arxiv.org/abs/2606.20512

---

## Verification appendix

Verification date: 2026-10-19. Status: **PASS WITH NOTES**.

### URL check

The lead ran a `curl -L` GET against 38 of the 39 source URLs (20 s timeout, browser user-agent); S6 was checked separately and also returned HTTP 200. Every checked URL returned HTTP 200. Raw log: `outputs/.drafts/agent-context-files-urlcheck.txt` (42 checks, including four sources later removed). The verifier separately fetched cited pages in readable mode and checked title/author metadata via arXiv and OpenAlex. An earlier verification draft contained an abbreviated Pillar URL and an incorrect count; both are corrected here. This revision does **not** claim two independent indexes for every paper: some papers were cross-checked against OpenAlex, eight were checked against arXiv metadata only, and Semantic Scholar returned HTTP 429.

### Quantitative claims checked against primary sources

Where a row says "Confirmed" without a quoted excerpt, the check was a title/abstract match rather than an independent re-derivation.

| Claim | Source and location | Check |
|---|---|---|
| LLM-generated files: −0.5% SWE-bench (p=.87), −2% CTXbench (p=.37); cost +20%/+23%; steps +2.45/+3.92 | [S8] §4.2, Tables 2–3 | Confirmed: "reduced by 0.5% and 2%"; cost "20% and 23%"; steps 2.45 and 3.92 |
| Developer files: +2.4% (p=.21), better than generated (p=.038), CTXbench only | [S8] §4.2, Table 3 | Confirmed: Table 3 has no SWE-bench developer arm; +2.4% and p=.038 are CTXbench |
| SWE-bench Lite 300 tasks; CTXbench 138 instances / 12 repos / 5,694 PRs; 4 agent-model pairs | [S8] §3.2, §4.1 | Confirmed |
| Documentation-stripped ablation: +2.7% for LLM-generated files | [S8] Appendix B, Fig. 12 | Confirmed: "improve performance by 2.7% on average" |
| Length: no clear relationship with resolution rate or cost; no test reported | [S8] Appendix B, Fig. 13 | Confirmed: "no correlation between the resolution rate and the length"; "no clear dependency between the success rate or the per-instance cost and the context file length"; descriptive binning, no test statistic |
| Overviews: "do not meaningfully reduce" steps; Table 7 accuracy 62.32% without vs 68.12% with (p=.15) | [S8] §4.3, Table 7 | Confirmed; accuracy difference is non-significant and directionally pro-overview |
| Two-agent ablation: 288 runs, 17 tasks, 3 repos; unpowered descriptive bound ≤10–15 pp; 10 pp needs ~120–200 tasks | [S9] abstract, §3–§5 | Confirmed: "not a powered equivalence claim"; MDE >30 pp at n=17×3 |
| Lulla: 10 repos, 124 PRs; median runtime −28.64%, output tokens −16.58%; correctness out of scope | [S10] abstract, §1 | Confirmed: "comparable task completion behavior"; output quality explicitly beyond scope |
| Probe-refine: 33.0% vs 28.3% static vs 25.5% unguided; 4 trials; Qwen3.5-35B-A3B; length confound; Nemotron transfer degrades | [S42] abstract, limitations | Confirmed; cross-model transfer described as degrading, not transferring |
| Agent READMEs: 2,303 files / 1,925 repos; median 485/535/335.5 words; 59–67% multiple commits; intervals 23.8/22.3/68.0 h; 57 words/commit is Claude-specific; deletions <15 | [S11] abstract, §4.1–4.2 | Confirmed |
| Configuration smells: lint 62%, bloat 42%, skill 35%, fossilization 24%, blind 16, conflicts 28 instances / 12 FP / 57% precision | [S13] abstract, Table II, §VI-F | Confirmed: Table II column is "Instances"; "28 cases ... only 16 confirmed (57%)" |
| Growth: 247,694 instruction lifetimes, 1,867 repos, +226%, +4.9 net/commit, log-hazard −0.032 | [S14] abstract | Confirmed |
| IFScale: 20 models, 500 instructions, best 68%, earlier-instruction bias | [S15] abstract | Confirmed |
| Context Rot: 18 models; length degradation; distractors | [S17] report | Confirmed |
| Control Illusion: system/user separation fails as reliable hierarchy | [S18] abstract | Confirmed |
| ConInstruct: DeepSeek-R1 F1 91.5%, Claude-4.5-Sonnet 87.3%; rarely notify/clarify | [S19] abstract | Confirmed from abstract; F1 figures not independently re-derived |
| ContextCov: 88.3% vs 67.0% vs 50.3%, 3.4× lower feedback cost, 300 tasks | [S20] abstract | Confirmed |
| RepoNorm: +7.42–10.77% NCR; +5.79–9.09 pp functional, n.s. | [S21] abstract | Confirmed |
| IDEsaster: 30+ vulns, 24 CVEs, 10+ products, 100% of tested | [S24] post | Confirmed |
| Codex exec: `cp ~/.aws/credentials`, before user request, no approval, payload patched | [S32] post | Confirmed |
| NVIDIA: malicious Go dependency writes `AGENTS.md`; `time.Sleep(5*time.Minute)`; suppress PR summary; OpenAI declined changes | [S33b] post | Confirmed |
| Mitiga: ~1m51s; AWS creds, kube config, Terraform state, MCP exfiltration; synthetic take-home | [S34] post | Confirmed |
| Prompt Security: VS Code auto-injects `AGENTS.md`; payload redacted; "enabled by default" is the researcher's claim | [S33a] post | Confirmed as researcher claim; vendor docs describe a configurable setting |
| Claude Code: <200 lines, >4 MiB skipped, imports load at launch, concatenation, user message, v2.1.277 | [S2] docs | Confirmed |
| Codex: 32 KiB combined default; one file/dir; root→CWD concat | [S4], [S35] | Confirmed; guide says combined, config reference says per-file |
| Copilot: 2-page generator limit; ~1,000-line review guide; nearest `AGENTS.md` | [S5], [S36] | Confirmed |
| Cursor: <500 lines; Team→Project→User; `.mdc` required | [S7] | Confirmed |
| VS Code: nested `AGENTS.md` off by default; no precedence guarantee | [S38] | Confirmed |
| vLLM: <200 lines; reject one-off rules and hardcoded paths; offset additions; "Tested?" | [S26] | Confirmed |

### Corrections applied in this revision

1. **"Reliably change behavior" → "can change behavior"** in the thesis and executive summary (F1).
2. **"28 files" → "28 flagged instances, 16 confirmed"** in the executive summary, §3.6, and the failure table (M1).
3. **Length claim** replaced with the paper's descriptive "no clear relationship" and explicitly labeled as binned, not an ablation or a significance test (M2).
4. **Overview claim** softened to "do not meaningfully reduce"; added the Table 7 accuracy result (62.32% vs 68.12%, p=.15) and reframed "overview myth" as "overview question" (M3).
5. **Superlatives removed or hedged**: "most comprehensive" → "largest controlled evaluation reviewed here"; "most consequential" / "highest-severity" replaced with hedged, single-axis language (security impact vs false confidence) (M4, M6).
6. **Developer-file denominator disclosed**: +2.4% and p=.038 are CTXbench-only (138 instances); the design row now distinguishes the SWE-bench 2-arm and CTXbench 3-arm designs (M5).
7. **Two-agent bound** described as an unpowered descriptive bound, with the 120–200-task power requirement (M6).
8. **"Guarantee" → "detectable and blockable"**; 88.3% identified as a measured ceiling in one author-developed evaluation (M7).
9. **Probe-refine** reframed as a narrow single-model result; added the guidance-length confound and cross-model degradation (M8).
10. **Security demos labeled**: all five are now introduced as researcher/vendor proof-of-concepts; `[S33]` split into `[S33a]` (Prompt Security) and `[S33b]` (NVIDIA); VS Code "by default" attributed and dated; Mitiga labeled a synthetic take-home report (M9).
11. **GitHub blog caveat added**: it analyzes `.github/agents/*.md` personas, not root context files (M10).
12. **Minor fixes**: "predictable and measurable" → "common and detectable"; vendor-size "convergence" → a range with the 1,000-line artifact separated; "site recommends" → "lists as popular"; 57 words/commit labeled Claude-specific; "time-to-relevant-file" → steps; "originated with OpenAI" → multi-vendor emergence; "waste less time" → "in some setups, reduce runtime and token use"; `Chakrabarti et al.` → `Chakrabarti`; `CLAUDE.local.md` added to the Claude default.

### Limitations and unresolved items

- **Preprints.** [S8]–[S15], [S18], [S20], [S21], and [S42] are preprints (some with venue acceptance noted). Their findings are reported as the authors state them; none is independently reproduced here.
- **Security demos.** No attack was reproduced. Four are researcher/vendor PoCs; Mitiga is a vendor report about a synthetic take-home repository. None is a field observation, and the exact payloads may be patched.
- **Vendor docs are living documents.** [S2]–[S7], [S35], [S38]–[S39] are undated or versioned pages; claims are as of 2026-10-19.
- **Metadata indexes.** Semantic Scholar returned HTTP 429. Several papers were cross-checked against OpenAlex; eight against arXiv metadata only. This is not two independent indexes for every paper.
- **Author-name variant.** arXiv lists "Niels Mündler-Sasahara"; OpenAlex lists "Niels Mündler". The brief uses the arXiv form and includes Mark Niklas Müller, the third author.
- **TACL year.** arXiv says "Accepted … TACL 2023"; the brief cites TACL 2024, the published volume year.
- **Removed sources.** S22, S37, S40, and S41 were removed as uncited; numbering gaps remain to preserve the other markers.
- **No claims of "verified" or "reproduced".** The checks above are document-level: URL liveness, metadata, and claim-to-text mapping. They are not replications.
