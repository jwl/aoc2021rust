use aocutils;


fn main() {
    if let Ok(lines) = aocutils::load_input_as_ints("./input.txt".to_string()) {
        for line in lines {
            println!("{}", line)
        }
    }
    //if let Ok(lines) = read_lines("./input.txt") {
        //for line in lines {
            //if let Ok(ip) = line {
                //println!("{}", ip);
            //}
        //}
    //}
}
