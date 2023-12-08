use regex::Regex;
use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut result: u32 = 0;
    let mut game_number = 0;
    let pattern = Regex::new(r"(\d+) (\w+)(?:, (\d+) (\w+))?(?:, (\d+) (\w+))?(?:;|$)");


    for line in _input.lines() {
        game_number += 1;
        let mut impossible: bool = false;
        match &pattern {
            Ok(regex) => {
                for color in regex.captures_iter(line) {
                    let count1: u32 = color.get(1).map_or(0, |m| m.as_str().parse().unwrap_or
                    (0));
                    let color1 = color.get(2).map_or("", |m| m.as_str());

                    impossible = color_match(color1, count1);
                    if impossible {
                        break;
                    }

                    let count2: u32 = color.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    let color2 = color.get(4).map_or("", |m| m.as_str());

                    impossible = color_match(color2, count2);
                    if impossible {
                        break;
                    }
                    let count3: u32 = color.get(5).map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    let color3 = color.get(6).map_or("", |m| m.as_str());

                    impossible = color_match(color3, count3);
                    if impossible {
                        break;
                    }

                    // println!(
                    //     "Color1: {}:{}, Color2: {}:{}, Color3: {}:{}, game number: {}",
                    //     color1, count1, color2, count2, color3, count3, game_number
                    // );
                }
                if impossible == true {
                    // println!("Game {} is impossible.", game_number);
                    continue;
                } else {
                    // println!("Game {} is possible.", game_number);
                    result += game_number;
                }
            }
            Err(err) => eprintln!("Regex error: {}", err),
        }
    }

    Ok(result.to_string())
}

fn color_match(color: &str, count: u32) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let res: bool = match color {
        "red" => count > max_red,
        "green" => count > max_green,
        "blue" => count > max_blue,
        _ => false,
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input1.txt");
        assert_eq!("8", process(input)?);
        Ok(())
    }
}