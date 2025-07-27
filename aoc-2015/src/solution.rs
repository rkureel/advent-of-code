use std::fmt::Display;

use anyhow::{anyhow, Result};

use crate::input::ProvideInput;

mod day_1;

pub struct Solution {
    pub part_1: Option<String>,
    pub part_2: Option<String>
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Solution for part 1: [{}]\nSolution for part 2: [{}]",
            get_displayed_solution_for_part(&self.part_1),
            get_displayed_solution_for_part(&self.part_2))
    }
}

pub struct SolutionProvider {
    input_provider: Box<dyn ProvideInput>
}

impl SolutionProvider {
    pub fn new(input_provider: Box<dyn ProvideInput>) -> Self {
        Self { 
            input_provider: input_provider 
        }
    }

    pub fn get_solution(&self, day: u8) -> Result<Solution> {
        let input: String = self.input_provider.get_input_as_string(day)
            .expect("Unable to read input");
        self.get_solution_for_day(day, &input)    
    }

    fn get_solution_for_day(&self, day: u8, input: &str) -> Result<Solution> {
        match day {
            1 => day_1::solve(input),
            _ => Err(anyhow!("Solution not implemented for day"))
        }
    }
}

fn get_displayed_solution_for_part(optional_output: &Option<String>) -> String {
   match optional_output {
       Some(output) => output.clone(),
       None => String::from("NOT_IMPLEMENTED")
   } 
}
