// basically mostly taken from https://github.com/schubart/AdventOfCode_2021_Rust/blob/master/day08/src/lib.rs
use std::collections::HashSet;

type SignalSet = HashSet<char>;
type Mapping = [SignalSet; 10];

fn remove_only<T, F>(input: &mut Vec<T>, predicate: F) -> T
where
    T: Clone,
    F: Fn(&&T) -> bool + Copy,
{
    let mut results = input.iter().filter(predicate);
    let result = results.next().expect("No element found").clone();
    assert!(results.next().is_none(), "Multiple elements found");

    input.retain(|x| !predicate(&x));

    result
}

fn decode(input: &mut Vec<SignalSet>) -> Mapping {
    // Cases where length alone can determine number
    // 1 = 2 segments
    // 4 = 4 segments
    // 7 = 3 segments
    // 8 = 7 segments

    let n1 = remove_only(input, |x| x.len() == 2);
    let n4 = remove_only(input, |x| x.len() == 4);
    let n7 = remove_only(input, |x| x.len() == 3);
    let n8 = remove_only(input, |x| x.len() == 7);

    // 5 segment numbers; 3 is the only one that shares 2 segments with 1
    let n3 = remove_only(input, |x| x.len() == 5 && (*x & &n1).len() == 2);
    // 2 and 4 share 2 segments
    let n2 = remove_only(input, |x| x.len() == 5 && (*x & &n4).len() == 2);
    // 5 is the only remaining 5-segment number
    let n5 = remove_only(input, |x| x.len() == 5);

    // 6 segment numbers
    // 6 shares 1 segment with 1
    let n6 = remove_only(input, |x| x.len() == 6 && (*x & &n1).len() == 1);
    // 9 shares 4 segments with 4
    let n9 = remove_only(input, |x| x.len() == 6 && (*x & &n4).len() == 4);
    // 0 is the only number left
    let n0 = remove_only(input, |x| x.len() == 6);

    assert!(input.is_empty());

    [n0, n1, n2, n3, n4, n5, n6, n7, n8, n9]
}

fn apply(mapping: &Mapping, output: &[SignalSet]) -> usize {
    output.iter().fold(0, |result, x| {
        result * 10
            + mapping
                .iter()
                .enumerate()
                .find(|(_, y)| x == *y)
                .map(|(index, _)| index)
                .unwrap()
    })
}

fn day8b() -> usize {
    include_str!("../input.txt")
        .lines()
        .map(|line| {
            let mut output: Vec<SignalSet> = line.split(' ').map(|x| x.chars().collect()).collect();
            let mut input = output.drain(0..10).collect();
            output.remove(0); // Remove | separator.

            let lookup = decode(&mut input);
            apply(&lookup, &output)
        })
        .sum()
}

fn main() {
    // Day 08b

    println!("sum is: {}", day8b())
}
