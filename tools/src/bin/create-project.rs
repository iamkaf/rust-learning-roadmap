use anyhow::Result;
use colored::*;
use dialoguer::{Select, Input, Confirm};
use std::fs;
use tools::{get_project_filename, is_project_implemented, parse_projects, get_workspace_root};

fn main() -> Result<()> {
    println!("{}", "🎯 Rust Learning Roadmap - Project Creator".cyan().bold());
    println!();

    let projects = parse_projects()?;

    // Find next uncompleted project
    let next_project = projects
        .iter()
        .find(|p| !p.completed && !is_project_implemented(p).unwrap_or(false));

    if let Some(next) = next_project {
        println!("Next recommended: {} - {}",
            format!("Project {}", next.number).yellow().bold(),
            next.title.white()
        );
        println!();
    }

    let options = vec![
        "Create next project (recommended)",
        "Choose specific project number",
        "Quit"
    ];

    let selection = Select::new()
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => {
            // Create next project
            if let Some(project) = next_project {
                create_project(project)?;
            } else {
                println!("{}", "🎉 All projects are completed!".green().bold());
            }
        }
        1 => {
            // Choose specific project
            let input: String = Input::new()
                .with_prompt("Enter project number (1-150)")
                .validate_with(|input: &String| -> Result<(), String> {
                    match input.parse::<u32>() {
                        Ok(n) if n >= 1 && n <= 150 => Ok(()),
                        Ok(_) => Err("Project number must be between 1 and 150".to_string()),
                        Err(_) => Err("Please enter a valid number".to_string()),
                    }
                })
                .interact_text()?;

            let project_number: u32 = input.parse().unwrap(); // Safe because of validation

            if let Some(project) = projects.iter().find(|p| p.number == project_number) {
                println!();
                println!("Selected: {} - {}",
                    format!("Project {}", project.number).yellow().bold(),
                    project.title.white()
                );
                println!("Description: {}", project.description.dimmed());

                if is_project_implemented(project)? {
                    println!("{}", "⚠️  This project already exists!".yellow());
                    if !Confirm::new()
                        .with_prompt("Overwrite existing file?")
                        .default(false)
                        .interact()? {
                        println!("Cancelled.");
                        return Ok(());
                    }
                }

                println!();
                if Confirm::new()
                    .with_prompt("Create this project?")
                    .default(true)
                    .interact()? {
                    create_project(project)?;
                } else {
                    println!("Cancelled.");
                }
            } else {
                println!("{}", format!("Project {} not found", project_number).red());
            }
        }
        2 => {
            println!("Goodbye! 👋");
            return Ok(());
        }
        _ => unreachable!()
    }

    Ok(())
}

fn create_project(project: &tools::Project) -> Result<()> {
    let workspace_member = match &project.workspace_member {
        Some(member) => member,
        None => {
            println!("{}", "❌ Cannot determine workspace for this project".red());
            return Ok(());
        }
    };

    let root = get_workspace_root()?;
    let filename = get_project_filename(project.number, &project.title);
    let file_path = root
        .join(workspace_member)
        .join("src")
        .join("bin")
        .join(&filename);

    // Ensure directory exists
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Generate starter template
    let template = generate_template(project);

    fs::write(&file_path, template)?;

    println!();
    println!("{}", "✨ Project created successfully!".green().bold());
    println!("{}: {}", "File".cyan(), file_path.display().to_string().white());
    println!("{}: {}", "Run".cyan(),
        format!("cargo run --bin {} -p {}",
            filename.strip_suffix(".rs").unwrap(),
            workspace_member
        ).white()
    );
    println!();
    println!("{}", "Happy coding! 🦀".green());

    Ok(())
}

fn generate_template(project: &tools::Project) -> String {
    let level = get_project_level(project.number);
    let level_name = get_level_name(level);

    format!(
        "/// Project {}: {}\n/// Level {}: {}\n/// {}\n\nfn main() {{\n    // TODO: Implement the project logic here\n    // Description: {}\n    \n    println!(\"{}\");\n}}\n",
        project.number,
        project.title,
        level,
        level_name,
        project.description,
        project.description,
        project.title
    )
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