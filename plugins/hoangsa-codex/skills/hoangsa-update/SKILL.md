---
name: hoangsa-update
description: >
  HOANGSA Codex command for `/hoangsa:update`. Upgrade HOANGSA to the latest
  version with changelog review. Trigger when the user types
  `/hoangsa:update`, asks for `hoangsa update`, selects
  `/prompts:hoangsa-update`, or explicitly invokes `$hoangsa-update`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render update --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
