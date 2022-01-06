use std::fs;

struct Grid {
    values: Vec<usize>, // value at x and y found at values[x + y*width]
    width: usize,
    height: usize,
}

fn build_grid(input: &String) -> Grid {
    let width = input.lines().next().unwrap().chars().count() as usize;
    let height = input.lines().count() as usize;

    let values: Vec<usize> = input
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect();

    Grid {
        values,
        width,
        height,
    }
}

fn get_est_dist(x: usize, y: usize) -> i32 {
    -1
}

fn get_smallest_risk(grid: Grid, start: (usize, usize), end: (usize, usize)) -> i32 {
    -1
}

fn main() {
    //let input: String = fs::read_to_string("./sample_input.txt").unwrap();
    let input: String = fs::read_to_string("./input.txt").unwrap();

    let grid = build_grid(&aocutils::read_file_to_string(
        "./sample_input.txt".to_string(),
        //"./input.txt".to_string(),
    ));

    //let cave: Vec<i32> = input
        //.lines()
        //.flat_map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32))
        //.collect();

    let shortest_path: i32 = get_smallest_risk(grid, (0,0));
}
