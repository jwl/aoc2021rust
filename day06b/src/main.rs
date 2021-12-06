// Approach taken from https://github.com/hkennyv/aoc/blob/main/2021/day06/src/main.rs
fn simulate(fishes: &[usize], days: usize) -> u128 {
    // instead of hard simulating every fish, we take advantage of
    // the fact that we're only interested in their counts.

    // 9 possible values for a fish's internal count; ie. 0 through 8
    let mut count = [0; 9];

    // count up how many of each fish for each value of internal count
    for &fish in fishes {
        count[fish] += 1;
    }

    for _ in 0..days {
        // build the count for the next day
        // the number of fish with count 5 the previous day is now the number
        // of fish with count 4 in the current day
        // ie. current_count[4] = prev_count[5]
        count = [
            count[1],
            count[2],
            count[3],
            count[4],
            count[5],
            count[6],
            count[7] + count[0], // current_count[6] includes all the fish that reproduced
            count[8],
            count[0], // next_count[8] is the newly born fish, ie. prev_day_count[0]
        ];
    }
    count.iter().sum()
}



fn main() {
    // Day 06b
    let days = 256;
    //let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());

    let fishes: Vec<usize> = lines[0]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("Total number of fish is: {}", simulate(&fishes, days))
}
