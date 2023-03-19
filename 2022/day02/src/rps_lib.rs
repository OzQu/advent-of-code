use thiserror::Error;

pub enum Strategy {
    PartOne,
    PartTwo
}

#[derive(Debug, Error)]
pub enum InvalidChoiceError {
    #[error("Invalid choice: '{0}'")]
    InvalidChoice(String),
}

#[derive(Debug, Error)]
pub enum ParseLineError {
    #[error("Invalid line: {0}")]
    InvalidLine(String),

    #[error("Invalid choice: {0}")]
    InvalidChoice(#[from] InvalidChoiceError),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn wins(&self) -> Result<Self, InvalidChoiceError> {
        match self {
            Choice::Rock => Ok(Choice::Scissors),
            Choice::Paper => Ok(Choice::Rock),
            Choice::Scissors => Ok(Choice::Paper)
        }
    }
    pub fn draws(&self) -> Result<Self, InvalidChoiceError> {
        Ok(self.clone())
    }
    pub fn loses(&self) -> Result<Self, InvalidChoiceError> {
        match self {
            Choice::Rock => Ok(Choice::Paper),
            Choice::Paper => Ok(Choice::Scissors),
            Choice::Scissors => Ok(Choice::Rock)
        }
    }

    pub fn from_str(c: &str) -> Result<Self, InvalidChoiceError> {
        match c {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(InvalidChoiceError::InvalidChoice(format!(
                "Invalid choice: '{}'",
                c
            ))),
        }
    }
    pub fn from_desired_result(previous: &Choice, desired_result: &str) -> Result<Self, InvalidChoiceError> {
        match desired_result {
            "X" => previous.wins(),
            "Y" => previous.draws(),
            "Z" => previous.loses(),
            _ => Err(InvalidChoiceError::InvalidChoice(format!(
                "Invalid choice: '{}'",
                desired_result
            )))
        }
    }
}


pub type Score = (i32, i32);

pub fn calculate_points_from_line(line: &str, strategy: &Strategy) -> Result<Score, ParseLineError> {
    let mut iter = line.split_whitespace();

    let p1_choice = Choice::from_str(iter.next().ok_or(ParseLineError::InvalidLine("Missing player 1 choice".to_owned()))?)?;
    let p2_choice = match strategy {
        Strategy::PartOne => Choice::from_str(iter.next().ok_or(ParseLineError::InvalidLine("Missing player 2 choice".to_owned()))?)?,
        Strategy::PartTwo => Choice::from_desired_result(&p1_choice, iter.next().ok_or(ParseLineError::InvalidLine("Missing player 2 choice".to_owned()))?)?,
    };

    if let Some(_) = iter.next() {
        return Err(ParseLineError::InvalidLine("Unexpected third value".to_owned()));
    }


    let (p1_score, p2_score) = match (&p1_choice, &p2_choice) {
        (Choice::Rock, Choice::Scissors)
        | (Choice::Paper, Choice::Rock)
        | (Choice::Scissors, Choice::Paper) => (6, 0),
        (Choice::Scissors, Choice::Rock)
        | (Choice::Rock, Choice::Paper)
        | (Choice::Paper, Choice::Scissors) => (0, 6),
        _ => (3, 3),
    };

    let p1_selection_score = match &p1_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let p2_selection_score = match &p2_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let p1_total_score = p1_score + p1_selection_score;
    let p2_total_score = p2_score + p2_selection_score;

    Ok((p1_total_score, p2_total_score))
}

#[test]
fn test_from_str() {
    assert!(matches!(Choice::from_str("A"), Ok(Choice::Rock)));
    assert!(matches!(Choice::from_str("B"), Ok(Choice::Paper)));
    assert!(matches!(Choice::from_str("C"), Ok(Choice::Scissors)));
    assert!(matches!(Choice::from_str("X"), Ok(Choice::Rock)));
    assert!(matches!(Choice::from_str("Y"), Ok(Choice::Paper)));
    assert!(matches!(Choice::from_str("Z"), Ok(Choice::Scissors)));
    assert!(matches!(
        Choice::from_str("D"),
        Err(InvalidChoiceError::InvalidChoice(_))
    ));
}

    #[test]
fn test_from_desired_result() {
    assert!(matches!(
        Choice::from_desired_result(&Choice::Rock, "X"),
        Ok(Choice::Scissors)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Rock, "Y"),
        Ok(Choice::Rock)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Rock, "Z"),
        Ok(Choice::Paper)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Paper, "X"),
        Ok(Choice::Rock)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Paper, "Y"),
        Ok(Choice::Paper)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Paper, "Z"),
        Ok(Choice::Scissors)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Scissors, "X"),
        Ok(Choice::Paper)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Scissors, "Y"),
        Ok(Choice::Scissors)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Scissors, "Z"),
        Ok(Choice::Rock)
    ));
    assert!(matches!(
        Choice::from_desired_result(&Choice::Rock, "invalid"),
        Err(InvalidChoiceError::InvalidChoice(_))
    ));
}

