use std::{
    fs::File,
    io::{self, BufRead},
};

use miette::{Context, Error, IntoDiagnostic};
use shared::{DayRunner, Part};

const INPUT_FILE_PATH: &str = "inputs/day-one/input.txt";

type RunnerReturnType<T> = miette::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Runner {
    input: Vec<String>,
}

impl Runner {
    pub fn new(input: Option<Vec<String>>) -> RunnerReturnType<Self> {
        if let Some(input) = input {
            Ok(Runner { input })
        } else {
            let input = read_file_to_vec(INPUT_FILE_PATH)
                .wrap_err(format!("while reading input file {INPUT_FILE_PATH:#?}"))?;
            Ok(Runner { input })
        }
    }

    fn run_part_one(&self) -> RunnerReturnType<u32> {
        let values = self
            .input
            .iter()
            .map(|line| {
                let mut first = 0;
                let mut last = 0;
                for character in line.chars() {
                    if character.is_numeric() {
                        let parsed = u32::from(character) - b'0' as u32;
                        if first == 0 {
                            first = parsed;
                        } else {
                            last = parsed;
                        }
                    }
                }

                // handle the case where there is only one digit
                if last == 0 {
                    last = first;
                }

                // first digit will always be x10
                first * 10 + last
            })
            .collect::<Vec<u32>>();

        let output_value: u32 = values.iter().sum();
        println!("output: {output_value:#?}");
        Ok(output_value)
    }

    fn run_part_two(&self) -> RunnerReturnType<u32> {
        let lookup_digits = std::collections::HashMap::from([
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]);
        let lookup_vec: Vec<&str> = lookup_digits.clone().into_keys().collect();

        let values = self
            .input
            .clone()
            .into_iter()
            .map(|line| {
                let mut first = 0;
                let mut last = 0;
                let mut char_accumulator = "".to_string();
                for character in line.chars() {
                    if character.is_numeric() {
                        let parsed = u32::from(character) - b'0' as u32;
                        if first == 0 {
                            first = parsed;
                        } else {
                            last = parsed;
                        }
                        char_accumulator = "".to_string();
                    } else {
                        char_accumulator += character.to_string().as_str();
                        for replacement in lookup_vec.clone() {
                            if char_accumulator.contains(replacement) {
                                let parsed_replacement: u32 =
                                    lookup_digits.get(replacement).unwrap().parse().unwrap();
                                if first == 0 {
                                    first = parsed_replacement;
                                } else {
                                    last = parsed_replacement;
                                }
                                char_accumulator = "".to_string();
                            }
                        }
                    }
                }

                // handle the case where there is only one digit
                if last == 0 {
                    last = first;
                }

                // first digit will always be x10
                first * 10 + last
            })
            .collect::<Vec<u32>>();

        let output_value: u32 = values.iter().sum();
        println!("output: {output_value:#?}");
        Ok(output_value)
    }
}

impl DayRunner for Runner {
    type ReturnType = RunnerReturnType<u32>;

    fn run(&self, part: &shared::Part) -> Self::ReturnType {
        match part {
            Part::PartOne => self.run_part_one(),
            Part::PartTwo => self.run_part_two(),
        }
    }
}

fn read_file_to_vec(file_path: &str) -> RunnerReturnType<Vec<String>> {
    let file = File::open(file_path)
        .into_diagnostic()
        .wrap_err(format!("while opening file {file_path:#?}"))?;

    let file_content = io::BufReader::new(file)
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            line.into_diagnostic().wrap_err(format!("while reading line {idx:#?}"))
        })
        .collect::<RunnerReturnType<Vec<String>>>()
        .wrap_err("while reading file contents")?;

    Ok(file_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART_ONE_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const PART_ONE_OUTPUT: u32 = 142;

    static PART_TWO_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    const PART_TWO_OUTPUT: u32 = 281;

    fn convert_test_input(input: &str) -> Vec<String> {
        input
            .to_string()
            .lines()
            .map(|line| line.to_owned())
            .collect::<Vec<String>>()
    }

    #[test]
    fn part_one() {
        let test_input = convert_test_input(PART_ONE_INPUT);
        let runner = Runner::new(Some(test_input)).unwrap();
        let output = runner.run(&Part::PartOne);
        assert!(output.is_ok());
        assert!(output.unwrap() == PART_ONE_OUTPUT);
    }

    #[test]
    fn part_two() {
        let test_input = convert_test_input(PART_TWO_INPUT);
        let runner = Runner::new(Some(test_input)).unwrap();
        let output = runner.run(&Part::PartTwo);
        assert!(output.is_ok());
        assert!(output.unwrap() == PART_TWO_OUTPUT);
    }
}
