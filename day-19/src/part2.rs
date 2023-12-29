use std::collections::HashMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let workflows: HashMap<String, Vec<(String, i32, String)>> = _input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim_end_matches('}').split('{');

            let key = parts.next().unwrap_or("").trim().to_string();

            let values: Vec<(String, i32, String)> = parts
                .next()
                .unwrap_or("")
                .split(',')
                .map(|s| {
                    let mut greater: bool = false;
                    let mut components = match s.chars().nth(1) {
                        Some('>') => {
                            greater = true;
                            s.trim().split('>')
                        }
                        Some('<') => {
                            greater = false;
                            s.trim().split('<')
                        }
                        _ => s.trim().split(':'),
                    };

                    let first = components.next().unwrap_or("").trim().to_string();

                    let second_third = components.next().unwrap_or("").trim().split(':');

                    let second = if greater {
                        second_third.clone().next().unwrap_or("").trim().parse().unwrap_or(0)
                    } else {
                        -second_third.clone().next().unwrap_or("").trim().parse().unwrap_or(0)
                    };

                    let third = second_third.last().unwrap_or("").trim().to_string();

                    if first == "x" || first == "m" || first == "a" || first == "s" {
                        (first, second, third)
                    } else {
                        ("default".to_string(), 0, first)
                    }
                })
                .collect();

            (key, values)
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
        assert_eq!("167409079868000", process(input)?);
        Ok(())
    }
}