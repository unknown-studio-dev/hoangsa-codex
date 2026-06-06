---
name: hoangsa-init
description: >
  HOANGSA Codex command for `/hoangsa:init`. Initialize HOANGSA for a project:
  detect codebase, setup preferences, model routing, and memory indexing.
  Trigger when the user types `/hoangsa:init`, asks for `hoangsa init`,
  selects `/prompts:hoangsa-init`, or explicitly invokes `$hoangsa-init`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render init --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
