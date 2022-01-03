// heavily borrowed from https://github.com/CJX3M/AdventOfCode/blob/master/2021/day11.py
use colored::Colorize;

const STEPS: usize = 100;

fn print_snapshot(input: &Vec<i32>) {
    for y in 0..10 {
        for x in 0..10 {
            let value: i32 = input[(x + y * 10) as usize];
            if value == 0 {
                print!("{} ", value.to_string().red().bold());
            } else {
                print!("{} ", value);
            }
        }
        println!();
    }
}

fn increase_energy(input: &mut Vec<i32>, x: usize, y: usize) -> &mut Vec<i32> {
    if x >= 1 {
        input[((x - 1) + y * 10)] += 1;
    }
    if x + 1 <= 9 {
        input[((x + 1) + y * 10)] += 1;
    }
    if y >= 1 {
        input[(x + (y - 1) * 10)] += 1;
    }
    if y + 1 <= 9 {
        input[(x + (y + 1) * 10)] += 1;
    }
    if x >= 1 && y >= 1 {
        input[((x - 1) + (y - 1) * 10)] += 1;
    }
    if x + 1 <= 9 && y >= 1 {
        input[((x + 1) + (y - 1) * 10)] += 1;
    }
    if x >= 1 && y + 1 <= 9 {
        input[((x - 1) + (y + 1) * 10)] += 1;
    }
    if x + 1 <= 9 && y + 1 <= 9 {
        input[((x + 1) + (y + 1) * 10)] += 1;
    }

    input
}

// transforms octopi and returns number of flashes that occurred in this step
fn process_step(mut octopi: &mut Vec<i32>) -> usize {
    let mut total_flashes = 0;

    // First, the energy level of each octopus increases by 1.
    octopi.iter_mut().for_each(|x| *x += 1);

    // Then, any octopus with an energy level greater than 9 flashes.
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.
    // If this causes an octopus to have an energy level greater than 9, it also flashes.
    loop {
        let mut flashes = 0;

        for y in 0..10 { for x in 0..10 {
            if octopi[(x+y*10)] > 9 {
                octopi = increase_energy(octopi, x, y);
                octopi[(x+y*10)] = i32::MIN; // ensure that this position only flashes once
                flashes += 1;
            }
        }}
        total_flashes += flashes;
        if flashes == 0 {break}
    }

    // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    for value in octopi {
        if *value > 9 || *value < 0 {
            *value = 0;
        }
    }
    total_flashes
}

fn main() {
    //let t0: Vec<i32> = aocutils::read_file_to_string("./sample_input.txt".to_string())
    let mut octopi: Vec<i32> = aocutils::read_file_to_string("./input.txt".to_string())
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect();
    println!("Initial state:");
    print_snapshot(&octopi);
    println!();

    let mut total_flashes: usize = 0;
    for _step in 1..(STEPS+1) {
        let flashes: usize = process_step(&mut octopi);
        total_flashes += flashes;
    }
    println!("After total {} steps, there were total flashes of {}", STEPS, total_flashes);
}
