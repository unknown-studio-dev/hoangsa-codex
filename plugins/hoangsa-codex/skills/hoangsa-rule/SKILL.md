---
name: hoangsa-rule
description: >
  HOANGSA Codex command for `/hoangsa:rule`. Add, remove, or list project
  enforcement rules. Trigger when the user types `/hoangsa:rule`, asks for
  `hoangsa rule`, selects `/prompts:hoangsa-rule`, or explicitly invokes
  `$hoangsa-rule`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render rule --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
