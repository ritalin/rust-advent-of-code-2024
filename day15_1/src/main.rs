use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (p0, mut board, moves) = read_file(path)?;

    board.move_robot(p0, &moves);

    let total = board.map.iter().enumerate()
        .filter_map(|(i, obj)| match obj {
            Object::Box => Some(board.from_index(i)),
            Object::Floor | Object::Wall => None,
        })
        .map(|(x, y)| x + 100 * y)
        .sum::<usize>()
    ;

    Ok(total)
}

type Point = (usize, usize);

struct Board {
    width: usize,
    height: usize,
    map: Vec<Object>,
}

impl Board {
    fn move_robot(&mut self, mut p0: Point, moves: &[Direction]) {
        for d in moves {
            if let Some(p) = self.move_robot_internal(p0, d.clone()) {
                p0 = p;
            }
        }
    } 

    fn move_robot_internal(&mut self, p0: Point, d: Direction) -> Option<Point> {
        let Some(p) = d.next(p0, self.width, self.height) else {
            return None;
        };

        let index = self.to_index(p);
        match self.map[index] {
            Object::Box => {
                if self.try_push_box(p, index, &d).is_none() {
                    return None;
                }
            },
            Object::Floor => {},
            Object::Wall => return None,
        }

        Some(p)
    }

    fn try_push_box(&mut self, p0: Point, index_from: usize, d: &Direction) -> Option<usize> {
        let Some(p) = d.next(p0, self.width, self.height) else {
            return None;
        };

        let index_to = self.to_index(p);
        match self.map[index_to] {
            Object::Box => {
                if self.try_push_box(p, index_to, d).is_none() {
                    return None;
                };
            },
            Object::Floor => {}
            Object::Wall => return None,
        }

        self.map.swap(index_from, index_to);

        Some(index_to)
    } 

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    N, E, S, W,
}

impl Direction {
    fn next(&self, (x, y): Point, width: usize, height: usize) -> Option<Point> {
        match self {
            Direction::N if y > 0 => Some((x, y - 1)),
            Direction::E if x < width - 1 => Some((x + 1, y)),
            Direction::S if y < height - 1 => Some((x, y + 1)),
            Direction::W if x > 0 => Some((x - 1, y)),
            _ => None,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Floor,
    Box,
    Wall,
}

#[derive(Debug)]
enum PatternError {
    InvalidMap(String),
    InvalidMoves(String),
    InvalidPosition(String),
}
impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::InvalidMap(msg) | 
            PatternError::InvalidMoves(msg) |
            PatternError::InvalidPosition(msg) => write!(f, "PatternError: {}", msg),
        }
    }
}
impl std::error::Error for PatternError {}

fn read_file<P>(path: P) -> Result<(Point, Board, Vec<Direction>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);

    let (p0, board) = read_map(&mut reader)?;
    let moves = read_moves(&mut reader)?;

    Ok((p0, board, moves))
}

fn read_map<R>(reader: &mut R) -> Result<(Point, Board), Box<dyn std::error::Error>> 
    where R: BufRead 
{
    let mut buf = String::new();
    let mut width: usize = 0;
    let mut map = Vec::<String>::new();
    let mut p0 = Option::<Point>::None;

    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim_end();
        if s.len() == 0 {break; }

        if let Some(x) = s.chars().position(|ch| ch == '@') {
            p0 = Some((x, map.len()));
        }

        width = s.len();
        map.push(s.to_string());
        buf.clear();
    }

    let board = Board {
        width,
        height: map.len(),
        map: map.iter()
            .flat_map(|s| {
                s.chars().map(|ch| match ch {
                    '#' => Ok(Object::Wall),
                    '@' | '.' => Ok(Object::Floor),
                    'O' => Ok(Object::Box),
                    _ => Err(Box::new(PatternError::InvalidMap(format!("Unexpected char in map: {}", ch)))),
                })
            })
            .collect::<Result<_, _>>()?
    };

    match p0 { 
        Some(p) => Ok((p, board)),
        None => Err(Box::new(PatternError::InvalidPosition("RobotNotFound".to_string()))),
    }
}

