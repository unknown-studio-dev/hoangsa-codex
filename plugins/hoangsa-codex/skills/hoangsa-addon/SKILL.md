---
name: hoangsa-addon
description: >
  HOANGSA Codex command for `/hoangsa:addon`. List, add, or remove
  framework-specific worker rule addons. Trigger when the user types
  `/hoangsa:addon`, asks for `hoangsa addon`, selects
  `/prompts:hoangsa-addon`, or explicitly invokes `$hoangsa-addon`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render addon --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
