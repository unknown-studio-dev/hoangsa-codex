---
name: hoangsa-prepare
description: >
  HOANGSA Codex command for `/hoangsa:prepare`. Turn DESIGN-SPEC.md and
  TEST-SPEC.md into an executable plan.json task DAG. Trigger when the user
  types `/hoangsa:prepare`, asks for `hoangsa prepare`, selects
  `/prompts:hoangsa-prepare`, or explicitly invokes `$hoangsa-prepare`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render prepare --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
