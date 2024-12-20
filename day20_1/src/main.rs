use std::{collections::VecDeque, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt", 100)?);
    Ok(())
}

fn solve<P>(path: P, threshold: u64) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (p0, goal, mut board) = read_file(path)?;
    
    let _ = board.find_route(p0, goal);

    let total = board.map.iter().enumerate()
        .filter_map(|(i, _)| board.apply_cheat(board.from_index(i)))
        .flat_map(std::convert::identity)
        .filter(|cheat| *cheat >= threshold)
        .count()
    ;

    Ok(total)
}

type Point = (usize, usize);

#[derive(PartialEq)]
enum Direction {
    N, E, S, W,
}

impl Direction {
    fn iter() -> Vec<Direction> {
        vec![Direction::N, Direction::E, Direction::S, Direction::W]
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
    score: u64,
}

struct Board {
    width: usize,
    height: usize,
    map: Vec<Object>,
}

impl Board {
    fn find_route(&mut self, p0: Point, goal: Point) -> Option<u64> {
        let mut q = VecDeque::<QItem>::from([QItem{ p: p0, score: 0 }]);
        let index = self.to_index(p0);
        self.map[index] = Object::Road(Some(0));

        while let Some(QItem{ p , score: score0 }) = q.pop_front() {
            let index = self.to_index(p);

            if let Object::Road(Some(score)) = self.map[index] {
                if score0 <= score {
                    self.find_route_internal(p, score, &mut q);
                }
            }
        }

        let index = self.to_index(goal);

        match self.map[index] {
           Object::Road(score) => score,
           _ => None,
        }
    }

    fn find_route_internal(&mut self, p0: Point, score0: u64, q: &mut VecDeque<QItem>) {
        for d in Direction::iter() {
            if let Some(p) = d.next(p0, self.width, self.height) {
                let index = self.to_index(p);

                match self.map[index] {
                    Object::Road(None) => {
                        let new_score = score0 + 1;
                        self.map[index] = Object::Road(Some(new_score));
                        q.push_back(QItem{p, score: new_score});
                    }
                    Object::Road(Some(score)) => {
                        let new_score = score0 + 1;
                        if score > new_score {
                            self.map[index] = Object::Road(Some(new_score));
                            q.push_back(QItem{p, score: new_score});
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn apply_cheat(&self, p0: Point) -> Option<Vec<u64>> {
        let mut cheats = vec![];
        
        let index0 = self.to_index(p0);
        let Some(Object::Road(Some(score0))) = self.map.get(index0) else {
            return None;
        };

        for d1 in Direction::iter() {
            let Some(p1) = d1.next(p0, self.width, self.height) else {
                continue;
            };

            let index = self.to_index(p1);
            let Some(Object::Wall) = self.map.get(index) else {
                continue;
            };
                    
            for d2 in Direction::iter() {
                let Some(p2) = d2.next(p1, self.width, self.height) else {
                    continue;
                };

                let index = self.to_index(p2);
                let Some(Object::Road(Some(score))) = self.map.get(index) else {
                    continue;
                };

                if score > score0 {
                    cheats.push(score - score0 - 2);
                }
            }
        }

        (cheats.len() > 0).then(|| cheats)
    }


    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
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
        assert_eq!(44, solve("./aoc_input_example.txt", 2)?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, goal, board) = read_file("./aoc_input_example_small.txt")?;

        assert_eq!(5, board.width);
        assert_eq!(6, board.height);

        assert_eq!((1, 4), p0);
        assert_eq!((4, 4), goal);

        let expect_map = vec![
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,       
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall, 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Road(None),       
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,       
        ];
        assert_eq!(expect_map, board.map);

        Ok(())
    }

    #[test]
    fn find_path_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, goal, mut board) = read_file("./aoc_input_example.txt")?;
    
        assert_eq!(Some(84), board.find_route(p0, goal));
        
        Ok(())
    }

    #[test]
    fn apply_cheet_example() -> Result<(), Box<dyn std::error::Error>> {
        let (p0, goal, mut board) = read_file("./aoc_input_example.txt")?;
        let _ = board.find_route(p0, goal);

        assert_eq!(None, board.apply_cheat((3, 3)));
        assert_eq!(Some(vec![0]), board.apply_cheat((2, 1)));
        assert_eq!(Some(vec![12]), board.apply_cheat((7, 1)));
        assert_eq!(Some(vec![40, 64]), board.apply_cheat((7, 7)));

        Ok(())
    }
}

