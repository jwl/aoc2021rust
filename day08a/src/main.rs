
fn process_output(output: String) -> usize {
    println!("in process_output, trying to process: {}", output);
    let mut sum: usize = 0;
    let digits: Vec<&str> = output.split(' ').collect::<Vec<&str>>();

    for digit in digits {
        //println!("digit.len() is: {}", digit.len());
        if digit.len() == 2 || digit.len() == 4 || digit.len() == 3 || digit.len() == 7 {
            // 1 segment = 2
            // 4 segments = 4
            // 3 segments = 7
            // 7 segments = 8
            println!("digit {} has length of {}", digit, digit.len());
            sum += 1;
        }
    }

    sum
}

fn day8a(lines: &[String]) -> usize {
    let mut sum: usize = 0;
    for line in lines {
        sum += process_output(line.split(" | ").collect::<Vec<&str>>()[1].to_string());
        //sum += process_output(line.split(" | ").next_back().unwrap().to_string());
    }

    sum
}


fn main() {
    // Day 08a
    //let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    //let lines = aocutils::load_input_as_strings("./sample_input2.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());

    println!("Total number of times 1, 4, 7 or 8 appear: {}", day8a(&lines));
}
