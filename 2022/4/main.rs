use std::fs;

fn is_countained_by(assignement1: &Vec<&str>, assignement2: &Vec<&str>) -> bool {
	assignement1[0].parse::<i32>().unwrap() >= assignement2[0].parse::<i32>().unwrap()
		&& assignement1[1].parse::<i32>().unwrap() <= assignement2[1].parse::<i32>().unwrap()
}

fn is_overlaped_by(assignement1: &Vec<&str>, assignement2: &Vec<&str>) -> bool {
	assignement2[0].parse::<i32>().unwrap() <= assignement1[1].parse::<i32>().unwrap()
}

fn main() {
	let input =
		fs::read_to_string("./2022/4/input.txt").expect("Should have been able to read the file");
	let lines = input.split("\n");

	let mut score1 = 0;
	let mut score2 = 0;
	for line in lines {
		let assignements: Vec<&str> = line.split(",").collect();
		let assignement1: Vec<&str> = assignements[0].split("-").collect();
		let assignement2: Vec<&str> = assignements[1].split("-").collect();
		if is_countained_by(&assignement1, &assignement2)
			|| is_countained_by(&assignement2, &assignement1)
		{
			score1 += 1;
		}
		if is_overlaped_by(&assignement1, &assignement2)
			&& is_overlaped_by(&assignement2, &assignement1)
		{
			score2 += 1;
		}
	}
	println!("{}", score1);
	println!("{}", score2);
}
