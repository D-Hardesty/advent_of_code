use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut result: u32 = 0;

    for line in _input.lines() {
        let mut cur_val = String::new();

        for char in line.chars() {
            if char < ':' {
                cur_val.push(char);
            }
        }
        let mut digit = String::new();
        digit.push(cur_val.chars().next().unwrap());
        digit.push(cur_val.chars().last().unwrap());

        let digit: u32 = digit.parse().unwrap();
        result += digit;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("142", process(input)?);
        Ok(())
    }
}