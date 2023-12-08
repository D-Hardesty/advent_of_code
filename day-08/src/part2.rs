use std::collections::HashMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // grab directions
    let directions = _input.lines().next();

    // set up the maze
    let maze: HashMap<&str, (&str, &str)> = _input
        .lines()
        .skip(2)
        .map(|key| {
            let mut iter = key.split("=");
            let key = iter.next().unwrap_or_default().trim();
            let values = iter.next().unwrap_or_default().trim();

            let values = values[1..values.len() - 1]
                .split(',')
                .map(|str| str.trim())
                .collect::<Vec<_>>();

            let value1 = values.get(0).cloned().unwrap_or_default();
            let value2 = values.get(1).cloned().unwrap_or_default();

            (key, (value1, value2))
        })
        .collect();

    // find the starting values and store in vector
    let locations: Vec<&str> = maze
        .keys()
        .filter(|&key| key.ends_with('A'))
        .cloned()
        .collect();

    // println!("{:?}", locations);
    let mut result: Vec<usize> = Vec::new();

    for loc in locations.iter() {
        // println!("loc = {}", loc);
        let mut location = loc;
        let mut steps: usize = 0;

        // Find the length of each cycle
        for ch in directions.unwrap().chars().cycle() {
            // println!("Direction: {}", ch);
            // println!("Location Start: {}", location);
            steps += 1;
            match ch {
                'R' => {
                    let direction = maze.get(location).unwrap();
                    // println!("went right");
                    location = &direction.1;
                }
                'L' => {
                    let direction = maze.get(location).unwrap();
                    // println!("went left");
                    location = &direction.0;
                }
                _ => panic!("No Direction Specified"),
            }
            // println!("Location End: {}", location);

            if location.ends_with('Z') {
                // println!("{}", location);
                break;
            }
        }
        // println!("{}", steps);
        result.push(steps);
    }

    // find least common multiple of all the steps
    let min_cycle = lcm(&result);
    Ok(min_cycle.to_string())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("6", process(input)?);
        Ok(())
    }
}