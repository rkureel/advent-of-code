use anyhow::Result;

use crate::solution::Solution;

pub fn solve(input: &str) -> Result<Solution> {
    let part_1_solution: Option<String> = solve_part_1(input);
    Ok(Solution { part_1: part_1_solution, part_2: Option::None })
}

fn solve_part_1(input: &str) -> Option<String> {
    let mut floor: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                panic!("Unknown character in input file")
            }
        }
    }
     
    Some(String::from(floor.to_string()))
}
