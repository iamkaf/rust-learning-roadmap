use anyhow::Result;
use chrono::{Local, NaiveDate};
use clap::{Arg, Command};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tools::{get_workspace_root, is_project_implemented, parse_projects};

#[derive(Serialize, Deserialize, Clone)]
struct StreakData {
    current: u32,
    best: u32,
    last_activity: NaiveDate,
}

#[derive(Serialize, Deserialize, Clone)]
struct XpData {
    total: u32,
    level: u32,
    xp_to_next_level: u32,
}

#[derive(Serialize, Deserialize, Clone)]
struct ProjectProgress {
    completed_date: NaiveDate,
    lines_of_code: u32,
    time_spent_minutes: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Stats {
    total_lines_of_code: u32,
    projects_this_week: u32,
    average_lines_per_project: f32,
    coding_velocity_trend: String,
}

#[derive(Serialize, Deserialize)]
struct ProgressData {
    streak: StreakData,
    xp: XpData,
    projects: HashMap<u32, ProjectProgress>,
    stats: Stats,
}

impl Default for ProgressData {
    fn default() -> Self {
        Self {
            streak: StreakData {
                current: 0,
                best: 0,
                last_activity: Local::now().date_naive(),
            },
            xp: XpData {
                total: 0,
                level: 1,
                xp_to_next_level: 100,
            },
            projects: HashMap::new(),
            stats: Stats {
                total_lines_of_code: 0,
                projects_this_week: 0,
                average_lines_per_project: 0.0,
                coding_velocity_trend: "starting".to_string(),
            },
        }
    }
}

fn main() -> Result<()> {
    let matches = Command::new("progress-tracker")
        .about("🦀 Rust Learning Roadmap Progress Analytics Dashboard")
        .arg(
            Arg::new("stats")
                .long("stats")
                .action(clap::ArgAction::SetTrue)
                .help("Show detailed statistics"),
        )
        .arg(
            Arg::new("ascii")
                .long("ascii")
                .action(clap::ArgAction::SetTrue)
                .help("Big ASCII celebration art"),
        )
        .get_matches();

    let projects = parse_projects()?;
    let mut progress_data = load_progress_data()?;

    // Update progress data with current state
    update_progress_data(&mut progress_data, &projects)?;

    // Save updated progress data
    save_progress_data(&progress_data)?;

    let show_stats = matches.get_flag("stats");
    let ascii_mode = matches.get_flag("ascii");

    if ascii_mode {
        show_ascii_celebration(&progress_data);
    } else if show_stats {
        show_detailed_stats(&progress_data, &projects);
    } else {
        show_main_dashboard(&progress_data, &projects)?;
    }

    Ok(())
}

fn load_progress_data() -> Result<ProgressData> {
    let root = get_workspace_root()?;
    let progress_file = root.join(".progress.json");

    if progress_file.exists() {
        let content = fs::read_to_string(progress_file)?;
        let data: ProgressData = serde_json::from_str(&content)?;
        Ok(data)
    } else {
        Ok(ProgressData::default())
    }
}

fn save_progress_data(data: &ProgressData) -> Result<()> {
    let root = get_workspace_root()?;
    let progress_file = root.join(".progress.json");
    let content = serde_json::to_string_pretty(data)?;
    fs::write(progress_file, content)?;
    Ok(())
}

fn update_progress_data(progress_data: &mut ProgressData, projects: &[tools::Project]) -> Result<()> {
    let today = Local::now().date_naive();
    let mut total_lines = 0;
    let mut _new_completions_today = 0;

    for project in projects {
        // Check if project is implemented (has a file)
        let is_implemented = is_project_implemented(project)?;

        if is_implemented && !progress_data.projects.contains_key(&project.number) {
            // New completion detected - add to progress tracking
            let lines = count_lines_of_code(project)?;
            progress_data.projects.insert(
                project.number,
                ProjectProgress {
                    completed_date: today,
                    lines_of_code: lines,
                    time_spent_minutes: None,
                },
            );
            _new_completions_today += 1;
        }

        // Count total lines for all implemented projects
        if is_implemented {
            if let Some(project_progress) = progress_data.projects.get(&project.number) {
                total_lines += project_progress.lines_of_code;
            } else {
                // Fallback: count lines even if not in progress data yet
                total_lines += count_lines_of_code(project)?;
            }
        }
    }

    // Update streak - check if ANY projects were completed today (not just newly detected ones)
    let projects_completed_today = progress_data
        .projects
        .values()
        .filter(|p| p.completed_date == today)
        .count();

    if projects_completed_today > 0 {
        // There are projects completed today - update streak
        if progress_data.streak.last_activity == today.pred_opt().unwrap_or(today) {
            // Yesterday was also active, increment streak
            progress_data.streak.current += 1;
        } else {
            // Either first day or gap in activity - start new streak
            progress_data.streak.current = 1;
        }
        progress_data.streak.last_activity = today;
        progress_data.streak.best = progress_data.streak.best.max(progress_data.streak.current);
    } else if progress_data.streak.last_activity < today.pred_opt().unwrap_or(today) {
        // No activity today and last activity was before yesterday - streak broken
        progress_data.streak.current = 0;
    }

    // Update XP and level
    let completed_projects = progress_data.projects.len() as u32;
    progress_data.xp.total = calculate_total_xp(completed_projects);
    progress_data.xp.level = calculate_level(progress_data.xp.total);
    progress_data.xp.xp_to_next_level = calculate_xp_to_next_level(progress_data.xp.level, progress_data.xp.total);

    // Update stats
    progress_data.stats.total_lines_of_code = total_lines;
    progress_data.stats.average_lines_per_project = if completed_projects > 0 {
        total_lines as f32 / completed_projects as f32
    } else {
        0.0
    };

    // Count projects this week
    let week_ago = today - chrono::Duration::days(7);
    progress_data.stats.projects_this_week = progress_data
        .projects
        .values()
        .filter(|p| p.completed_date >= week_ago)
        .count() as u32;

    Ok(())
}

fn count_lines_of_code(project: &tools::Project) -> Result<u32> {
    let workspace_member = match &project.workspace_member {
        Some(member) => member,
        None => return Ok(0),
    };

    let root = get_workspace_root()?;
    let bin_dir = root
        .join(workspace_member)
        .join("src")
        .join("bin");

    // Try to find the actual file (using same logic as is_project_implemented)
    let expected_filename = tools::get_project_filename(project.number, &project.title);
    let mut actual_file_path = bin_dir.join(&expected_filename);

    // If exact match doesn't exist, find by project number prefix
    if !actual_file_path.exists() {
        let project_prefix = format!("{:02}_", project.number);

        if bin_dir.exists() {
            for entry in fs::read_dir(&bin_dir)? {
                let entry = entry?;
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with(&project_prefix) && filename.ends_with(".rs") {
                        actual_file_path = entry.path();
                        break;
                    }
                }
            }
        }
    }

