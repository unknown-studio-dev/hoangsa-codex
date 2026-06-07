use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const CODEX_MEMORY_SKILLS: &[&str] = &[
    "memory-discipline",
    "memory-reflect",
    "memory-guide",
    "memory-impact-analysis",
    "memory-exploring",
    "memory-debugging",
    "memory-refactoring",
    "memory-cli",
];

const CODEX_COMMAND_SKILLS: &[&str] = &[
    "hoangsa-command-player",
    "hoangsa-help",
    "hoangsa-init",
    "hoangsa-index",
    "hoangsa-check",
    "hoangsa-brainstorm",
    "hoangsa-menu",
    "hoangsa-prepare",
    "hoangsa-cook",
    "hoangsa-taste",
    "hoangsa-fix",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("repo root")
        .to_path_buf()
}

fn read_json(path: &Path) -> Value {
    let raw = fs::read_to_string(path).unwrap_or_else(|e| {
        panic!("read {}: {e}", path.display());
    });
    serde_json::from_str(&raw).unwrap_or_else(|e| {
        panic!("parse {}: {e}", path.display());
    })
}

#[test]
fn codex_plugin_manifest_exposes_skills_and_mcp() {
    let plugin_root = repo_root().join("plugins").join("hoangsa-codex");
    let manifest = read_json(&plugin_root.join(".codex-plugin").join("plugin.json"));

    assert_eq!(manifest["name"], "hoangsa-codex");
    assert_eq!(manifest["skills"], "./skills/");
    assert_eq!(manifest["mcpServers"], "./.mcp.json");
    assert_eq!(manifest["author"]["name"], "Unknown Studio");
    assert!(
        manifest["interface"]["capabilities"]
            .as_array()
            .expect("capabilities array")
            .iter()
            .any(|v| v.as_str() == Some("Workflows")),
        "plugin must advertise workflow capability"
    );

    let mcp = read_json(&plugin_root.join(".mcp.json"));
    let server = &mcp["mcpServers"]["hoangsa-memory"];
    assert_eq!(server["command"], "hoangsa-memory-mcp");
    assert_eq!(server["startup_timeout_sec"], 20);
    assert_eq!(server["tool_timeout_sec"], 120);
    assert_eq!(server["env"]["RUST_LOG"], "info");
    assert!(
        server["env"].get("HOANGSA_MEMORY_ROOT").is_none(),
        "plugin MCP config must not pin a global memory root"
    );
}

#[test]
fn codex_plugin_marketplace_points_at_repo_plugin() {
    let marketplace = read_json(
        &repo_root()
            .join(".agents")
            .join("plugins")
            .join("marketplace.json"),
    );
    assert_eq!(marketplace["name"], "hoangsa-local");

    let plugins = marketplace["plugins"].as_array().expect("plugins array");
    let entry = plugins
        .iter()
        .find(|entry| entry["name"] == "hoangsa-codex")
        .expect("hoangsa-codex marketplace entry");
    assert_eq!(entry["source"]["source"], "local");
    assert_eq!(entry["source"]["path"], "./plugins/hoangsa-codex");
    assert_eq!(entry["policy"]["installation"], "AVAILABLE");
    assert_eq!(entry["policy"]["authentication"], "ON_INSTALL");
}

#[test]
fn codex_plugin_packages_codex_safe_memory_skills() {
    let skills_root = repo_root()
        .join("plugins")
        .join("hoangsa-codex")
        .join("skills");

    for skill in CODEX_MEMORY_SKILLS {
        let skill_md = skills_root.join(skill).join("SKILL.md");
        let text = fs::read_to_string(&skill_md).unwrap_or_else(|e| {
            panic!("read {}: {e}", skill_md.display());
        });
        assert!(
            !text.contains(".claude/") && !text.contains("~/.claude"),
            "{} must not contain Claude-only paths",
            skill_md.display()
        );
        assert!(
            !text.contains(".mcp.json"),
            "{} must not reference Claude MCP config",
            skill_md.display()
        );
    }
}

#[test]
fn codex_plugin_packages_command_workflow_skills() {
    let skills_root = repo_root()
        .join("plugins")
        .join("hoangsa-codex")
        .join("skills");

    for skill in CODEX_COMMAND_SKILLS {
        let skill_md = skills_root.join(skill).join("SKILL.md");
        let text = fs::read_to_string(&skill_md).unwrap_or_else(|e| {
            panic!("read {}: {e}", skill_md.display());
        });
        assert!(
            text.contains("hoangsa-cli codex render") || *skill == "hoangsa-command-player",
            "{} must route through the Codex command renderer",
            skill_md.display()
        );
        assert!(
            text.contains("Codex"),
            "{} must be Codex-specific workflow guidance",
            skill_md.display()
        );
    }
}
