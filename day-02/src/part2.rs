use regex::Regex;
use crate::custom_error::AocError;

struct ColorCount {
    red: u32,
    green: u32,
    blue: u32,
}

impl ColorCount {
    fn new() -> Self {
        ColorCount {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
    fn get_red(&self) -> u32 {
        self.red
    }
    fn get_blue(&self) -> u32 {
        self.blue
    }
    fn get_green(&self) -> u32 {
        self.green
    }
    fn set_red(&mut self, count: u32) {
        self.red = count;
    }
    fn set_green(&mut self, count: u32) {
        self.green = count;
    }
    fn set_blue(&mut self, count: u32) {
        self.blue = count;
    }
}

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut result: u32 = 0;
    let pattern = Regex::new(r"(\d+) (\w+)(?:, (\d+) (\w+))?(?:, (\d+) (\w+))?(?:;|$)");

    for line in _input.lines() {
        let mut color_count = ColorCount::new();

        match &pattern {
            Ok(regex) => {
                for color in regex.captures_iter(line) {
                    let count1: u32 = color.get(1).map_or(0, |m| m.as_str().parse().unwrap_or
                    (0));
                    let color1 = color.get(2).map_or("", |m| m.as_str());

                    let count2: u32 = color.get(3).map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    let color2 = color.get(4).map_or("", |m| m.as_str());

                    let count3: u32 = color.get(5).map_or(0, |m| m.as_str().parse().unwrap_or(0));
                    let color3 = color.get(6).map_or("", |m| m.as_str());

                    color_match(color1, count1, &mut color_count);
                    color_match(color2, count2, &mut color_count);
                    color_match(color3, count3, &mut color_count);
                }
            }
            Err(err) => eprintln!("Regex error: {}", err),
        }

        let cube_power = color_count.get_green() * color_count.get_blue() * color_count
            .get_red();
        result += cube_power;
    }

    Ok(result.to_string())
}

fn color_match(color: &str, count: u32, min: &mut ColorCount) {
    match color {
        "red" => if count > min.get_red() {
            min.set_red(count);
        },
        "green" => if count > min.get_green() {
            min.set_green(count);
        },
        "blue" => if count > min.get_blue() {
            min.set_blue(count);
        },
        _ => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}