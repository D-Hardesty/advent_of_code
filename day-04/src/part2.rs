use regex::Regex;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let lines: Vec<String> = _input
        .lines()
        .map(|line| line.to_string())
        .collect();

    let mut card_numbers: Vec<Vec<u32>> = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();

    // create vector containing only numbers
    for (index, line) in lines.iter().enumerate() {
        card_numbers.push(re
            .find_iter(&line)
            .map(|m| m.as_str().parse().unwrap())
            .collect());
        // change all card values to 1
        if let Some(first_num) = card_numbers[index].first_mut() {
            *first_num = 1;
        }
    }

    let mut result = 0;

    for index in 0..card_numbers.len() {
        let copies = card_numbers[index][0];

        // winning numbers = 1..11
        // my numbers = 11..
        let num_matches = card_numbers[index][1..11]
            .iter()
            .filter(|&x| card_numbers[index][11..].contains(x))
            .count();
        // println!("Card Index: {}, Card scratching: {:?}",index, card_numbers[index]);

        // Test input
        // let num_matches = card_numbers[index][1..6]
        //     .iter()
        //     .filter(|&x| card_numbers[index][6..].contains(x))
        //     .count();
        // println!("matches: {}", num_matches);

        for _ in 0..copies {
            clone_tickets(&mut card_numbers, num_matches, index + 1);
        }
        result += copies;
        // println!("Results: {}\n", result);
    }
    Ok(result.to_string())
}

fn clone_tickets(vec: &mut Vec<Vec<u32>>, clones: usize, mut index: usize) {
    for _ in 0..clones {
        if let Some(first_num) = vec[index].first_mut() {
            *first_num += 1;
        } else {
            println!("End of Vector: Out of range.")
        }
        index += 1;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_process() -> miette::Result<()> {
//         let input = include_str!("../test_input2.txt");
//         assert_eq!("30", process(input)?);
//         Ok(())
//     }
// }