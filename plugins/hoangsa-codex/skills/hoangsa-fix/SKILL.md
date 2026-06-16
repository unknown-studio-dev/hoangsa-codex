---
name: hoangsa-fix
description: >
  HOANGSA Codex command for `/hoangsa:fix`. Trace a bug to root cause, make a
  minimal fix, and verify it. Trigger when the user types `/hoangsa:fix`, asks
  for `hoangsa fix`, selects `/prompts:hoangsa-fix`, or explicitly invokes
  `$hoangsa-fix`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render fix --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.

Fix uses Codex subagents for cross-layer tracing when triggered, for each
implementation fix task, and for simplify passes when enabled. Before any
rendered fix step that spawns a research/worker/simplify agent, discover the
Codex multi-agent spawn tool if it is not already visible. If a required fix
worker cannot be spawned after discovery, stop and report fix as blocked; do not
implement the fix directly in the orchestrator thread unless the user explicitly
instructs that fallback. If an optional simplify pass cannot spawn, follow the
rendered simplify failure recovery behavior and continue only when the workflow
marks simplify as non-blocking.
