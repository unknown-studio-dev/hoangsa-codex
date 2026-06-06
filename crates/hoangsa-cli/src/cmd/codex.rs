use crate::helpers;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MANAGED_START: &str = "<!-- hoangsa-codex-managed-start -->";
const MANAGED_END: &str = "<!-- hoangsa-codex-managed-end -->";

#[derive(Debug, Clone, Copy)]
pub struct CodexCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub workflow: &'static str,
    pub workflow_body: &'static str,
}

#[derive(Debug, Serialize)]
struct CommandView {
    name: &'static str,
    legacy_slash: String,
    skill: String,
    prompt: String,
    workflow: &'static str,
    description: &'static str,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct InstallReport {
    pub copied: Vec<PathBuf>,
    pub skipped: Vec<PathBuf>,
}

pub const COMMANDS: &[CodexCommand] = &[
    CodexCommand {
        name: "help",
        description: "Show HOANGSA commands and workflow.",
        workflow: "help",
        workflow_body: include_str!("../../../../templates/commands/hoangsa/help.md"),
    },
    CodexCommand {
        name: "init",
        description: "Initialize HOANGSA for a project: detect codebase, setup preferences, model routing, and memory indexing.",
        workflow: "init",
        workflow_body: include_str!("../../../../templates/workflows/init.md"),
    },
    CodexCommand {
        name: "index",
        description: "Index the workspace with hoangsa-memory for code intelligence and navigation.",
        workflow: "index",
        workflow_body: include_str!("../../../../templates/workflows/index.md"),
    },
    CodexCommand {
        name: "check",
        description: "Show session progress with wave structure, budget usage, and artifacts.",
        workflow: "check",
        workflow_body: include_str!("../../../../templates/workflows/check.md"),
    },
    CodexCommand {
        name: "brainstorm",
        description: "Explore a vague idea, compare approaches, and produce BRAINSTORM.md before design.",
        workflow: "brainstorm",
        workflow_body: include_str!("../../../../templates/workflows/brainstorm.md"),
    },
    CodexCommand {
        name: "menu",
        description: "Design a task from idea to DESIGN-SPEC.md and TEST-SPEC.md.",
        workflow: "menu",
        workflow_body: include_str!("../../../../templates/workflows/menu.md"),
    },
    CodexCommand {
        name: "prepare",
        description: "Turn DESIGN-SPEC.md and TEST-SPEC.md into an executable plan.json task DAG.",
        workflow: "prepare",
        workflow_body: include_str!("../../../../templates/workflows/prepare.md"),
    },
    CodexCommand {
        name: "cook",
        description: "Execute plan.json wave-by-wave with fresh context per task.",
        workflow: "cook",
        workflow_body: include_str!("../../../../templates/workflows/cook.md"),
    },
    CodexCommand {
        name: "taste",
        description: "Run acceptance tests and report pass/fail results per task.",
        workflow: "taste",
        workflow_body: include_str!("../../../../templates/workflows/taste.md"),
    },
    CodexCommand {
        name: "fix",
        description: "Trace a bug to root cause, make a minimal fix, and verify it.",
        workflow: "fix",
        workflow_body: include_str!("../../../../templates/workflows/fix.md"),
    },
];

fn command_view(cmd: &CodexCommand) -> CommandView {
    CommandView {
        name: cmd.name,
        legacy_slash: format!("/hoangsa:{}", cmd.name),
        skill: skill_name(cmd.name),
        prompt: prompt_name(cmd.name),
        workflow: cmd.workflow,
        description: cmd.description,
    }
}

pub fn skill_name(name: &str) -> String {
    format!("hoangsa-{name}")
}

pub fn prompt_name(name: &str) -> String {
    format!("hoangsa-{name}")
}

fn normalize_name(raw: &str) -> String {
    raw.trim()
        .trim_start_matches('/')
        .strip_prefix("hoangsa:")
        .unwrap_or_else(|| raw.trim().trim_start_matches('/'))
        .strip_prefix("hoangsa-")
        .unwrap_or_else(|| {
            raw.trim()
                .trim_start_matches('/')
                .strip_prefix("hoangsa:")
                .unwrap_or_else(|| raw.trim().trim_start_matches('/'))
        })
        .to_string()
}

pub fn find_command(raw: &str) -> Option<&'static CodexCommand> {
    let normalized = normalize_name(raw);
    COMMANDS.iter().find(|cmd| cmd.name == normalized)
}

