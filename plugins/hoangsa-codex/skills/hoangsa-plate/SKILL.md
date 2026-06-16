---
name: hoangsa-plate
description: >
  HOANGSA Codex command for `/hoangsa:plate`. Stage changed files and commit
  with a conventional commit message. Trigger when the user types
  `/hoangsa:plate`, asks for `hoangsa plate`, selects `/prompts:hoangsa-plate`,
  or explicitly invokes `$hoangsa-plate`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render plate --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
