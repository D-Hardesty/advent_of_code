use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<&str> = _input
        .lines()
        .collect();

    let input: Vec<u64> = lines
        .iter()
        .flat_map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .ok()
        })
        .collect();

    let low = search_left(input[0], input[1]);
    // let high = search_right(input[0], input[1]);

    Ok(((input[0] - low) - low + 1).to_string())
}

fn search_left(bounds: u64, target: u64) -> u64 {
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
        let input = include_str!("../test_input2.txt");
        assert_eq!("71503", process(input)?);
        Ok(())
    }
}