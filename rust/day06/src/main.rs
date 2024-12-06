// Adapted from my own 'wordsearcher' program.
type Coord = [usize; 2];
type Vector = [isize; 2];

fn quarter_turn_clockwise([x, y]: Vector) -> Vector {
    [-y, x]
}

fn half_turn([x, y]: Vector) -> Vector {
    [-x, -y]
}

struct Grid {
    letters: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_2d(lines: Vec<Vec<char>>) -> Self {
        // We assume AoC has good input.
        let width = lines[0].len();
        let letters: Vec<char> = lines.into_iter().flatten().collect();
        let height = letters.len() / width;
        Self {
            letters,
            width,
            height,
        }
    }

    /// Get the coordinate representation of an index.
    fn get_coord(&self, index: usize) -> Coord {
        [index % self.width, index / self.width]
    }

    /// Get the index which a coordinate represents.
    fn get_index(&self, [x, y]: Coord) -> usize {
        x + self.width * y
    }

    /// Check if a coordinate is in bounds.
    fn in_bounds(&self, [a, b]: Vector) -> bool {
        a >= 0 && b >= 0 && a < self.width as isize && b < self.height as isize
    }

    /// Adds a vector to an index, returning [`Option::None`] if the vector takes the index
    /// out-of-bounds.
    fn index_plus_vector(&self, index: usize, vector: Vector) -> Option<usize> {
        let [x, y] = self.get_coord(index);
        let new_coord = [x as isize + vector[0], y as isize + vector[1]];
        if self.in_bounds(new_coord) {
            Some(self.get_index(new_coord.map(|i| i as usize)))
        } else {
            None
        }
    }

    fn find_index_of(&self, to_find: char) -> Option<usize> {
        (0..self.letters.len()).find(|&i| self.letters[i] == to_find)
    }

    fn mark_visited(&mut self) {
        let mut guard_position: usize = self.find_index_of('^').expect("We know AoC has good input");
        let mut vector: Vector = [0, -1];
        self.letters[guard_position] = 'X';
        while let Some(new_guard_position) = self.index_plus_vector(guard_position, vector) {
            if self.letters[new_guard_position] == '#' {
                // We first need to take a step back.
                guard_position = self.index_plus_vector(new_guard_position, half_turn(vector)).unwrap();
                // And then turn the vector.
                vector = quarter_turn_clockwise(vector);
            } else {
                guard_position = new_guard_position;
                self.letters[guard_position] = 'X';
            }
        }
    }

    fn has_loop(&self) -> bool {
        let mut guard_position: usize = self.find_index_of('^').expect("We know AoC has good input");
        let mut vector: Vector = [0, -1];
        let mut positions_and_vectors: Vec<(usize, Vector)> = Vec::new();
        while let Some(new_guard_position) = self.index_plus_vector(guard_position, vector) {
            if self.letters[new_guard_position] == '#' {
                // We first need to take a step back.
                guard_position = self.index_plus_vector(new_guard_position, half_turn(vector)).unwrap();
                // And then turn the vector.
                vector = quarter_turn_clockwise(vector);
            } else {
                if positions_and_vectors.contains(&(new_guard_position, vector)) {
                    return true;
                }
                guard_position = new_guard_position;
                positions_and_vectors.push((guard_position, vector));
            }
        }
        false
    }
}

impl IntoIterator for Grid {
    type Item = char;
    type IntoIter = std::vec::IntoIter<char>;
    fn into_iter(self) -> Self::IntoIter {
        self.letters.into_iter()
    }
}

// impl Display for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut output = String::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 output.push(self.letters[self.get_index([x, y])]);
//             }
//             output.push('\n');
//         }
//         write!(f, "{output}")
//     }
// }

fn part1(lines: impl Iterator<Item = String>) -> String {
    let mut grid = Grid::from_2d(lines.map(|line| line.chars().collect()).collect());
    grid.mark_visited();
    grid.into_iter().filter(|c| *c == 'X').count().to_string()
}

fn part2(lines: impl Iterator<Item = String>) -> String {
    let mut grid = Grid::from_2d(lines.map(|line| line.chars().collect()).collect());
    (0..grid.letters.len())
        .map(|i| {
            let mut has_loop = false;
            if grid.letters[i] == '.' {
                grid.letters[i] = '#';
                has_loop = grid.has_loop();
                grid.letters[i] = '.';
            }
            has_loop
        })
        .filter(|b| *b)
        .count()
        .to_string()
}

fn main() -> std::io::Result<()> {
    let lines = std::io::stdin()
        .lines()
        .collect::<std::io::Result<Vec<String>>>()?;
    println!("Part 1: {}", part1(lines.clone().into_iter()));
    println!("Part 2: {}", part2(lines.into_iter()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_vector() {
        let vector: Vector = [2, 1];
        assert_eq!([-1, 2], quarter_turn_clockwise(vector));
    }

    const SAMPLE1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE1.lines().map(ToOwned::to_owned)), "41");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE1.lines().map(ToOwned::to_owned)), "6");
    }
}