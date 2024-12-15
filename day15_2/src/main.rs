use std::{collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}, path::Path};

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
            Object::BoxL => Some(board.from_index(i)),
            Object::BoxR | Object::Floor | Object::Wall => None,
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
            if let Some(p) = self.move_robot_internal(p0, d) {
                p0 = p;
            }
        }
    } 

    fn move_robot_internal(&mut self, p0: Point, d: &Direction) -> Option<Point> {
        let Some(p) = d.next(p0, self.width, self.height) else {
            return None;
        };

        let index = self.to_index(p);
        match self.map[index] {
            Object::BoxL | Object::BoxR => {
                match self.try_push_box(p0, p, &d) {
                    Some(q) => {
                        self.swap_box(q);
                        Some(p)
                    }
                    None => None,
                }
            }
            Object::Floor => {
                Some(p)
            },
            Object::Wall => return None,
        }
    }

    fn try_push_box(&self, current: Point, next: Point, d: &Direction) -> Option<VecDeque<(usize, usize)>> {
        match d {
            Direction::W | Direction::E => {
                self.try_push_box_horizontal(next, self.to_index(next), &d)
            }
            Direction::N | Direction::S => {
                let mut trace = HashSet::<usize>::new();
                self.try_push_box_vertical(current, self.to_index(current), &d, &self.map[self.to_index(current)], &mut trace)
            }
        }
    }

    fn try_push_box_horizontal(&self, p0: Point, index_from: usize, d: &Direction) -> Option<VecDeque<(usize, usize)>> {
        let Some(p) = d.next(p0, self.width, self.height) else {
            return None;
        };

        let index_to = self.to_index(p);
        match self.map[index_to] {
            Object::BoxL | Object::BoxR => {
                self.try_push_box_horizontal(p, index_to, &d)
                    .map(|mut q| {
                        q.push_back((index_from, index_to));
                        q
                    })
            },
            Object::Floor => {
                Some(VecDeque::<(usize, usize)>::from([(index_from, index_to)]))
            }
            Object::Wall => {
                None
            }
        }
    }

    fn try_push_box_vertical(&self, p0: Point, index_from: usize, d: &Direction, obj0: &Object, trace: &mut HashSet<usize>) -> Option<VecDeque<(usize, usize)>> {
        let Some(p) = d.next(p0, self.width, self.height) else {
            return None;
        };

        let index_to = self.to_index(p);
        if trace.contains(&index_to) {
            return Some(VecDeque::<(usize, usize)>::new());
        }

        trace.insert(index_to);

        let obj = &self.map[index_to];
        match obj {
            Object::BoxL | Object::BoxR if *obj == *obj0 => {
                match self.try_push_box_vertical(p, index_to, d, obj, trace) {
                    Some(mut q) => {
                        q.push_back((index_from, index_to));
                        Some(q)
                    }
                    None => None,
                }
            }
            Object::BoxL => {
                let l_p = p;
                let l_index_to = index_to;
                let r_p = (p.0 + 1, p.1);
                let r_index_to = self.to_index(r_p);
                
                match (self.try_push_box_vertical(l_p, l_index_to, d, obj, trace), self.try_push_box_vertical(r_p, r_index_to, d, &self.map[r_index_to], trace)) {
                    (Some(mut l_q), Some(mut r_q)) => {
                        l_q.append(&mut r_q);
                        l_q.push_back((index_from, index_to));
                        Some(l_q)
                    }
                    _ => None,
                }
            }
            Object::BoxR => {
                let l_p = (p.0 - 1, p.1);
                let l_index_to = self.to_index((p.0 - 1, p.1));
                let r_p = p;
                let r_index_to = index_to;
                
                match (self.try_push_box_vertical(l_p, l_index_to, d, &self.map[l_index_to], trace), self.try_push_box_vertical(r_p, r_index_to, d, obj, trace)) {
                    (Some(mut l_q), Some(mut r_q)) => {
                        l_q.append(&mut r_q);
                        l_q.push_back((index_from, index_to));
                        Some(l_q)
                    }
                    _ => None,
                }
            }
            Object::Floor => {
                Some(VecDeque::<(usize, usize)>::from([(index_from, index_to)]))
            }
            Object::Wall => {
                None
            }
        }
    }

    fn swap_box(&mut self, mut queue: VecDeque<(usize, usize)>) {
        while let Some((index_from, index_to)) = queue.pop_front() {
            self.map.swap(index_from, index_to);
        }
    }

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
    }

    #[allow(unused)]
    fn dump(&self, p0: Point) {
        let p_index = self.to_index(p0);

        for r in 0..self.height {
            for c in 0..self.width {
                let index = self.to_index((c, r));
                let ch = match self.map[index] {
                    Object::Floor if index == p_index => '@',
                    Object::Floor => '.',
                    Object::BoxL => '[',
                    Object::BoxR => ']',
                    Object::Wall => '#',
                };
                eprint!("{}", ch);
            }
            eprintln!();
        }
        eprintln!();
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
    BoxL,
    BoxR,
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
            p0 = Some((x * 2, map.len()));
        }

        width = s.len();
        map.push(s.to_string());
        buf.clear();
    }

    let board = Board {
        width: width * 2,
        height: map.len(),
        map: map.iter()
            .flat_map(|s| {
                s.chars()
                .flat_map(|ch| match ch {
                    '#' => vec![Ok(Object::Wall), Ok(Object::Wall)],
                    '@' | '.' => vec![Ok(Object::Floor), Ok(Object::Floor)],
                    'O' => vec![Ok(Object::BoxL), Ok(Object::BoxR)],
                    _ => vec![Err(Box::new(PatternError::InvalidMap(format!("Unexpected char in map: {}", ch))))],
                })
            })
            .collect::<Result<Vec<_>, _>>()?
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
    #[ignore = "reason"]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(2028, solve("./aoc_input_example_1.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, board, moves) = read_file("aoc_input_example_1.txt")?;

        assert_eq!(14, board.width);
        assert_eq!(7, board.height);

        assert_eq!((10, 3), p0);

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        assert_eq!(expect_map, board.map);

        let expect_moves = vec![
            Direction::W, Direction::S, Direction::S, Direction::W, 
            Direction::W, Direction::N, Direction::N, Direction::W, 
            Direction::W, Direction::N, Direction::N, 
        ];

        assert_eq!(expect_moves, moves);

        Ok(())
    }

    #[test]
    fn move_robot_internal_example() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 14,
            height: 7,
            map: initial_map.clone(),
        };
        let p0: Point = (2, 1);

        assert_eq!(None, board.move_robot_internal(p0, &Direction::N));

        let p1 = board.move_robot_internal(p0, &Direction::E);
        assert_eq!(Some((3, 1)), p1);
        assert_eq!(initial_map, board.map);

        Ok(())
    }

    #[test]
    fn move_robot_and_push_box() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 14,
            height: 7,
            map: initial_map.clone(),
        };
        let p0: Point = (10, 3);
        let p1 = board.move_robot_internal(p0, &Direction::W);
        assert_eq!(Some((9, 3)), p1);

        let p0: Point = (7, 5);
        let p1 = board.move_robot_internal(p0, &Direction::N);
        assert_eq!(Some((7, 4)), p1);

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        assert_eq!(expect_map, board.map);

        assert_eq!(None, board.move_robot_internal(p1.unwrap(), &Direction::N));

        Ok(())
    }

    #[test]
    fn move_robot_and_push_box_vertical() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 14,
            height: 7,
            map: initial_map.clone(),
        };
        let p0: Point = (6, 5);
        let p1 = board.move_robot_internal(p0, &Direction::N);
        assert_eq!(Some((6, 4)), p1);

        board.dump(p1.unwrap());

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        assert_eq!(expect_map, board.map);

        Ok(())
    }

    #[test]
    fn move_robot_and_push_box_vertical_2() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 14,
            height: 7,
            map: initial_map.clone(),
        };
        let p0: Point = (7, 5);
        board.dump(p0);
        let p1 = board.move_robot_internal(p0, &Direction::N);
        assert_eq!(Some((7, 4)), p1);

        board.dump(p1.unwrap());

        let expect_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Wall,  Object::Wall , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        assert_eq!(expect_map, board.map);

        let p2 = board.move_robot_internal(p1.unwrap(), &Direction::N);
        assert_eq!(None, p2);

        Ok(())
    }

    #[test]
    fn move_robot_all() -> Result<(), Box<dyn std::error::Error>> {
        let initial_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        let mut board = Board {
            width: 8,
            height: 8,
            map: initial_map.clone(),
        };
        let p0: Point = (2, 2);

        let moves = vec![
            Direction::W, Direction::S, Direction::S, Direction::W, 
            Direction::W, Direction::N, Direction::N, Direction::W, 
            Direction::W, Direction::N, Direction::N, 
        ];

        board.move_robot(p0, &moves);
        board.dump((0, 0));

        let expected_map = vec![
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall,  Object::Wall,  Object::Floor, Object::Floor, Object::Wall, Object::Wall, 
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL , Object::BoxR , Object::BoxL , Object::BoxR , Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::BoxL,  Object::BoxR,  Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Floor, Object::Wall, Object::Wall,
            Object::Wall, Object::Wall, Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall,  Object::Wall, Object::Wall, 
        ];

        assert_eq!(expected_map, board.map);

        Ok(())
    }
}

