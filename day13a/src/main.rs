use std::collections::HashSet;
use std::fs;

type Dots = HashSet<(u32, u32)>;
type Instructions = Vec<(char, u32)>;

fn process_input(input: String) -> (Dots, Instructions) {
    let mut dots: Dots = HashSet::new();
    let mut instructions: Instructions = Vec::new();

    let (dots_input, ins_input) = input.split_once("\n\n").unwrap();
    //println!("dots_input is:\n{}", dots_input);
    //println!("ins_input is:\n{}", ins_input);

    // process dots
    for dot in dots_input.split('\n') {
        let (x, y) = dot.split_once(",").unwrap();
        dots.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
    }


    // process instructions
    for ins in ins_input.split('\n') {
        println!("ins is: {}", ins);
    }

    (dots, instructions)
}

fn main() {
    let input: String = fs::read_to_string("./sample_input.txt").unwrap();

    //let mut dots: HashSet<(u32, u32)>;
    //let mut instructions: Vec<(char, u32)>;
    let (dots, instructions) = process_input(input);

    println!("dots:\n{:?}", dots);
    println!("instructions:\n{:?}", instructions);
}
