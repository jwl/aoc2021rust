use std::collections::HashMap;

fn get_all_coords(line: &String) -> Vec<String> {
    //println!("line is {}", line);

    let mut split = line.split(" -> ").collect::<Vec<&str>>();
    let mut coords: Vec<String> = Vec::new();

    //println!("split[0] is {} and split[0].get(0) is {}", split[0], split[1]);
    //println!("split[0].get(0..1) is {}", split[0].get(0..1).unwrap());
    //println!("split[0].get(0..1) is {}", split[0].get(0..1).unwrap());

    let coord1: Vec<String> = split[0].to_string().split(",").map(|s| s.parse().unwrap()).collect();
    //for c in coord1 {
        //println!("\t{}", c);
    //}
    let coord2: Vec<String> = split[1].to_string().split(",").map(|s| s.parse().unwrap()).collect();
    //for c in coord2 {
        //println!("\t{}", c);
    //}

    // horizontal line, y's are the same
    //if split[0].get(0) == split.get()
    if coord1[1] == coord2[1] {
        let x1: i32 = coord1[0].parse().unwrap();
        let x2: i32 = coord2[0].parse().unwrap();
        println!("horizontal line");
        if x1 > x2 {
            for n in x2..=x1 {
                //println!("\tPushing {}", format!("{},{}", n, coord1[1]));
                coords.push(format!("{},{}", n, coord1[1]));
            }
        } else {
            for n in x1..=x2 {
                //println!("\tPushing {}", format!("{},{}", n, coord1[1]));
                coords.push(format!("{},{}", n, coord1[1]));
            }
        }
    } else if coord1[0] == coord2[0] {
        // vertical line, x's are the same
        println!("vertical line");
        let y1: i32 = coord1[1].parse().unwrap();
        let y2: i32 = coord2[1].parse().unwrap();
        if y1 > y2 {
            for n in y2..=y1 {
                //println!("\tPushing {}", format!("{},{}", n, coord1[0]));
                coords.push(format!("{},{}", coord1[0], n));
            }
        } else {
            for n in y1..=y2 {
                //println!("\tPushing {}", format!("{},{}", n, coord1[0]));
                coords.push(format!("{},{}", coord1[0], n));
            }
        }
    } else {
        // diagonal line, guaranteed to be at exactly 45 degrees
        println!("diagonal line");
        let x1: i32 = coord1[0].parse().unwrap();
        let x2: i32 = coord2[0].parse().unwrap();
        let y1: i32 = coord1[1].parse().unwrap();
        let y2: i32 = coord2[1].parse().unwrap();
        let (xdir, ydir): (i32, i32);
        let distance: i32 = (x1 - x2).abs();
        if x1 > x2 {
            xdir = -1;
        } else {
            xdir = 1;
        }
        if y1 > y2 {
            ydir = -1;
        } else {
            ydir = 1;
        }

        for i in 0..=distance {
            let xcoord = x1 + (xdir * i);
            let ycoord = y1 + (ydir * i);
            coords.push(format!("{},{}", xcoord, ycoord));
        }


    }

    coords
}

fn parse_line(mut map: HashMap<String, i32>, line: &String) -> HashMap<String, i32> {
    print!("line is {} ... ", line);
    let coords = get_all_coords(&line);

    if coords.is_empty() {
        //println!("\tNot a straight line");
        return map;
    }

    //assert_eq!(map.is_empty(), true);

    //println!("before adding coords, map is:");
    //for (key, val) in map.iter() {
        //println!("\tkey: {} val: {}", key, val);
    //}

    for coord in coords {
        //println!("coord is {}", coord);
        let counter = map.entry(coord).or_insert(0);
        *counter += 1;
    }
    map
}

fn main() {
    // Day 05b
    //let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());

    //let mut ld = Dict::<String>::new();
    let mut map: HashMap<String, i32> = HashMap::new();
    for line in lines {
        map = parse_line(map, &line);
        //println!("{}", line)
    }

    println!("map is:");
    let mut answer: u32 = 0;
    for (key, val) in map.iter() {
        if val > &1 {
            answer += 1;
        }
        println!("\tkey: {} val: {}", key, val);
    }

    println!("Total number of points where at least 2 or more lines overlap: {}", answer);

}

