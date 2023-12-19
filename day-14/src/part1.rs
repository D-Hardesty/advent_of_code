use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut result = 0;

    // collect lines into a vector
    let lines: Vec<&str> = _input.lines().collect();

    // transpose the vector
    let transposed: Vec<String> = (0..lines[0].len())
        .map(|col| {
            lines
                .iter()
                .map(|line| line.chars().nth(col).unwrap())
                .collect()
        })
        .collect();

    // find score
    for row in transposed.iter() {
        let mut dist_from_edge = row.len();
        for (dist, ch) in row.chars().enumerate() {
            match ch {
                '#' => {
                    dist_from_edge = row.len() - dist - 1;
                }
                'O' => {
                    result += dist_from_edge;
                    dist_from_edge -= 1;
                }
                _ => {}
            }
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
        assert_eq!("136", process(input)?);
        Ok(())
    }
}
