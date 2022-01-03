use colored::Colorize;

struct Snapshot {
    energy: [usize; 100],
    flashed: [bool; 100],
}

impl Snapshot {
    fn get_energy(&self, x: usize, y: usize) -> usize {
        self.energy[(x + y * 10) as usize]
    }

    fn has_flashed(&self, x: usize, y: usize) -> bool {
        self.flashed[(x + y * 10) as usize]
    }

    fn print(&self) {
        println!("printing snapshot!");
        for y in 0..9 {
            for x in 0..9 {
                //println!("x is {}, y is {}", x, y);
                print!("{}", self.energy[(x + y * 10) as usize]);
            }
            println!("");
        }
    }
}

fn build_snapshot(input: &str) -> Snapshot {
    //let energy: [usize; 100] = input
    //.lines()
    //.flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as usize))
    //.collect();
    println!("input is {}", input);
    let energy: [usize; 100] = [3; 100];
    let flashed: [bool; 100] = [false; 100];

    Snapshot { energy, flashed }
}

fn print_snapshot(input: &Vec<usize>) {
    for y in 0..9 {
        for x in 0..9 {
            let value: usize = input[(x + y * 10) as usize];
            if value == 0 {
                print!("{} ", value.to_string().red().bold());
            } else {
                print!("{} ", value);
            }
        }
        println!();
    }
}

fn increase_energy(mut input: Vec<usize>, x: usize, y: usize) -> Vec<usize> {
    input[(x + y * 10)] += 1;

    if x - 1 >= 0 {
        input[((x - 1) + y * 10)] += 1;
    }
    if x + 1 <= 9 {
        input[((x + 1) + y * 10)] += 1;
    }
    if y - 1 >= 0 {
        input[(x + (y - 1) * 10)] += 1;
    }
    if y + 1 <= 9 {
        input[(x + (y + 1) * 10)] += 1;
    }
    if x - 1 >= 0 && y - 1 >= 0 {
        input[((x - 1) + (y - 1) * 10)] += 1;
    }
    if x + 1 <= 9 && y - 1 >= 0 {
        input[((x + 1) + (y - 1) * 10)] += 1;
    }
    if x - 1 >= 0 && y + 1 <= 9 {
        input[((x - 1) + (y + 1) * 10)] += 1;
    }
    if x + 1 <= 9 && y + 1 <= 9 {
        input[((x + 1) + (y + 1) * 10)] += 1;
    }

    input
}

// returns the snapshot after the next step and the number of flashes
//fn process_step(t0: &Vec<usize>) -> Vec<usize> {
fn process_step(t0: &[usize]) -> Vec<usize> {
    //let mut t1: Vec<usize> = t0.clone();
    let mut t1: Vec<usize> = t0.to_owned();

    // First, the energy level of each octopus increases by 1.
    t1.iter_mut().for_each(|x| *x += 1);

    // Then, any octopus with an energy level greater than 9 flashes.
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.
    let mut flashes: Vec<(usize, usize)> = Vec::new();
    for y in 0..9 {
        for x in 0..9 {
            if t1[(x + y * 10) as usize] > 9 {
                flashes.push((x, y));
            }
        }
    }
    println!("{:?}", flashes);
    //while flashes.len() > 0 {
        //for flash in flashes {

        //}
    //}

    // If this causes an octopus to have an energy level greater than 9, it also flashes.

    // This process continues as long as new octopuses keep having their energy level increased beyond 9.

    // Finally, any octopus that flashed during this step has its energy level set to 0, as it used all of its energy to flash.
    for value in &mut t1 {
        if *value > 9 {
            *value = 0;
        }
    }
    t1
}

fn main() {
    let t0: Vec<usize> = aocutils::read_file_to_string("./sample_input.txt".to_string())
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect();
    println!("Initial state:");
    print_snapshot(&t0);
    println!();

    let mut tnext: Vec<usize> = t0.clone();
    let mut flashes: usize;
    for step in 1..3 {
        tnext = process_step(&tnext);
        flashes = tnext.iter().filter(|&n| *n == 0).count();
        println!(
            "For step {}, flashes was {} and resulting snapshot is:",
            step, flashes
        );
        print_snapshot(&tnext);
        println!();
    }
}
