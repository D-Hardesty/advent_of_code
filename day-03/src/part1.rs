use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut schematics: Vec<Vec<char>> = Vec::new();
    let mut parts: Vec<u32> = Vec::new();
    let mut symbols: Vec<Vec<(usize, usize)>> = Vec::new();

    for line in _input.lines() {
        let mut inner_vec: Vec<char> = Vec::new();
        for char in line.chars() {
            inner_vec.push(char);
        }
        schematics.push(inner_vec);
    }

    for (row_i, row) in schematics.iter().enumerate() {
        for (col_i, &char) in row.iter().enumerate() {
            if char.is_ascii_punctuation() && char != '.' {
                symbols.push(vec![
                    (row_i - 1, col_i - 1), (row_i - 1, col_i), (row_i - 1, col_i + 1),
                    (row_i, col_i - 1), (row_i, col_i + 1), (row_i + 1, col_i - 1),
                    (row_i + 1, col_i), (row_i + 1, col_i + 1)]);
            }
        }
    }

    let mut last_part = 0;
    for symbol in symbols.iter() {
        for &(row, col) in symbol {
            if schematics[row][col].is_digit(10) {
                let part_num = get_full_number(&schematics[row], col);
                // println!("last: {}, new: {}", last_part, part_num);
                if last_part != part_num {
                    // println!("added: {}", part_num);
                    last_part = part_num;
                    parts.push(part_num);
                }
            }
        }
        last_part = 0;
    }
    let result: u32 = parts.iter().sum();

    Ok(result.to_string())
}

fn get_full_number(vec: &Vec<char>, col: usize) -> u32 {
    let mut number = String::new();
    let mut cur_index = col;

    if vec[col].is_digit(10) {
        // go left
        while cur_index > 0 && vec[cur_index - 1].is_digit(10) {
            // println!("In first while");
            cur_index -= 1;
        }

        // go right
        while cur_index < vec.len() && vec[cur_index].is_digit(10) {
            // println!("In second while");
            number.push(vec[cur_index]);
            cur_index += 1;
        }
        // println!("number: {}", number);
    }

    number.parse::<u32>().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}