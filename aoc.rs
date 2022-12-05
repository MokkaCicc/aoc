use std::fs;
use std::ops::{Range, RangeFrom};
use std::path::Path;
use std::process::Command;

use chrono::prelude::*;
use clap::Parser;

const YEAR_RANGE: RangeFrom<i32> = 2015..;
const DAY_RANGE: Range<u32> = 1..25;

#[derive(Parser)]
struct Args {
	#[arg(short, long, default_value_t = Utc::now().year(), value_parser = year_in_range)]
	year: i32,

	#[arg(short, long, default_value_t = Utc::now().day(), value_parser = day_in_range)]
	day: u32,
}

fn year_in_range(s: &str) -> Result<i32, String> {
	let year: i32 = s
		.parse()
		.map_err(|_| format!("`{}` isn't a year number", s))?;
	if YEAR_RANGE.contains(&year) {
		Ok(year)
	} else {
		Err("AOC only began in 2015".to_string())
	}
}

fn day_in_range(s: &str) -> Result<u32, String> {
	let day: u32 = s
		.parse()
		.map_err(|_| format!("`{}` isn't a day number", s))?;
	if DAY_RANGE.contains(&day) {
		Ok(day)
	} else {
		Err("AOC is only between the first day of December and the 25th".to_string())
	}
}

fn is_argument_overrided(args: &Args) -> bool {
	!(args.year == Utc::now().year() && args.day == Utc::now().day())
}

fn create_today_script(path: &Path) {
	let dir = path.parent().unwrap();
	fs::create_dir_all(dir).unwrap();
	match fs::File::create(path) {
		Err(_) => println!("Unable to create the file {}", path.display()),
		Ok(_) => println!("File {} created", path.display()),
	}
}

fn run_today_script(path: &Path) {
	let output = if cfg!(target_os = "windows") {
		Command::new("cmd")
			.args([
				"/C",
				&format!("rustfmt {}", path.display()),
				&format!("&& rustc {}", path.display()),
				"&& main.exe",
				"&& del main.exe",
			])
			.output()
			.expect("failed to execute process")
	} else {
		Command::new("sh")
			.args([
				"-c",
				&format!("rustfmt {}", path.display()),
				&format!("&& rustc {}", path.display()),
				"&& ./main",
				"&& rm ./main",
			])
			.output()
			.expect("failed to execute process")
	};

	println!("{}", String::from_utf8(output.stdout).unwrap());
	println!("{}", String::from_utf8(output.stderr).unwrap());
}

fn main() {
	let args = Args::parse();
	let argument_overrided = is_argument_overrided(&args);
	// If we run this script the 12 january 2023, it will creates the folder `./2023/12/`.
	// This is not very logical.
	if !argument_overrided {
		println!("No arguments provided, using today date");
	}
	let script_path_string = &format!("./{}/{}/main.rs", args.year, args.day);
	let input_path_string = &format!("./{}/{}/input.txt", args.year, args.day);
	let script_path = Path::new(script_path_string);
	let input_path = Path::new(input_path_string);
	if script_path.exists() {
		println!("Script `{}` found, running it...", script_path_string);
		println!();
		run_today_script(&script_path);
	} else {
		println!("Script not found, creating files...");
		create_today_script(&script_path);
		create_today_script(&input_path);
	}
}
