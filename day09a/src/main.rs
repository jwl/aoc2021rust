use std::collections::HashSet;

struct Grid {
    values: Vec<usize>, // value at x and y found at values[x + y*width]
    width: usize,
    height: usize,
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<usize> {
        let in_bounds = x < self.width && y < self.height;
        in_bounds.then(|| self.values[(x + y * self.width) as usize])
    }
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

fn get_low_points(grid: &Grid) -> HashSet<(usize, usize)> {
    (0..grid.width)
        .flat_map(|x| {
            (0..grid.height).filter_map(move |y| {
                let center = grid.get(x, y).unwrap();
                let adjacent_pts = [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)];

                let is_low_point = adjacent_pts
                    .iter()
                    .filter_map(|(xp, yp)| grid.get(*xp, *yp))
                    .all(|x| center < x);

                //is_low_point.then_some((x, y)) // this is a nightly-only function
                is_low_point.then(|| (x, y)) // this is a stable branch alternative
            })
        })
        .collect()
}

fn main() {
    println!("Day09a");

    let grid = build_grid(&aocutils::read_file_to_string(
        //"./sample_input.txt".to_string(),
        "./input.txt".to_string(),
    ));

    let low_points = get_low_points(&grid);

    // sum up all low points and add 1 to each low point
    let risk_sum: usize = low_points
        .iter()
        .map(|(x, y)| grid.get(*x, *y).unwrap() as usize + 1)
        .sum();

    println!("risk_sum is: {}", risk_sum);
}
