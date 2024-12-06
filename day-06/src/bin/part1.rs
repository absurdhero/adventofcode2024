fn main() {
    let input = include_str!("../../input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut map = Map::from_string(input);
    map.walk();
    map.count().to_string()
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

        while true {
            self.grid[guard_pos.1 as usize][guard_pos.0 as usize] = 'X';
            let next = self.char_at(Self::moved_pos(&mut guard_pos, &mut dir));
            print!("{}", next);
            if next == '!' {
                return results;
            } else if next == '.' || next == 'X' {
                results += 1;
                guard_pos = Self::moved_pos(&mut guard_pos, &mut dir)
            } else if next == '#' {
                dir = dir.rotate();
                println!(" turned {:?}", dir);
            }
        }
        results
    }

    fn count(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|r| r.iter())
            .filter(|c| **c == 'X')
            .count()
    }
    fn moved_pos(pos: &mut (isize, isize), dir: &mut Direction) -> (isize, isize) {
        (pos.0 + dir.offsets().0, pos.1 + dir.offsets().1)
    }

    fn char_at(&self, pos: (isize, isize)) -> char {
        if pos.1 >= self.rows as isize || pos.0 >= self.cols as isize || pos.1 < 0 || pos.0 < 0 {
            return '!';
        }
        self.grid[pos.1 as usize][pos.0 as usize]
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
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
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn from(c: char) -> Direction {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => panic!("Unknown direction: {}", c),
        }
    }

    fn offsets(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
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
        assert_eq!(result, "41");
    }
}
