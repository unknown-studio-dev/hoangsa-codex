---
name: hoangsa-audit
description: >
  HOANGSA Codex command for `/hoangsa:audit`. Audit the codebase for security,
  debt, coverage, performance, and maintainability issues. Trigger when the
  user types `/hoangsa:audit`, asks for `hoangsa audit`, selects
  `/prompts:hoangsa-audit`, or explicitly invokes `$hoangsa-audit`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render audit --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.

Audit requires parallel Codex subagents for dimension scanning. Before the
parallel scanning step, discover the Codex multi-agent spawn tool if it is not
already visible. If no spawn tool is available after discovery, stop and report
audit as blocked; do not perform the dimension scan sequentially in the
orchestrator thread unless the user explicitly instructs that fallback.
