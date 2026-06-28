---
name: escalate
description: >
  Consult GPT-5.5, Gemini 3.1 Pro, DeepSeek V4 Pro, and Claude Opus 4.8
  in parallel for independent analysis. Modes: diagnosis (stubborn bugs),
  review (code/approach review), architecture (design decisions), freeform
  (any question). Use when you've struck out on a fix 3+ times.
user_invocable: true
---

# Escalate: Multi-Model Panel

Consult four frontier models in parallel for independent analysis. Not just
for bugs — use it for code review, architecture decisions, or any question
where diverse expert perspectives add value.

<!-- === CONFIGURATION START === -->
## Configuration

| Setting | Value |
|---------|-------|
| **Diagnose CLI** | `E:/development/patterns/diagnostics/diagnosis/diagnose.ts` |
| **API Keys .env** | `E:/development/patterns/.env` |
| **Input file** | `/tmp/diagnosis-input.json` |
| **Output file** | `/tmp/diagnosis-report.json` |

<!-- === CONFIGURATION END === -->

## Modes

| Mode | When to use | Input JSON fields |
|------|-------------|-------------------|
| `diagnosis` | Stubborn, recurring bug (3+ failed fixes) | `bugDescription`, `fixAttempts` |
| `review` | Code review, approach validation | `bugDescription` (as context), `question` |
| `architecture` | Design decisions, tradeoff analysis | `bugDescription` (as context), `question` |
| `freeform` | Any question that benefits from multi-model perspective | `question` |

Default mode is `diagnosis` (backward-compatible).

## Phases

### Phase 1: Determine Mode

Read the conversation context to determine which mode fits:

- **User hit strike 3 on a bug?** → `diagnosis`
- **User wants a second opinion on code or an approach?** → `review`
- **User is weighing architectural options?** → `architecture`
- **Anything else (general question, strategy, analysis)?** → `freeform`

### Phase 2: Gather

Collect everything the panel needs. **Be thorough.** The models have no
context from this conversation — you must give them everything they need
to reason independently. Include:

- Full bug description with observable symptoms
- All fix attempts with outcomes
- Relevant source code (read the actual files, don't summarize)
- Capture data, wire traces, log output — anything empirical
- What works vs what doesn't
- Reference implementations or working examples to compare against

**Do NOT bias the models.** Present facts and data. Let them form their
own hypotheses. Don't lead with "I think the issue is X."

### Phase 3: Construct

Write the input to `/tmp/diagnosis-input.json`.

The `additionalContext` field is your main canvas — use it generously.
Include source code snippets, hex dumps, capture data, struct layouts,
and anything else the models need. Don't be stingy with context.

```json
{
  "mode": "diagnosis",
  "bugDescription": "Clear summary of the bug with observable symptoms",
  "fixAttempts": [
    { "description": "What was tried", "outcome": "What happened" }
  ],
  "additionalContext": "FULL context here — source code, captures, struct layouts, wire traces, working vs broken comparisons. Be exhaustive."
}
```

### Phase 4: Query

Source the API keys from the .env file and run the CLI. The CLI's internal
`~/Development/patterns/.env` path doesn't resolve correctly on Windows
(wrong drive), so we source the keys as env vars explicitly.

```bash
export $(cat E:/development/patterns/.env | grep -E "^(OPENAI|GEMINI|DEEPSEEK|ANTHROPIC)_API_KEY=" | xargs) && \
cd E:/development/adif && \
npx tsx E:/development/patterns/diagnostics/diagnosis/diagnose.ts \
  --input /tmp/diagnosis-input.json \
  --files "file1.rs,file2.rs,file3.h" \
  --output /tmp/diagnosis-report.json \
  --verbose
```

**Important:**
- Use the Bash tool, not PowerShell (the `export` syntax is bash)
- Set timeout to 300000 (5 minutes) — models take time
- The `--files` flag sends FULL file contents to every model. Include
  the 2-5 most critical files. For large reference files (like
  titanium_structs.h at 1000+ lines), put the relevant excerpts in
  `additionalContext` instead
- Always use `--verbose` so we can see which models succeed/fail

### Phase 5: Evaluate

Read `/tmp/diagnosis-report.json`. For each model that returned a result:

**Diagnosis mode — evaluate:**
1. Root cause — What does this model think is fundamental?
2. Why previous fixes failed — Any new insights?
3. Proposed fix — Concrete and actionable?
4. Confidence level (0.0-1.0)

**Review/Architecture/Freeform — evaluate:**
1. Key insights and recommendations
2. Strengths and concerns identified
3. Actionable next steps

**Across all modes, synthesize:**
- **Consensus** — Where do 2+ models agree?
- **Divergence** — Where do models disagree? Which view has strongest
  reasoning? Verify claims against actual code before trusting.
- **Novel insights** — Did any model surface something unexpected?

### Phase 6: Present

Present your synthesized analysis to the user:

```
## Panel Results

**Models consulted:** [list with success/failure status]
**Mode:** [diagnosis|review|architecture|freeform]

### Consensus
[Where 2+ models agree]

### Key Insight
[The most important new understanding from the panel]

### Divergences
[Where models disagree, and which view you find most compelling and why]

### Recommended [Fix Plan | Next Steps | Approach | Action Items]
[Your concrete, step-by-step plan informed by the panel]
1. ...
2. ...
3. ...

### Confidence & Caveats
[Overall confidence level and things to watch for]
```

## Golden Rule

**YOU evaluate the model responses. YOU determine how they guide your next
plan. Do NOT present the raw model outputs to the user for evaluation.**
The user should see your synthesized analysis and informed plan — not four
competing walls of JSON.

## When NOT to Escalate

- The issue is straightforward and you haven't exhausted your own analysis
- It's a simple configuration or typo problem
- You already understand the root cause and just need to decide between
  approaches (use planning)
- The question doesn't benefit from diverse perspectives

## Requirements

- API keys at `E:/development/patterns/.env` (OPENAI, GEMINI, DEEPSEEK,
  ANTHROPIC — need at least one)
- Node.js with `tsx` available (`npx tsx --version` to verify)
- Diagnose CLI at `E:/development/patterns/diagnostics/diagnosis/diagnose.ts`
