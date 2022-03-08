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

fn bin_to_deci(bin: &str) -> u32 {
    u32::from_str_radix(bin, 2).unwrap()
}

// given packet that starts at start, returns literal value as a u32 and the index after end
fn process_literal_packet(start: u32, p: &str) -> u32 {
    // first 3 bits are packet version, second three bits are type ID
    let mut index: usize = 6;
    let mut literal_value: String = "".to_string();
    let mut is_last_group: &str;
    while index < p.len() {
        println!("for index {}, five bit group is {}", index, &p[index..index+5]);
        is_last_group = &p[index..index+1];
        literal_value.push_str(&p[index+1..index+5]);

        println!("p[index] is {}", is_last_group);
        if is_last_group != "1" {
            break
        }
        // go to next group of 5 bits
        index += 5;
    }
    println!("raw final literal value is: {}", literal_value);
    println!("literal value in decimal is: {}", bin_to_deci(literal_value.as_str()));

    return bin_to_deci(literal_value.as_str());
}

//fn process_packet()

fn process(p: &str) {
    //let packet: String = p.to_owned();

    //let version_bin: String = packet[0..3].to_string();
    //let version_int: u32 = u32::from_str_radix(version_bin.as_str(), 2).unwrap();
    //println!("version_bin: {}, version_int: {}", version_bin, version_int);

    //let version = get_packet_version(p);
    let version = bin_to_deci(&p[0..3]);
    println!("version is: {}", version);


    let mut index: usize = 6;

    //while index < p.len() {
    let type_id = bin_to_deci(&p[3..6]);
    println!("packet type ID is: {}", type_id);

    println!("p.len() is {:?}", p.len());

    if type_id == 4 {
        // literal value
        let mut literal_value: String = "".to_string();
        let mut is_last_group: &str;
        while index < p.len() {
            println!("for index {}, five bit group is {}", index, &p[index..index+5]);
            is_last_group = &p[index..index+1];
            literal_value.push_str(&p[index+1..index+5]);

            println!("p[index] is {}", is_last_group);
            if is_last_group != "1" {
                break
            }
            // go to next group of 5 bits
            index += 5;
        }
        println!("raw final literal value is: {}", literal_value);
        println!("literal value in decimal is: {}", bin_to_deci(literal_value.as_str()))
    } else {
        // operator
    }


        //break;
}

fn main() {
    let raw_input: Vec<u32> = fs::read_to_string("./sample_input.txt")
        .unwrap()
        .lines()
        .flat_map(|x| x.chars().map(|c| c.to_digit(16).unwrap() as u32))
        .collect();
    println!("raw_input in hexadecimal: {:?}", raw_input);

    let bin_input: String = to_binary(raw_input);
    println!("translated packet to binary: {}", bin_input);

    process(&bin_input);

}

//let x: usize = 9; // 1001 in binary, 9 in hexadecimal
//let y: usize = 18; // 10010 in binary 12 in hexadecimal
//println!("{:b}", y); // 10010
//println!("{:#b}", y); // 0b10010
//println!("{:X}", y); // 12
