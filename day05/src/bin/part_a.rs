use std::str::Lines;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
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
    return almanac
        .seeds
        .iter()
        .map(|seed| almanac.map_seed_to_location(*seed))
        .min()
        .unwrap();
}

impl Almanac {
    fn parse(almanac_str: &str) -> Almanac {
        let mut lines = almanac_str.lines();
        let seeds_str = lines.next().unwrap();
        let seeds = Self::parse_seed_numbers(seeds_str);
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

    fn parse_seed_numbers(seed_line: &str) -> Vec<u64> {
        let seed_values_str = seed_line.split(": ").collect::<Vec<&str>>()[1];
        return seed_values_str
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        if !self.seeds.contains(&seed) {
            panic!("Invalid seed: {seed}")
        }
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);
        return self.humidity_to_location.map(humidity);
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

    fn map(&self, source: u64) -> u64 {
        return self
            .ranges
            .iter()
            .find(|range| range.applies_to(source))
            .map(|range| range.map(source))
            .unwrap_or(source);
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

    fn applies_to(&self, source: u64) -> bool {
        return self.source_start <= source && source <= self.source_start + self.range;
    }

    fn map(&self, source: u64) -> u64 {
        if !self.applies_to(source) {
            panic!("Cannot map {source}");
        }
        let offset = self.destination_start as i64 - self.source_start as i64;
        return ((source as i64) + offset) as u64;
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

        assert_eq!(solution, 35);
    }
}
