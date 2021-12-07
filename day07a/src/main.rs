
fn calc_fuel(crabs: &[i32], median: i32) -> i32 {
    let mut sum: i32 = 0;
    for crab in crabs {
        sum += (crab - median).abs();
    }
    sum
}

fn main() {
    //let mut initial_state: Vec<i32> = aocutils::load_input_as_strings("./sample_input.txt".to_string())[0]
    let mut initial_state: Vec<i32> = aocutils::load_input_as_strings("./input.txt".to_string())[0]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    initial_state.sort();
    println!("{:?}", initial_state);

    let median: i32 = initial_state[initial_state.len() / 2];
    println!("median is: {}", median);

    println!("fuel needed: {}", calc_fuel(&initial_state, median))

}
