use std::fs;

fn main() {
	let input =
		fs::read_to_string("./2022/1/input.txt").expect("Should have been able to read the file");
	let lines = input.split("\n");

	let mut elves = Vec::new();
	let mut current_elf = 0;
	for line in lines {
		if line.is_empty() {
			elves.push(current_elf.clone());
			current_elf = 0;
		} else {
			current_elf += line.parse::<i32>().unwrap();
		}
	}

	elves.sort();
	let one = elves.pop().unwrap();
	let two = elves.pop().unwrap();
	let three = elves.pop().unwrap();
	println!("{}", one);
	println!("{}", one + two + three);
}
