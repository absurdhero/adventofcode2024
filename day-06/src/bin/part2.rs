use std::ops::Add;

fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut map = Map::from_string(input);
    map.walk();
    let result = map.count().to_string();
    for row in map.grid {
        println!("{}", row.iter().collect::<String>());
    }
    result
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = grid[0].len();

        Map { grid, rows, cols }
    }

    fn from_string(s: &str) -> Self {
        Self::new(s.split('\n').map(|l| l.chars().collect()).collect())
    }

    fn find_guard(&self) -> (isize, isize) {
        for col in 0..self.cols {
            for row in 0..self.rows {
                if Direction::all()
                    .map(Direction::character)
                    .contains(&self.grid[row][col])
                {
                    return (col as isize, row as isize);
                }
            }
        }
        (0, 0)
    }
    fn walk(&mut self) -> usize {
        let mut results = 0;
        let mut guard_pos = self.find_guard();
        let mut dir = Direction::Up;

        loop {
            let curr = self.grid[guard_pos.1 as usize][guard_pos.0 as usize];
            self.grid[guard_pos.1 as usize][guard_pos.0 as usize] = dir.trail(curr);
            let next = self.char_at(Self::moved_pos(guard_pos, dir));

            print!("{}", next);
            if next == '!' {
                return results;
            } else if next == '#' {
                dir = dir.rotate();
                println!(" turned {:?}", dir);
            } else {
                results += 1;
                guard_pos = Self::moved_pos(guard_pos, dir)
            }
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for col in 0..self.cols {
            for row in 0..self.rows {
                if self.grid[row][col] == '+' {
                    for dir in Direction::all() {
                        let flipped = dir.flip();
                        // if self.char_at(Self::moved_pos((row, col), dir))
                        // if ()
                        // if Self::moved_pos((row as isize, col as isize), flipped)
                        //     == Self::moved_pos((row as isize, col as isize), dir.rotate())
                        // {}
                    }
                }
            }
        }
        count
    }
    fn moved_pos(pos: (isize, isize), dir: Direction) -> (isize, isize) {
        (pos.0 + dir.offsets().0, pos.1 + dir.offsets().1)
    }

    fn char_at<T: Into<isize> + PartialOrd>(&self, pos: (T, T)) -> char {
        let x = pos.1.into();
        let y = pos.0.into();
        if x as usize >= self.rows || y as usize >= self.cols || x < 0 || y < 0 {
            return '!';
        }
        self.grid[y as usize][x as usize]
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    Unknown,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }

    fn character(self) -> char {
        match self {
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Unknown => '+',
        }
    }

    fn trail(self, prev_char: char) -> char {
        let c = match self {
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Unknown => '+',
        };

        if (c == '<' || c == '>') && (prev_char == 'v' || prev_char == '^') {
            '+'
        } else if (prev_char == '<' || prev_char == '>') && (c == 'v' || c == '^') {
            '+'
        } else {
            c
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Unknown => Direction::Unknown,
        }
    }

    fn flip(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Unknown => Direction::Unknown,
        }
    }

    fn from(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => Direction::Unknown,
        }
    }

    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Unknown => panic!("Unknown direction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let result = process(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, "6");
    }
}
