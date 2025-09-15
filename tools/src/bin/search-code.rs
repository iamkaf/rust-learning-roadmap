use anyhow::Result;
use clap::{Arg, Command};
use colored::*;
use regex::Regex;
use std::fs;
use tools::get_workspace_members;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct SearchMatch {
    file_path: String,
    line_number: usize,
    line_content: String,
    workspace: String,
}

fn main() -> Result<()> {
    let matches = Command::new("search-code")
        .about("Search for patterns in Rust code across all workspace members")
        .arg(
            Arg::new("pattern")
                .help("Pattern to search for (supports regex)")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("context")
                .short('C')
                .long("context")
                .value_name("NUM")
                .help("Show NUM lines of context around each match"),
        )
        .arg(
            Arg::new("after-context")
                .short('A')
                .long("after-context")
                .value_name("NUM")
                .help("Show NUM lines after each match"),
        )
        .arg(
            Arg::new("before-context")
                .short('B')
                .long("before-context")
                .value_name("NUM")
                .help("Show NUM lines before each match"),
        )
        .arg(
            Arg::new("workspace")
                .short('w')
                .long("workspace")
                .value_name("WORKSPACE")
                .help("Search only in specific workspace member"),
        )
        .arg(
            Arg::new("structs")
                .long("structs")
                .action(clap::ArgAction::SetTrue)
                .help("Search for struct definitions"),
        )
        .arg(
            Arg::new("functions")
                .long("functions")
                .action(clap::ArgAction::SetTrue)
                .help("Search for function definitions"),
        )
        .arg(
            Arg::new("traits")
                .long("traits")
                .action(clap::ArgAction::SetTrue)
                .help("Search for trait definitions"),
        )
        .arg(
            Arg::new("enums")
                .long("enums")
                .action(clap::ArgAction::SetTrue)
                .help("Search for enum definitions"),
        )
        .arg(
            Arg::new("case-insensitive")
                .short('i')
                .long("ignore-case")
                .action(clap::ArgAction::SetTrue)
                .help("Case insensitive search"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(clap::ArgAction::SetTrue)
                .help("Only show count of matches per file"),
        )
        .get_matches();

    let pattern_str = matches.get_one::<String>("pattern").unwrap();

    // Handle built-in shortcuts
    let actual_pattern = if matches.get_flag("structs") {
        r"struct\s+\w+"
    } else if matches.get_flag("functions") {
        r"fn\s+\w+\s*\("
    } else if matches.get_flag("traits") {
        r"trait\s+\w+"
    } else if matches.get_flag("enums") {
        r"enum\s+\w+"
    } else {
        pattern_str
    };

    let case_insensitive = matches.get_flag("case-insensitive");
    let workspace_filter = matches.get_one::<String>("workspace");
    let show_count_only = matches.get_flag("count");

    let context_lines = matches.get_one::<String>("context").map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
    let after_lines = matches.get_one::<String>("after-context").map(|s| s.parse().unwrap_or(0)).unwrap_or(0);
    let before_lines = matches.get_one::<String>("before-context").map(|s| s.parse().unwrap_or(0)).unwrap_or(0);

    let final_before = if context_lines > 0 { context_lines } else { before_lines };
    let final_after = if context_lines > 0 { context_lines } else { after_lines };

    // Compile regex
    let mut regex_builder = regex::RegexBuilder::new(actual_pattern);
    regex_builder.case_insensitive(case_insensitive);
    let regex = regex_builder.build()?;

    // Get workspace members to search
    let workspace_members = get_workspace_members()?;
    let filtered_members: Vec<_> = workspace_members
        .iter()
        .filter(|member| {
            if let Some(filter) = workspace_filter {
                &member.name == filter
            } else {
                true
            }
        })
        .collect();

    if filtered_members.is_empty() {
        eprintln!("No workspace members found matching the criteria");
        return Ok(());
    }

    let mut total_matches = 0;
    let mut total_files = 0;

    for member in &filtered_members {
        let matches = search_in_workspace_member(member, &regex)?;

        if matches.is_empty() {
            continue;
        }

        total_files += 1;

        if show_count_only {
            println!("{}: {}",
                format!("{}/", member.name).blue().bold(),
                matches.len().to_string().yellow().bold()
            );
            total_matches += matches.len();
        } else {
            display_matches(&matches, final_before, final_after)?;
            total_matches += matches.len();
        }
    }

    // Summary
    if !show_count_only && total_matches > 0 {
        println!();
    }

    if total_matches == 0 {
        println!("{}", "No matches found.".yellow());
    } else {
        println!("{}",
            format!("Found {} matches in {} files across {} workspace(s)",
                total_matches, total_files, filtered_members.len())
                .green().bold()
        );
    }

    Ok(())
}

fn search_in_workspace_member(member: &tools::WorkspaceMember, regex: &Regex) -> Result<Vec<SearchMatch>> {
    let mut matches = Vec::new();

    // Skip the tools workspace to avoid recursive searching
    if member.name == "tools" {
        return Ok(matches);
    }

    let src_path = member.path.join("src");
    if !src_path.exists() {
        return Ok(matches);
    }

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let file_path = entry.path();
        let content = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(_) => continue, // Skip files we can't read
        };

        let relative_path = file_path
            .strip_prefix(&member.path)
            .unwrap_or(file_path)
            .to_string_lossy()
            .to_string();

        for (line_no, line) in content.lines().enumerate() {
            if regex.is_match(line) {
                matches.push(SearchMatch {
                    file_path: relative_path.clone(),
                    line_number: line_no + 1,
                    line_content: line.to_string(),
                    workspace: member.name.clone(),
                });
            }
        }
    }

    Ok(matches)
}

fn display_matches(matches: &[SearchMatch], before_lines: usize, after_lines: usize) -> Result<()> {
    let mut current_file = "";

    for search_match in matches {
        // Print file header if it's a new file
        if search_match.file_path != current_file {
            if !current_file.is_empty() {
                println!(); // Separator between files
            }

            println!("{}{}",
                format!("{}/", search_match.workspace).blue().bold(),
                search_match.file_path.white().bold()
            );
            current_file = &search_match.file_path;
        }

        // For now, just show the matching line (context would require reading files again)
        if before_lines > 0 || after_lines > 0 {
            println!("  {}:{} {}",
                search_match.line_number.to_string().cyan(),
                ":".dimmed(),
                search_match.line_content.trim()
            );
        } else {
            println!("  {}:{} {}",
                search_match.line_number.to_string().cyan(),
                ":".dimmed(),
                search_match.line_content.trim()
            );
        }
    }

    Ok(())
}