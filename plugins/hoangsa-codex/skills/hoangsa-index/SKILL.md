---
name: hoangsa-index
description: >
  HOANGSA Codex command for `/hoangsa:index`. Index the workspace with
  hoangsa-memory for code intelligence and navigation. Trigger when the user
  types `/hoangsa:index`, asks for `hoangsa index`, selects
  `/prompts:hoangsa-index`, or explicitly invokes `$hoangsa-index`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render index --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
