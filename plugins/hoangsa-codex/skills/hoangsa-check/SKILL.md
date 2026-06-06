---
name: hoangsa-check
description: >
  HOANGSA Codex command for `/hoangsa:check`. Show session progress with wave
  structure, budget usage, and artifacts. Trigger when the user types
  `/hoangsa:check`, asks for `hoangsa check`, selects `/prompts:hoangsa-check`,
  or explicitly invokes `$hoangsa-check`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render check --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
