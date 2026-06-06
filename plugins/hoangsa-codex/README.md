# HOANGSA Codex Plugin

This plugin packages the Codex-facing HOANGSA memory integration:

- memory discipline skills under `skills/`
- Codex-native HOANGSA command workflow skills under `skills/`
- a `hoangsa-memory` MCP server entry in `.mcp.json`
- plugin metadata in `.codex-plugin/plugin.json`

The MCP command expects `hoangsa-memory-mcp` to be available on `PATH`.
For direct project setup, `hoangsa-cli install --target codex --local` remains
the most explicit install path because it writes the project-local Codex config
and hook entries.

Hooks are intentionally not bundled in this plugin yet. Use the direct CLI
installer for Codex hook setup until plugin hook packaging is validated against
the target Codex release.

Workflow commands are exposed as Codex skills such as `$hoangsa-menu`,
`$hoangsa-prepare`, and `$hoangsa-cook`. For slash-menu shortcuts, run the
direct installer so it can write managed custom prompts into
`~/.codex/prompts/`; Codex exposes those as `/prompts:hoangsa-menu` style
commands.
