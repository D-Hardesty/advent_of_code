use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    // let hash: i32 = _input
    //     .split(',')
    //     .map(|str| {
    //         let mut val = 0;
    //         for ch in str.chars() {
    //             val += ch as i32;
    //             val *= 17;
    //             val = val % 256;
    //         }
    //         val
    //     }).sum();

    let hash: i32 = _input
        .split(',')
        .map(|str| str.chars().fold(0, |acc, ch| (acc + ch as i32) * 17 % 256))
        .sum();

    Ok(hash.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("1320", process(input)?);
        Ok(())
    }
}
