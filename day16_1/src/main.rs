use std::{collections::VecDeque, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<Option<u64>, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (p0, goal, mut board) = read_file(path)?;
    
    let score = board.find_route(p0, goal);

    Ok(score)
}

type Point = (usize, usize);

#[derive(PartialEq)]
enum Direction {
    N, E, S, W,
}

impl Direction {
    fn iter(d0: &Direction) -> Vec<Direction> {
        [Direction::N, Direction::E, Direction::S, Direction::W].into_iter()
            .filter(|d| match *d0 {
                Direction::N | Direction::S if (*d == *d0) || (*d == Direction::W) || (*d == Direction::E) => true,
                Direction::E | Direction::W if (*d == *d0) || (*d == Direction::N) || (*d == Direction::S) => true,
                _ => false,
            })
            .collect()
    }

    fn next(&self, (x0, y0): Point, width: usize, height: usize) -> Option<Point> {
        let (x, y) = match self {
            Direction::N if y0 > 0 => (x0, y0 - 1),
            Direction::E => (x0 + 1, y0),
            Direction::S => (x0, y0 + 1),
            Direction::W if x0 > 0 => (x0 -1, y0),
            _ => return None,
        };

        if x >= width { return None; }
        if y >= height { return None; }

        Some((x, y))
    }
}

#[derive(PartialEq, Debug)]
enum Object {
    Wall,
    Road(Option<u64>),
}

#[derive(Debug)]
enum PatternError {
    InvalidMap(String),
}
impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::InvalidMap(msg) => write!(f, "PatternError: {}", msg),
        }
    }
}
impl std::error::Error for PatternError {}

struct QItem {
    p: Point,
    d: Direction,
    score: u64,
}

struct Board {
    width: usize,
    height: usize,
    map: Vec<Object>,
}

impl Board {
    fn find_route(&mut self, p0: Point, goal: Point) -> Option<u64> {
        let mut q = VecDeque::<QItem>::from([QItem{ p: p0, d: Direction::E, score: 0 }]);
        let index = self.to_index(p0);
        self.map[index] = Object::Road(Some(0));

        while let Some(QItem{ p, d , score: score0 }) = q.pop_front() {
            let index = self.to_index(p);

            if let Object::Road(Some(score)) = self.map[index] {
                if score0 <= score {
                    self.find_route_internal(p, d, score, &mut q);
                }
            }
        }

        let index = self.to_index(goal);

        match self.map[index] {
           Object::Road(score) => score,
           _ => None,
        }
    }

    fn find_route_internal(&mut self, p0: Point, d0: Direction, score0: u64, q: &mut VecDeque<QItem>) {
        for d in Direction::iter(&d0) {
            if let Some(p) = d.next(p0, self.width, self.height) {
                let index = self.to_index(p);

                match self.map[index] {
                    Object::Road(None) => {
                        let new_score = score0 + 1 + if d == d0 { 0 } else { 1000 };
                        self.map[index] = Object::Road(Some(new_score));
                        q.push_back(QItem{p, d, score: new_score});
                    }
                    Object::Road(Some(score)) => {
                        let new_score = score0 + 1 + if d == d0 { 0 } else { 1000 };
                        if score > new_score {
                            self.map[index] = Object::Road(Some(new_score));
                            q.push_back(QItem{p, d, score: new_score});
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }
}

fn read_file<P>(path: P) -> Result<(Point, Point, Board), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let mut p0: Option<Point> = None;
    let mut goal: Option<Point> = None;
    let mut width: usize = 0;
    let mut height: usize = 0;
    let mut map = vec![];

    for (r, line) in reader.lines().enumerate() {
        let line = line?;
        width = line.len();
        height += 1;

        p0 = p0.or_else(|| {
            match line.find('S') {
                Some(c) => Some((c, r)),
                None => None,
            }
        });
        goal = goal.or_else(|| {
            match line.find('E') {
                Some(c) => Some((c, r)),
                None => None,
            }
        });

        let row = line.chars().into_iter()
            .map(|ch| match ch {
                '#' => Ok(Object::Wall),
                'S' | 'E' | '.' => Ok(Object::Road(None)),
                _ => Err(PatternError::InvalidMap("Unexpected char for map".into()))
            })
            .collect::<Result<Vec<_>, _>>()
        ;
        map.append(&mut row?);
    }

    let board = Board {
        width,
        height,
        map,
    };

    Ok((p0.unwrap(), goal.unwrap(), board))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Some(7036), solve("./aoc_input_example_1.txt")?);
        assert_eq!(Some(11048), solve("./aoc_input_example_2.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, goal, board) = read_file("aoc_input_example_1.txt")?;

        assert_eq!(15, board.width);
        assert_eq!(15, board.height);

        assert_eq!((1, 13), p0);
        assert_eq!((13, 1), goal);

        let expect_map = vec![
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall, 
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall, 
        ];
        assert_eq!(expect_map, board.map);
        Ok(())
    }

    #[test]
    fn find_route_5() -> Result<(), Box<dyn std::error::Error>> {
        let map = vec![
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,
        ];

        let mut board = Board {
            width: 5,
            height: 4,
            map,
        };
        let p0 = (1, 2);
        let goal = (4, 0);

        let p = board.find_route(p0, goal);

        let expect_map = vec![
            Object::Wall, Object::Road(Some(1002)), Object::Road(Some(2003)), Object::Road(Some(2004)), Object::Road(Some(2005)), 
            Object::Wall, Object::Road(Some(1001)), Object::Wall,       Object::Wall,       Object::Wall,
            Object::Wall, Object::Road(Some(0)), Object::Road(Some(1)), Object::Road(Some(2)), Object::Wall,
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,
        ];
        assert_eq!(expect_map, board.map);

        assert_eq!(Some(2005), p);

        Ok(())
    }
}