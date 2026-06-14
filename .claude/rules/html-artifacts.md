# HTML Artifacts Over Markdown

When Claude needs to communicate complex analysis, research, specs, or design explorations, **prefer HTML artifacts over markdown files**.

## Why

Markdown breaks down past ~100 lines. HTML can convey:
- Tabular data, interactive elements, SVG diagrams
- Visual hierarchy with color, layout, and typography
- Two-way interaction (decision cards, sliders, copy-as-prompt buttons)
- Mobile-responsive layouts that are easy to share

## How

Use the `/html` skill. It generates a self-contained HTML document (Tailwind CDN, inline JS, dark theme) and publishes it to the `ai_artifacts` Supabase table. The app renders it at `/ai-chat/<slug>` in a sandboxed iframe.

If `/html` isn't available, write a standalone `.html` file and open it in the browser. The key principle: **HTML is for reading, markdown is for editing.** If nobody will edit the file, use HTML.

## When to use HTML instead of markdown

- Architecture analysis with multiple options to compare
- Research synthesis with tables, diagrams, and recommendations
- Implementation plans with visual structure
- Code review explainers (annotated diffs, flow diagrams)
- Status reports, postmortems, design explorations
- Any document over ~100 lines that needs to be read, not edited

## When markdown is still fine

- CLAUDE.md, README.md, and files humans will edit directly
- Short documents under ~50 lines
- Git-tracked specs that need readable diffs
- ADRs and other documents that follow a standard text format