pub fn render_command(raw: &str, arguments: Option<&str>) -> Result<String, String> {
    let cmd = find_command(raw).ok_or_else(|| {
        format!(
            "unknown Codex Hoangsa command `{raw}` (expected one of: {})",
            COMMANDS
                .iter()
                .map(|c| c.name)
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;
    Ok(render_command_body(cmd, arguments.unwrap_or("").trim()))
}

fn render_command_body(cmd: &CodexCommand, arguments: &str) -> String {
    let workflow = adapt_workflow_for_codex(cmd.workflow_body);
    let args = if arguments.is_empty() {
        "(none)"
    } else {
        arguments
    };
    format!(
        r#"{MANAGED_START}
# HOANGSA Codex command: /hoangsa:{name}

Use the `${skill}` skill and follow this Codex-adapted Hoangsa workflow.

## Invocation
- Legacy Claude command name: `/hoangsa:{name}`
- Codex skill name: `${skill}`
- Codex prompt shortcut: `/prompts:{prompt}`
- User arguments: {args}

## Codex Runtime Adaptation
- Resolve Hoangsa CLI with `command -v hoangsa-cli`, then `$HOME/.hoangsa/bin/hoangsa-cli`.
- Use `hoangsa-cli` commands directly; do not read legacy Claude workflow directories in Codex mode.
- Use available `memory_*` MCP tools before non-trivial code edits or codebase factual claims.
- Replace Claude structured-question steps with concise Codex user questions. In Plan Mode, use the available user-input mechanism when the environment provides one.
- Replace Claude `Task` tool steps with explicit Codex subagent instructions. Spawn subagents only when the user asked for parallel agent work or the workflow explicitly requires it and the session supports subagents.
- Respect Codex sandbox, approval, and hook behavior. Do not bypass approvals or hooks.

## Workflow
{workflow}
{MANAGED_END}
"#,
        name = cmd.name,
        skill = skill_name(cmd.name),
        prompt = prompt_name(cmd.name),
        args = args,
        workflow = workflow
    )
}

fn adapt_workflow_for_codex(raw: &str) -> String {
    raw.replace("Claude Code", "Codex")
        .replace("Claude", "Codex")
        .replace("AskUserQuestion", "Codex user question")
        .replace("using the `Task` tool", "using Codex subagent workflow")
        .replace("Task tool", "Codex subagent workflow")
        .replace("using Task tool", "using Codex subagent workflow")
        .replace("`Task`", "`Codex subagent`")
        .replace("./.claude/hoangsa", "Hoangsa installed workflow templates")
        .replace("~/.claude/hoangsa", "Hoangsa installed workflow templates")
        .replace(".claude/hoangsa", "Hoangsa installed workflow templates")
        .replace("~/.claude", "$HOME/.codex")
        .replace(".claude", ".codex")
}

fn command_skill_text(cmd: &CodexCommand) -> String {
    let name = skill_name(cmd.name);
    format!(
        r#"---
name: {name}
description: >
  HOANGSA Codex command for `/hoangsa:{command}`. {description}
  Trigger when the user types `/hoangsa:{command}`, asks for `hoangsa {command}`,
  selects `/prompts:{prompt}`, or explicitly invokes `${name}`.
---

First read and follow the shared `$hoangsa-command-player` skill.

Render the command prompt with:

```sh
hoangsa-cli codex render {command} --arguments "$ARGUMENTS"
```

If `$ARGUMENTS` is unavailable, pass an empty string. Follow the rendered
workflow exactly, using Codex-native questions, subagents, MCP tools, sandbox,
approvals, and hooks.
"#,
        name = name,
        command = cmd.name,
        prompt = prompt_name(cmd.name),
        description = cmd.description
    )
}

fn command_player_skill_text() -> &'static str {
    r#"---
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
"#
}

pub fn install_command_skills_to(skills_root: &Path) -> io::Result<InstallReport> {
    let mut report = InstallReport::default();
    fs::create_dir_all(skills_root)?;
    write_if_changed(
        &skills_root.join("hoangsa-command-player").join("SKILL.md"),
        command_player_skill_text(),
        &mut report,
    )?;
    for cmd in COMMANDS {
        write_if_changed(
            &skills_root.join(skill_name(cmd.name)).join("SKILL.md"),
            &command_skill_text(cmd),
            &mut report,
        )?;
    }
    Ok(report)
}

fn write_if_changed(path: &Path, body: &str, report: &mut InstallReport) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    match fs::read_to_string(path) {
        Ok(prev) if prev == body => {
            report.skipped.push(path.to_path_buf());
            Ok(())
        }
        _ => {
            fs::write(path, body)?;
            report.copied.push(path.to_path_buf());
            Ok(())
        }
    }
}

pub fn codex_home_dir() -> Result<PathBuf, String> {
    if let Some(raw) = std::env::var_os("CODEX_HOME") {
        let s = raw.to_string_lossy().into_owned();
        if !s.is_empty() {
            if s == "~" {
                return home_dir();
            }
            if let Some(rest) = s.strip_prefix("~/") {
                return Ok(home_dir()?.join(rest));
            }
            return Ok(PathBuf::from(s));
        }
    }
    Ok(home_dir()?.join(".codex"))
}

fn home_dir() -> Result<PathBuf, String> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .ok_or_else(|| "cannot resolve $HOME".to_string())
}

pub fn prompt_shortcuts_dir() -> Result<PathBuf, String> {
    Ok(codex_home_dir()?.join("prompts"))
}

