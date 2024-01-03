use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let hailstones: Vec<Vec<(f64, f64, f64)>> = _input
        .lines()
        .map(|line| {
            line
                .split('@')
                .map(|s| {
                    s.split(',')
                        .map(|ch| ch.trim().parse::<f64>().unwrap())
                        .collect::<Vec<f64>>()
                })
                .map(|v| {
                    (
                        v[0],
                        v[1],
                        v[2],
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // println!("{:?}", hailstones);

    // create target bounds
    let bounds: (f64, f64) = (200_000_000_000_000.0, 400_000_000_000_000.0);
    // let bounds: (f64, f64) = (7.0, 27.0);


    let mut result = 0;
    for (index, stone_a) in hailstones.iter().enumerate() {
        let second_point = (stone_a[0].0 - stone_a[1].0, stone_a[0].1 - stone_a[1].1);
        // find slope for primary stone
        let a_m = calculate_slope(stone_a[0].0, stone_a[0].1, second_point.0, second_point.1);
        let a_b = calculate_intercept(stone_a[0].0, stone_a[0].1, a_m);
        for stone_b in (index + 1)..hailstones.len() {
            let second_point = (
                hailstones[stone_b][0].0 - hailstones[stone_b][1].0,
                hailstones[stone_b][0].1 - hailstones[stone_b][1].1
            );
            let b_m = calculate_slope(
                hailstones[stone_b][0].0,
                hailstones[stone_b][0].1,
                second_point.0,
                second_point.1,
            );
            let b_b = calculate_intercept(hailstones[stone_b][0].0, hailstones[stone_b][0].1, b_m);
            // println!("a_m: {:?}, a_b: {:?}", a_m, a_b);
            // println!("b_m: {:?}, b_b: {:?}", b_m, b_b);

            if let Some(intersect_point) = line_intersect(a_m, a_b, b_m, b_b) {

                // check a
                if !check_if_already_passed(stone_a, intersect_point) {
                    continue;
                }
                // check b
                if !check_if_already_passed(&hailstones[stone_b], intersect_point) {
                    continue;
                }

                if intersect_point.0 >= bounds.0
                    && intersect_point.0 <= bounds.1
                    && intersect_point.1 >= bounds.0
                    && intersect_point.1 <= bounds.1
                {
                    // println!("stone_a: {:?}, slope: {:?}, stone_b: {:?}, slope: {:?}, intersection: {:?}", stone_a[0], a_m,
                    //          hailstones[stone_b][0], b_m, intersect_point);
                    // println!("Passed");
                    result += 1;
                }
            }
        }
    }

    Ok(result.to_string())
}

fn check_if_already_passed(origin: &Vec<(f64, f64, f64)>, intersect_point: (f64, f64)) -> bool {
    let origin_x = (origin[0].0, origin[1].0);
    let origin_y = (origin[0].1, origin[1].1);

    if origin_x.1 < 0f64 && origin_x.0 < intersect_point.0 {
        return false;
    } else if origin_x.1 > 0f64 && origin_x.0 > intersect_point.0 {
        return false;
    }
    if origin_y.1 < 0f64 && origin_y.0 < intersect_point.1 {
        return false;
    } else if origin_y.1 > 0f64 && origin_y.0 > intersect_point.1 {
        return false;
    }
    true
}


fn calculate_intercept(x0: f64, y0: f64, slope: f64) -> f64 {
    y0 - slope * x0
}

fn calculate_slope(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    (y1 - y0) / (x1 - x0)
}

fn line_intersect(m1: f64, b1: f64, m2: f64, b2: f64) -> Option<(f64, f64)> {
    if m1 == m2 {
        // lines are parallel
        None
    } else {
        let x = (b2 - b1) / (m1 - m2);
        let y = m1 * x + b1;
        Some((x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("2", process(input)?);
        Ok(())
    }
}