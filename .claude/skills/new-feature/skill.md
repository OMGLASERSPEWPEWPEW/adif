---
name: new-feature
description: Disciplined waterfall process for implementing new features. Orchestrates planning, requirements, design, implementation, and quality review in proper sequence.
---

# New Feature Implementation Workflow

You are executing the **disciplined new feature workflow**. This ensures every feature is approached consistently with proper planning before implementation.

<!-- === CONFIGURATION START === -->
## Configuration

| Setting | Value |
|---------|-------|
| **PRD Directory** | `.claude/docs/prd/` |
| **Test Command** | `npm run test` |
| **Build Command** | `npm run build` |
| **Main Documentation File** | `CLAUDE.md` |

<!-- === CONFIGURATION END === -->

## Waterfall Process

Execute these phases IN ORDER. Do not skip phases.

```
+----------------------------------------------------------------------+
|                    NEW FEATURE WATERFALL                               |
+----------------------------------------------------------------------+
|                                                                       |
|  Phase 1: STRATEGY (Orchestrator)                                    |
|  +-> Analyze request, check roadmap fit, identify dependencies       |
|                          |                                            |
|  Phase 2: REQUIREMENTS (PRD Specialist)                              |
|  +-> Create comprehensive PRD with specs, acceptance criteria        |
|                          |                                            |
|  Phase 3: UX RESEARCH (if user-facing)                               |
|  +-> UX and mobile optimization input                                |
|                          |                                            |
|  Phase 4: ARCHITECTURE (Code Architect)                              |
|  +-> Design folder structure, data models, API contracts             |
|                          |                                            |
|  Phase 4b: QA DOC                                                    |
|  +-> Write docs/qa/<feature>.md checklist for bot verification       |
|                          |                                            |
|  Phase 5: IMPLEMENTATION (Developers)                                |
|  +-> Build the feature following the PRD and architecture            |
|                          |                                            |
|  Phase 6: QUALITY (Code Reviewer + Test Engineer)                    |
|  +-> Review code, write tests, verify acceptance criteria            |
|                          |                                            |
|  Phase 7: DOCUMENTATION                                              |
|  +-> Update main docs, ADRs, feature inventory as needed            |
|                                                                       |
+----------------------------------------------------------------------+
```

<!-- === PHASE AGENTS START === -->

### Phase-to-Agent Mapping

| Phase | Agent(s) | Purpose |
|-------|----------|---------|
| 1. Strategy | `orchestrator` | Roadmap fit, dependencies, go/no-go |
| 2. Requirements | `prd-specialist` | PRD creation |
| 3. UX Research | `ui-designer`, `mobile-ux-optimizer` | User experience |
| 4. Architecture | `code-architect` | Folder structure, data models |
| 4b. QA Doc | (main context) | Bot-verifiable checklist in docs/qa/ |
| 5. Implementation | `frontend-developer`, `backend-architect` | Building |
| 6. Quality | `code-reviewer`, `test-engineer` | Review and testing |
| 7. Documentation | (main context) | Doc updates |

<!-- === PHASE AGENTS END === -->

## Phase 1: Strategic Analysis

Invoke the orchestrator agent to:

1. **Validate roadmap fit**: Does this feature align with current phase goals?
2. **Check dependencies**: What must exist before this feature?
3. **Assess priority**: Score the feature on reach, impact, confidence, effort
4. **Identify stakeholders**: Who cares about this feature?
5. **Define success metrics**: How will we know it worked?

**Output**: Go/no-go decision with strategic context

## Phase 2: Product Requirements

Invoke the PRD specialist agent to create a PRD:

1. **Executive Summary**: Problem, solution, impact
2. **User Stories**: As a [user], I want [goal], so that [benefit]
3. **Functional Requirements**: Numbered, testable requirements
4. **Non-Functional Requirements**: Performance, security, accessibility
5. **Technical Considerations**: Data model, API changes, integrations
6. **UI/UX Specifications**: Wireframes, flows, states
7. **Success Metrics**: Measurable outcomes
8. **Rollout Plan**: Phased deployment strategy
9. **Risks & Mitigations**: What could go wrong

