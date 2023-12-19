use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let hot_springs: Vec<_> = _input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .and_then(|format_string| {
                    line.split_whitespace()
                        .nth(1)
                        .map(|numbers_str| {
                            let numbers: Vec<usize> = numbers_str
                                .split(',')
                                .filter_map(|n| n.parse().ok())
                                .collect();
                            (format_string.to_string(), numbers)
                        })
                })
        })
        .filter_map(|x| x)
        .collect();
    println!("hot_springs: {:?}", hot_springs);

    let mut result = 0;

    for row in hot_springs {
        let str_len = row.0.len();
        let group_num = row.1.len();
        let mut inner_bounds: usize = row.1.iter().sum();
        inner_bounds += group_num - 1;

        println!("str_len: {}, group_num: {}, inner_bounds: {}", str_len, group_num, inner_bounds);

        // if str_len - inner < 3 permutations = 1;
        if str_len - inner_bounds < 3 {
            result += 1;
            continue;
        }

        // println!("{:?}", row);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("21", process(input)?);
        Ok(())
    }
}

