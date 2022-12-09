
pub static EXAMPLE_INPUT: &str = include_str!("../example_input/day2.txt");

fn get_input(custom_input: Option<String>) -> String {
    return match custom_input {
        Some(custom_input) => custom_input,
        None => EXAMPLE_INPUT.to_string(),
    };
}

#[derive(Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Winner {
    Opponent,
    Tied,
    You,
}

fn parse_opponent_choice(choice: &str) -> Result<Choice, String> {
    return match choice {
        "A" => Ok(Choice::Rock),
        "B" => Ok(Choice::Paper),
        "C" => Ok(Choice::Scissors),
        _ => Err(format!("Invalid opponent choice: '{}'", choice))
    }
}

fn parse_your_choice(choice: &str) -> Result<Choice, String> {
    return match choice {
        "X" => Ok(Choice::Rock),
        "Y" => Ok(Choice::Paper),
        "Z" => Ok(Choice::Scissors),
        _ => Err(format!("Invalid choice: '{}'", choice))
    }
}

fn get_choice_score(choice: &Choice) -> i32 {
    return match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn get_winner(opponent_choice: &Choice, your_choice: &Choice) -> Winner {
    return match (opponent_choice, your_choice) {
        (Choice::Rock, Choice::Rock) => Winner::Tied,
        (Choice::Paper, Choice::Paper) => Winner::Tied,
        (Choice::Scissors, Choice::Scissors) => Winner::Tied,
        (Choice::Rock, Choice::Scissors) => Winner::Opponent,
        (Choice::Scissors, Choice::Paper) => Winner::Opponent,
        (Choice::Paper, Choice::Rock) => Winner::Opponent,
        (Choice::Scissors, Choice::Rock) => Winner::You,
        (Choice::Paper, Choice::Scissors) => Winner::You,
        (Choice::Rock, Choice::Paper) => Winner::You,
    }
}

fn get_outcome_score(opponent_choice: &Choice, your_choice: &Choice) -> i32 {
    return match get_winner(opponent_choice, your_choice) {
        Winner::Opponent => 0,
        Winner::Tied => 3,
        Winner::You => 6,
    }
}

fn parse_line(line: &str) -> Result<(Choice, Choice), String> {
    let opponents_choice: Choice;
    let your_choice: Choice;

    match line.split_once(" ") {
        Some((theirs, yours)) => {
            let parsed_opponent_choice = parse_opponent_choice(theirs);
            let parsed_your_choice = parse_your_choice(yours);
            match (parsed_opponent_choice, parsed_your_choice) {
                (Ok(valid_opponent_choice), Ok(valid_your_choice)) => {
                    opponents_choice = valid_opponent_choice;
                    your_choice = valid_your_choice;
                }
                (Err(error), ..) => return Err(error),
                (.., Err(error)) => return Err(error),
            }
        }
        None => return Err(format!("Unable to split line '{}'", line))
    }

    return Ok((opponents_choice, your_choice));
}

pub fn solve_part_1(custom_input: Option<String>) -> Result<String, String> {
    let input = get_input(custom_input);

    let mut score = 0;
    for line in input.lines() {
        let opponent_choice: Choice;
        let your_choice: Choice;

        match parse_line(line) {
            Ok((them, you)) => {
                opponent_choice = them;
                your_choice = you;
            }
            Err(error) => return Err(error)
        }

        let choice_score = get_choice_score(&your_choice);
        let outcome_score = get_outcome_score(&opponent_choice, &your_choice);
        let round_score = choice_score + outcome_score;
        score += round_score;
    }

    return Ok(format!("{}", score));
}

pub fn solve_part_2(custom_input: Option<String>) -> Result<String, String> {
    let _input = get_input(custom_input);
    return Err("Not implemented".to_string());
}
