use aocutils;

fn calculate_final_position(input: Vec<String>) -> (i64, i64, i64) {
    // given input and starting position of (0, 0, 0),
    // returns final position of (x, y, aim)
    let mut current_pos = (0, 0, 0);
    for line in input {
        let instructions = line.split(" ").collect::<Vec<&str>>();
        current_pos = calculate_position(current_pos, instructions)
    }
    return current_pos
}


fn calculate_position(position: (i64, i64, i64), instructions: Vec<&str>) -> (i64, i64, i64) {
    let value = instructions[1].parse::<i64>().unwrap();
    match instructions[0] {
        "forward" => return (position.0 + value, position.1 + (position.2 * value), position.2),
        "down" => return (position.0, position.1, position.2 - value),
        "up" => return (position.0, position.1, position.2 + value),

        _ => println!("Something fucked up.")
    }
    return position;
}


fn main() {
    let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let position = calculate_final_position(lines);
    println!("Final position is {}, {}, {}", position.0, position.1, position.2);
    println!("final horizontal position multiplied by inverse of y position is {}", position.0 * -position.1);
}

