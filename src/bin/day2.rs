#![feature(iterator_try_collect)]
use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::stdin;

#[derive(Debug)]
pub enum Errors {
    InputParsingError(InputParsingError),
    InputsSizeInvalid(Vec<char>),
    InputsInvalid(std::io::Error),
}

impl From<std::io::Error> for Errors {
    fn from(e: std::io::Error) -> Self {
        Self::InputsInvalid(e)
    }
}

impl From<InputParsingError> for Errors {
    fn from(e: InputParsingError) -> Self {
        Self::InputParsingError(e)
    }
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::InputsInvalid(e) => write!(f, "IO error reading input file: {}", e),
            Self::InputsSizeInvalid(v) => {
                write!(f, "Invalid input size: {:?} should have length of 2", v)
            }
            Self::InputParsingError(e) => e.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct InputParsingError {}

impl Error for InputParsingError {}

impl Display for InputParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "Could not parse input")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub enum Action {
    Rock,
    Paper,
    Scissors,
}

impl Action {
    pub fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn score(&self, other: &Self) -> u32 {
        self.value()
            + match self.cmp(other) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }

    pub fn choose_action_from_result(&self, result: Ordering) -> Action {
        match result {
            Ordering::Equal => self.clone(),
            Ordering::Greater => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Ordering::Less => self
                .choose_action_from_result(Ordering::Greater)
                .choose_action_from_result(Ordering::Greater),
        }
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (s, o) if s.eq(o) => Ordering::Equal,
            (Self::Rock, Self::Scissors)
            | (Self::Scissors, Self::Paper)
            | (Self::Paper, Self::Rock) => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

impl TryFrom<&char> for Action {
    type Error = InputParsingError;

    fn try_from(value: &char) -> Result<Self, Self::Error> {
        Ok(match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => Err(InputParsingError {})?,
        })
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match &self {
            Self::Rock => write!(f, "Rock"),
            Self::Paper => write!(f, "Paper"),
            Self::Scissors => write!(f, "Scissors"),
        }
    }
}

fn main() -> Result<(), Errors> {
    let strategy_guide = load_strategy_guide()?;
    println!(
        "part 1: {:}",
        total_score(&use_strategy_guide_v1(&strategy_guide)?)
    );
    println!(
        "part 2: {:}",
        total_score(&use_strategy_guide_v2(&strategy_guide)?)
    );
    Ok(())
}

fn use_strategy_guide_v1(
    strategy_guide: &[[char; 2]],
) -> Result<Vec<(Action, Action)>, InputParsingError> {
    strategy_guide
        .iter()
        .map(|[a, b]| Ok::<(Action, Action), InputParsingError>((a.try_into()?, b.try_into()?)))
        .try_collect::<Vec<(Action, Action)>>()
}

fn use_strategy_guide_v2(
    strategy_guide: &[[char; 2]],
) -> Result<Vec<(Action, Action)>, InputParsingError> {
    strategy_guide
        .iter()
        .map(|[a, b]| {
            let action: Action = a.try_into()?;
            let chosen_action = action.choose_action_from_result(try_result_from_char(b)?);
            Ok((action, chosen_action))
        })
        .try_collect::<Vec<(Action, Action)>>()
}

fn try_result_from_char(c: &char) -> Result<Ordering, InputParsingError> {
    Ok(match c {
        'X' => Ordering::Less,
        'Y' => Ordering::Equal,
        'Z' => Ordering::Greater,
        _ => Err(InputParsingError {})?,
    })
}

fn total_score(actions: &[(Action, Action)]) -> u32 {
    actions.iter().map(|pair| pair.1.score(&pair.0)).sum()
}

fn load_strategy_guide() -> Result<Vec<[char; 2]>, Errors> {
    stdin()
        .lines()
        .map(|line| {
            Ok(line?
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
                .try_into()
                .map_err(|v| Errors::InputsSizeInvalid(v))?)
        })
        .try_collect()
}
