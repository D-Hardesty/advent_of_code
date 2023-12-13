use std::collections::HashSet;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut col_w_galaxy: HashSet<usize> = HashSet::new();
    let mut row_w_galaxy: HashSet<usize> = HashSet::new();

    // get rows and col with galaxies
    for (row, line) in _input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                col_w_galaxy.insert(col);
                row_w_galaxy.insert(row);
            }
        }
    }
    let mut galaxy_loc: Vec<(i32, i32)> = Vec::new();

    // // create string with extra space and locate galaxies
    // // can maybe get rid of the map
    // let _galaxy_map: Vec<Vec<String>> = _input
    //     .lines()
    //     .enumerate()
    //     .map(|(row, line)| {
    //         line
    //             .chars()
    //             .enumerate()
    //             .map(|(col, ch)| {
    //                 if ch == '#' {
    //                     galaxy_loc.push((row as i32, col as i32));
    //                 }
    //                 if row_w_galaxy.contains(&row) || col_w_galaxy.contains(&col) {
    //                     ch.to_string()
    //                 } else {
    //                     ".".to_string()
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();

    // for row in _galaxy_map {
    //     println!("{}", row.join(""));
    // }

    let mut row_offset = 0;
    for (row, line) in _input.lines().enumerate() {
        if !row_w_galaxy.contains(&row) {
            row_offset += 1;
            continue;
        }
        let mut col_offset = 0;
        for (col, ch) in line.chars().enumerate() {
            if !col_w_galaxy.contains(&col) {
                col_offset += 1;
            } else {
                if ch == '#' {
                    galaxy_loc.push(((row + row_offset) as i32, (col + col_offset) as i32))
                }
            }
        }
    }


    // println!("rows with galaxy: {:?}", row_w_galaxy);
    // println!("col with galaxy: {:?}", col_w_galaxy);
    // println!("Galaxy locations: {:?}", galaxy_loc);

    let mut result = 0;
    for (i, gal) in galaxy_loc.iter().enumerate() {
        for other_gal in galaxy_loc.iter().skip(i + 1) {
            let steps = (gal.0 - other_gal.0).abs() + (gal.1 - other_gal.1).abs();
            // println!("gal {} - ogal {} = {}, gal {} - ogal {} = {}, steps {}", gal.0, other_gal.0, gal.0 - other_gal.0,
            //          gal.1, other_gal.1, gal.1 - other_gal.1, steps);
            result += steps;
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
        assert_eq!("374", process(input)?);
        Ok(())
    }
}