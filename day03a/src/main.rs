use aocutils;

fn get_gamma_bit(input: &Vec<String>) -> bool {
    // returns true if right-most gamma bit for input is 1
    // println!("second number in input is {}", input[1]);
    let mut total_even = 0;
    let mut total_odd = 0;
    for number_as_str in input {
        if is_odd(&i64::from_str_radix(&number_as_str, 2).unwrap()) {
            total_odd += 1;
        } else {
            total_even += 1;
        }
    }
    // println!(
    //     "total_even is {} and total_odd is {}",
    //     total_even, total_odd
    // );
    if total_odd > total_even {
        return true;
    }
    return false;
}

fn is_odd(&input: &i64) -> bool {
    // returns false if number is even
    if input & 1 == 1 {
        return true;
    }
    return false;
}

fn shift_all_numbers_right(input: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for number in input {
        result.push(format!(
            "{:b}",
            (i64::from_str_radix(number, 2).unwrap() >> 1)
        ))
    }
    return result;
}

fn get_gamma_epsilon(input: Vec<String>, width: i64) -> (i64, i64) {
    // Returns (gamma, epsilon) as a tuple
    let mut n = 0;
    let mut gamma_str = String::from("");
    let mut epsilon_str = String::from("");

    let mut numbers_as_str = input.clone();

    while n < width {
        if get_gamma_bit(&numbers_as_str) {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }

        numbers_as_str = shift_all_numbers_right(&numbers_as_str);
        n += 1;
        // println!("intermediate value for gamma_str is {}", gamma_str)
    }

    // reverse order of string since we assembled it it backwards
    gamma_str = gamma_str.chars().rev().collect::<String>();
    epsilon_str = epsilon_str.chars().rev().collect::<String>();
    println!("final value for n is {} and string version of gamma_str is {}, string version of epsilon is {}", n, gamma_str, epsilon_str);
    return (
        isize::from_str_radix(gamma_str.as_str(), 2)
            .unwrap()
            .try_into()
            .unwrap(),
        isize::from_str_radix(epsilon_str.as_str(), 2)
            .unwrap()
            .try_into()
            .unwrap(),
    );
}

fn main() {
    // let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());
    let gamma_epsilon_tuple =
        get_gamma_epsilon(lines.clone(), lines[0].chars().count().try_into().unwrap());
    println!("gamma is {}", gamma_epsilon_tuple.0);
    println!("epsilon is {}", gamma_epsilon_tuple.1);
    println!(
        "gamma * epsilon is: {}",
        gamma_epsilon_tuple.0 * gamma_epsilon_tuple.1
    )
}
