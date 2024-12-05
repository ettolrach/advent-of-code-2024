type Coord = [usize; 2];
const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const EIGHT_DIRECTIONS: [[isize; 2]; 8] = [
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
    [1, -1],
];
const MAS_CROSSES: [[([isize; 2], char); 5]; 4] = [
    // -> M S
    //     A
    //    M S
    [
        ([0, 0], 'M'),
        ([2, 0], 'S'),
        ([1, 1], 'A'),
        ([0, 2], 'M'),
        ([2, 2], 'S'),
    ],
    //    S S
    //     A
    // -> M M
    [
        ([0, 0], 'M'),
        ([0, -2], 'S'),
        ([1, -1], 'A'),
        ([2, 0], 'M'),
        ([2, -2], 'S'),
    ],
    // S M
    //  A
    // S M <-
    [
        ([0, 0], 'M'),
        ([-2, 0], 'S'),
        ([-1, -1], 'A'),
        ([0, -2], 'M'),
        ([-2, -2], 'S'),
    ],
    // M M <-
    //  A
    // S S
    [
        ([0, 0], 'M'),
        ([0, 2], 'S'),
        ([-1, 1], 'A'),
        ([-2, 0], 'M'),
        ([-2, 2], 'S'),
    ],
];

// Adapted from my own 'wordsearcher' program.

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
    fn in_bounds(&self, [a, b]: [isize; 2]) -> bool {
        a >= 0 && b >= 0 && a < self.width as isize && b < self.height as isize
    }

    /// Adds a vector to an index, returning [`Option::None`] if the vector takes the index
    /// out-of-bounds.
    fn index_plus_vector(&self, index: usize, vector: [isize; 2]) -> Option<usize> {
        let [x, y] = self.get_coord(index);
        let new_coord = [x as isize + vector[0], y as isize + vector[1]];
        if self.in_bounds(new_coord) {
            Some(self.get_index(new_coord.map(|i| i as usize)))
        } else {
            None
        }
    }

    /// Checks if a word is at a position while going in one direction.
    fn is_word_at_pos_with_direction(
        &self,
        word: &[char],
        position: usize,
        direction: [isize; 2],
    ) -> bool {
        let mut grid_index: usize = position;
        let mut letter: usize = 0;
        if word.is_empty() {
            return false;
        }
        // While we still have unfound letters.
        while letter < word.len() {
            if self.letters[grid_index] != word[letter] {
                return false;
            }
            if letter == word.len() - 1 {
                return true;
            }
            if let Some(next_index) = self.index_plus_vector(grid_index, direction) {
                grid_index = next_index;
            } else {
                // If we went out of bounds, then we know for certain that the word isn't here.
                return false;
            }
            letter += 1;
        }
        true
    }

    /// Finds the positions of the word and its multiplicity (how often it occurred at that
    /// position).
    fn find_positions(&self, word: &[char]) -> Vec<(Coord, usize)> {
        let mut to_return: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.letters.len() {
            if self.letters[i] == word[0] {
                let multiplicity = EIGHT_DIRECTIONS
                    .iter()
                    .map(|direction| self.is_word_at_pos_with_direction(word, i, *direction))
                    .filter(|b| *b)
                    .count();
                if multiplicity > 0 {
                    to_return.push((i, multiplicity));
                }
            }
        }
        to_return
            .into_iter()
            .map(|index| (self.get_coord(index.0), index.1))
            .collect()
    }

    /// Finds occurrences of the given formation.
    ///
    /// If one of the vectors will bring the position out-of-bounds, the function immediately
    /// returns false.
    ///
    /// # Arguments
    ///
    /// * `position` - the vectors will be applied with respect to this index.
    /// * `vectors_with_chars` - the vectors will be added to the position, then the character at
    ///   this offset position will be compared with the character in the tuple.
    #[must_use]
    fn check_relative_positions(
        &self,
        position: usize,
        vectors_with_chars: Vec<([isize; 2], char)>,
    ) -> bool {
        for (vector, c) in vectors_with_chars {
            if let Some(index) = self.index_plus_vector(position, vector) {
                if self.letters[index] != c {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    /// Finds MAS-crosses and their multiplicity (how many crosses each result has).
    fn find_mas_crosses(&self) -> Vec<(Coord, usize)> {
        let mut to_return: Vec<(usize, usize)> = Vec::new();
        for i in 0..self.letters.len() {
            // Do an early return if we don't have an M.
            if self.letters[i] != 'M' {
                continue;
            }
            let mut multiplicity: usize = 0;
            for mas_cross in MAS_CROSSES {
                if self.check_relative_positions(i, mas_cross.to_vec()) {
                    multiplicity += 1;
                }
            }
            if multiplicity > 0 {
                to_return.push((i, multiplicity));
            }
        }
        to_return
            .into_iter()
            .map(|index| (self.get_coord(index.0), index.1))
            .collect()
    }
}

fn part1(lines: impl Iterator<Item = String>) -> String {
    let grid = Grid::from_2d(lines.map(|line| line.chars().collect()).collect());
    let found_positions = grid.find_positions(XMAS.as_slice());
    found_positions
        .into_iter()
        .map(|(_, multiplicity)| multiplicity)
        .sum::<usize>()
        .to_string()
}

fn part2(lines: impl Iterator<Item = String>) -> String {
    let grid = Grid::from_2d(lines.map(|line| line.chars().collect()).collect());
    let mas_occurrences = grid.find_mas_crosses();
    mas_occurrences
        .into_iter()
        .map(|(_, multiplicity)| multiplicity)
        .sum::<usize>()
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

    const SAMPLE0: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
    const SAMPLE1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    const SAMPLE2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

    #[test]
    fn sample_part1_0() {
        assert_eq!(part1(SAMPLE0.lines().map(ToOwned::to_owned)), "4");
    }

    #[test]
    fn sample_part1() {
        assert_eq!(part1(SAMPLE1.lines().map(ToOwned::to_owned)), "18");
    }

    #[test]
    fn sample_part2() {
        assert_eq!(part2(SAMPLE2.lines().map(ToOwned::to_owned)), "9");
    }
}