    if !actual_file_path.exists() {
        return Ok(0);
    }

    let content = fs::read_to_string(actual_file_path)?;
    let lines = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            // Count lines that are not empty, not comments, and not use statements
            !trimmed.is_empty()
                && !trimmed.starts_with("//")   // Excludes both // and ///
                && !trimmed.starts_with("use ")
        })
        .count() as u32;

    Ok(lines)
}

fn calculate_total_xp(completed_projects: u32) -> u32 {
    // Basic project: 10 XP, with bonus for milestones
    let base_xp = completed_projects * 10;
    let milestone_bonus = if completed_projects >= 150 {
        1000 // Platinum milestone bonus
    } else if completed_projects >= 100 {
        500  // Gold milestone bonus
    } else if completed_projects >= 65 {
        250  // Silver milestone bonus
    } else if completed_projects >= 30 {
        100  // Bronze milestone bonus
    } else {
        0
    };
    base_xp + milestone_bonus
}

fn calculate_level(total_xp: u32) -> u32 {
    // Level progression: 100 XP for level 1, then increases
    match total_xp {
        0..=99 => 1,
        100..=249 => 2,
        250..=499 => 3,
        500..=849 => 4,
        850..=1299 => 5,
        1300..=1849 => 6,
        1850..=2499 => 7,
        2500..=3249 => 8,
        3250..=4099 => 9,
        _ => 10,
    }
}

fn calculate_xp_to_next_level(level: u32, current_xp: u32) -> u32 {
    let next_level_xp = match level {
        1 => 100,
        2 => 250,
        3 => 500,
        4 => 850,
        5 => 1300,
        6 => 1850,
        7 => 2500,
        8 => 3250,
        9 => 4100,
        _ => current_xp, // Max level
    };
    next_level_xp.saturating_sub(current_xp)
}

