use std::{
    fmt::{Debug, Display},
    fs::File,
    io::{self, BufRead},
};

use clap::ValueEnum;
use miette::{Context, IntoDiagnostic};

pub type Result<T> = miette::Result<T, miette::Error>;

// defines common behaviour for each daily challenge
pub trait DayRunner {
    type ReturnType;

    fn run(&self, part: &Part) -> Self::ReturnType;
}

#[derive(Default, Clone, Copy, ValueEnum)]
pub enum Day {
    #[default]
    DayOne,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::DayOne => write!(f, "day-one"),
        }
    }
}

impl Debug for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DayOne => write!(f, "1"),
        }
    }
}

#[derive(Default, Clone, Copy, ValueEnum)]
pub enum Part {
    #[default]
    PartOne,
    PartTwo,
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::PartOne => write!(f, "part-one"),
            Part::PartTwo => write!(f, "part-two"),
        }
    }
}

impl Debug for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PartOne => write!(f, "1"),
            Self::PartTwo => write!(f, "2"),
        }
    }
}

pub fn read_file_to_vec(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)
        .into_diagnostic()
        .wrap_err(format!("while opening file {file_path:#?}"))?;

    let file_content = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            line.into_diagnostic()
                .wrap_err(format!("while reading line {idx:#?}"))
        })
        .collect::<Result<Vec<String>>>()
        .wrap_err("while reading file contents")?;

    Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
