---
name: hoangsa-command-player
description: >
  Shared runtime rules for Codex-native HOANGSA commands. Use before any
  `hoangsa-*` command skill, `/prompts:hoangsa-*` shortcut, or typed
  `/hoangsa:*` compatibility request.
---

Use this skill as the adapter between Claude-shaped HOANGSA workflows and Codex.

1. Resolve Hoangsa with `command -v hoangsa-cli`; if missing, try `$HOME/.hoangsa/bin/hoangsa-cli`.
2. Render the requested command with `hoangsa-cli codex render <command> --arguments "$ARGUMENTS"`.
3. Never read `.claude/hoangsa` or `~/.claude/hoangsa` in Codex mode.
4. Use available `memory_*` MCP tools before non-trivial edits or factual codebase claims.
5. Convert Claude `AskUserQuestion` steps into concise Codex user questions.
6. Convert Claude `Task` orchestration into explicit Codex subagent instructions.
   Before declaring subagents unavailable, search for Codex multi-agent tooling
   (for example `multi_agent_v1.spawn_agent`) with the available tool discovery
   mechanism. If the rendered workflow explicitly requires workers/subagents and
   no spawn tool is available after discovery, stop and report a blocker instead
   of executing the worker task directly. Only use direct execution when the user
   explicitly overrides the worker requirement.
7. Respect Codex sandboxing, approvals, hooks, skills, and AGENTS.md instructions.
8. Treat custom prompts as shortcuts only. The skill workflow is canonical.
