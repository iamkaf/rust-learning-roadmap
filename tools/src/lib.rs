use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

/// Workspace member information
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: PathBuf,
}

/// Project information parsed from README
#[derive(Debug, Clone)]
pub struct Project {
    pub number: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub workspace_member: Option<String>,
}

/// Get all workspace members from the root directory
pub fn get_workspace_members() -> Result<Vec<WorkspaceMember>> {
    let root = get_workspace_root()?;
    let cargo_toml_path = root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;

    let members_regex = Regex::new(r#"(?s)members\s*=\s*\[(.*?)\]"#)?;
    let member_regex = Regex::new(r#""([^"]+)""#)?;

    let members_section = members_regex
        .captures(&content)
        .context("Could not find workspace members in Cargo.toml")?
        .get(1)
        .unwrap()
        .as_str();

    let mut members = Vec::new();
    for cap in member_regex.captures_iter(members_section) {
        let member_name = cap.get(1).unwrap().as_str().to_string();
        let member_path = root.join(&member_name);

        if member_path.exists() {
            members.push(WorkspaceMember {
                name: member_name,
                path: member_path,
            });
        }
    }

    Ok(members)
}

/// Find the workspace root directory
pub fn get_workspace_root() -> Result<PathBuf> {
    let mut current = std::env::current_dir()?;

    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            let content = fs::read_to_string(&cargo_toml)?;
            if content.contains("[workspace]") {
                return Ok(current);
            }
        }

        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return Err(anyhow::anyhow!("Could not find workspace root")),
        }
    }
}

/// Parse projects from README.md
pub fn parse_projects() -> Result<Vec<Project>> {
    let root = get_workspace_root()?;
    let readme_path = root.join("README.md");
    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read {}", readme_path.display()))?;

    let project_regex = Regex::new(
        r"- \[([ x])\] (\d+)\. (?:[🎯👋🤝🧮🎲🌡️🎂⚖️💰➕🔢📏📅🎵💎🧪📊🔤🔍❗🔄📝🔢🔐✂️✖️📅🎨🕵️📚🏆🔒🔗🎛️📦✂️📚✅🔄⏰🧮🏠🔗📦🧹🚀🎓📐🔄❓🏦🌈🃏👥🔺📄🧮📁🌐⌨️🌍🎮🆔📅🎨🖨️📦🔄⏭️💾🔗🎭📦🔌🗃️👁️🏗️✅🔗🔌🌍📈🧪❌⬆️📁🌐✅🔄📝⚠️📊🔌🌍📧🗃️🔍🌳🗂️🕸️🔄🗃️🗄️📊📈🔍🧠🧵📨🔒🌐📁🏭🖥️⚡🔌🗄️📧🔄📊🗃️🌍🖼️🎨💻📱🎮🌟🎯🖥️📊🎪🖌️🔍📝🎵🖨️🗄️🌍🤖💬🔗🌐🔍🎮🖥️🌟]+ )?\*\*([^*]+)\*\* - (.+)"
    )?;

    let mut projects = Vec::new();

    for cap in project_regex.captures_iter(&content) {
        let completed = cap.get(1).unwrap().as_str() == "x";
        let number: u32 = cap.get(2).unwrap().as_str().parse()?;
        let title = cap.get(3).unwrap().as_str().trim().to_string();
        let description = cap.get(4).unwrap().as_str().trim().to_string();

        let workspace_member = determine_workspace_member(number);

        projects.push(Project {
            number,
            title,
            description,
            completed,
            workspace_member,
        });
    }

    Ok(projects)
}

/// Determine which workspace member a project belongs to based on project number
pub fn determine_workspace_member(project_number: u32) -> Option<String> {
    match project_number {
        1..=30 => Some("basic-projects".to_string()),
        31..=45 => Some("ownership-projects".to_string()),
        46..=65 => Some("basic-projects".to_string()), // Structs & Enums still basic
        66..=85 => Some("advanced-projects".to_string()), // Traits & Generics
        86..=100 => Some("advanced-projects".to_string()), // Error Handling
        101..=110 => Some("advanced-projects".to_string()), // Collections & Algorithms
        111..=125 => Some("web-projects".to_string()), // Concurrency & Async (mainly web)
        126..=140 => Some("desktop-projects".to_string()), // Graphics & Desktop
        141..=150 => Some("advanced-projects".to_string()), // Masterpiece projects
        _ => None,
    }
}

/// Get the expected binary filename for a project
pub fn get_project_filename(project_number: u32, title: &str) -> String {
    let clean_title = title
        .to_lowercase()
        .replace(" ", "_")
        .replace("-", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    format!("{:02}_{}.rs", project_number, clean_title)
}

/// Check if a project has been implemented (file exists)
pub fn is_project_implemented(project: &Project) -> Result<bool> {
    let workspace_member = match &project.workspace_member {
        Some(member) => member,
        None => return Ok(false),
    };

    let root = get_workspace_root()?;
    let bin_dir = root
        .join(workspace_member)
        .join("src")
        .join("bin");

    // Try exact filename match first
    let expected_filename = get_project_filename(project.number, &project.title);
    let exact_path = bin_dir.join(&expected_filename);
    if exact_path.exists() {
        return Ok(true);
    }

    // Try pattern matching for existing files with same project number
    let project_prefix = format!("{:02}_", project.number);

    if bin_dir.exists() {
        for entry in std::fs::read_dir(&bin_dir)? {
            let entry = entry?;
            if let Some(filename) = entry.file_name().to_str() {
                if filename.starts_with(&project_prefix) && filename.ends_with(".rs") {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}