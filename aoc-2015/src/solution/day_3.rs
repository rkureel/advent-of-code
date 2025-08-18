use std::collections::HashSet;

use anyhow::Result;
use crate::{input::Input, solution::Solution};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct HouseLocation {
    x: i32,
    y: i32,
}

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
    let mut visited_houses: HashSet<HouseLocation> = HashSet::new();
    let mut current_location: HouseLocation = HouseLocation{x: 0, y: 0};
    visited_houses.insert(current_location.clone());

    input.chars()
        .for_each(|instruction| {
            update_current_location(instruction, &mut current_location);
            visited_houses.insert(current_location.clone());
        });

    Some(visited_houses.len().to_string())
}

fn solve_part_2(input: &str) -> Option<String> {
    let mut visited_houses: HashSet<HouseLocation> = HashSet::new();
    let mut santa_location: HouseLocation = HouseLocation{x: 0, y: 0};
    let mut robo_location: HouseLocation = HouseLocation{x: 0, y: 0};
    visited_houses.insert(santa_location.clone());

    input.chars()
        .enumerate()
        .for_each(|(ind, instruction)| {
            if ind % 2 == 0 {
                update_current_location(instruction, &mut santa_location);
                visited_houses.insert(santa_location.clone());
            } else {
                update_current_location(instruction, &mut robo_location);
                visited_houses.insert(robo_location.clone());
            }
        });

    Some(visited_houses.len().to_string())
}

fn update_current_location(instruction: char, current_location: &mut HouseLocation) {
    match instruction {
        '^' => current_location.y += 1,
        '>' => current_location.x += 1,
        'v' => current_location.y -= 1,
        '<' => current_location.x -= 1,
        other => panic!("Invalid character in input: [{}]", other)
    }
}
