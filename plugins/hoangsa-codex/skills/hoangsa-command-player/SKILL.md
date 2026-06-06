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
6. Convert Claude `Task` orchestration into explicit Codex subagent instructions; only spawn subagents when appropriate for the active session.
7. Respect Codex sandboxing, approvals, hooks, skills, and AGENTS.md instructions.
8. Treat custom prompts as shortcuts only. The skill workflow is canonical.
