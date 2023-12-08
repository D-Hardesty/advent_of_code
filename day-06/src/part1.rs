use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<&str> = _input
        .lines()
        .collect();

    let input: Vec<(u32, u32)> = lines[0]
        .split_whitespace()
        .zip(lines[1].split_whitespace())
        .filter_map(|(first, second)| {
            match (first.parse(), second.parse()) {
                (Ok(first_num), Ok(second_num)) => Some((first_num, second_num)),
                _ => None,
            }
        })
        .collect();

    let mut result = 1;

    for item in input.iter() {
        let low = search_left(item.0, item.1);
        // println!("low: {}", low);
        // println!("high: {}", item.0 - low);
        // let high = search_right(item.0, item.1);
        result *= (item.0 - low) - low + 1;
    }

    // println!("{:?}\n", input);
    Ok(result.to_string())
}

fn search_left(bounds: u32, target: u32) -> u32 {
    let mut left = 0;
    let mut right = bounds / 2;

    while left < right {
        let mid = left + (right - left) / 2;

        if mid * (bounds - mid) > target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("288", process(input)?);
        Ok(())
    }
}