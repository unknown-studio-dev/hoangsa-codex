---
name: hoangsa-ship
description: >
  HOANGSA Codex command for `/hoangsa:ship`. Review code and security, then
  push or create a PR. Trigger when the user types `/hoangsa:ship`, asks for
  `hoangsa ship`, selects `/prompts:hoangsa-ship`, or explicitly invokes
  `$hoangsa-ship`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render ship --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.

Ship requires parallel Codex subagents for the code review and security review
gate before push/PR actions. Before the parallel review step, discover the Codex
multi-agent spawn tool if it is not already visible. If no spawn tool is
available after discovery, stop and report ship as blocked; do not replace the
required review/security workers with local orchestrator review unless the user
explicitly instructs that fallback.
