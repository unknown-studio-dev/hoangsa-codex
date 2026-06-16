---
name: hoangsa-research
description: >
  HOANGSA Codex command for `/hoangsa:research`. Research a topic with codebase
  analysis and external context, producing RESEARCH.md. Trigger when the user
  types `/hoangsa:research`, asks for `hoangsa research`, selects
  `/prompts:hoangsa-research`, or explicitly invokes `$hoangsa-research`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render research --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.

Research uses parallel Codex subagents for codebase research when the rendered
workflow enters the codebase-research phase. Before that phase, discover the
Codex multi-agent spawn tool if it is not already visible. If no spawn tool is
available after discovery and the requested research scope requires codebase
agents, stop and report research as blocked; do not silently replace the
parallel research agents with single-thread orchestrator research unless the
user explicitly instructs that fallback.
