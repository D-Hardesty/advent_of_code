use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let workflows: Vec<String> = _input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .flat_map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|s| s.trim().to_string())
        })
        .collect();

    println!("workflows: {:?}", workflows);

    let result = 0;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}