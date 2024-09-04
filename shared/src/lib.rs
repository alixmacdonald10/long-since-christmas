use std::fmt::{Display, Debug};

use clap::ValueEnum;

// defines common behaviour for each dayly challenge
pub trait DayRunner {
    type ReturnType;

    fn run(&self, part: &Part) -> Self::ReturnType;

}

#[derive(Default, Clone, Copy, ValueEnum)]
pub enum Day {
    #[default]
    DayOne
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

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
