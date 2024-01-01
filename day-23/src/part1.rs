use std::collections::HashSet;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let hiking_path: Vec<Vec<char>> = _input.lines().map(|line| {
        line.chars().map(|ch| ch).collect()
    }).collect();


    // let mut path_lengths: Vec<usize> = Vec::new();

    let end_point = (hiking_path.len() - 1, hiking_path[0].len() - 2);
    // println!("hiking path: {:?}", hiking_path);
    let mut last_loc: (usize, usize) = (0, 1);
    let mut path_stack: Vec<((usize, usize), usize, HashSet<(usize, usize)>, (usize, usize))> =
        Vec::new();

    // start at second location with 1 step (x, y, steps)
    let mut result = 0;
    let directions = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut cur_steps = 1;
    path_stack.push(((1, 1), cur_steps, visited, last_loc));
    // println!("end point: {:?}", end_point);
    let mut cur_loc_hold = (0, 0);
    let mut last_loc_hold = (0, 0);

    while let Some(cur_path) = path_stack.pop() {
        // println!("{:?}", cur_path);
        cur_steps = cur_path.1;

        let mut cur_loc = cur_path.0;
        last_loc = cur_path.3;
        visited = cur_path.2;

        while cur_loc != end_point {
            let mut no_slope = false;
            // println!("cur_loc: {:?}", cur_loc);
            // println!("steps: {:?}", cur_steps);
            if visited.contains(&cur_loc) {
                // println!("Path already traveled");
                break;
            }
            visited.insert(cur_loc);
            cur_steps += 1;
            // println!("cur_loc: {:?}", cur_loc);

            // no dead ends?
            let mut walls = 0;
            // x, y, steps, visited, last
            for dir in directions.iter() {
                let neighbor = ((cur_loc.0 as isize + dir.0) as usize, (cur_loc.1 as isize + dir.1) as usize);

                if neighbor != last_loc {
                    // println!("in neighbor");
                    // println!("neighbor: {:?}", neighbor);
                    // println!("neighbor symbol: {}", hiking_path[neighbor.0][neighbor.1]);
                    match hiking_path[neighbor.0][neighbor.1] {
                        '.' => {
                            no_slope = true;
                            last_loc_hold = cur_loc;
                            cur_loc_hold = neighbor;
                        }
                        '<' => {
                            if cur_loc != (neighbor.0, neighbor.1 - 1) {
                                path_stack.push((neighbor, cur_steps,
                                                 visited.clone(), cur_loc.clone()));
                            }
                        }
                        '>' => {
                            if cur_loc != (neighbor.0, neighbor.1 + 1) {
                                path_stack.push((neighbor, cur_steps,
                                                 visited.clone(), cur_loc.clone()));
                            }
                        }
                        'v' => {
                            if cur_loc != (neighbor.0 + 1, neighbor.1) {
                                path_stack.push((neighbor, cur_steps,
                                                 visited.clone(), cur_loc.clone()));
                            }
                        }
                        '^' => {
                            if cur_loc != (neighbor.0 - 1, neighbor.1) {
                                path_stack.push((neighbor, cur_steps,
                                                 visited.clone(), cur_loc.clone()));
                            }
                        }
                        _ => {
                            walls += 1;
                        }
                    }
                }
            }
            if no_slope {
                cur_loc = cur_loc_hold;
                last_loc = last_loc_hold;
            }
            if walls == 3 {
                println!("Dead end");
                break;
            }
        }
        if cur_loc == end_point {
            // path_lengths.push(cur_steps);
            if result < cur_steps {
                result = cur_steps;
            }
        }
        visited.clear();
        // println!("result: {}", result);
    }

    // println!("path_lengths: {:?}", path_lengths);
    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("94", process(input)?);
        Ok(())
    }
}