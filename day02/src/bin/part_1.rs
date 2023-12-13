use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<CubeSet>,
}

#[derive(Debug)]
struct CubeSet {
    blue: u64,
    red: u64,
    green: u64,
}

fn solve(input: &str) -> u64 {
    let games = input.lines().map(Game::parse);
    return games.filter(Game::is_possible).map(|game| game.id).sum();
}

impl Game {
    fn parse(game_str: &str) -> Game {
        let regex = Regex::new(r"^Game (?<game_id>\d+): (?<sets>.*)").unwrap();
        let captures = regex.captures(game_str).unwrap();
        let game_id = captures["game_id"].parse::<u64>().unwrap();
        let sets_str = &captures["sets"];
        let sets: Vec<CubeSet> = sets_str.split(';').map(CubeSet::parse).collect();

        return Game { id: game_id, sets };
    }

    fn is_possible(&self) -> bool {
        return self.sets.iter().all(CubeSet::is_possible);
    }
}

impl CubeSet {
    fn parse(set_str: &str) -> CubeSet {
        let color_strs = set_str.split(",").map(str::trim);
        let color_count_pairs: Vec<(&str, u64)> = color_strs
            .map(|s| {
                let pair: Vec<&str> = s.split(" ").collect();
                let count = pair[0].parse::<u64>().unwrap();
                let color = pair[1];
                return (color, count);
            })
            .collect();

        return CubeSet {
            blue: find_color_count(&color_count_pairs, "blue").unwrap_or(0),
            red: find_color_count(&color_count_pairs, "red").unwrap_or(0),
            green: find_color_count(&color_count_pairs, "green").unwrap_or(0),
        };
    }

    fn is_possible(&self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

fn find_color_count(color_count_pairs: &Vec<(&str, u64)>, color: &str) -> Option<u64> {
    return color_count_pairs
        .iter()
        .find(|(c, _)| *c == color)
        .map(|(_, count)| *count);
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "};

        let solution = solve(sample);

        assert_eq!(solution, 8);
    }
}
