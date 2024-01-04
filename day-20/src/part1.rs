use crate::custom_error::AocError;
use crate::part1::PulseType::HighPulse;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending};
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Switch<'a> {
    switch_type: SwitchType,
    id: &'a str,
    output: Vec<&'a str>,
}

#[derive(Debug)]
enum SwitchType {
    Broadcast,
    FlipFlop { input: PulseType, state: FFState },
    Conjunction { input: PulseType, last: PulseType },
}

#[derive(Debug)]
enum FFState {
    On,
    Off,
}

#[derive(PartialEq, Debug, Clone)]
enum PulseType {
    HighPulse,
    LowPulse,
    None,
}

fn parse_broadcast(input: &str) -> IResult<&str, Switch> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Switch {
            switch_type: SwitchType::Broadcast,
            id: "broadcaster",
            output: outputs,
        },
    ))
}

fn parse_flip_flop(input: &str) -> IResult<&str, Switch> {
    let (input, _) = tag("%")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Switch {
            switch_type: SwitchType::FlipFlop {
                input: HighPulse,
                state: FFState::Off,
            },
            id: name,
            output: outputs,
        },
    ))
}

fn parse_conjunction(input: &str) -> IResult<&str, Switch> {
    let (input, _) = tag("&")(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        Switch {
            switch_type: SwitchType::Conjunction {
                input: PulseType::None,
                last: PulseType::None,
            },
            id: name,
            output: outputs,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, HashMap<&str, Switch>> {
    let (input, switchs) = separated_list1(
        line_ending,
        alt((parse_broadcast, parse_flip_flop, parse_conjunction)),
    )(input)?;

    Ok((
        input,
        switchs
            .into_iter()
            .map(|switch| (switch.id, switch))
            .collect(),
    ))
}

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String, AocError> {
    let (input, circuit_board) = parse(_input).expect("Failed to parse");
    println!("circuit_board: {:?}", circuit_board);

    // for _ in 0..1000 {
    // check all paths of start, sending low freq
    // for path in start.unwrap().path.iter() {
    //     switch_queue.push_back(circuit.get_mut(path).unwrap());
    // }

    // while !switch_queue.is_empty() {
    //     let current_switch = switch_queue.pop_front();
    //     println!("{:?}", current_switch);
    //     }
    // }

    let mut result = 0;

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("102", process(input)?);
        Ok(())
    }
}