#[test]
fn test_calculate_points_from_line_part_a() {
    assert!(
        matches!(calculate_points_from_line("A X", &Strategy::PartOne), Ok((4, 4))),
        "Expected 'A X' (Rock Rock) to result in a draw (4-4)"
    );
    assert!(
        matches!(calculate_points_from_line("A Y", &Strategy::PartOne), Ok((1, 8))),
        "Expected 'A Y' (Rock Paper) to result in a win for player 2 (1-8)"
    );
    assert!(
        matches!(calculate_points_from_line("A Z", &Strategy::PartOne), Ok((7, 3))),
        "Expected 'A Z' (Rock Scissors) to result in a win for player 1 (7-3)"
    );
    assert!(
        matches!(calculate_points_from_line("B X", &Strategy::PartOne), Ok((8, 1))),
        "Expected 'B X' (Paper Rock) to result in a win for player 1 (8-1)"
    );
    assert!(
        matches!(calculate_points_from_line("B Y", &Strategy::PartOne), Ok((5, 5))),
        "Expected 'B Y' (Paper Paper) to result in a draw (5-5)"
    );
    assert!(
        matches!(calculate_points_from_line("B Z", &Strategy::PartOne), Ok((2, 9))),
        "Expected 'B Z' (Paper Scissors) to result in a win for player 2 (2-9)"
    );
    assert!(
        matches!(calculate_points_from_line("C X", &Strategy::PartOne), Ok((3, 7))),
        "Expected 'C X' (Scissors Rock) to result in a win for player 2 (3-7)"
    );
    assert!(
        matches!(calculate_points_from_line("C Y", &Strategy::PartOne), Ok((9, 2))),
        "Expected 'C Y' (Scissors Paper) to result in a win for player 1 (9-2)"
    );
    assert!(
        matches!(calculate_points_from_line("C Z", &Strategy::PartOne), Ok((6, 6))),
        "Expected 'C Z' (Scissors Scissors) to result in a draw (6-6)"
    );
    assert!(
        matches!(calculate_points_from_line("invalid input", &Strategy::PartOne), Err(ParseLineError::InvalidChoice(_))),
        "Expected 'invalid input' to result in an InvalidChoice error"
    );
    assert!(
        matches!(calculate_points_from_line("A B C", &Strategy::PartOne), Err(ParseLineError::InvalidLine(_))),
        "Expected 'A B C' to result in an InvalidLine error"
    );
    assert!(
        matches!(calculate_points_from_line("_", &Strategy::PartOne), Err(ParseLineError::InvalidChoice(_))),
        "Expected '_' to result in an InvalidLine error"
    );
}

#[test]
fn test_calculate_points_from_line_part_b() {
    assert!(
        matches!(calculate_points_from_line("A X", &Strategy::PartTwo), Ok((7, 3))),
        "Expected 'A X' (Rock Lose/Scissors) to result in a win for player 1 (7-3)"
    );
    assert!(
        matches!(calculate_points_from_line("A Y", &Strategy::PartTwo), Ok((4, 4))),
        "Expected 'A Y' (Rock Draw/Rock) to result in a draw (4-4)"
    );
    assert!(
        matches!(calculate_points_from_line("A Z", &Strategy::PartTwo), Ok((1, 8))),
        "Expected 'A Z' (Rock Win/Paper) to result in a win for player 2 (1-8)"
    );
    assert!(
        matches!(calculate_points_from_line("B X", &Strategy::PartTwo), Ok((8, 1))),
        "Expected 'B X' (Paper Lose/Rock) to result in a win for player 1 (8-1)"
    );
    assert!(
        matches!(calculate_points_from_line("B Y", &Strategy::PartTwo), Ok((5, 5))),
        "Expected 'B Y' (Paper Draw/Paper) to result in a draw (5-5)"
    );
    assert!(
        matches!(calculate_points_from_line("B Z", &Strategy::PartTwo), Ok((2, 9))),
        "Expected 'B Z' (Paper Win/Scissors) to result in a win for player 2 (2-9)"
    );
    assert!(
        matches!(calculate_points_from_line("C X", &Strategy::PartTwo), Ok((9, 2))),
        "Expected 'C X' (Scissors Lose/Paper) to result in a win for player 1 (9-2)"
    );
    assert!(
        matches!(calculate_points_from_line("C Y", &Strategy::PartTwo), Ok((6, 6))),
        "Expected 'C Y' (Scissors Draw/Scissors) to result in a draw (6-6)"
    );
    assert!(
        matches!(calculate_points_from_line("C Z", &Strategy::PartTwo), Ok((3, 7))),
        "Expected 'C Z' (Scissors Win/Rock) to result in a win for player 2 (3-7)"
    );
    assert!(
        matches!(calculate_points_from_line("invalid input", &Strategy::PartTwo), Err(ParseLineError::InvalidChoice(_))),
        "Expected 'invalid input' to result in an InvalidChoice error"
    );
    assert!(
        matches!(calculate_points_from_line("A X C", &Strategy::PartTwo), Err(ParseLineError::InvalidLine(_))),
        "Expected 'A B C' to result in an InvalidLine error"
    );
    assert!(
        matches!(calculate_points_from_line("_", &Strategy::PartTwo), Err(ParseLineError::InvalidChoice(_))),
        "Expected '_' to result in an InvalidLine error"
    );
}