fn read_moves<R>(reader: &mut R) -> Result<Vec<Direction>, Box<dyn std::error::Error>> 
    where R: BufRead 
{
    let moves = reader.lines()
        .flat_map(|s| {
            s.unwrap().chars()
                .map(|ch| match ch {
                    '^' => Ok(Direction::N),
                    '>' => Ok(Direction::E),
                    'v' => Ok(Direction::S),
                    '<' => Ok(Direction::W),
                    _ => Err(Box::new(PatternError::InvalidMoves(format!("Unexpected char in moves: {}", ch)))),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Result<_, _>>()?
    ;

    Ok(moves)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(2028, solve("./aoc_input_example_1.txt")?);
        assert_eq!(10092, solve("./aoc_input_example_2.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, board, moves) = read_file("aoc_input_example_1.txt")?;

        assert_eq!(8, board.width);
        assert_eq!(8, board.height);

        assert_eq!((2, 2), p0);

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Box, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
        ];

        assert_eq!(expect_map, board.map);

        let expect_moves = vec![
            Direction::W, Direction::N, Direction::N, Direction::E, 
            Direction::E, Direction::E, Direction::S, Direction::S, 
            Direction::W, Direction::S, Direction::E, Direction::E, 
            Direction::S, Direction::W, Direction::W, 
        ];

        assert_eq!(expect_moves, moves);

        Ok(())
    }

    #[test]
    fn next_position() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board {width: 2, height: 2, map: vec![]};
        let p0: Point = (0, 0);
        let p1 = Direction::N.next(p0, board.width, board.height);
        assert_eq!(None, p1);
        let p1 = Direction::W.next(p0, board.width, board.height);
        assert_eq!(None, p1);
        let p1 = Direction::E.next(p0, board.width, board.height);
        assert_eq!(Some((1, 0)), p1);
        let p2 = Direction::E.next(p1.unwrap(), board.width, board.height);
        assert_eq!(None, p2);
        let p2 = Direction::S.next(p1.unwrap(), board.width, board.height);
        assert_eq!(Some((1, 1)), p2);
        let p3 = Direction::S.next(p2.unwrap(), board.width, board.height);
        assert_eq!(None, p3);
        let p3 = Direction::W.next(p2.unwrap(), board.width, board.height);
        assert_eq!(Some((0, 1)), p3);
        let p4 = Direction::W.next(p3.unwrap(), board.width, board.height);
        assert_eq!(None, p4);
        Ok(())
    }

    #[test]
    fn move_robot_internal_example() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Box, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 8,
            height: 8,
            map: initial_map.clone(),
        };
        let p0: Point = (1, 1);

        assert_eq!(None, board.move_robot_internal(p0, Direction::N));

        let p1 = board.move_robot_internal(p0, Direction::E);
        assert_eq!(Some((2, 1)), p1);
        assert_eq!(initial_map, board.map);

        Ok(())
    }

    #[test]
    fn move_robot_and_push_box() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Box, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 8,
            height: 8,
            map: initial_map.clone(),
        };
        let p0: Point = (4, 1);
        let p1 = board.move_robot_internal(p0, Direction::S);
        assert_eq!(Some((4, 2)), p1);

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Box, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, 
        ];
        assert_eq!(expect_map, board.map);

        assert_eq!(None, board.move_robot_internal(p1.unwrap(), Direction::S));

        Ok(())
    }

    #[test]
    fn move_robot_all() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Box,   Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall,  Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall,  Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, 
        ];

        let mut board = Board {
            width: 8,
            height: 8,
            map: initial_map.clone(),
        };
        let p0: Point = (2, 2);

        let moves = vec![
            Direction::W, Direction::N, Direction::N, Direction::E, 
            Direction::E, Direction::E, Direction::S, Direction::S, 
            Direction::W, Direction::S, Direction::E, Direction::E, 
            Direction::S, Direction::W, Direction::W, 
        ];

        board.move_robot(p0, &moves);

        let expected_map = vec![
            Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Box,   Object::Wall, 
            Object::Wall, Object::Wall,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Wall, 
            Object::Wall, Object::Floor, Object::Wall,  Object::Box,   Object::Floor, Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Box,   Object::Floor, Object::Floor, Object::Wall, 
            Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, 
        ];

        assert_eq!(expected_map, board.map);

        Ok(())
    }
}

