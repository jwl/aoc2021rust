use aocutils;

fn get_filtering_bit(position: i32, input: &Vec<String>, is_o2: bool) -> char {
    // For given bit position, determine what is the most common value if is_o2 is true
    // or the least common value if is_o2 is false and return it
    // if there is a tie for most common value and is_o2 is true, return 1, else 0.
    let mut total_ones = 0;
    let mut total_zeroes = 0;

    for line in input {
        if line.chars().nth(position.try_into().unwrap()).unwrap() == '0' {
            total_zeroes += 1;
        } else {
            total_ones += 1
        }
    }

    if is_o2 {
        // o2
        if total_ones >= total_zeroes {
            return '1';
        } else {
            return '0';
        }
    } else {
        // co2
        if total_ones >= total_zeroes {
            return '0';
        } else {
            return '1';
        }
    }
}

fn get_all_matching(position: i32, value: char, input: &Vec<String>) -> Vec<String> {
    // Return the collection of binary values that have the given value
    // at the given position
    let mut result: Vec<String> = Vec::new();

    for line in input {
        // if value at position matches, put it into the result
        if line.chars().nth(position.try_into().unwrap()).unwrap() == value {
            result.push(line.clone())
        }
    }

    return result;
}

fn get_rating(input: &Vec<String>, width: i32, is_o2: bool) -> String {
    let mut candidate_pool = input.clone();
    let mut bit_position = 0;
    while candidate_pool.len() > 1 && bit_position < width {
        let filtering_bit = get_filtering_bit(bit_position, &candidate_pool, is_o2);
        candidate_pool = get_all_matching(bit_position, filtering_bit, &candidate_pool);
        bit_position += 1
    }
    if candidate_pool.len() == 1 {
        return candidate_pool[0].clone();
    } else {
        println!("Something fucked up in get_rating...");
        return 1.to_string();
    }
}

fn main() {
    // let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());
    let width = lines[0].chars().count().try_into().unwrap();

    let o2 = get_rating(&lines, width, true);
    let co2 = get_rating(&lines, width, false);

    println!(
        "o2 is {}, co2 is {}, life support rating is {}",
        o2,
        co2,
        i64::from_str_radix(o2.as_str(), 2).unwrap()
            * i64::from_str_radix(co2.as_str(), 2).unwrap()
    )
}
