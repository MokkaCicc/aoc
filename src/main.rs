use std::fs;
use std::io::{Error, ErrorKind};
use std::ops::{Range, RangeFrom};
use std::process::{Command, ExitStatus};

use chrono::prelude::*;
use clap::Parser;
use toml_edit::{value, Array, Document};

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

fn run_command(command: &str) -> Result<String, Error> {
	#[allow(unused_assignments)]
	let mut shell = "";
	let mut args: Vec<&str> = Vec::new();
	if cfg!(target_os = "windows") {
		shell = "cmd";
		args.push("/C");
	} else {
		shell = "sh";
		args.push("-c");
	}
	args.push(command);

	let output = Command::new(shell).args(args).output()?;
	// unable to use `output.status.exit_ok()?;` because this is unstable feature.
	if !ExitStatus::success(&output.status) {
		let message = String::from_utf8(output.stderr).unwrap();
		return Err(Error::new(ErrorKind::Interrupted, message));
	}
	let message = String::from_utf8(output.stdout).unwrap();
	Ok(message)
}

fn read_cargo() -> Result<Document, Error> {
	let toml = fs::read_to_string("./Cargo.toml")?;
	// TODO: Proper TomlError handling.
	match toml.parse::<Document>() {
		Ok(document) => Ok(document),
		Err(err) => Err(Error::new(ErrorKind::InvalidData, err.to_string())),
	}
}

fn is_workspace_in_cargo(name: &str) -> Result<bool, Error> {
	let document = read_cargo()?;

	let workspace = match document.get("workspace") {
		Some(item) => item,
		None => return Ok(false),
	};

	let members = match workspace.get("members") {
		Some(item) => item.as_array().unwrap(),
		None => return Ok(false),
	};

	for member in members {
		if member.as_str().unwrap() == name {
			return Ok(true);
		}
	}
	Ok(false)
}

fn add_workspace_to_cargo(name: &str) -> Result<(), Error> {
	let mut document = read_cargo()?;

	let workspace = match document.get_mut("workspace") {
		// TODO: create automatically the [workspace] in the `Cargo.toml` file.
		Some(item) => Ok(item.as_table_mut().unwrap()),
		None => Err(Error::new(
			ErrorKind::InvalidData,
			"Please create the field [workspace] in the `Cargo.toml` file.",
		)),
	}?;

	if !workspace.contains_key("members") {
		workspace.insert("members", value(Array::default()));
	}

	let members = workspace
		.get_mut("members")
		.unwrap()
		.as_array_mut()
		.unwrap();

	members.push(name);
	members.fmt(); // This format the array as inline.
	fs::write("Cargo.toml", document.to_string())?;
	Ok(())
}

fn add_worspace(name: &str) -> Result<(), Error> {
	add_workspace_to_cargo(name)?;
	let output = run_command(format!("cargo new {}", name).as_str())?;
	// TODO
	// copy the content of template.main.rs to [name]/src/main.rs (replace text)
	// copy the file /src/template.input.txt to [name]/src/input.txt
	println!("{}", output);
	Ok(())
}

fn run_workspace(name: &str) -> Result<(), Error> {
	let output = run_command(format!("cargo fmt -p {}", name).as_str())?;
	println!("{}", output);
	let output = run_command(format!("cargo run -p {}", name).as_str())?;
	println!("{}", output);
	Ok(())
}

fn main() {
	let args = Args::parse();
	let argument_overrided = is_argument_overrided(&args);
	// If we run this script the 12 january 2023, it will creates the folder `./2023/12/`.
	// This is not very logical.
	if !argument_overrided {
		println!("No arguments provided, using today date");
	}

	let name = format!("aoc-{}-{}", args.year, args.day);
	if !is_workspace_in_cargo(&name).unwrap() {
		println!("Create workspace {}", name);
		match add_worspace(&name) {
			Ok(_) => (),
			Err(err) => eprintln!("{}", err),
		}
	} else {
		println!("Running workspace {}", name);
		match run_workspace(&name) {
			Ok(_) => (),
			Err(err) => eprintln!("{}", err),
		};
	}
}
