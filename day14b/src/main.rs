// General approach from https://github.com/c-kk/aoc/blob/master/2021-go/day14/solve.go
use regex::Regex;
use std::collections::HashMap;
use std::fs;

type Instructions = HashMap<String, char>;
type EleCount = HashMap<char, u64>;
type CachedCount = HashMap<String, EleCount>;

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

fn merge(a: EleCount, b: EleCount) -> EleCount {
    let mut c: EleCount = a.clone();
    for (key, val) in b.iter() {
        *c.entry(*key).or_insert(0) += val;
    }
    c
}

fn init_ele_count(polymer: &str) -> EleCount{
    let mut ele_count = HashMap::new();
    for c in polymer.chars() {
        *ele_count.entry(c).or_insert(0) += 1;
    }
    ele_count
}

fn recursive_count(polymer: String, mut steps: usize, instructions: &Instructions, cache: &mut CachedCount) -> EleCount {
    if steps == 0 {
        return HashMap::<char, u64>::new();
    }

    let cache_key = format!("{}{}", polymer, steps);
    if cache.contains_key(&cache_key) {
        return cache[&cache_key].clone();
    }

    steps -= 1;
    let mut ele_count = HashMap::new();

    for index in 0..polymer.len() - 1 {
        let left: char = polymer.chars().nth(index).unwrap();
        let right: char = polymer.chars().nth(index + 1).unwrap();
        let pair: String = format!("{}{}", left, right);
        if instructions.contains_key(&pair) {
            let between: char = instructions[&pair];
            *ele_count.entry(between).or_insert(0) += 1;
            ele_count = merge(
                ele_count.clone(),
                recursive_count(format!("{}{}{}", left, between, right), steps, instructions, cache),
            );
        }
    }

    cache.insert(cache_key, ele_count.clone());
    ele_count
}

fn get_min_max(count: EleCount) -> (u64, u64) {
    let mut min: u64 = u64::MAX;
    let mut max: u64 = 0;

    for (_ele, value) in count.iter() {
        if min > *value {
            min = *value;
        }
        if max < *value {
            max = *value;
        }
    }

    (min, max)
}

fn main() {
    //let input: String = fs::read_to_string("./sample_input.txt").unwrap();
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let (polymer, instructions) = process_input(input);

    println!("polymer: {}", polymer);
    println!("instructions:\n{:?}", instructions);

    let mut r_count = init_ele_count(&polymer);
    let mut cache = HashMap::<String, HashMap::<char, u64>>::new();
    r_count = merge(r_count, recursive_count(polymer, 40, &instructions, &mut cache));

    println!();
    println!("final recursive_count is:");
    println!("{:?}", r_count);

    println!();
    let (min, max) = get_min_max(r_count);
    println!("most common element has quantity of {}, least common element has quantity of {}", min, max);
    println!("Difference between them is: {}", max-min);
}
