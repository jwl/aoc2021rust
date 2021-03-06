use aocutils;

struct Board {
    numbers: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn has_winner(&self) -> bool {
        // check for horizontal row winner
        for row in 0..5 {
            let mut hwinner = true;
            for col in 0..5 {
                if !self.marked[row][col] {
                    hwinner = false;
                    break;
                }
            }
            // horizontal winner found
            if hwinner {
                println!("horizontal winner found in row {}", row);
                return hwinner;
            }
        }

        // check for vertical row winner
        for col in 0..5 {
            let mut vwinner = true;
            for row in 0..5 {
                if !self.marked[row][col] {
                    vwinner = false;
                    break;
                }
            }
            if vwinner {
                println!("vertical winner found in col {}", col);
                return vwinner;
            }
        }

        // no winners found
        false
    }

    fn calculate_unmarked_sum(&self) -> u32 {
        let mut score = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.marked[row][col] {
                    score += self.numbers[row][col];
                }
            }
        }
        score
    }

    fn mark_board(&mut self, number: u32) {
        // mark all positions whose value matches the number
        for row in 0..self.numbers.len() {
            for col in 0..self.numbers[row].len() {
                if self.numbers[row][col] == number {
                    self.marked[row][col] = true;
                }
            }
        }
    }

    fn print_board(&self) {
        for row in 0..self.numbers.len() {
            print!("\t");
            for col in 0..self.numbers[row].len() {
                print!("{} ", self.numbers[row][col]);
            }
            println!();
        }
        for row in 0..self.marked.len() {
            print!("\t");
            for col in 0..self.marked[row].len() {
                if self.marked[row][col] {
                    print!("* ")
                } else {
                    print!("- ")
                }
            }
            println!();
        }
    }
}

fn parse_input_drawnumbers(input: &String) -> Vec<u32> {
    println!("drawnumbers input is {}", input);
    return input.split(",").map(|s| s.parse().unwrap()).collect();
}

fn parse_board(input: &Vec<String>) -> Board {
    let mut nums: [[u32; 5]; 5] = [[0; 5]; 5];

    for row in 0..5 {
        let line: Vec<u32> = input[row]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for col in 0..5 {
            nums[row][col] = line[col]
        }
    }

    Board {
        numbers: nums,
        marked: [[false; 5]; 5],
    }
}

fn parse_input_boards(input: &Vec<String>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();
    let mut line = 0;

    while line < input.len() {
        boards.push(parse_board(&input[(line + 1)..(line + 6)].to_vec()));
        line += 6;
    }

    boards
}

fn run_game(draw_numbers: Vec<u32>, mut boards: Vec<Board>) -> u32 {
    for number in draw_numbers {
        for board in &mut boards {
            board.mark_board(number);
            if board.has_winner() {
                println!("We have a winner!");
                board.print_board();

                return number * board.calculate_unmarked_sum();
            }
        }
    }

    0
}

fn main() {
    // let lines = aocutils::load_input_as_strings("./sample_input.txt".to_string());
    let lines = aocutils::load_input_as_strings("./input.txt".to_string());
    let draw_numbers = parse_input_drawnumbers(&lines[0]);
    let boards = parse_input_boards(&lines[1..].to_vec());

    println!(
        "final score of winning board is {}",
        run_game(draw_numbers, boards)
    );
}
