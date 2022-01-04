// heavily cribbing from https://github.com/McSick/AdventOfCode2021/blob/main/12/tree-pathfind/src/main.rs
// and https://github.com/AndrewTweddle/CodingExercises/blob/master/AdventOfCode/aoc2021/src/bin/day12_problem1.rs
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

const MAX: usize = 25;

pub struct Graph {
    // index when adding a new cave is always caves.len()
    caves: HashMap<String, usize>,
    paths: [[i32; MAX]; MAX], // -1 is a path from a small cave, 1 is a path from a large cave, 0 means no path exists
}

impl Graph {
    fn new() -> Graph {
        Graph {
            //size: 0,
            caves: HashMap::<String, usize>::new(),
            paths: [[0; MAX]; MAX],
        }
    }

    fn add_cave(&mut self, new_cave: &str) {
        let new_index: usize = self.caves.len();
        self.caves.entry(new_cave.to_string()).or_insert(new_index);
    }

    // returns index of a given cave name, if it exists
    fn get_cave(&self, cave: &str) -> usize {
        let found = self.caves.get(&cave.to_string());
        match found {
            Some(index) => *index,
            None => panic!("No cave found!"),
        }
    }

    fn add_path(&mut self, from: &str, to: &str) {
        let from_index = self.get_cave(from);
        let to_index = self.get_cave(to);

        let mut from_larger = 1;
        let mut to_larger = 1;

        // -1 for paths from a small cave
        if from.chars().next().unwrap().is_lowercase() {
            from_larger = -1;
        }
        if to.chars().next().unwrap().is_lowercase() {
            to_larger = -1;
        }

        if to == "start" || from == "end" {
            self.paths[to_index][from_index] = from_larger;
        } else {
            self.paths[from_index][to_index] = to_larger;
            self.paths[to_index][from_index] = from_larger;
        }
    }

    fn get_path(&self, from: usize, to: usize) -> i32 {
        self.paths[from][to]
    }

    fn traverse(&mut self, from: usize, mut visited: HashSet<usize>, visited_twice: bool) -> i32 {
        visited.insert(from);
        let paths = self.paths[from];
        let mut count = 0;
        for to in 0..paths.len() {
            let is_valid_path = self.get_path(from, to);

            if is_valid_path == 1 || is_valid_path == -1 {
                if self.get_cave("start") == to {
                    // skip since there's no point in going back to start again
                    continue;
                } else if self.get_cave("end") == to {
                    // we've reached the end! end recursion and return 1
                    count += 1;
                } else if is_valid_path > 0 || !visited.contains(&to) {
                    // moving to a large cave OR moving to a cave we haven't visited before
                    count += self.traverse(to, visited.clone(), visited_twice);
                } else if visited.contains(&to) && !visited_twice {
                    // if we're visiting a small cave that we haven't visited twice before
                    count += self.traverse(to, visited.clone(), true);
                }
            }
        }

        count
    }
}

fn main() {
    let mut graph = Graph::new();

    //let input = fs::read_to_string("./sample_input.txt").unwrap();
    //let input = fs::read_to_string("./sample_input2.txt").unwrap();
    //let input = fs::read_to_string("./sample_input3.txt").unwrap();
    let input = fs::read_to_string("./input.txt").unwrap();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();

        graph.add_cave(from);
        graph.add_cave(to);
        graph.add_path(from, to);
    }

    let result = graph.traverse(graph.get_cave("start"), HashSet::new(), false);
    println!("Total number of valid paths is: {}", result);

}
