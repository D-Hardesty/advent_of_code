use std::collections::BTreeMap;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let directions = _input.lines().next();
    let maze: BTreeMap<&str, (&str, &str)> = _input
        .lines()
        .skip(2)
        .map(|key| {
            let mut iter = key.split("=");
            let key = iter.next().unwrap_or_default().trim();
            let values = iter.next().unwrap_or_default().trim();

            let values = values[1..values.len() - 1]
                .split(',')
                .map(|str| str.trim())
                .collect::<Vec<_>>();

            let value1 = values.get(0).cloned().unwrap_or_default();
            let value2 = values.get(1).cloned().unwrap_or_default();

            (key, (value1, value2))
        })
        .collect();
    // println!("{:?}", maze);

    let mut result = 0;
    let (key, (_, _)) = maze.iter().next().unwrap();
    let mut location = key;

    for ch in directions.unwrap().chars().cycle() {
        result += 1;
        match ch {
            'R' => {
                let direction = maze.get(location).unwrap();
                location = &direction.1;
            }
            'L' => {
                let direction = maze.get(location).unwrap();
                location = &direction.0;
            }
            _ => panic!("No Direction Specified"),
        }
        if location == &"ZZZ" {
            break;
        }
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("6", process(input)?);
        Ok(())
    }
}