**Output**: Complete PRD document saved to the PRD directory

## Phase 3: UX Research (if user-facing)

For features with UI, invoke design agents:

- User mental models
- Touch target sizing (44px minimum)
- Progressive disclosure strategy
- Mobile-first layout
- Accessibility requirements
- A/B test recommendations

**Output**: UX recommendations integrated into PRD

## Phase 4: Architecture Design

Invoke the code architect agent to:

1. **Folder structure**: Where do new files go?
2. **Data models**: Interfaces, schema changes
3. **API contracts**: Function signatures, return types
4. **Component hierarchy**: Component tree
5. **State management**: Hooks, context, queries

**Output**: Architecture document or section in PRD

## Phase 4b: QA Doc

Before any implementation, write the feature's QA checklist to `docs/qa/<feature-slug>.md` (full contract in `.claude/rules/qa-docs.md`):

1. **Derive items from the PRD's acceptance criteria** and the architecture doc — one `- [ ]` checkbox per observable user-facing behavior, grouped under `###` sections
2. **Headers**: `**Date:**`, `**Scope:**`, plus `**Entry:**` (URL path where testing starts) and `**Todo:**` (the feature's `docs/todo/` file — enables automatic stage advancement) when known
3. **Mark human-only items** with `<!-- qa:human <reason> -->` — mobile gestures, audio, visual polish
4. **Add `## Regression Risks`** with `- **High/Medium/Low:**` bullets

The transformers bot network sweeps these docs (`npm run qa`), verifies each item by squad consensus, files bug reports into `docs/todo/`, and advances the linked todo's stage. Writing the checklist now — while acceptance criteria are fresh — is what makes the feature testable later.

**Output**: `docs/qa/<feature-slug>.md` committed alongside the PRD

## Phase 5: Implementation

Based on the feature type, invoke the appropriate developer agents:

**Frontend-heavy features** -> frontend developer:
- Components, hooks, UI state management, styling

**Backend-heavy features** -> backend architect:
- Schema changes, Edge Functions, API integrations, data migrations

**Full-stack features** -> Both agents in sequence

**Output**: Working code with inline comments

## Phase 6: Quality Assurance

After implementation, ALWAYS invoke:

**Code Reviewer**: Code quality, security audit, performance, best practices

**Test Engineer**: Unit tests, E2E tests, edge case coverage

**Verification** (using configured commands):
```bash
# Run test command
# Run build command
```

**Output**: Reviewed, tested code ready for main

## Phase 7: Documentation

Update relevant documentation:

| Document | When to Update |
|----------|----------------|
| Main documentation file | Architecture, patterns, or key files changed |
| Feature inventory | New feature added |
| ADR directory | Significant architectural decision made |
| PRD | Deviations from original spec |

## Checkpoints

Before proceeding to the next phase, confirm:

- [ ] **Phase 1 -> 2**: Orchestrator approved the feature
- [ ] **Phase 2 -> 3**: PRD created and saved
- [ ] **Phase 3 -> 4**: UX recommendations documented (if applicable)
- [ ] **Phase 4 -> 4b**: Architecture defined
- [ ] **Phase 4b -> 5**: QA doc created in `docs/qa/`
- [ ] **Phase 5 -> 6**: Implementation complete
- [ ] **Phase 6 -> 7**: Tests pass, code reviewed
- [ ] **Phase 7 -> Done**: Docs updated, ready to commit

## Shortcuts

For small features (< 1 day effort), phases can be combined:
- Phases 1+2: Orchestrator does quick strategic check, creates mini-PRD inline
- Phases 3+4: Skip if no UI or obvious architecture
- Phase 4b: never skipped for user-facing changes — a small feature still gets a QA doc, even if it's 3 items
- Phases 6+7: Combine review and docs

For large features (> 1 week effort):
- Each phase may require multiple sessions
- Use task tracking to monitor progress across phases
- Consider feature branches

---

**Remember**: Discipline enables speed. Taking time to plan properly prevents costly rework later.
