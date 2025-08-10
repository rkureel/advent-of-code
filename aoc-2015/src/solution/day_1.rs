use anyhow::Result;

use crate::{input::Input, solution::Solution};

pub fn solve(input: &Input) -> Result<Solution> {
    let input_str = input.get_str();
    let part_1_solution: Option<String> = solve_part_1(&input_str);
    let part_2_solution: Option<String> = solve_part_2(&input_str);
    Ok(Solution {
        part_1: part_1_solution,
        part_2: part_2_solution,
    })
}

fn solve_part_1(input: &str) -> Option<String> {
    let mut floor: i32 = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                panic!("Unknown character in input file: {c}")
            }
        }
    }

    Some(String::from(floor.to_string()))
}

fn solve_part_2(input: &str) -> Option<String> {
    let mut floor: i32 = 0;
    let mut ind: u32 = 0;
    for c in input.chars() {
        ind += 1;
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {
                panic!("Unknown character in input file")
            }
        }
        if floor < 0 {
            break;
        }
    }

    Some(String::from((ind).to_string()))
}
