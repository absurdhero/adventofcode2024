fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid = WordGrid::from_string(input);
    grid.count("XMAS").to_string()
}

#[derive(Debug)]
struct WordGrid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl WordGrid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();

        WordGrid { grid, rows, cols }
    }

    fn from_string(s: &str) -> Self {
        Self::new(s.split('\n').map(|l| l.chars().collect()).collect())
    }

    fn count(&self, word: &str) -> usize {
        let mut results = 0;

        for col in 0..self.cols {
            for row in 0..self.rows {
                if self.grid[row][col] == word.chars().next().unwrap() {
                    for direction in Direction::all() {
                        if self.check_word(word, col, row, direction) {
                            results += 1
                        } else {
                            continue;
                        }
                    }
                }
            }
        }

        results
    }

    fn check_word(&self, word: &str, col: usize, row: usize, direction: Direction) -> bool {
        for (i, c) in word.chars().enumerate() {
            let char_row = row as isize + (i as isize * direction.offsets().1);
            let char_col = col as isize + (i as isize * direction.offsets().0);
            if char_row < 0 || char_col < 0 || char_row >= self.rows as isize || char_col >= self.cols as isize {
                return false;
            }

            let test = self.grid[char_row as usize][char_col as usize];
            if c != test {
                return false;
            }
        }
        true
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    RightDown,
    Down,
    LeftDown,
    Left,
    LeftUp,
    Up,
    RightUp,
}

impl Direction {
    fn all() -> [Direction; 8] {
        [
            Direction::Right,
            Direction::RightDown,
            Direction::Down,
            Direction::LeftDown,
            Direction::Left,
            Direction::LeftUp,
            Direction::Up,
            Direction::RightUp,
        ]
    }

    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::RightDown => (1, 1),
            Direction::Down => (0, 1),
            Direction::LeftDown => (-1, 1),
            Direction::Left => (-1, 0),
            Direction::LeftUp => (-1, -1),
            Direction::Up => (0, -1),
            Direction::RightUp => (1, -1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, "18");
    }
}
