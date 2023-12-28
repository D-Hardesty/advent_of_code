use std::collections::HashMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    // {"px": ("a", 2006, "qkq"), ("m", -2090, "A"), ("default", 0, "rfg")}

    let workflows: HashMap<&str, Vec<&str>> = _input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.trim_end_matches('}').split('{');
            let key = parts.next().unwrap().trim();
            let values: Vec<&str> = parts
                .next().unwrap()
                .split(',')
                .map(|s| s.replace(">","").replace("<","-").trim()).collect();
            (key, values)
        }).collect();


    let item_stats: Vec<Vec<String>> = _input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        })
        .collect();

    println!("workflows: {:?}", workflows);
    println!("item stats: {:?}", item_stats);

    let result = 0;
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}