fn show_main_dashboard(progress_data: &ProgressData, projects: &[tools::Project]) -> Result<()> {
    println!("{}", "🦀 RUST LEARNING ROADMAP PROGRESS 🦀".cyan().bold());
    println!("{}", "════════════════════════════════════════════════════════════".cyan());
    println!();

    // Overall progress
    let completed_count = progress_data.projects.len();
    let total_projects = 150;
    let completion_percentage = (completed_count as f32 / total_projects as f32) * 100.0;

    let progress_bar = ProgressBar::new(total_projects as u64);
    progress_bar.set_style(
        ProgressStyle::with_template("[{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
    );
    progress_bar.set_position(completed_count as u64);

    println!("{}", "🎯 OVERALL PROGRESS".yellow().bold());
    progress_bar.finish_and_clear();
    println!("[{}{}] {}/{} ({:.1}%) - {}",
        "█".repeat((completion_percentage / 5.0) as usize).green(),
        "░".repeat(20 - (completion_percentage / 5.0) as usize).dimmed(),
        completed_count,
        total_projects,
        completion_percentage,
        get_rank_title(completed_count).purple().bold()
    );
    println!();

    // Achievement status
    println!("{}", "🏆 ACHIEVEMENT STATUS".yellow().bold());
    show_achievements(completed_count);
    println!();

    // XP and Level
    println!("{}", "⚡ EXPERIENCE & LEVEL".yellow().bold());
    println!("Level: {} | XP: {} | Next Level: {} XP",
        progress_data.xp.level.to_string().green().bold(),
        progress_data.xp.total.to_string().cyan(),
        progress_data.xp.xp_to_next_level.to_string().yellow()
    );
    println!();

    // Streak and stats
    println!("{}", "🔥 STREAK & STATS".yellow().bold());
    println!("Current Streak: {} days {}",
        progress_data.streak.current.to_string().green().bold(),
        if progress_data.streak.current > 0 { "🔥" } else { "" }
    );
    println!("Best Streak: {} days", progress_data.streak.best.to_string().purple().bold());
    println!("Projects This Week: {}", progress_data.stats.projects_this_week.to_string().cyan());
    println!("Lines of Code Written: {} 📝", progress_data.stats.total_lines_of_code.to_string().green().bold());
    println!("Average Lines/Project: {}", format!("{:.1}", progress_data.stats.average_lines_per_project).white());
    println!();

    // Level progress summary
    show_level_summary(progress_data, projects);

    // Always show motivation
    println!();
    show_motivation(completed_count);

    Ok(())
}

fn show_achievements(completed_count: usize) {
    let bronze_unlocked = completed_count >= 30;
    let silver_progress = (completed_count.min(65) as f32 / 65.0) * 100.0;
    let gold_progress = (completed_count.min(100) as f32 / 100.0) * 100.0;
    let platinum_progress = (completed_count.min(150) as f32 / 150.0) * 100.0;

    println!("🥉 Bronze: {} ({})",
        if bronze_unlocked { "UNLOCKED! ✨".green().bold() } else { "Locked".red() },
        "Rust Syntax Master".dimmed()
    );

    if bronze_unlocked && completed_count < 65 {
        println!("🥈 Silver: {}/{} projects ({:.1}% to Rust Ownership Master)",
            completed_count, 65, silver_progress);
    } else if completed_count >= 65 {
        println!("🥈 Silver: {} ({})", "UNLOCKED! ✨".green().bold(), "Rust Ownership Master".dimmed());
    } else {
        println!("🥈 Silver: {} (Complete 65 projects)", "Locked".red());
    }

    if completed_count >= 65 && completed_count < 100 {
        println!("🥇 Gold: {}/{} projects ({:.1}% to Complete Rust Developer)",
            completed_count, 100, gold_progress);
    } else if completed_count >= 100 {
        println!("🥇 Gold: {} ({})", "UNLOCKED! ✨".green().bold(), "Complete Rust Developer".dimmed());
    } else {
        println!("🥇 Gold: {} (Complete 100 projects)", "Locked".red());
    }

    if completed_count >= 100 && completed_count < 150 {
        println!("💎 Platinum: {}/{} projects ({:.1}% to Rust Systems Architect)",
            completed_count, 150, platinum_progress);
    } else if completed_count >= 150 {
        println!("💎 Platinum: {} ({})", "UNLOCKED! ✨".green().bold(), "Rust Systems Architect".dimmed());
    } else {
        println!("💎 Platinum: {} (The Ultimate Goal - 150 projects)", "Locked".red());
    }
}

fn show_level_summary(progress_data: &ProgressData, projects: &[tools::Project]) {
    println!("{}", "📊 LEVEL PROGRESS SUMMARY".yellow().bold());

    for level in 1..=10 {
        let (level_name, _level_range) = get_level_info(level);
        let completed_in_level = projects
            .iter()
            .filter(|p| get_project_level(p.number) == level)
            .filter(|p| progress_data.projects.contains_key(&p.number))
            .count();

        let total_in_level = projects
            .iter()
            .filter(|p| get_project_level(p.number) == level)
            .count();

        let percentage = if total_in_level > 0 {
            (completed_in_level as f32 / total_in_level as f32) * 100.0
        } else {
            0.0
        };

        let status = if completed_in_level == total_in_level && total_in_level > 0 {
            "✅".to_string()
        } else {
            format!("{}%", percentage as u32)
        };

        println!("Level {} - {}: [{}{}] {}/{} {}",
            level,
            level_name,
            "█".repeat((percentage / 5.0) as usize).green(),
            "░".repeat(20 - (percentage / 5.0) as usize).dimmed(),
            completed_in_level,
            total_in_level,
            status
        );
    }
}

fn show_detailed_stats(progress_data: &ProgressData, projects: &[tools::Project]) {
    println!("{}", "📈 DETAILED PROGRESS STATISTICS".cyan().bold());
    println!("{}", "══════════════════════════════════".cyan());
    println!();

    // More detailed stats here...
    println!("Total Projects Completed: {}/150", progress_data.projects.len());
    println!("Total Lines of Code: {}", progress_data.stats.total_lines_of_code);
    println!("Average Lines per Project: {:.1}", progress_data.stats.average_lines_per_project);

    // Show workspace breakdown
    println!();
    println!("{}", "📦 WORKSPACE BREAKDOWN".yellow().bold());
    show_workspace_breakdown(progress_data, projects);
}

fn show_workspace_breakdown(progress_data: &ProgressData, projects: &[tools::Project]) {
    let mut workspace_stats: HashMap<String, (u32, u32)> = HashMap::new();

    for project in projects {
        if let Some(workspace) = &project.workspace_member {
            let (completed, total) = workspace_stats.entry(workspace.clone()).or_insert((0, 0));
            *total += 1;
            if progress_data.projects.contains_key(&project.number) {
                *completed += 1;
            }
        }
    }

    for (workspace, (completed, total)) in workspace_stats {
        let percentage = if total > 0 { (completed as f32 / total as f32) * 100.0 } else { 0.0 };
        println!("{}: {}/{} ({:.1}%)", workspace, completed, total, percentage);
    }
}


fn show_ascii_celebration(_progress_data: &ProgressData) {
    println!("{}", r#"
    🎉 CONGRATULATIONS! 🎉

       🦀 RUST MASTER 🦀

    ░░░░░░░░░░░░░░░░░░░░░░░
    ░░░██╗░░░██╗██████╗░░░
    ░░░██║░░░██║██╔══██╗░░
    ░░░╚██╗░██╔╝██████╔╝░░
    ░░░░╚████╔╝░██╔═══╝░░░
    ░░░░░╚██╔╝░░██║░░░░░░░
    ░░░░░░╚═╝░░░╚═╝░░░░░░░
    ░░░░░░░░░░░░░░░░░░░░░░░
    "#.green());
}

fn show_motivation(completed_count: usize) {
    let quotes = vec![
        "\"Rust doesn't just prevent bugs, it prevents entire classes of bugs.\" - Keep coding! 🦀",
        "\"The borrow checker is your friend, not your enemy.\" - You're getting stronger! 💪",
        "\"Every compilation error is a learning opportunity.\" - You're leveling up! ⬆️",
        "\"Rust makes you a better systems programmer.\" - Keep pushing forward! 🚀",
        "\"Memory safety without garbage collection - you're mastering the impossible!\" ⚡",
    ];

    let encouragement = match completed_count {
        0..=10 => "🌱 Great start! Every expert was once a beginner.",
        11..=30 => "🔥 You're on fire! The basics are becoming second nature.",
        31..=65 => "⚡ Impressive progress! Ownership concepts are clicking.",
        66..=100 => "🚀 You're in the advanced zone! Nothing can stop you now.",
        101..=149 => "💎 Almost there! You're becoming a Rust systems architect!",
        _ => "🏆 LEGENDARY! You've mastered the ultimate systems language!",
    };

    println!("{}", "💭 MOTIVATION BOOST".purple().bold());
    println!("{}", quotes[completed_count % quotes.len()].italic());
    println!("{}", encouragement.bold());
}

fn get_rank_title(completed: usize) -> &'static str {
    match completed {
        0..=10 => "Rust Rookie",
        11..=30 => "Code Apprentice",
        31..=65 => "Syntax Warrior",
        66..=100 => "Memory Guardian",
        101..=149 => "Systems Architect",
        _ => "Rust Legend",
    }
}

fn get_level_info(level: u32) -> (&'static str, (u32, u32)) {
    match level {
        1 => ("First Steps", (1, 15)),
        2 => ("Basic Data Structures", (16, 30)),
        3 => ("Ownership & Borrowing", (31, 45)),
        4 => ("Structs & Enums", (46, 65)),
        5 => ("Traits & Generics", (66, 85)),
        6 => ("Error Handling", (86, 100)),
        7 => ("Collections & Algorithms", (101, 110)),
        8 => ("Concurrency & Async", (111, 125)),
        9 => ("Graphics & Desktop Apps", (126, 140)),
        10 => ("Masterpiece Projects", (141, 150)),
        _ => ("Unknown", (0, 0)),
    }
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