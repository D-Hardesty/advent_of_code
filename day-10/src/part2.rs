use crate::custom_error::AocError;
use std::collections::{HashMap, HashSet};

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let graph: HashMap<(i32, i32), Vec<(i32, i32)>> = _input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate().flat_map(move |(index, val)| {
                let mut r1: i32 = row as i32;
                let mut r2: i32 = row as i32;

                let mut c1: i32 = index as i32;
                let mut c2: i32 = index as i32;
                match val {
                    '|' => {
                        r1 -= 1;
                        r2 += 1;
                    }
                    '-' => {
                        c1 += 1;
                        c2 -= 1;
                    }
                    'L' => {
                        r1 -= 1;
                        c2 += 1;
                    }
                    'J' => {
                        r1 -= 1;
                        c2 -= 1;
                    }
                    '7' => {
                        r1 += 1;
                        c2 -= 1;
                    }
                    'F' => {
                        r1 += 1;
                        c2 += 1;
                    }
                    _ => {}
                }
                vec![
                    ((row as i32, index as i32), (r1, c1)),
                    ((row as i32, index as i32), (r2, c2)),
                ]
            })
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert(vec![]).push(value);
            acc
        });
    let mut starting_node: Vec<(i32, i32)> = Vec::new();
    'outer: for (row, line) in _input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == 'S' {
                starting_node.push((row as i32, col as i32));

                starting_node.push((row as i32 + 1, col as i32));
                starting_node.push((row as i32 - 1, col as i32));
                starting_node.push((row as i32, col as i32 - 1));
                starting_node.push((row as i32, col as i32 + 1));
                break 'outer;
            }
        }
    }

    println!("starting: {:?}", starting_node);
    // println!("{:?}", graph);
    let mut result: usize = 0;
    for &start_node in starting_node.iter().skip(1) {
        println!("{:?}", start_node);
        let path_len = traverse_maze(&graph, &start_node, &starting_node[0]);
        if result < path_len {
            result = path_len;
        }
    }
    result /= 2;


    Ok(result.to_string())
}

fn traverse_maze(graph: &HashMap<(i32, i32), Vec<(i32, i32)>>, start: &(i32, i32), origin: &(i32,
                                                                                             i32))
                 -> usize {
    let mut last_node = origin.clone();
    let mut cur_node = start.clone();
    let mut length: usize = 0;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while !visited.contains(&cur_node) || cur_node != *origin {
        visited.insert(cur_node);
        if let Some(nodes) = graph.get(&cur_node) {
            let node1 = nodes[0];
            let node2 = nodes[1];

            if node1 == last_node {
                last_node = cur_node;
                cur_node = node2;
            } else if node2 == last_node {
                last_node = cur_node;
                cur_node = node1;
            } else if node1 == *origin || node2 == *origin {
                println!("At start");
                length += 1;
                break;
            } else {
                println!("Dead end");
                break;
            }
            length += 1;
        }
    }
    length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("4", process(input)?);
        Ok(())
    }
}
