use std::str::FromStr;

type Point = (usize, usize);

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input));
}

#[derive(Debug)]
struct Image {
    pixels: Vec<Vec<char>>,
}

fn solve(input: &str) -> u64 {
    let image: Image = input.parse().unwrap();
    let expanded_image = image.expand();
    let galaxies = expanded_image.find_galaxies();

    let mut sum_of_shortest_distances: u64 = 0;
    for (i, source_galaxy) in galaxies.iter().enumerate() {
        for (j, destination_galaxy) in galaxies[i + 1..].iter().enumerate() {
            let shortest_distance =
                expanded_image.calculate_shortest_distance(*source_galaxy, *destination_galaxy);
            sum_of_shortest_distances += shortest_distance;
        }
    }

    sum_of_shortest_distances
}

impl Image {
    fn expand(&self) -> Image {
        let mut pixels: Vec<Vec<char>> = Vec::new();

        for row in &self.pixels {
            pixels.push(row.clone());
            if row.iter().all(|c| *c == '.') {
                pixels.push(row.clone());
            }
        }

        let mut empty_column_count: usize = 0;
        for j in 0..self.pixels[0].len() {
            let is_empty_column = self.pixels.iter().all(|row| row[j] == '.');
            if is_empty_column {
                for row in &mut pixels {
                    row.insert(j + empty_column_count, '.');
                }
                empty_column_count += 1;
            }
        }

        Image { pixels }
    }

    fn find_galaxies(&self) -> Vec<Point> {
        let mut galaxies: Vec<Point> = Vec::new();
        for (i, row) in self.pixels.iter().enumerate() {
            for (j, pixel) in row.iter().enumerate() {
                if *pixel == '#' {
                    galaxies.push((i, j));
                }
            }
        }
        galaxies
    }

    fn calculate_shortest_distance(&self, source: Point, destination: Point) -> u64 {
        let dx = (destination.0 as i64 - source.0 as i64).abs();
        let dy = (destination.1 as i64 - source.1 as i64).abs();
        (dx + dy) as u64
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Image {
            pixels: s.lines().map(|line| line.chars().collect()).collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample() {
        let sample = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};

        let solution = solve(sample);

        assert_eq!(solution, 374);
    }
}
