use anyhow::Result;
use crate::{input::Input, solution::Solution};

struct Dimensions {
    length: u32,
    width: u32,
    height: u32
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

    let total_paper_required: u64 = input.lines()
        .map(|line| {
            let dimensions: Dimensions = parse_dimensions(line);
            let paper_required_for_present: u32 = get_total_area_for_present(&dimensions) + get_area_of_smallest_side(&dimensions);
            paper_required_for_present as u64
        })
        .sum();

    Option::Some(total_paper_required.to_string())
}

fn solve_part_2(input: &str) -> Option<String> {
    let total_ribbon_required: u64 = input.lines()
        .map(|line| {
            let dimensions: Dimensions = parse_dimensions(line);
            let ribbon_required_for_present: u32 = get_perimeter_of_smallest_side(&dimensions) + get_volume(&dimensions);
            ribbon_required_for_present as u64
        })
        .sum();

    Option::Some(total_ribbon_required.to_string())
}

fn parse_dimensions(line: &str) -> Dimensions {
    let dims: Vec<&str> = line.split("x").collect();    
    assert!(dims.len() == 3);

    Dimensions { 
        length: dims.get(0).unwrap().parse().unwrap(), 
        width: dims.get(1).unwrap().parse().unwrap(), 
        height: dims.get(2).unwrap().parse().unwrap()
    }
}

fn get_total_area_for_present(dimensions: &Dimensions) -> u32 {
    return 2 * dimensions.height * dimensions.width
        + 2 * dimensions.height * dimensions.length
        + 2 * dimensions.length * dimensions.width;
}

fn get_area_of_smallest_side(dimensions: &Dimensions) -> u32 {
    if dimensions.length >= dimensions.height && dimensions.length >=dimensions.width {
        return dimensions.height * dimensions.width
    } else if dimensions.width >= dimensions.height && dimensions.width >= dimensions.height {
        return dimensions.height * dimensions.length
    } else {
        return dimensions.length * dimensions.width
    }
}

fn get_perimeter_of_smallest_side(dimensions: &Dimensions) -> u32 {
    if dimensions.length >= dimensions.height && dimensions.length >=dimensions.width {
        return 2 * (dimensions.height + dimensions.width)
    } else if dimensions.width >= dimensions.height && dimensions.width >= dimensions.height {
        return 2 * (dimensions.height + dimensions.length)
    } else {
        return 2 * (dimensions.length + dimensions.width)
    }
}

fn get_volume(dimensions: &Dimensions) -> u32 {
    dimensions.width * dimensions.height * dimensions.length
}
