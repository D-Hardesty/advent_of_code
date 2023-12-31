use std::collections::HashSet;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut start: Option<(usize, usize)> = None;
    let garden: Vec<Vec<usize>> = _input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line
                .chars()
                .enumerate()
                .map(|(col, ch)| {
                    match ch {
                        '.' => 1,
                        '#' => 0,
                        'S' => {
                            start = Some((row, col));
                            1
                        }
                        _ => {
                            0
                        }
                    }
                }).collect()
        }).collect();
    // println!("Garden: {:?}", garden);
    // println!("Start: {:?}", start);

    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut new_locations: HashSet<(usize, usize)> = HashSet::new();
    stack.push(start.unwrap());
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let num_of_steps = 64;
    for step in 0..num_of_steps {
        while !stack.is_empty() {
            let cur_loc = stack.pop().unwrap();
            for dir in &directions {
                let neighbor = ((cur_loc.0 as isize + dir.0) as usize, (cur_loc.1 as isize + dir.1) as usize);

                if garden.get(neighbor.0).map_or(false, |row| row.get(neighbor.1).map_or(false, |&value| value == 1)) {
                    new_locations.insert(neighbor);
                }
            }
            // println!("newLocations: {:?}", new_locations);
        }
        if step < num_of_steps - 1 {
            for loc in new_locations.iter() {
                stack.push(*loc);
            }
            new_locations.clear();
        }
        // println!("stack: {:?}", stack);
    }

    let result = new_locations.len();
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("16", process(input)?);
        Ok(())
    }
}