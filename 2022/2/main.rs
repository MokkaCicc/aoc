use std::fs;

// Rock: A X
// Paper: B Y
// Scissors: C Z

const LOSE: [&str; 3] = ["A Z", "B X", "C Y"];
const DRAW: [&str; 3] = ["A X", "B Y", "C Z"];
const WIN: [&str; 3] = ["A Y", "B Z", "C X"];

fn part2_score(patterns: [&str; 3], his_letter: char) -> i32 {
	for pattern in patterns {
		if his_letter == pattern.chars().nth(0).unwrap() {
			return match pattern.chars().nth(2).unwrap() {
				'X' => 1,
				'Y' => 2,
				'Z' => 3,
				_ => 0,
			};
		}
	}
	return 0;
}

fn main() {
	let input =
		fs::read_to_string("./2022/2/input.txt").expect("Should have been able to read the file");
	let lines = input.split("\n");

	// part 1
	let mut score = 0;
	for line in lines.clone() {
		let my_letter = line.chars().nth(2).unwrap();

		score += match my_letter {
			'X' => 1,
			'Y' => 2,
			'Z' => 3,
			_ => 0,
		};

		if LOSE.contains(&line) {
			score += 0;
		} else if DRAW.contains(&line) {
			score += 3;
		} else if WIN.contains(&line) {
			score += 6;
		}
	}
	println!("{}", score);

	// part 2
	score = 0;
	for line in lines.clone() {
		let my_letter = line.chars().nth(2).unwrap();
		let his_letter = line.chars().nth(0).unwrap();

		if my_letter == 'X' {
			score += 0;
			score += part2_score(LOSE, his_letter);
		} else if my_letter == 'Y' {
			score += 3;
			score += part2_score(DRAW, his_letter);
		} else if my_letter == 'Z' {
			score += 6;
			score += part2_score(WIN, his_letter);
		}
	}
	println!("{}", score);
}
