---
name: hoangsa-serve
description: >
  HOANGSA Codex command for `/hoangsa:serve`. Sync task status and reports with
  connected task managers. Trigger when the user types `/hoangsa:serve`, asks
  for `hoangsa serve`, selects `/prompts:hoangsa-serve`, or explicitly invokes
  `$hoangsa-serve`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render serve --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
