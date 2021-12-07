
fn calc_fuel(crabs: &[i32], target_pos: i32) -> i32 {
    let difference_array: Vec<i32>;
    let mut sum: i32 = 0;
    for crab in crabs {
        let difference: i32 = crab - target_pos;
        let abs_diff: i32 = difference.abs();
        let fuel: i32 = (0..=abs_diff).collect::<Vec<i32>>().iter().sum();
        sum += fuel;
        //println!("for crab {}, fuel is {}", crab, fuel);
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

    let mean: i32 = initial_state.iter().sum::<i32>() / (initial_state.len() as i32);
    let meanx: i32 = mean + 1;
    let meany: i32 = mean - 1;
    println!("mean is: {}", mean);
    println!("meanx is: {}", meanx);
    println!("meany is: {}", meany);

    println!("mean, fuel needed: {}", calc_fuel(&initial_state, mean));
    println!("meanx, fuel needed: {}", calc_fuel(&initial_state, meanx));
    println!("meany, fuel needed: {}", calc_fuel(&initial_state, meany));

}

