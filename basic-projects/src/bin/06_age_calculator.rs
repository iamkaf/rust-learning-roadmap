use chrono::prelude::*;
use std::io;

/// Project 6: Age Calculator
/// Level 1: First Steps
/// Calculate age in years, months, days from birthdate

fn main() {
    // TODO: Implement the project logic here
    // Description: Calculate age in years, months, days from birthdate

    println!("What's your birthday? [mm/dd/yyyy]");

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let parts: Vec<&str> = input.trim().split("/").collect();

    if parts.len() != 3 {
        println!("Please input a valid date in the [mm/dd/yyyy] format.");
        return;
    }

    let month = match parts[0].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid date in the [mm/dd/yyyy] format.");
            return;
        }
    };

    let day = match parts[1].parse::<u32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid date in the [mm/dd/yyyy] format.");
            return;
        }
    };

    let year = match parts[2].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please input a valid date in the [mm/dd/yyyy] format.");
            return;
        }
    };


    let today: DateTime<Local> = Local::now();
    let birthday: DateTime<Local> = match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => {
            let datetime = date.and_hms_opt(12, 0, 0).unwrap();
            Local.from_local_datetime(&datetime).unwrap()
        }
        None => {
            println!("Invalid date!");
            return;
        },
    };

    println!("Birthday: {}", birthday.format("%m/%d/%Y"));
    println!("Today: {}", today.format("%m/%d/%Y"));

    let mut age_years = today.years_since(birthday).expect("Birthday cannot be in the future.");
    let mut age_months = today.month0() as i32 - birthday.month0() as i32;
    let mut age_days = today.day() as i32 - birthday.day() as i32;

    // If the day calculation went negative, we need to borrow from the month.
    if age_days < 0 {
        age_months -= 1;
        // To find how many days to add, we get the number of days in the *previous* month.
        // A trick for this is to get day 0 of the current month.
        let days_in_last_month = today.date_naive().with_day(1).unwrap().pred_opt().unwrap().day();
        age_days += days_in_last_month as i32;
    }

    // If the month calculation went negative, we need to borrow from the year.
    if age_months < 0 {
        age_years -= 1;
        age_months += 12;
    }
    println!("You are {age_years} years, {age_months} months and {age_days} days old.");
}
