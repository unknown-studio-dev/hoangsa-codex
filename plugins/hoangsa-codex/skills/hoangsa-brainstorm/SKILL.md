---
name: hoangsa-brainstorm
description: >
  HOANGSA Codex command for `/hoangsa:brainstorm`. Explore a vague idea,
  compare approaches, and produce BRAINSTORM.md before design. Trigger when
  the user types `/hoangsa:brainstorm`, asks for `hoangsa brainstorm`,
  selects `/prompts:hoangsa-brainstorm`, or explicitly invokes
  `$hoangsa-brainstorm`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render brainstorm --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
