use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let numeric_words: [(&str, &str); 9] = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let reconstructed_input: Vec<String> = _input
        .lines()
        .map(|line| {
            numeric_words.iter().fold(String::from(line), |acc, (word, digit)| {
                acc.replace(word, digit)
            })
        }).collect();

    let result: u32 = reconstructed_input
        .iter()
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
        let input = include_str!("../test_input2.txt");
        assert_eq!("281", process(input)?);
        Ok(())
    }
}
