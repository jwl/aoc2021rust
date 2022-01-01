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

fn get_basin_size(grid: &Grid, x: usize, y: usize) -> usize {
    let mut points = HashSet::new();
    get_pts_in_basin(grid, x, y, &mut points).len()
}

fn get_pts_in_basin<'a>(
    grid: &Grid,
    x: usize,
    y: usize,
    points: &'a mut HashSet<(usize, usize)>,
) -> &'a mut HashSet<(usize, usize)> {
    // Remember that all points other than a 9 are always part of EXACTLY one basin
    // So flood in all directions until you hit a 9 and add all those points.
    if points.insert((x, y)) {
        for (xp, yp) in [(x-1, y), (x, y-1), (x+1, y), (x, y+1)] {
            if let Some(val) = grid.get(xp, yp) {
                if val != 9 {
                    get_pts_in_basin(grid, xp, yp, points);
                }
            }
        }
    }
    points
}

fn main() {
    println!("Day09b");

    let grid = build_grid(&aocutils::read_file_to_string(
        //"./sample_input.txt".to_string(),
        "./input.txt".to_string(),
    ));

    let low_points = get_low_points(&grid);

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|(x, y)| get_basin_size(&grid, *x, *y))
        .collect();

    basin_sizes.sort_unstable();

    let largest_3_multiplied: usize = basin_sizes.iter().rev().take(3).product();

    println!(
        "Largest 3 basins multiplied together is: {}",
        largest_3_multiplied
    );
}
