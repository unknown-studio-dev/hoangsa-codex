---
name: hoangsa-cook
description: >
  HOANGSA Codex command for `/hoangsa:cook`. Execute plan.json wave-by-wave
  with fresh context per task. Trigger when the user types `/hoangsa:cook`,
  asks for `hoangsa cook`, selects `/prompts:hoangsa-cook`, or explicitly
  invokes `$hoangsa-cook`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render cook --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.

Cook requires one Codex subagent worker per plan task and fresh context per
worker. Before Step 3 of the rendered workflow, discover the Codex multi-agent
spawn tool if it is not already visible. If no spawn tool is available after
discovery, stop and report cook as blocked; do not implement plan tasks in the
orchestrator thread unless the user explicitly instructs a direct-execution
fallback.
