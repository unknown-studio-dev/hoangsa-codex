---
name: hoangsa-menu
description: >
  HOANGSA Codex command for `/hoangsa:menu`. Design a task from idea to
  DESIGN-SPEC.md and TEST-SPEC.md. Trigger when the user types
  `/hoangsa:menu`, asks for `hoangsa menu`, selects `/prompts:hoangsa-menu`,
  or explicitly invokes `$hoangsa-menu`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render menu --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
