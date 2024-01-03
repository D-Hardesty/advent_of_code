use crate::custom_error::AocError;
// use nalgebra::{Vector3, Point3, Vector2, Point2};

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    todo!("day 01 - part 1")
    // let hailstones: Vec<Vec<(i64, i64, i64)>> = _input
    //     .lines()
    //     .map(|line| {
    //         line
    //             .split('@')
    //             .map(|s| {
    //                 s.split(',')
    //                     .map(|ch| ch.trim().parse::<i64>().unwrap())
    //                     .collect::<Vec<i64>>()
    //             })
    //             .map(|v| {
    //                 (
    //                     v[0],
    //                     v[1],
    //                     v[2],
    //                 )
    //             })
    //             .collect::<Vec<_>>()
    //     })
    //     .collect();
    //
    // println!("{:?}", hailstones);
    //
    //
    // let mut result = 0;
    // for (index, stone) in hailstones.iter().enumerate() {
    //     // assign start and vector to first stone
    //     let primary_origin = Point2::new(stone[0].0, stone[0].1);
    //     let primary_vec = Vector2::new(stone[1].0, stone[1].1);
    //     println!("p_loc: {:?}, p_vec: {:?}", primary_origin, primary_vec);
    //
    //     for other in (index + 1)..hailstones.len() {
    //         let secondary_origin = Point3::new(hailstones[other][0].0, hailstones[other][0].1, hailstones[other][0].2);
    //         let secondary_vector = Vector3::new(hailstones[other][1].0, hailstones[other][0].1, hailstones[other][0]
    //             .2);
    //         println!("s_loc: {:?}, s_vec: {:?}", secondary_origin, secondary_vector);
    //         let mut iter = 0;
    //         loop {
    //             let cur_p_loc = {
    //                 (
    //                     primary_location.0 + (primary_vector.0 * iter),
    //                     primary_location.1 + (primary_vector.1 * iter),
    //                     primary_location.2 + (primary_vector.2 * iter),
    //                 )
    //             };
    //             let cur_s_loc = {
    //                 (
    //                     secondary_location.0 + (secondary_vector.0 * iter),
    //                     secondary_location.1 + (secondary_vector.1 * iter),
    //                     secondary_location.2 + (secondary_vector.2 * iter),
    //                 )
    //             };
    //             if cur_p_loc.0 > lower_bound && cur_p_loc.0 < upper_bound {
    //                 println!("cur_p: {:?}, cur_s: {:?}", cur_p_loc, cur_s_loc);
    //                 if cur_p_loc == cur_s_loc {
    //                     result += 1;
    //                     break;
    //                 }
    //                 iter += 1;
    //             } else {
    //                 break;
    //             }
    //         }
    //     }
    // }
    //
    // Ok(result.to_string())
}

// fn check_intesect(a_origin: Point3<f64>,
//                   a_direction: Vector3<f64>,
//                   b_origin: Point3<f64>,
//                   b_direction: Vector3<f64>,
//                   start_point: Point3<f64>,
//                   end_point: Point3<f64>,
// ) -> bool {
//     let v_a = a_direction;
//     let v_b = b_direction;
//
//     let a_params = {
//         let a_diff = start_point - a_origin;
//         a_diff.component_div(&v_a)
//     };
//
//     let b_params = {
//         let b_diff = start_point - b_origin;
//         b_diff.component_div(&v_b)
//     };
//
//     // Check if A and B intersect within range
//     let t_a_start = a_params.x.max(a_params.y.max(a_params.z));
//     let t_a_end = a_params.x.min(a_params.y.min(a_params.z));
//
//     let t_b_start = b_params.x.max(b_params.y.max(b_params.z));
//     let t_b_end = b_params.x.min(b_params.y.min(b_params.z));
//
//     t_a_start <= t_b_end && t_a_end >= t_b_start
// };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        todo!("haven't built test yet");
        let input = include_str!("../test_input2.txt");
        assert_eq!("", process(input)?);
        Ok(())
    }
}