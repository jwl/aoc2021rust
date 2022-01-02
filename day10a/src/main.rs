static PAIRS: [char; 8] = ['(', ')', '{', '}', '[', ']', '<', '>'];

enum LineType {
    Incomplete,
    Corrupted,
    Correct,
}

enum BraceType {
    Open,
    Close,
}

fn get_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn get_brace_type(c: char) -> BraceType {
    let mut btype = BraceType::Close;
    for (ind, v) in PAIRS.iter().enumerate() {
        if *v == c && ind % 2 == 0 {
            btype = BraceType::Open;
            break;
        }
        if *v == c && ind % 2 != 0 {
            btype = BraceType::Close;
            break;
        }
    }
    btype
}

fn get_pair(c: char) -> char {
    let mut result = 'x';
    for (index, v) in PAIRS.iter().enumerate() {
        if *v == c && index % 2 == 0 {
            result = PAIRS[index + 1];
        } else if *v == c {
            result = PAIRS[index - 1];
        }
    }
    result
}

fn lint_line(line: &[char]) -> (u64, LineType) {
    let mut line_type = LineType::Correct;
    let mut score = 0;
    let line_length = line.len();
    let mut i = 0;

    let mut stack: Vec<char> = Vec::new();

    //while stack.len() >= 0 {
    loop {
        //if stack.len() > 0 && i == line_length {
        if !stack.is_empty() && i == line_length {
            //println!("incomplete");
            //score = get_incomplete_score(&stack);
            line_type = LineType::Incomplete;
            break;
        }
        //if stack.len() == 0 && i == line_length {
        if !stack.is_empty() && i == line_length {
            //println!("complete and correct");
            score = 0;
            line_type = LineType::Correct;
            break;
        }

        let pair = get_pair(line[i]);

        match get_brace_type(line[i]) {
            BraceType::Open => {
                stack.push(line[i]);
            }
            BraceType::Close => {
                if let Some(top) = stack.pop() {
                    if pair == top {
                        //println!("Matched {} with {}", top, line[i]);
                    } else {
                        //println!("Corrupted line found!");
                        score = get_score(line[i]);
                        line_type = LineType::Corrupted;
                        break;
                    }
                }
            }
        }
        i += 1;
    }
    (score, line_type)
}

fn main() {
    println!("Day10a");

    //let input = include_str!("../sample_input.txt");
    let input = include_str!("../input.txt");
    let mut corrupted_score = 0;

    for (i, line) in input.lines().enumerate() {
        //println!("at line number {}, line is {}", i, line);

        let line_chars = line.chars().collect::<Vec<char>>();
        let (score, line_type) = lint_line(&line_chars);

        match line_type {
            LineType::Corrupted => {
                corrupted_score += score;
            }
            LineType::Incomplete => {
                //println!("incomplete line, skipping...");
            }
            LineType::Correct => {
                //println!("line parsed as correct!");
            }

        }
    }

    println!("Corrupted line score is: {}", corrupted_score);

}
