# aoc2021rust


Modeled after https://github.com/timvisee/advent-of-code-2020 and https://github.com/timvisee/advent-of-code-2021

## Run solutions

Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo run --release

# or run everything in parallel
cd ../runner
cargo run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo run --release --bin bench
```
