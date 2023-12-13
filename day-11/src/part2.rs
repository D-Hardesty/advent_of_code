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
    let mut galaxy_loc: Vec<(i64, i64)> = Vec::new();

    let mut row_offset = 0;
    for (row, line) in _input.lines().enumerate() {
        if !row_w_galaxy.contains(&row) {
            row_offset += 999999;
            continue;
        }
        let mut col_offset = 0;
        for (col, ch) in line.chars().enumerate() {
            if !col_w_galaxy.contains(&col) {
                col_offset += 999999;
            } else {
                if ch == '#' {
                    galaxy_loc.push(((row + row_offset) as i64, (col + col_offset) as i64))
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
            result += steps;
        }
    }


    Ok(result.to_string())
}