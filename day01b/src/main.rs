use aocutils;

fn count_increases(input: Vec<i64>) -> i64 {
    // Returns number of increases in the input in a three measurement window
    let mut prev = input[0] + input[1] + input[2];
    let mut total = 0;
    let mut index = 0;
    let mut window: i64;
    while index+2 < input.len() {
        window = input[index] + input[index+1] + input[index+2];
        if window > prev {
            //println!("at index {}, window is {} and prev is {}", index, window, prev);
            total += 1;
        }
        prev = window;
        index += 1;
    }
    println!("after final loop, index is {}, with input.len() at {}", index, input.len());
    return total
}


fn main() {
    if let Ok(lines) = aocutils::load_input_as_ints("./input.txt".to_string()) {
        println!("total number of times depth increases is {}", count_increases(lines))
    }
}

