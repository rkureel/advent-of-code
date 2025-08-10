use anyhow::{Result, anyhow};

use crate::input::Input;

mod day_1;

pub trait SolutionProvider {
    fn provide_solution(&self, day: u8, input: &Input) -> Result<Solution>;
}

pub struct Solution {
    pub part_1: Option<String>,
    pub part_2: Option<String>,
}

impl Solution {
    pub fn print(&self) {
        let part_1_solution: String = get_displayed_solution_for_part(&self.part_1);
        let part_2_solution: String = get_displayed_solution_for_part(&self.part_2);
        println!("Part 1: [{}]", &part_1_solution);
        println!("Part 2: [{}]", &part_2_solution);
    }
}

pub struct DefaultSolutionProvider {}

impl DefaultSolutionProvider {
    pub fn new() -> Self {
        Self {}
    }
}

impl SolutionProvider for DefaultSolutionProvider {
    fn provide_solution(&self, day: u8, input: &Input) -> Result<Solution> {
        match day {
            1 => day_1::solve(input),
            _ => Err(anyhow!("Solution not implemented for day")),
        }
    }
}

fn get_displayed_solution_for_part(optional_output: &Option<String>) -> String {
    match optional_output {
        Some(output) => output.clone(),
        None => String::from("NOT_IMPLEMENTED"),
    }
}
