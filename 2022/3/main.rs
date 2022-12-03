use std::fs;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn find_common_item_2(bag1: &str, bag2: &str) -> char {
	for item in bag1.chars() {
		if bag2.contains(item) {
			return item;
		}
	}
	'a'
}

fn find_common_item_3(bag1: &str, bag2: &str, bag3: &str) -> char {
	for item in bag1.chars() {
		if bag2.contains(item) && bag3.contains(item) {
			return item;
		}
	}
	'a'
}

fn main() {
	let input =
		fs::read_to_string("./2022/3/input.txt").expect("Should have been able to read the file");
	let lines = input.split("\n");

	// part 1
	let mut score = 0;
	for line in lines.clone() {
		let bag_size = line.chars().count() / 2;
		let first_bag = &line[0..bag_size];
		let second_bag = &line[bag_size..];

		let common_item = find_common_item_2(&first_bag, &second_bag);
		score += ALPHABET.find(common_item).unwrap() + 1;
	}
	println!("{}", score);

	// part 2
	let mut groups = Vec::new();
	let mut group = Vec::new();
	for (i, line) in lines.clone().enumerate() {
		match i % 3 {
			0 | 1 => group.push(line),
			2 => {
				group.push(line);

				groups.push(group);
				group = Vec::new();
			}
			_ => panic!(),
		}
	}

	score = 0;
	for group in groups {
		let common_item = find_common_item_3(group[0], group[1], group[2]);
		score += ALPHABET.find(common_item).unwrap() + 1;
	}
	println!("{}", score);
}
