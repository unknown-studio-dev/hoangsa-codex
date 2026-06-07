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
