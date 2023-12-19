use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let graph: Vec<Vec<i32>> = _input
        .lines()
        .map(|line| line
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as i32).collect()
        ).collect();

    println!("{:?}", graph);

    let start = (0, 0);
    let end = (graph[0].len() - 1, graph.len() - 1);

    let result = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("102", process(input)?);
        Ok(())
    }
}