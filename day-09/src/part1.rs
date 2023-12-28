use crate::custom_error::AocError;
use itertools::{Itertools, Position};

todo!("Refactor");
#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result = _input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let mut end_nums: Vec<i64> = vec![];
            loop {
                if nums.iter().all(|n| n == &0) {
                    break;
                }
                nums = nums
                    .iter()
                    .tuple_windows::<(&i64, &i64)>()
                    .with_position()
                    .map(|(pos, (left, right))| {
                        match pos {
                            Position::Last | Position::Only => {
                                end_nums.push(*right);
                            }
                            _ => {}
                        }
                        right - left
                    })
                    .collect::<Vec<i64>>();
            }
            let result = end_nums.iter().fold(0, |acc, num| acc + num);
            result
        })
        .sum::<i64>();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("114", process(input)?);
        Ok(())
    }
}
