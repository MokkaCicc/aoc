# Advent of Code

This repository hosts all my [advent of code](https://adventofcode.com) rust files. I am still learning rust, so don't expect beautiful or oprimized code here.

I manage all my script with another script called `aoc`.
You can build it with cargo :
```bash
$ cargo build --release
```

The `./target/release/aoc.exe` file will be created, move it to the root of a cargo project.

When running this script, it will try to run the file `./aoc-[year]-[day]/src/main.rs`. If the file does not exist, it will create it within a new cargo workspace.

By default, the script uses the current `year` and `day`, but it can be overrided with flags.
```bash
$ aoc --year 2020 --day 12
```
