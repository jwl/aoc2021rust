use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct Grid {
    values: Vec<u32>, // value at x and y found at values[x + y*width]
    width: usize,
    height: usize,
}

struct MinHeap {
    max_heap: BinaryHeap<(Reverse<u32>, (usize, usize))>,
}
impl MinHeap {
    fn new() -> MinHeap {
        let max_heap = BinaryHeap::new();
        MinHeap { max_heap }
    }
    fn push(&mut self, pos: (usize, usize), dist: u32) {
        self.max_heap.push((Reverse(dist), pos));
    }
    fn pop(&mut self) -> Option<((usize, usize), u32)> {
        let result = self.max_heap.pop();
        match result {
            Some((Reverse(dist), pos)) => Some((pos, dist)),
            None => None,
        }
    }

    fn is_empty(&self) -> bool {
        self.max_heap.is_empty()
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u32> {
        let in_bounds = x < self.width && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }

    fn is_valid_pos(&self, p: (usize, usize)) -> bool {
        p.0 < self.width && p.1 < self.height
    }

    fn enlarge_grid(&mut self) {
        let new_width: usize = self.width * 5;
        let new_height: usize = self.height * 5;
        let mut new_values: Vec<u32> = Vec::new();

        for y in 0..new_height {
            for x in 0..new_width {
                new_values.insert(x+y * new_width, get_adjusted_risk(&self, (x, y)));
            }
        }
        self.width = new_width;
        self.height = new_height;
        self.values = new_values;
    }

    fn print_grid(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(x, y).unwrap());
            }
            println!();
        }
    }
}

// returns risk of this position, given pos on the big grid, based on small grid
fn get_adjusted_risk(grid: &Grid, pos: (usize, usize)) -> u32 {
    let small_x: usize = pos.0 % grid.width;
    let x_chunk: u32 = pos.0 as u32 / grid.width as u32;
    let small_y: usize = pos.1 % grid.height;
    let y_chunk: u32 = pos.1 as u32 / grid.height as u32;
    (grid.get(small_x, small_y).unwrap() - 1 + x_chunk + y_chunk) % 9 + 1
}

fn build_init_grid(input: &String) -> Grid {
    let width = input.lines().next().unwrap().chars().count() as usize;
    let height = input.lines().count() as usize;

    let values: Vec<u32> = input
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as u32))
        .collect();

    Grid {
        values,
        width,
        height,
    }
}

fn get_manhattan(pos: (usize, usize), end: (usize, usize)) -> u32 {
    (end.0 + end.1 - pos.0 - pos.1) as u32
}

fn adj_pos(pos: (usize, usize)) -> [(usize, usize); 4] {
    let (i, j) = pos;
    [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]
}

fn get_smallest(grid: &Grid, start: (usize, usize), end: (usize, usize)) -> u32 {
    let mut explore_queue = MinHeap::new();
    let mut distance_map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut smallest: u32 = u32::MAX;

    distance_map.insert(start, 0);
    explore_queue.push(start, 0);

    while !explore_queue.is_empty() {
        let (current_pos, _) = explore_queue.pop().unwrap();
        let start_to_current = *distance_map.get(&current_pos).unwrap();
        //println!("current_pos is {:?}, distance from start is {:?}", current_pos, start_to_current);

        if current_pos == end {
            println!(
                "Shortest distance from start to end is: {}",
                start_to_current
            );
            smallest = start_to_current;
            break;
        }

        for (i, j) in adj_pos(current_pos) {
            if grid.is_valid_pos((i, j)) {
                let start_to_next = start_to_current + grid.get(i, j).unwrap();
                let est_dist_from_next = start_to_next + get_manhattan((i, j), end);
                match distance_map.get_mut(&(i, j)) {
                    // check cache to see if we've already calculated this
                    Some(neighbor_dist) => {
                        if start_to_next < *neighbor_dist {
                            *neighbor_dist = start_to_next;
                            explore_queue.push((i, j), est_dist_from_next);
                        }
                    }
                    None => {
                        // we haven't calculated this position yet, put it in the explore queue
                        distance_map.insert((i, j), start_to_next);
                        explore_queue.push((i, j), est_dist_from_next);
                    }
                }
            }
        }
    }
    smallest
}

fn main() {
    let mut grid = build_init_grid(&aocutils::read_file_to_string(
        //"./sample_input.txt".to_string(),
        //"./sample_input2.txt".to_string(),
        "./input.txt".to_string(),
    ));

    grid.enlarge_grid();

    let start = (0, 0);
    let end = (grid.width - 1, grid.height - 1);

    let smallest_risk = get_smallest(&grid, start, end);
    println!("smallest risk is: {}", smallest_risk);
}
