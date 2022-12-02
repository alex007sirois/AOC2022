#![feature(iterator_try_collect)]
#![feature(binary_heap_into_iter_sorted)]
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::io::stdin;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum Errors {
    NotEnoughInputs,
    InvalidInputs(std::io::Error),
    InvalidInteger(ParseIntError),
}

impl From<ParseIntError> for Errors {
    fn from(e: ParseIntError) -> Self {
        Self::InvalidInteger(e)
    }
}

impl From<std::io::Error> for Errors {
    fn from(e: std::io::Error) -> Self {
        Self::InvalidInputs(e)
    }
}

impl Error for Errors {}

impl Display for Errors {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::NotEnoughInputs => write!(f, "Error: input should contain at least 3 elves"),
            Self::InvalidInputs(e) => write!(f, "IO error reading input file: {}", e),
            Self::InvalidInteger(e) => write!(f, "Error parsing calories: {}", e),
        }
    }
}

fn main() -> Result<(), Errors> {
    let calories = total_calories_of_elves(&load_inventory()?);

    let most_calories = extract_best_elves_total(calories.clone(), 1)?;
    println!("part 1: {:?}", most_calories);

    let most_calories = extract_best_elves_total(calories.clone(), 3)?;
    println!("part 2: {:?}", most_calories);

    Ok(())
}

fn load_inventory() -> Result<Vec<Option<u64>>, Errors> {
    stdin()
        .lines()
        .map(|line| match line?.as_str() {
            "" => Ok(None),
            l => Ok(Some(u64::from_str_radix(l, 10)?)),
        })
        .try_collect()
}

fn total_calories_of_elves(inventory: &[Option<u64>]) -> BinaryHeap<u64> {
    inventory
        .into_iter()
        .chain([None].iter())
        .scan(0u64, |acc, calories| {
            Some(match calories {
                None => {
                    let cumulative_calories = *acc;
                    *acc = 0;
                    Some(cumulative_calories)
                }
                Some(c) => {
                    *acc += c;
                    None
                }
            })
        })
        .filter_map(|calories| match calories? {
            0 => None,
            c => Some(c),
        })
        .collect()
}

fn extract_best_elves_total(calories: BinaryHeap<u64>, n: usize) -> Result<u64, Errors> {
    if calories.len() < n {
        Err(Errors::NotEnoughInputs)
    } else {
        Ok(calories.into_iter_sorted().take(n).sum())
    }
}
