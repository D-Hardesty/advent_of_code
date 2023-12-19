use std::collections::HashSet;
use crate::custom_error::AocError;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,

}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mirror_config: Vec<Vec<char>> = _input.lines()
        .map(|line|
            line.chars()
                .map(|ch| ch).collect()
        ).collect();

    let width: i32 = mirror_config[0].len() as i32;
    let height: i32 = mirror_config.len() as i32;
    let mut energized: Vec<Vec<bool>> = vec![vec![false; width as usize]; height as usize];

    let mut stack: Vec<(i32, i32, Direction)> = Vec::new();
    stack.push((0, 0, Direction::Right));
    let mut visited: HashSet<(i32, i32, Direction)> = HashSet::new();
    while let Some(mut laser) = stack.pop() {
        // run laser through maze till it hits a wall
        // println!("Start Laser: {:?}", laser);
        while laser.0 >= 0 && laser.0 < height && laser.1 >= 0 && laser.1 < width {
            energized[laser.0 as usize][laser.1 as usize] = true;
            // println!("symbol {}, laser: {:?}", mirror_config[laser.0 as usize][laser.1 as usize],
            //          laser);
            match mirror_config[laser.0 as usize][laser.1 as usize] {
                '\\' => {
                    match laser.2 {
                        Direction::Right => {
                            laser = (laser.0 + 1, laser.1, Direction::Down);
                        }
                        Direction::Left => {
                            laser = (laser.0 - 1, laser.1, Direction::Up);
                        }
                        Direction::Up => {
                            laser = (laser.0, laser.1 - 1, Direction::Left);
                        }
                        Direction::Down => {
                            laser = (laser.0, laser.1 + 1, Direction::Right);
                        }
                    }
                }
                '/' => {
                    match laser.2 {
                        Direction::Right => {
                            laser = (laser.0 - 1, laser.1, Direction::Up);
                        }
                        Direction::Left => {
                            laser = (laser.0 + 1, laser.1, Direction::Down);
                        }
                        Direction::Up => {
                            laser = (laser.0, laser.1 + 1, Direction::Right);
                        }
                        Direction::Down => {
                            laser = (laser.0, laser.1 - 1, Direction::Left);
                        }
                    }
                }
                '-' => {
                    match laser.2 {
                        Direction::Right => {
                            laser = (laser.0, laser.1 + 1, Direction::Right);
                        }
                        Direction::Left => {
                            laser = (laser.0, laser.1 - 1, Direction::Left);
                        }
                        Direction::Up => {
                            if visited.contains(&laser) {
                                break;
                            }
                            visited.insert((laser.0, laser.1, Direction::Up));
                            visited.insert((laser.0, laser.1, Direction::Down));
                            stack.push((laser.0, laser.1 + 1, Direction::Right));
                            laser = (laser.0, laser.1 - 1, Direction::Left);
                        }
                        Direction::Down => {
                            if visited.contains(&laser) {
                                break;
                            }
                            visited.insert((laser.0, laser.1, Direction::Up));
                            visited.insert((laser.0, laser.1, Direction::Down));
                            stack.push((laser.0, laser.1 + 1, Direction::Right));
                            laser = (laser.0, laser.1 - 1, Direction::Left);
                        }
                    }
                }
                '|' => {
                    match laser.2 {
                        Direction::Right => {
                            if visited.contains(&laser) {
                                break;
                            }
                            visited.insert((laser.0, laser.1, Direction::Right));
                            visited.insert((laser.0, laser.1, Direction::Left));
                            stack.push((laser.0 - 1, laser.1, Direction::Up));
                            laser = (laser.0 + 1, laser.1, Direction::Down);
                        }
                        Direction::Left => {
                            if visited.contains(&laser) {
                                break;
                            }
                            visited.insert((laser.0, laser.1, Direction::Right));
                            visited.insert((laser.0, laser.1, Direction::Left));
                            stack.push((laser.0 - 1, laser.1, Direction::Up));
                            laser = (laser.0 + 1, laser.1, Direction::Down);
                        }
                        Direction::Up => {
                            laser = (laser.0 - 1, laser.1, Direction::Up);
                        }
                        Direction::Down => {
                            laser = (laser.0 + 1, laser.1, Direction::Down);
                        }
                    }
                }
                _ => {
                    match laser.2 {
                        Direction::Right => laser.1 += 1,
                        Direction::Left => laser.1 -= 1,
                        Direction::Up => laser.0 -= 1,
                        Direction::Down => laser.0 += 1,
                    }
                }
            }

        }
    }

    let result: usize = energized
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&b| b == true)
        .count();


    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
