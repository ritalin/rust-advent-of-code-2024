use std::{collections::{HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<Option<usize>, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (p0, goal, mut board) = read_file(path)?;

    board.find_route(p0, goal);

    let total = board.count_pass(goal);

    Ok(total)
}

type Point = (usize, usize);

#[derive(PartialEq, Clone, Debug)]
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

    fn mirror(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Wall,
    Road(Option<Hiscore>),
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

#[derive(PartialEq, Clone, Debug)]
struct Hiscore {
    score: u64,
    d: Direction,
    pass: usize,
}

struct Board {
    width: usize,
    height: usize,
    map: Vec<Object>,
}

impl Board {
    fn find_route(&mut self, p0: Point, goal: Point) -> Option<usize> {
        let mut q = VecDeque::<QItem>::from([QItem{ p: p0, d: Direction::E, score: 0 }]);
        let index = self.to_index(p0);
        self.map[index] = Object::Road(Some(Hiscore{score: 0, d: Direction::E, pass: 0}));

        while let Some(item) = q.pop_front() {
            let index = self.to_index(item.p);

            if let Object::Road(Some(hiscore)) = self.map[index].clone() {   
           
                self.find_route_internal(item, hiscore, &mut q);
            }
        }

        let index = self.to_index(goal);

        match &self.map[index] {
           Object::Road(Some(hiscore)) => Some(hiscore.pass),
           _ => None,
        }
    }

    fn find_route_internal(&mut self, QItem{ p: p0, d: d0, score: score0 }: QItem, hiscore: Hiscore, q: &mut VecDeque<QItem>) {
        for d in Direction::iter(&d0) {
            if let Some(p) = d.next(p0, self.width, self.height) {
                let index = self.to_index(p);

                match self.map.get_mut(index) {
                    Some(Object::Road(None)) => {
                        let new_score = score0 + 1 + if d == d0 { 0 } else { 1000 };
                        self.map[index] = Object::Road(Some(Hiscore{ score: new_score, d: d.clone(), pass: hiscore.pass + 1 }));
                        q.push_back(QItem{ p, d, score: new_score });
                    }
                    Some(Object::Road(Some(score))) => {
                        let new_score = score0 + 1 + if d == d0 { 0 } else { 1000 };

                        let score = score.clone();

                        if score.pass == hiscore.pass + 1 {
                        }
                        if score.score > new_score {
                            self.map[index] = Object::Road(Some(Hiscore{ score: new_score, d: d.clone(), pass: hiscore.pass + 1 }));
                            q.push_back(QItem{ p, d, score: new_score });
                        }
                    }
                    _ => {}
                }
            }
        }        
    }

    fn count_pass(&self, goal: Point) -> Option<usize> {
        let index0 = self.to_index(goal);
        let Some(Object::Road(Some(Hiscore{ d, score, .. }))) = self.map.get(index0) else {
            return None;
        };

        let mut q = VecDeque::<QItem>::from([QItem{ p: goal, d: d.mirror(), score: *score }]);
        let mut total: usize = 1;
        let mut index_trace = HashSet::<usize>::new();

        while let Some(item) = q.pop_front() {
            let index = self.to_index(item.p);
            if let Some(Object::Road(Some(Hiscore{ pass: last_pass, .. }))) = self.map.get(index) {
                self.count_pass_internal(item, *last_pass, &mut q, &mut index_trace, &mut total);
            }
        }

        Some(total)
    }

    fn count_pass_internal(&self, QItem{ p: p0, d: d0, score: last_score, ..}: QItem, last_pass: usize, q: &mut VecDeque<QItem>, index_trace: &mut HashSet<usize>, acc: &mut usize) {
        for d in Direction::iter(&d0) {
            if let Some(p) = d.next(p0, self.width, self.height) {
                let index = self.to_index(p);

                if index_trace.contains(&index) { continue; }

                index_trace.insert(index);

                if let Some(Object::Road(Some(hiscore))) = self.map.get(index) {
                    let hiscore = hiscore.clone();

                    if (hiscore.pass + 1 == last_pass) && (hiscore.score <= last_score) {
                        *acc += 1;

                        let score = last_score - 1 - if d == d0 { 0 } else { 1000 };
                        q.push_back(QItem{ p, d, score });
                    }
                }
            }
        }
    }

    #[allow(unused)]
    fn dump(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let index = self.to_index((c, r));

                match self.map.get(index) {
                    Some(Object::Wall) => eprint!("[##]"),
                    Some(Object::Road(Some(hiscore))) => eprint!("[{:>2}]", hiscore.pass),
                    _ => {},
                }
            }
            eprintln!();
        }
        eprintln!();
    }

    #[allow(unused)]
    fn dump_score(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let index = self.to_index((c, r));

                match self.map.get(index) {
                    Some(Object::Wall) => eprint!("[#####]"),
                    Some(Object::Road(Some(hiscore))) => eprint!("[{:>5}]", hiscore.score),
                    _ => {},
                }
            }
            eprintln!();
        }
        eprintln!();
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
    type D = Direction;
    type O = Object;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Some(45), solve("./aoc_input_example_1.txt")?);
        assert_eq!(Some(64), solve("./aoc_input_example_2.txt")?);
        Ok(())
    }

    #[test]
    fn find_route_7() -> Result<(), Box<dyn std::error::Error>> {
        let map = vec![
            Object::Wall, Object::Wall,       Object::Wall,       Object::Road(None), Object::Wall,       
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Road(None), Object::Wall,       
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Road(None), 
            Object::Wall, Object::Road(None), Object::Wall,       Object::Wall,       Object::Wall,
            Object::Wall, Object::Road(None), Object::Road(None), Object::Road(None), Object::Wall,
            Object::Wall, Object::Wall,       Object::Wall,       Object::Wall,       Object::Wall,
        ];

        let mut board = Board {
            width: 5,
            height: 7,
            map,
        };
        let p0 = (1, 5);
        let goal = (3, 0);

        let p = board.find_route(p0, goal);

        let expect_map = vec![
            O::Wall, O::Wall,                                          O::Wall,                                          O::Road(Some(Hiscore{score:3007,d:D::N,pass:7})), O::Wall,       
            O::Wall, O::Road(Some(Hiscore{score:1004,d:D::N,pass:4})), O::Road(Some(Hiscore{score:2005,d:D::E,pass:5})), O::Road(Some(Hiscore{score:2006,d:D::E,pass:6})), O::Road(Some(Hiscore{score:2007,d:D::E,pass:7})), 
            O::Wall, O::Road(Some(Hiscore{score:1003,d:D::N,pass:3})), O::Wall,                                          O::Road(Some(Hiscore{score:3005,d:D::N,pass:5})), O::Wall,       
            O::Wall, O::Road(Some(Hiscore{score:1002,d:D::N,pass:2})), O::Road(Some(Hiscore{score:2003,d:D::E,pass:3})), O::Road(Some(Hiscore{score:2004,d:D::E,pass:4})), O::Road(Some(Hiscore{score:2005,d:D::E,pass:5})), 
            O::Wall, O::Road(Some(Hiscore{score:1001,d:D::N,pass:1})), O::Wall,                                          O::Wall,                                          O::Wall,
            O::Wall, O::Road(Some(Hiscore{score:0,d:D::E,pass:0})),    O::Road(Some(Hiscore{score:1,d:D::E,pass:1})),    O::Road(Some(Hiscore{score:2,d:D::E,pass:2})),    O::Wall,
            O::Wall, O::Wall,                                          O::Wall,                                          O::Wall,                                          O::Wall,
        ];
        assert_eq!(expect_map, board.map);

        assert_eq!(Some(7), p);

        Ok(())
    }

    #[test]
    fn count_pass_route_7() -> Result<(), Box<dyn std::error::Error>> {
        let map = vec![
            O::Wall, O::Wall,                                          O::Wall,                                          O::Road(Some(Hiscore{score:3007,d:D::N,pass:7})), O::Wall,       
            O::Wall, O::Road(Some(Hiscore{score:1004,d:D::N,pass:4})), O::Road(Some(Hiscore{score:2005,d:D::E,pass:5})), O::Road(Some(Hiscore{score:2006,d:D::E,pass:6})), O::Road(Some(Hiscore{score:2007,d:D::E,pass:7})), 
            O::Wall, O::Road(Some(Hiscore{score:1003,d:D::N,pass:3})), O::Wall,                                          O::Road(Some(Hiscore{score:3005,d:D::N,pass:5})), O::Wall,       
            O::Wall, O::Road(Some(Hiscore{score:1002,d:D::N,pass:2})), O::Road(Some(Hiscore{score:2003,d:D::E,pass:3})), O::Road(Some(Hiscore{score:2004,d:D::E,pass:4})), O::Road(Some(Hiscore{score:2005,d:D::E,pass:5})), 
            O::Wall, O::Road(Some(Hiscore{score:1001,d:D::N,pass:1})), O::Wall,                                          O::Wall,                                          O::Wall,
            O::Wall, O::Road(Some(Hiscore{score:0,d:D::E,pass:0})),    O::Road(Some(Hiscore{score:1,d:D::E,pass:1})),    O::Road(Some(Hiscore{score:2,d:D::E,pass:2})),    O::Wall,
            O::Wall, O::Wall,                                          O::Wall,                                          O::Wall,                                          O::Wall,
        ];
        let board = Board {
            width: 5,
            height: 7,
            map,
        };
        let goal = (3, 0);

        assert_eq!(Some(11), board.count_pass(goal));
        Ok(())
    }
}