use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Instructions = HashMap<String, char>;

fn process_input(input: String) -> (String, Instructions) {
    let mut ins: Instructions = HashMap::new();

    let (start, ins_input) = input.split_once("\n\n").unwrap();

    for line in ins_input.split('\n') {
        if line.is_empty() {
            continue;
        }
        let ins_regex: Regex = Regex::new(r"^([A-Z]{2}) -> ([A-Z]{1})").unwrap();
        let caps = ins_regex.captures(line).unwrap();
        let pair: String = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let insertion: char = caps
            .get(2)
            .map_or("", |m| m.as_str())
            .chars()
            .next()
            .unwrap();

        ins.insert(pair, insertion);
    }
    (start.to_string(), ins)
}

fn execute_step(polymer: String, instructions: &Instructions) -> String {
    let mut final_polymer: String = "".to_string();
    final_polymer.push(polymer.chars().next().unwrap()); // push the first character onto final_polymer

    for (i, right_c) in polymer.chars().enumerate() {
        if i == 0 {
            continue;
        }
        let left_c: char = final_polymer.chars().rev().next().unwrap();
        let mut pair: String = String::from(left_c);
        pair.push(right_c);
        if instructions.contains_key(&pair) {
            final_polymer.push(instructions[&pair]);
        }
        final_polymer.push(right_c);
    }
    final_polymer
}

fn count_elements(polymer: String) -> HashMap<char, u32> {
    let mut cmap = HashMap::new();

    for c in polymer.chars() {
        *cmap.entry(c).or_insert(0) += 1;
    }

    cmap
}

fn main() {
    //let input: String = fs::read_to_string("./sample_input.txt").unwrap();
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let (mut polymer, instructions) = process_input(input);

    println!("polymer: {}", polymer);
    println!("instructions:\n{:?}", instructions);

    for step in 1..11 {
        print!("executing step {}...", step);
        polymer = execute_step(polymer, &instructions);
        print!("   polymer has length {}\n", polymer.len());
    }

    println!();
    println!("After all steps, polymer has length: {}", polymer.len());
    let element_count: HashMap<char, u32> = count_elements(polymer);
    println!("{:?}", element_count);
}
