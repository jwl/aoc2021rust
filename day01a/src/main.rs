use aocutils;

fn count_increases(input: Vec<i64>) -> i64 {
    // Returns number of increases in the input
    let mut prev = input[0];
    let mut total = 0;
    for number in input {
        if number > prev {
            total += 1;
        }
        prev = number
    }
    return total
}

fn main() {
    if let Ok(lines) = aocutils::load_input_as_ints("./input.txt".to_string()) {
        println!("total number of times depth increases is {}", count_increases(lines))
    }
}
