---
name: hoangsa-taste
description: >
  HOANGSA Codex command for `/hoangsa:taste`. Run acceptance tests and report
  pass/fail results per task. Trigger when the user types `/hoangsa:taste`,
  asks for `hoangsa taste`, selects `/prompts:hoangsa-taste`, or explicitly
  invokes `$hoangsa-taste`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render taste --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
