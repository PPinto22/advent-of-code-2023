use std::{
    cmp::{max, min},
    collections::HashSet,
    str::FromStr,
};

type Point = (usize, usize);

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", solve(input, 1_000_000));
}

#[derive(Debug)]
struct Image {
    pixels: Vec<Vec<char>>,
    empty_rows: HashSet<usize>,
    empty_columns: HashSet<usize>,
    expansion_factor: u64,
}

fn solve(input: &str, expansion_factor: u64) -> u64 {
    let mut image: Image = input.parse().unwrap();
    image.expansion_factor = expansion_factor;
    let galaxies = image.find_galaxies();

    let mut sum_of_shortest_distances: u64 = 0;
    for (i, source_galaxy) in galaxies.iter().enumerate() {
        for (j, destination_galaxy) in galaxies[i + 1..].iter().enumerate() {
            let shortest_distance =
                image.calculate_shortest_distance(*source_galaxy, *destination_galaxy);
            sum_of_shortest_distances += shortest_distance;
        }
    }

    sum_of_shortest_distances
}

impl Image {
    fn new(pixels: Vec<Vec<char>>) -> Image {
        let empty_rows: HashSet<usize> = (0..pixels.len())
            .filter(|i| pixels[*i].iter().all(|c| *c == '.'))
            .collect();
        let empty_columns: HashSet<usize> = (0..pixels[0].len())
            .filter(|j| pixels.iter().all(|row| row[*j] == '.'))
            .collect();

        Image {
            pixels,
            empty_rows,
            empty_columns,
            expansion_factor: 1,
        }
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
        let start_x = min(source.0, destination.0);
        let start_y = min(source.1, destination.1);
        let end_x = max(source.0, destination.0);
        let end_y = max(source.1, destination.1);

        let mut distance: u64 = 0;
        for x in start_x..end_x {
            if self.empty_rows.contains(&x) {
                distance += self.expansion_factor
            } else {
                distance += 1
            }
        }
        for y in start_y..end_y {
            if self.empty_columns.contains(&y) {
                distance += self.expansion_factor
            } else {
                distance += 1
            }
        }
        distance
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s.lines().map(|line| line.chars().collect()).collect();
        Ok(Image::new(pixels))
    }
}

#[cfg(test)]
mod test {
    use super::solve;
    use indoc::indoc;

    #[test]
    fn solves_sample_given_expansion_factor_of_10() {
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

        let solution = solve(sample, 10);

        assert_eq!(solution, 1030);
    }

    #[test]
    fn solves_sample_given_expansion_factor_of_100() {
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

        let solution = solve(sample, 100);

        assert_eq!(solution, 8410);
    }
}
