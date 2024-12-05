fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let grid = WordGrid::from_string(input);
    grid.count("MAS").to_string()
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

        for col in 1..self.cols {
            for row in 1..self.rows {
                let mut local_count = 0;
                for direction in Direction::all() {
                    if self.check_word(word, col, row, direction) {
                        local_count += 1;
                    } else {
                        continue;
                    }
                }
                if local_count == 2 {
                    results += 1;
                }
            }
        }

        results
    }

    fn check_word(&self, word: &str, col: usize, row: usize, direction: Direction) -> bool {
        // walk back one for col and row to check the X pattern of our 3 letter word
        let col = col as isize - direction.offsets().0;
        let row = row as isize - direction.offsets().1;

        for (i, c) in word.chars().enumerate() {
            let char_col = col + (i as isize * direction.offsets().0);
            let char_row = row + (i as isize * direction.offsets().1);
            if char_row < 0
                || char_col < 0
                || char_row >= self.rows as isize
                || char_col >= self.cols as isize
            {
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
    RightDown,
    LeftDown,
    LeftUp,
    RightUp,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::RightDown,
            Direction::LeftDown,
            Direction::LeftUp,
            Direction::RightUp,
        ]
    }

    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::RightDown => (1, 1),
            Direction::LeftDown => (-1, 1),
            Direction::LeftUp => (-1, -1),
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
        assert_eq!(result, "9");
    }
}
