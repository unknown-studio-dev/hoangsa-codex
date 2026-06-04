# Follow up Codex support: real hook fixtures and local memory-root decision

## Context

The Codex support implementation is mostly in place on `master`, including
the Codex plugin distribution package. Local verification passed after building
the UI assets first.

Verification performed:

```sh
npm ci --cache .npm-cache
npm run build
cargo test --workspace
```

Result: workspace tests passed.

## Remaining follow-up items

- Capture real Codex hook payloads for supported lifecycle events and commit
  them as fixtures.
- Extend hook adapter tests to use those real fixtures, especially
  `PreToolUse`, `PostToolUse`, `PreCompact`, and `Stop`.
- Decide whether project-local explicit `HOANGSA_MEMORY_ROOT` should get a
  first-class install flag. Current behavior preserves an existing local
  override and drops global overrides, but does not create one intentionally.
- Decide whether Codex TOML config needs comment/format-preserving edits.
  Current behavior is parse/write semantic preservation, not byte-for-byte
  formatting preservation.
- Keep plugin hook bundling out until the Codex plugin hook schema is
  validated, or add plugin hook packaging once that schema is confirmed.

## Current implementation state

- HS-CX.1: implemented, with MCP config, timeouts, env, no global memory root,
  and initialize instructions.
- HS-CX.2: implemented, with Codex memory skills and `AGENTS.md` guidance.
- HS-CX.3: implemented enough for adapter/install behavior, pending real Codex
  payload fixtures.
- HS-CX.4: implemented for plugin skills and MCP metadata; hooks intentionally
  not bundled yet.
