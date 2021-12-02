use aocutils;

fn calculate_final_position(input: Vec<String>) -> (i64, i64) {
    // given input and assuming starting position of (0, 0), returns final position of (x, y
    let mut current_pos = (0, 0);
    for line in input {
        let instructions = line.split(" ").collect::<Vec<&str>>();
        //println!("{}, {}", instructions[0], instructions[1]);
        current_pos = calculate_position(current_pos, instructions)
    }
    return current_pos
}

fn calculate_position(position: (i64, i64), instructions: Vec<&str>) -> (i64, i64) {
    match instructions[0] {
        "forward" => return (position.0 + instructions[1].parse::<i64>().unwrap(), position.1),
        "down" => return (position.0, position.1 - instructions[1].parse::<i64>().unwrap()),
        "up" => return (position.0, position.1 + instructions[1].parse::<i64>().unwrap()),
        _ => println!("Something fucked up.")
    }
    return position;
}


fn main() {
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());
    let position = calculate_final_position(lines);
    println!("Final position is {}, {}", position.0, position.1);
    println!("final horizontal position multiplied by inverse of y position is {}", position.0 * -position.1);

}



