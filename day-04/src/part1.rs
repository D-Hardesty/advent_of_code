use regex::Regex;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let re = Regex::new(r"(\d+)").unwrap();
    let mut result = 0;

    for line in _input.lines() {
        let digits: Vec<u32> = re
            .find_iter(&line)
            .map(|m| m.as_str().parse().unwrap())
            .collect();

        let winning_numbers: Vec<u32> = digits[1..11].to_vec();
        let my_numbers: Vec<u32> = digits[11..].to_vec();

        let num_matches = my_numbers
            .iter()
            .filter(|&x| winning_numbers.contains(x))
            .count();

        match num_matches {
            0 => result += 0,
            1 => result += 1,
            n if n >= 2 => {
                result += scoring(num_matches as i32 - 1)
            }
            _ => unreachable!(),
        }

        // println!("winning: {:?}", winning_numbers);
        // println!("my numbers: {:?}", my_numbers);
        // println!("matches: {}", num_matches);
        // println!("results {}", result);
    }

    Ok(result.to_string())
}

pub fn scoring(mut matches: i32) -> u32 {
    let mut score = 1;
    while matches > 0 {
        score *= 2;
        matches -= 1;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("1364", process(input)?);
        Ok(())
    }
}