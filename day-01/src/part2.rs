use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
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

    let mut result: u32 = 0;

    for line in _input.lines() {
        let mut reconstructed_string = String::from(line);

        for (word, digit) in &numeric_words {
            reconstructed_string = reconstructed_string.replace(word, digit);
        }

        let mut cur_val = String::new();
        for char in reconstructed_string.chars() {
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
        let input = include_str!("../test_input2.txt");
        assert_eq!("281", process(input)?);
        Ok(())
    }
}