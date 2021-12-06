
fn process_day(fishlist: &Vec<i64>) -> Vec<i64> {
    let mut new_list: Vec<i64> = Vec::new();
    for fish in fishlist {
        if *fish <= 0 {
            new_list.push(6);
            new_list.push(8);
        } else {
            new_list.push(fish - 1)
        }
    }
    new_list
}



fn main() {
    // Day 06a
    let days = 80;
    //let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());

    let mut fishlist = lines[0].split(",").map(|s| s.parse().unwrap()).collect::<Vec<i64>>();

    println!("Initial state: {:?}", fishlist);

    for _day in 0..days {
        fishlist = process_day(&fishlist);
        //println!("After day {}: {:?}", day, fishlist);
    }

    println!("Total number of lanternfish is {}", fishlist.len());
}
