use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    _input: &str,
) -> miette::Result<String, AocError> {
    let mut seeds: Vec<i64> = Vec::new();
    let mut cur_map = 0;

    let mut seed_soil: Vec<Vec<i64>> = Vec::new();
    let mut soil_fertilizer: Vec<Vec<i64>> = Vec::new();
    let mut fertilizer_water: Vec<Vec<i64>> = Vec::new();
    let mut water_light: Vec<Vec<i64>> = Vec::new();
    let mut light_temperature: Vec<Vec<i64>> = Vec::new();
    let mut temperature_humidity: Vec<Vec<i64>> = Vec::new();
    let mut humidity_location: Vec<Vec<i64>> = Vec::new();

    for (line_num, line) in _input.lines().enumerate() {
        let trimmed_line = line.trim().to_string();

        if line_num == 0 {
            if let Some(seeds_str) = trimmed_line.strip_prefix("seeds:") {
                seeds = seeds_str
                    .split_whitespace()
                    .map(|num_str| num_str.parse().unwrap())
                    .collect();
            }

            continue;
        }

        if trimmed_line.is_empty() {
            continue;
        }

        if trimmed_line.contains("map:") {
            cur_map += 1;
            continue;
        }

        let numbers: Vec<i64> = trimmed_line
            .split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .collect();

        match cur_map {
            1 => {
                // map_builder(&mut seed_soil, numbers)
                seed_soil.push(numbers);
            }
            2 => {
                // println!("soil-fertilizer");
                soil_fertilizer.push(numbers);
            }
            3 => {
                // println!("fertilizer-water");
                fertilizer_water.push(numbers);
            }
            4 => {
                // println!("water-light");
                water_light.push(numbers);
            }
            5 => {
                // println!("light-temperature");
                light_temperature.push(numbers);
            }
            6 => {
                // println!("temperature-humidity");
                temperature_humidity.push(numbers)
            }
            7 => {
                // println!("humidity-location");
                humidity_location.push(numbers)
            }
            _ => {
                eprintln!("End of file")
            }
        }
    }
    let mut result = i64::MAX;
    for i in (1..seeds.len()).step_by(2) {
        let mut seed = seeds[i - 1];
        let seed_range = seeds[i];

        for _ in 0..seed_range {
            let mut source = seed;

            // source = search_source(&humidity_location, source);
            // source = search_source(&temperature_humidity, source);
            // source = search_source(&light_temperature, source);
            // source = search_source(&water_light, source);
            // source = search_source(&fertilizer_water, source);
            // source = search_source(&soil_fertilizer, source);
            // source = search_source(&seed_soil, source);

            source = search_source(&seed_soil, source);
            // println!("Soil: seed: {}, source: {}", seed, source);
            source = search_source(&soil_fertilizer, source);
            // println!("Fert: seed: {}, source: {}", seed, source);
            source = search_source(&fertilizer_water, source);
            // println!("Water: seed: {}, source: {}", seed, source);
            source = search_source(&water_light, source);
            // println!("Light: seed: {}, source: {}", seed, source);
            source = search_source(&light_temperature, source);
            // println!("Temp: seed: {}, source: {}", seed, source);
            source = search_source(&temperature_humidity, source);
            // println!("Humid: seed: {}, source: {}", seed, source);
            source = search_source(&humidity_location, source);
            // println!("Loc: seed: {}, source: {}\n", seed, source);
            if source < result {
                result = source;
            }
            seed += 1;
        }
    }


    Ok(result.to_string())
}

fn search_source(vec: &Vec<Vec<i64>>, seed: i64) -> i64 {
    let mut source = seed;
    for loc in vec {
        if source >= loc[1] && source < loc[1] + loc[2] {
            source += loc[0] - loc[1];
            break;
        }
    }
    source
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = include_str!("../test_input2.txt");
        assert_eq!("46", process(input)?);
        Ok(())
    }
}