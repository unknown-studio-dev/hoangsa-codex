---
name: hoangsa-cook
description: >
  HOANGSA Codex command for `/hoangsa:cook`. Execute plan.json wave-by-wave
  with fresh context per task. Trigger when the user types `/hoangsa:cook`,
  asks for `hoangsa cook`, selects `/prompts:hoangsa-cook`, or explicitly
  invokes `$hoangsa-cook`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render cook --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
