use crate::custom_error::AocError;
use itertools::Itertools;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let horizontal: Vec<Vec<String>> = _input
        .lines()
        .group_by(|line| !line.trim().is_empty())
        .into_iter()
        .filter_map(|(_, group)| {
            let group: Vec<String> = group
                .filter(|line| !line.trim().is_empty())
                .map(|line| {
                    line.chars()
                        .map(|ch| if ch == '#' { '1' } else { '0' })
                        .collect()
                })
                .collect();
            if !group.is_empty() {
                Some(group)
            } else {
                None
            }
        })
        .collect();
    // println!("{:?}", horizontal);


    // horizontal to binary
    let horizontal_binary: Vec<Vec<u64>> = horizontal
        .iter()
        .map(|shape| {
            shape
                .iter()
                .map(|s| {
                    binary_string_to_number(&s)
                }).collect()
        }).collect();

    // println!("hori_b: {:?}", horizontal_binary);

    let vertical: Vec<Vec<u64>> = horizontal
        .iter()
        .map(|shape| {
            (0..shape[0].len())
                .map(|col| {
                    let vert_num: String = shape.iter().map(|row| row.chars().nth(col).unwrap()).collect();
                    binary_string_to_number(&vert_num)
                })
                .collect()
        })
        .collect();

    // println!("vert_b: {:?}", vertical);

    let mut result: usize = 0;
    for idx in 0..horizontal_binary.len() {
        result += mirror_check(&horizontal_binary[idx]) * 100;
        result += mirror_check(&vertical[idx]);
    }

    Ok(result.to_string())
}

fn mirror_check(shape: &Vec<u64>) -> usize {
    let mut result: usize = 0;
    let mut last: u64 = shape[0];
    let mut cur: u64;
    for idx in 1..shape.len() {
        cur = shape[idx];
        if cur == last {
            if bounce_back(idx - 1, idx, &shape) {
                result = idx;
                break;
            }
        }
        // println!("last: {}, cur {}", last, cur);
        last = cur;
    }
    // println!("res: {}", result);
    result
}

fn bounce_back(mut left: usize, mut right: usize, vec: &Vec<u64>) -> bool {
    let mut res = true;
    while right < vec.len() {
        if vec[left] != vec[right] {
            res = false
        }
        if left == 0 {
            break;
        }
        left -= 1;
        right += 1;
    }
    res
}

fn binary_string_to_number(binary_str: &str) -> u64 {
    u64::from_str_radix(binary_str, 2).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("405", process(input)?);
        Ok(())
    }
}