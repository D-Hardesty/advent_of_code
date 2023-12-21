use std::collections::VecDeque;
use crate::custom_error::AocError;

enum DirectionOfMovement {
    Right,
    Left,
    Up,
    Down,
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let directions: Vec<Vec<&str>> = _input
        .lines()
        .map(|line| line
            .split_whitespace()
            .map(|ch| ch.trim_start_matches('(').trim_end_matches(')')).collect()
        ).collect();

    let mut col_pad = 0;
    let mut row_pad = 0;
    let mut left_bound = 0;
    let mut right_bound = 0;
    let mut upper_bound = 0;
    let mut lower_bound = 0;
    for direction in directions.iter() {
        let parsed_val = direction[1].parse::<i32>().unwrap();
        match direction[0] {
            "R" => {
                col_pad += parsed_val;
                if col_pad > right_bound {
                    right_bound = col_pad;
                }
            }
            "L" => {
                col_pad -= parsed_val;
                if col_pad < left_bound {
                    left_bound = col_pad;
                }
            }
            "D" => {
                row_pad += parsed_val;
                if row_pad > lower_bound {
                    lower_bound = row_pad;
                }
            }
            "U" => {
                row_pad -= parsed_val;
                if row_pad < upper_bound {
                    upper_bound = row_pad;
                }
            }
            _ => {
                println!("No pattern matched.")
            }
        }
    }

    let row_length = left_bound.abs() + right_bound;
    let col_height = upper_bound.abs() + lower_bound;
    let mut graph: Vec<Vec<i32>> = vec![vec![0; row_length as usize + 1]; col_height as usize + 1];

    // println!("row: {}, col {}", row_length, col_height);
    // println!("{:?}", graph);

    let mut location: (usize, usize) = (upper_bound.abs() as usize, left_bound.abs() as usize);
    graph[location.0][location.1] = 1;
    println!("start: {:?}", location);
    let mut dom = DirectionOfMovement::Right;
    for direction in directions.iter() {
        let parsed_val = direction[1].parse::<i32>().unwrap();
        // println!("draw loc: {:?}", location);
        // println!("draw direct: {}, draw length: {}", direction[0], direction[1]);
        match direction[0] {
            "R" => {
                dom = DirectionOfMovement::Right;
            }
            "L" => {
                dom = DirectionOfMovement::Left;
            }
            "D" => {
                dom = DirectionOfMovement::Down;
            }
            "U" => {
                dom = DirectionOfMovement::Up;
            }
            _ => {
                println!("No pattern matched.");
            }
        }
        for _ in 0..parsed_val {
            match dom {
                DirectionOfMovement::Right => {
                    location.1 += 1;
                }
                DirectionOfMovement::Left => {
                    location.1 -= 1;
                }
                DirectionOfMovement::Up => {
                    location.0 -= 1;
                }
                DirectionOfMovement::Down => {
                    location.0 += 1;
                }
            }
            graph[location.0][location.1] = 1;
        }
        // println!("{:?}", graph);
    }




    fill_shape(&mut graph, location.0 + 1, location.1 + 1);

    for vec in graph.iter() {
        println!("{:?}", vec);
    }

    let result: usize = graph
        .iter()
        .flatten()
        .filter(|&&x| x == 1)
        .count();


    Ok(result.to_string())
}

fn fill_shape(matrix: &mut Vec<Vec<i32>>, start_row: usize, start_col: usize) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    if matrix[start_row][start_col] != 0 {
        return;
    }

    let mut stack: VecDeque<(usize, usize)> = VecDeque::new();
    stack.push_back((start_row, start_col));

    while let Some((row, col)) = stack.pop_back() {
        if matrix[row][col] == 0 {
            matrix[row][col] = 1;

            // Push neighboring cells onto the stack
            if row > 0 {
                stack.push_back((row - 1, col));
            }
            if row < rows - 1 {
                stack.push_back((row + 1, col));
            }
            if col > 0 {
                stack.push_back((row, col - 1));
            }
            if col < cols - 1 {
                stack.push_back((row, col + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("62", process(input)?);
        Ok(())
    }
}