use std::fs;

fn main() {
	// This unwrap is deliberate.
	let content = fs::read_to_string("./`workspace.name`/src/input.txt").unwrap();
	println!("{}", content);
}
