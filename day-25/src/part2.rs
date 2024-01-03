use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {

    todo!("day 01 - part 2");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("test");
        let input = include_str!("../test_input2.txt");
        assert_eq!("54", process(input)?);
        Ok(())
    }
}