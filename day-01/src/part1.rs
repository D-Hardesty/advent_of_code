use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let result: u32 = _input
        .lines()
        .map(|line| line.chars().filter(|&ch| ch < ':').collect::<String>())
        .map(|cur_val| {
            let digit = format!(
                "{}{}",
                cur_val.chars().next().unwrap(),
                cur_val.chars().last().unwrap()
            );
            digit.parse::<u32>().unwrap()
        })
        .sum();

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
