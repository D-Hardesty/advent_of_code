use std::collections::{HashMap, VecDeque};
use crate::custom_error::AocError;

#[derive(Debug, Clone)]
struct Switch {
    s_name: String,
    s_type: SwitchType,
    waiting: bool,
    state: FlipFlop,
    memory: PulseType,
    path: Vec<String>,
    connections: Vec<Switch>,
}

#[derive(PartialEq, Debug, Clone)]
enum SwitchType {
    FF,
    Con,
    Broadcaster,
}

#[derive(PartialEq, Debug, Clone)]
enum FlipFlop {
    On,
    Off,
}

#[derive(PartialEq, Debug, Clone)]
enum PulseType {
    HighPulse,
    LowPulse,
}

impl Switch {
    // fn send_to_switch(&mut self, queue: &mut VecDeque<&Switch>, map: &mut HashMap<String, Switch>) {
    //     for path in self.path.iter() {
    //         let next_switch = map.get_mut(path).unwrap();
    //         queue.push_back(next_switch);
    //
    //         if self.s_type == SwitchType::FF {
    //             self.send_signal_ff(next_switch);
    //         } else {
    //             // self.send_signal_con();
    //         }
    //     }
    // }
    // fn send_signal_ff(&mut self, switch: &mut Switch) {
    //     // false == low freq
    //     match self.incoming_pulse {
    //         PulseType::LowPulse => {
    //             if self.state == FlipFlop::On {
    //                 self.state = FlipFlop::Off;
    //                 switch.incoming_pulse = PulseType::LowPulse;
    //             } else {
    //                 self.state = FlipFlop::On;
    //                 switch.incoming_pulse = PulseType::HighPulse;
    //             }
    //         }
    //         PulseType::HighPulse => {}
    //     }
    // }

    // fn send_signal_con(&self, switch: &Switch) {}
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut circuit: HashMap<String, Switch> = _input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let s_type_name = iter.next().unwrap();
            let s_type_str = s_type_name.chars().next().unwrap().to_string();
            let name = s_type_name.chars().skip(1).collect::<String>();
            let path: Vec<String> = iter
                .flat_map(|s| s.split("->").map(|p| p.trim_end_matches(',').to_string()))
                .filter(|p| !p.is_empty())
                .collect();

            let s_type = if s_type_str == "%".to_string() {
                SwitchType::FF
            } else if s_type_str == "&".to_string() {
                SwitchType::Con
            } else {
                SwitchType::Broadcaster
            };

            let switch = Switch {
                s_type,
                s_name: name.clone(),
                waiting: false,
                state: FlipFlop::Off,
                memory: PulseType::HighPulse,
                path,
                connections: Vec::new(),
            };
            (name.to_string(), switch)
        }).collect();

    println!("circuit: {:?}", circuit);

    let mut switch_queue: VecDeque<&mut Switch> = VecDeque::new();
    // map out switch paths
    let mut broacaster = circuit.get_mut("roadcaster").unwrap().clone();
    switch_queue.push_back(&mut broacaster);
    while let Some(mut cur_switch) = switch_queue.pop_front() {
        for path in &cur_switch.path {
            if let Some(connection) = circuit.get_mut(&path) {
                cur_switch.connections.push(connection.clone());
                switch_queue.push_back(connection);
            }
        }
    }
    println!("Broadcaster: {:?}", broacaster);


    let mut high_pulse: u64 = 0;
    let mut low_pulse: u64 = 0;

    println!("{:?}", circuit);


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
        assert_eq!("", process(input)?);
        Ok(())
    }
}