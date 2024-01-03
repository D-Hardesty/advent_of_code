use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::{is_a, tag},
    character::complete,
    character::complete::space1,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use crate::custom_error::AocError;

#[derive(Debug)]
struct HotSpring<'a> {
    unknown_locations: u32,
    line: &'a str,
    batches: Vec<u32>,
}

impl<'a> HotSpring<'a> {
    #[tracing::instrument]
    fn generate_permutations(&self) -> Vec<String> {
        let options: Vec<String> =
            repeat_n([".", "#"].into_iter(), self.unknown_locations as usize)
                .multi_cartesian_product()
                .map(|v| v.join(""))
                .collect();

        // println!("{:?}", options);

        options
    }

    fn check_if_valid(&self, option: &str) -> bool {
        let mut option_iter = option.chars();
        let filled_option = self.line.chars().map(|ch| match ch {
            '?' => option_iter.next().expect("not valid?"),
            value => value
        }).collect::<String>();

        let count = filled_option
            .chars()
            .group_by(|ch| ch == &'#')
            .into_iter()
            .filter_map(|(is_hashes, group)| {
                is_hashes.then_some(
                    group.into_iter().count() as u32,
                )
            }).collect::<Vec<u32>>();

        &self.batches[..] == &count[..]
    }

    fn total_solution_counts(&self) -> usize {
        let options = self.generate_permutations();
        let count = options
            .iter()
            .filter(|option| self.check_if_valid(option))
            .count();
        count
    }
}

fn parser(input: &str) -> IResult<&str, HotSpring> {
    let (input, (line, batches)) = separated_pair(
        is_a("?.#"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let unknown_locations = line.chars().filter(|ch| ch == &'?').count() as u32;

    Ok((
        input,
        HotSpring {
            unknown_locations,
            line,
            batches,
        },
    ))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let hotsprings = _input
        .lines()
        .map(parser)
        .collect::<Result<
            Vec<(&str, HotSpring<'_>)>,
            nom::Err<nom::error::Error<&str>>,
        >>()
        .expect("failed to parse");

    let result = hotsprings
        .iter()
        .map(|(_, hotspring)| hotspring.total_solution_counts())
        .sum::<usize>();

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
