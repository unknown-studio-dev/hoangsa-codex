---
name: hoangsa-help
description: >
  HOANGSA Codex command for `/hoangsa:help`. Show HOANGSA commands and workflow.
  Trigger when the user types `/hoangsa:help`, asks for `hoangsa help`,
  selects `/prompts:hoangsa-help`, or explicitly invokes `$hoangsa-help`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render help --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
