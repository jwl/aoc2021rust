use regex::Regex;
use std::collections::HashSet;
use std::fs;

type Dots = HashSet<(u32, u32)>;
type Instructions = Vec<(char, u32)>;

fn process_input(input: String) -> (Dots, Instructions) {
    let mut dots: Dots = HashSet::new();
    let mut instructions: Instructions = Vec::new();

    let (dots_input, ins_input) = input.split_once("\n\n").unwrap();

    // process dots
    for dot in dots_input.split('\n') {
        let (x, y) = dot.split_once(",").unwrap();
        dots.insert((x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()));
    }

    // process instructions
    for ins in ins_input.split('\n') {
        if ins.is_empty() {
            continue;
        }
        let ins_regex: Regex = Regex::new(r"^fold along ([xy])=(\d+)").unwrap();
        let caps = ins_regex.captures(ins).unwrap();
        let dimension: char = caps
            .get(1)
            .map_or("", |m| m.as_str())
            .chars()
            .next()
            .unwrap();
        let value: u32 = u32::from_str_radix(caps.get(2).map_or("", |m| m.as_str()), 10).unwrap();

        instructions.push((dimension, value))
    }

    (dots, instructions)
}

fn fold(dots: Dots, direction: char, fvalue: u32) -> Dots {
    let mut folded_dots: Dots = HashSet::new();
    for dot in dots {
        if direction == 'x' {
            // fold left
            if dot.0 > fvalue {
                folded_dots.insert((2 * fvalue - dot.0, dot.1));
            } else {
                folded_dots.insert(dot);
            }
        } else {
            // fold up
            if dot.1 > fvalue {
                folded_dots.insert((dot.0, 2*fvalue-dot.1));
            } else {
                folded_dots.insert(dot);
            }
        }
    }

    folded_dots
}

fn visualize(dots: Dots) {
    let mut max_width: u32 = 0;
    let mut max_height: u32 = 0;

    for dot in &dots {
        if dot.0 > max_width {
            max_width = dot.0;
        }
        if dot.1 > max_height {
            max_height = dot.1;
        }
    }
    max_width += 1;
    max_height += 1;

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_width as usize]; max_height as usize];

    for dot in &dots {
        let x: usize = dot.0 as usize;
        let y: usize = dot.1 as usize;
        grid[y][x] = '#';
    }

    for y in 0..max_height {
        for x in 0..max_width {
            print!("{}", grid[y as usize][x as usize]);
        }
        println!();
    }
}

fn main() {
    let input: String = fs::read_to_string("./sample_input.txt").unwrap();
    let (mut dots, instructions) = process_input(input);

    //println!("dots:\n{:?}", dots);
    //println!("instructions:\n{:?}", instructions);

    dots = fold(dots, instructions[0].0, instructions[0].1);

    println!("dots after fold:\n{:?}", dots);
    println!("number of dots after fold is: {}", dots.len());
    println!();

    visualize(dots);
}