pub fn install_prompt_shortcuts_to(prompts_dir: &Path) -> io::Result<InstallReport> {
    let mut report = InstallReport::default();
    fs::create_dir_all(prompts_dir)?;
    for cmd in COMMANDS {
        let path = prompts_dir.join(format!("{}.md", prompt_name(cmd.name)));
        write_if_changed(&path, &prompt_shortcut_text(cmd), &mut report)?;
    }
    Ok(report)
}

pub fn install_prompt_shortcuts_global() -> Result<InstallReport, String> {
    let dir = prompt_shortcuts_dir()?;
    install_prompt_shortcuts_to(&dir).map_err(|e| format!("write {}: {e}", dir.display()))
}

fn prompt_shortcut_text(cmd: &CodexCommand) -> String {
    format!(
        r#"---
description: {description}
argument-hint: [ARGUMENTS]
---

Use the `${skill}` skill.

Render and follow the Codex-native Hoangsa command:

```sh
hoangsa-cli codex render {command} --arguments "$ARGUMENTS"
```

User arguments: $ARGUMENTS
"#,
        description = cmd.description,
        skill = skill_name(cmd.name),
        command = cmd.name
    )
}

pub fn cmd_commands(args: &[&str]) {
    let json_mode = args.iter().any(|a| *a == "--json");
    if json_mode {
        helpers::out(&json!(
            COMMANDS.iter().map(command_view).collect::<Vec<_>>()
        ));
        return;
    }
    for cmd in COMMANDS {
        println!(
            "{:<12} {:<20} {}",
            format!("/hoangsa:{}", cmd.name),
            format!("${}", skill_name(cmd.name)),
            cmd.description
        );
    }
}

pub fn cmd_render(args: &[&str]) {
    let Some(command) = args.first().copied() else {
        eprintln!("usage: hoangsa-cli codex render <command> [--arguments \"...\"]");
        std::process::exit(2);
    };
    let mut arguments: Option<&str> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i] {
            "--arguments" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("codex render: --arguments requires a value");
                    std::process::exit(2);
                }
                arguments = Some(args[i]);
            }
            s if s.starts_with("--arguments=") => {
                arguments = Some(&s["--arguments=".len()..]);
            }
            other => {
                eprintln!("codex render: unknown flag `{other}`");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    match render_command(command, arguments) {
        Ok(body) => print!("{body}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

pub fn cmd_install_prompts(args: &[&str]) {
    if args.iter().any(|a| *a == "--local") {
        eprintln!("codex install-prompts only supports global Codex prompts");
        std::process::exit(2);
    }
    if let Some(flag) = args.iter().find(|a| **a != "--global") {
        eprintln!("codex install-prompts: unknown flag `{flag}`");
        std::process::exit(2);
    }
    match install_prompt_shortcuts_global() {
        Ok(report) => helpers::out(&json!({
            "status": "ok",
            "prompts_dir": prompt_shortcuts_dir().ok(),
            "copied": report.copied,
            "skipped": report.skipped,
        })),
        Err(e) => {
            eprintln!("codex install-prompts: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_lookup_accepts_legacy_and_codex_names() {
        assert_eq!(find_command("/hoangsa:menu").unwrap().name, "menu");
        assert_eq!(find_command("hoangsa-menu").unwrap().name, "menu");
        assert_eq!(find_command("menu").unwrap().name, "menu");
    }

    #[test]
    fn render_adapts_claude_specific_terms() {
        let rendered = render_command("cook", Some("T-01")).expect("render");
        assert!(rendered.contains("HOANGSA Codex command: /hoangsa:cook"));
        assert!(rendered.contains("User arguments: T-01"));
        assert!(rendered.contains("Codex subagent"));
        assert!(!rendered.contains("AskUserQuestion"));
        assert!(!rendered.contains(".claude/hoangsa"));
    }

    #[test]
    fn install_prompts_is_idempotent_and_preserves_foreign_files() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let prompts = tmp.path().join("prompts");
        fs::create_dir_all(&prompts).expect("mkdir prompts");
        let foreign = prompts.join("mine.md");
        fs::write(&foreign, "do not touch\n").expect("write foreign");

        let first = install_prompt_shortcuts_to(&prompts).expect("first install");
        assert_eq!(first.copied.len(), COMMANDS.len());
        let second = install_prompt_shortcuts_to(&prompts).expect("second install");
        assert_eq!(second.skipped.len(), COMMANDS.len());
        assert_eq!(fs::read_to_string(foreign).unwrap(), "do not touch\n");
        assert!(
            fs::read_to_string(prompts.join("hoangsa-menu.md"))
                .unwrap()
                .contains("$hoangsa-menu")
        );
    }

    #[test]
    fn install_command_skills_writes_player_and_command_skill() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let report = install_command_skills_to(tmp.path()).expect("install skills");
        assert_eq!(report.copied.len(), COMMANDS.len() + 1);
        assert!(tmp.path().join("hoangsa-command-player/SKILL.md").exists());
        let menu = fs::read_to_string(tmp.path().join("hoangsa-menu/SKILL.md")).unwrap();
        assert!(menu.contains("name: hoangsa-menu"));
        assert!(menu.contains("/hoangsa:menu"));
    }
}
