use std::{iter, ops::Range, str::Lines};

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range<u64>>,
    seed_to_soil: ConversionMap,
    soil_to_fertilizer: ConversionMap,
    fertilizer_to_water: ConversionMap,
    water_to_light: ConversionMap,
    light_to_temperature: ConversionMap,
    temperature_to_humidity: ConversionMap,
    humidity_to_location: ConversionMap,
}

#[derive(Debug)]
struct ConversionMap {
    ranges: Vec<ConversionRange>,
}

#[derive(Debug)]
struct ConversionRange {
    source_start: u64,
    destination_start: u64,
    range: u64,
}

fn solve(input: &str) -> u64 {
    let almanac = Almanac::parse(input);

    return (1..u64::MAX)
        .find(|location| almanac.location_corresponds_to_any_seed(*location))
        .unwrap();
}

impl Almanac {
    fn parse(almanac_str: &str) -> Almanac {
        let mut lines = almanac_str.lines();
        let seeds_str = lines.next().unwrap();
        let seeds = Self::parse_seed_ranges(seeds_str);
        let _blank_line = lines.next();

        let seed_to_soil = ConversionMap::parse(&mut lines);
        let soil_to_fertilizer = ConversionMap::parse(&mut lines);
        let fertilizer_to_water = ConversionMap::parse(&mut lines);
        let water_to_light = ConversionMap::parse(&mut lines);
        let light_to_temperature = ConversionMap::parse(&mut lines);
        let temperature_to_humidity = ConversionMap::parse(&mut lines);
        let humidity_to_location = ConversionMap::parse(&mut lines);

        return Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        };
    }

    fn parse_seed_ranges(seed_line: &str) -> Vec<Range<u64>> {
        let seed_values_str = seed_line.split(": ").collect::<Vec<&str>>()[1];
        let numeric_values: Vec<u64> = seed_values_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        return numeric_values
            .chunks(2)
            .map(|chunk| Range {
                start: chunk[0],
                end: chunk[0] + chunk[1],
            })
            .collect();
    }

    fn contains_seed(&self, seed: u64) -> bool {
        return self.seeds.iter().any(|range| range.contains(&seed));
    }

    fn location_corresponds_to_any_seed(&self, location: u64) -> bool {
        return self.map_location_to_seed(location).is_some();
    }

    fn map_location_to_seed(&self, location: u64) -> Option<u64> {
        let humidity = self.humidity_to_location.map_reverse(location);
        let temperature = self.temperature_to_humidity.map_reverse(humidity);
        let light = self.light_to_temperature.map_reverse(temperature);
        let water = self.water_to_light.map_reverse(light);
        let fertilizer = self.fertilizer_to_water.map_reverse(water);
        let soil = self.soil_to_fertilizer.map_reverse(fertilizer);
        let seed = self.seed_to_soil.map_reverse(soil);

        if !self.contains_seed(seed) {
            return None;
        }
        return Some(seed);
    }
}

impl ConversionMap {
    fn parse(map_lines: &mut Lines) -> ConversionMap {
        let _section_title = map_lines.next();
        let ranges: Vec<ConversionRange> = map_lines
            .take_while(|line| !line.is_empty())
            .map(ConversionRange::parse)
            .collect();
        return ConversionMap { ranges };
    }

    fn map_reverse(&self, destination: u64) -> u64 {
        return self
            .ranges
            .iter()
            .find(|range| range.applies_to_destination(destination))
            .map(|range| range.map_reverse(destination))
            .unwrap_or(destination);
    }
}

impl ConversionRange {
    fn parse(range_str: &str) -> ConversionRange {
        let parts: Vec<u64> = range_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        return ConversionRange {
            source_start: parts[1],
            destination_start: parts[0],
            range: parts[2],
        };
    }

    fn applies_to_destination(&self, destination: u64) -> bool {
        return self.destination_start <= destination
            && destination <= self.destination_start + self.range;
    }

    fn map_reverse(&self, destination: u64) -> u64 {
        if !self.applies_to_destination(destination) {
            panic!("Cannot map {destination}");
        }
        let offset = self.source_start as i64 - self.destination_start as i64;
        return ((destination as i64) + offset) as u64;
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48
            
            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15
            
            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4
            
            water-to-light map:
            88 18 7
            18 25 70
            
            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13
            
            temperature-to-humidity map:
            0 69 1
            1 0 69
            
            humidity-to-location map:
            60 56 37
            56 93 4
        "};

        let solution = solve(sample);

        assert_eq!(solution, 46);
    }
}
