use std::collections::HashMap;
use crate::custom_error::AocError;


#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let workflows: HashMap<String, Vec<(String, i32, String)>> = _input
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.trim_end_matches('}').split('{');

            let key = parts.next().unwrap_or("").trim().to_string();

            let values: Vec<(String, i32, String)> = parts
                .next()
                .unwrap_or("")
                .split(',')
                .map(|s| {
                    let mut greater: bool = false;
                    let mut components = match s.chars().nth(1) {
                        Some('>') => {
                            greater = true;
                            s.trim().split('>')
                        }
                        Some('<') => {
                            greater = false;
                            s.trim().split('<')
                        }
                        _ => s.trim().split(':'),
                    };

                    let first = components.next().unwrap_or("").trim().to_string();

                    let second_third = components.next().unwrap_or("").trim().split(':');

                    let second = if greater {
                        second_third.clone().next().unwrap_or("").trim().parse().unwrap_or(0)
                    } else {
                        -second_third.clone().next().unwrap_or("").trim().parse().unwrap_or(0)
                    };

                    let third = second_third.last().unwrap_or("").trim().to_string();

                    if first == "x" || first == "m" || first == "a" || first == "s" {
                        (first, second, third)
                    } else {
                        ("default".to_string(), 0, first)
                    }
                })
                .collect();

            (key, values)
        })
        .collect();

    let items: Vec<Vec<i32>> = _input
        .lines()
        .skip_while(|line| !line.trim().is_empty())
        .skip(1)
        .map(|line| {
            line.trim_start_matches('{')
                .trim_end_matches('}')
                .split(',')
                .map(|s| s.trim().get(2..).unwrap_or("").parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing: {}", err);
                    Vec::new()
                })
        })
        .collect();
    //
    // println!("workflows: {:?}", workflows);
    // println!("item stats: {:?}", items);

    let mut result: i32 = 0;

    for item in items.iter() {
        let mut workflow = String::from("in");
        'outer: loop {
            let process = workflows.get(&workflow);
            'inner: for vec in process.iter() {
                for style in vec.iter() {
                    // println!("workflow: {:?}", workflow);
                    // println!("Full Vec: {:?}", vec);
                    // println!("Full style {:?}.", style);
                    match style.0.as_str() {
                        "x" => {
                            if style.1 < 0 {
                                if item[0] < style.1.abs() {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            } else {
                                if item[0] > style.1 {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            }
                        }
                        "m" => {
                            if style.1 < 0 {
                                if item[1] < style.1.abs() {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            } else {
                                if item[1] > style.1 {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            }
                        }
                        "a" => {
                            if style.1 < 0 {
                                if item[2] < style.1.abs() {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            } else {
                                if item[2] > style.1 {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            }
                        }
                        "s" => {
                            if style.1 < 0 {
                                if item[3] < style.1.abs() {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            } else {
                                if item[3] > style.1 {
                                    workflow = style.2.clone();
                                    break 'inner;
                                }
                            }
                        }
                        _ => {
                            workflow = style.2.clone();
                            if style.2 == "A" || style.2 == "R" {
                                break 'outer;
                            }
                        }
                    }
                    // println!("{:?}", item);
                    // println!("Workflow, {:?}", workflow);
                }
            }
            if workflow == "R" || workflow == "A" {
                break 'outer;
            }

            // println!("process: {:?}", process);
        }
        if workflow == "A" {
            let score: i32 = item.iter().sum();
            result += score;
        }
        // println!("workflow end: {:?}", workflow);
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("19114", process(input)?);
        Ok(())
    }
}