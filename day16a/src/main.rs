use std::fs;

fn to_binary(input: Vec<u32>) -> String {
    let mut result: String = "".to_string();
    for i in input {
        let bin_str: String = format!("{:04b}", i);
        //println!("{:04b}, {:#06b}", i, i);
        result.push_str(&bin_str);
    }
    result
}

fn main() {
    let raw_input: Vec<u32> = fs::read_to_string("./sample_input.txt")
        .unwrap()
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(16).unwrap() as u32))
        .collect();
    println!("raw_input: {:?}", raw_input);
    println!("{}", to_binary(raw_input));


}

//let x: usize = 9; // 1001 in binary, 9 in hexadecimal
//let y: usize = 18; // 10010 in binary 12 in hexadecimal
//println!("{:b}", y); // 10010
//println!("{:#b}", y); // 0b10010
//println!("{:X}", y); // 12
