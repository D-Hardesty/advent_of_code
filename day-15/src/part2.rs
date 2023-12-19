use std::collections::BTreeMap;
use std::hash::Hash;
use itertools::Itertools;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let mut boxes: [BTreeMap<&str, &str>; 256] = array_init::array_init(|_| BTreeMap::new());

    let mut result = 0;

    for lens_num in _input.split(',') {
        if lens_num.contains('=') {
            let parts: Vec<&str> = lens_num.split('=').collect();
            let hash: usize = hash(parts[0]);

            boxes[hash].insert(parts[0], parts[1]);
        } else {
            let removed_value = &lens_num[..lens_num.len() - 1];
            let hash: usize = hash(removed_value);
            println!("{}", removed_value);
            boxes[hash].remove(removed_value);
        }
    }

    for _box in boxes.iter() {
        println!("box: {:?}", _box);
    }
    Ok(result.to_string())
}

fn hash(_input: &str) -> usize {
    let hash: usize = _input
        .chars()
        .fold(0, |acc, ch| (acc + ch as usize) * 17 % 256);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("145", process(input)?);
        Ok(())
    }
}
