use anyhow::Result;
use clap::{Arg, Command};
use colored::*;
use tools::{get_project_filename, is_project_implemented, parse_projects};

fn main() -> Result<()> {
    let matches = Command::new("next-project")
        .about("Find the next uncompleted project in the Rust learning roadmap")
        .arg(
            Arg::new("all")
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .help("Show all uncompleted projects"),
        )
        .arg(
            Arg::new("level")
                .long("level")
                .value_name("LEVEL")
                .help("Show projects from specific level (1-10)"),
        )
        .arg(
            Arg::new("workspace")
                .long("workspace")
                .value_name("WORKSPACE")
                .help("Show projects from specific workspace member"),
        )
        .arg(
            Arg::new("implemented")
                .long("implemented")
                .action(clap::ArgAction::SetTrue)
                .help("Only show projects that have been implemented (files exist)"),
        )
        .get_matches();

    let projects = parse_projects()?;

    let level_filter = matches.get_one::<String>("level").map(|s| s.parse::<u32>());
    let workspace_filter = matches.get_one::<String>("workspace");
    let show_all = matches.get_flag("all");
    let implemented_only = matches.get_flag("implemented");

    let filtered_projects: Vec<_> = projects
        .iter()
        .filter(|project| {
            // Filter by completion status
            if implemented_only {
                is_project_implemented(project).unwrap_or(false)
            } else {
                !project.completed
            }
        })
        .filter(|project| {
            // Filter by level if specified
            if let Some(Ok(level)) = level_filter {
                get_project_level(project.number) == level
            } else {
                true
            }
        })
        .filter(|project| {
            // Filter by workspace if specified
            if let Some(workspace) = workspace_filter {
                project.workspace_member.as_ref().map_or(false, |w| w == workspace)
            } else {
                true
            }
        })
        .collect();

    if filtered_projects.is_empty() {
        if implemented_only {
            println!("{}", "🎉 No implemented projects found matching your criteria!".green().bold());
        } else {
            println!("{}", "🎉 All projects matching your criteria are completed!".green().bold());
        }
        return Ok(());
    }

    if show_all {
        println!("{}", format!("📋 {} uncompleted projects found:", filtered_projects.len()).cyan().bold());
        println!();

        for project in &filtered_projects {
            print_project_info(project, true)?;
        }
    } else {
        // Show just the next project
        let next_project = &filtered_projects[0];
        println!("{}", "🎯 Next Project:".cyan().bold());
        println!();
        print_project_info(next_project, false)?;
    }

    Ok(())
}

fn print_project_info(project: &tools::Project, compact: bool) -> Result<()> {
    let level = get_project_level(project.number);
    let level_name = get_level_name(level);

    let workspace_info = if let Some(workspace) = &project.workspace_member {
        format!(" ({})", workspace.blue())
    } else {
        " (unknown workspace)".red().to_string()
    };

    let implementation_status = if is_project_implemented(project)? {
        " ✅ Implemented".green()
    } else {
        " ❌ Not implemented".red()
    };

    println!("{}{}{}",
        format!("Project {}: {}", project.number, project.title).yellow().bold(),
        workspace_info,
        implementation_status
    );

    if !compact {
        println!("{}: {}", "Level".cyan(), format!("{} - {}", level, level_name).white());
        println!("{}: {}", "Description".cyan(), project.description.white());

        if let Some(workspace) = &project.workspace_member {
            let filename = get_project_filename(project.number, &project.title);
            println!("{}: {}", "Expected file".cyan(),
                format!("{}/src/bin/{}", workspace, filename).white());

            println!("{}: {}", "Run command".cyan(),
                format!("cargo run --bin {} -p {}",
                    filename.strip_suffix(".rs").unwrap(),
                    workspace).white());
        }
        println!();
    } else {
        println!("  {}: {}", "Description".dimmed(), project.description.white());
        println!();
    }

    Ok(())
}

fn get_project_level(project_number: u32) -> u32 {
    match project_number {
        1..=15 => 1,
        16..=30 => 2,
        31..=45 => 3,
        46..=65 => 4,
        66..=85 => 5,
        86..=100 => 6,
        101..=110 => 7,
        111..=125 => 8,
        126..=140 => 9,
        141..=150 => 10,
        _ => 0,
    }
}

fn get_level_name(level: u32) -> &'static str {
    match level {
        1 => "First Steps",
        2 => "Basic Data Structures",
        3 => "Ownership & Borrowing",
        4 => "Structs & Enums",
        5 => "Traits & Generics",
        6 => "Error Handling",
        7 => "Collections & Algorithms",
        8 => "Concurrency & Async",
        9 => "Graphics, GUI & Desktop Apps",
        10 => "Masterpiece Projects",
        _ => "Unknown Level",
    }
}