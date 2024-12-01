extern crate aoc_function_registry;
extern crate aoc_macros;
extern crate aoc_module_loader;

use aoc_function_registry::get_registry;
use std::{env, io};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use chrono::{Datelike, Utc};

// Dynamically load the solvers.
aoc_module_loader::include_year_modules!("2023");
aoc_module_loader::include_year_modules!("2024");

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("cargo run list");
        eprintln!("cargo run create <year> <day>");
        eprintln!("cargo run solve <year> <day> [part] [alternative implementation]");
        eprintln!("Examples:");
        eprintln!("cargo run create 2024 1 - to create the file for day 1 in 2024");
        eprintln!("cargo run solve 2024 1 - to run both parts of day 1");
        eprintln!("cargo run solve 2024 1 1 for_loop - to run the for_loop implementation of part 1 of day 1");
        return;
    }

    match args[1].as_str() {
        "list" => list(),
        "create" => {
            if args.len() != 4 {
                eprintln!("Usage: cargo run create <year> <day>");
                return;
            }

            if let Err(e) = create(&args[2], &args[3]) {
                eprintln!("Error creating file: {}", e);
            }
        }
        "solve" => {
            if args.len() < 4 {
                eprintln!("Usage: cargo run solve <year> <day> [part] [alternative implementation]");
                return;
            }

            run(&args[2..]);
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}

fn list() {
    println!("Available functions:");
    get_registry().lock().unwrap().keys().for_each(|r| {
        println!("{}", r);
    });
}

fn create(year: &str, day: &str) -> io::Result<()> {
    let day_number: u32 = day.parse().unwrap();
    let day_formatted: String = format!("day{:02}", day_number);

    let now = Utc::now();
    let current_year: i32 = if now.month() == 12 {
        now.year()
    } else {
        now.year() - 1
    };

    if !(2015..=current_year).contains(&year.parse::<i32>().unwrap()) {
        eprintln!("Year must be between 2015 and {}.", current_year);
        return Ok(());
    }

    if !(1..=25).contains(&day_number) {
        eprintln!("Day must be between 1 and 25.");
        return Ok(());
    }

    let template_path = "./templates/day.rs.tpl";
    let mut template_file = File::open(template_path)?;
    let mut template_content = String::new();
    template_file.read_to_string(&mut template_content)?;

    let content = template_content.replace("{YEAR}", year).replace("{DAY}", &day_formatted);

    let file_path = format!("./src/{}/{}.rs", year, day_formatted);
    let path = Path::new(&file_path);
    if path.exists() {
        eprintln!("File {} already exists.", file_path);
        return Ok(());
    }

    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    println!("Created file: {}", file_path);

    Ok(())
}

fn run(args: &[String]) {
    let year = &args[0];
    let day = format!("day{:02}", args[1].parse::<u32>().unwrap());
    let part = if args.len() > 2 {
        format!("part{}", args[2])
    } else {
        String::new()
    };
    let alternative = if args.len() == 4 {
        format!("_{}", args[3])
    } else {
        String::new()
    };

    let registry = get_registry().lock().unwrap();

    if part.is_empty() {
        // Try to run both part1 and part2 if no specific part is provided. It can become quite
        // expensive to loop over all keys in the registry if many years are present. As such,
        // we do not account for any alternative versions of each part.
        let part1_key: String = format!("{}_{}_part1", year, day);
        let part2_key: String = format!("{}_{}_part2", year, day);

        if let Some(function) = registry.get(&part1_key) {
            let result: String = function();
            println!("Result for part1 of {} ({}): {}", day, year, result);
        } else {
            println!("No implementation for part1 of {} ({}) exists...", day, year);
        }

        if let Some(function) = registry.get(&part2_key) {
            let result: String = function();
            println!("Result for part2 of {} ({}): {}", day, year, result);
        } else {
            println!("No implementation for part1 of {} ({}) exists...", day, year);
        }
    } else {
        let key = format!("{}_{}_{}{}", year, day, part, alternative);

        if let Some(function) = registry.get(&key) {
            let result = function();

            println!("Result for {}{} of {} ({}): {}", part, alternative, day, year, result);
        } else {
            println!("No implementation for {}{} of {} ({}) exists...", part, alternative, day, year);
        }
    }
}