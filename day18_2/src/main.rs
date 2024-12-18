use std::{collections::VecDeque, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt", (71, 71))?);
    Ok(())
}

fn solve<P>(path: P, (width, height): (usize, usize)) -> Result<Option<Point>, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let walls = read_file(path)?;
    let mut left = 0;
    let mut right = walls.len() -1;

    while left <= right {
        let mid = (left + right) / 2;

        match solve_internal(&walls, (width, height), mid) {
            Some(_) => left = mid + 1,
            None => right = mid - 1,
        }
    }

    Ok(Some(walls[right]))
}

fn solve_internal(walls: &[Point], (width, height): (usize, usize), limit: usize) -> Option<u64> {
    let mut board = Board::new(width, height, walls, limit);

    let p0 = (0, 0);
    let goal = (width - 1, height - 1);

    board.find_route(p0, goal)
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

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Wall,
    Road(Option<u64>),
}

struct QItem {
    p: Point,
    d: Direction,
    cost: u64,
}

struct Board {
    width: usize,
    height: usize,
    map: Vec<Object>,
}

impl Board {
    fn new(width: usize, height: usize, roads: &[Point], limit: usize) -> Self {
        let mut map = std::iter::repeat(Object::Road(None))
            .take(width * height)
            .collect::<Vec<_>>()
        ;

        roads.into_iter()
            .take(limit)
            .for_each(|p| {
                map[Board::to_index(*p, width)] = Object::Wall;
            })
        ;

        Board {
            width,
            height,
            map,
        }
    }
    
    fn find_route(&mut self, p0: Point, goal: Point) -> Option<u64> {
        let mut q = VecDeque::<QItem>::from([QItem{ p: p0, d: Direction::E, cost: 0 }]);
        let index = Board::to_index(p0, self.width);
        self.map[index] = Object::Road(Some(0));

        while let Some(QItem{ p, d , cost: distance0 }) = q.pop_front() {
            let index = Board::to_index(p, self.width);

            if let Object::Road(Some(distance)) = self.map[index] {
                if distance <= distance0 {
                    self.find_route_internal(p, d, distance, &mut q);
                }
            }
        }

        let index = Board::to_index(goal, self.width);

        match self.map[index] {
           Object::Road(score) => score,
           _ => None,
        }
    }

    fn find_route_internal(&mut self, p0: Point, d0: Direction, cost0: u64, q: &mut VecDeque<QItem>) {
        for d in Direction::iter(&d0) {
            if let Some(p) = d.next(p0, self.width, self.height) {
                let index = Board::to_index(p, self.width);

                match self.map[index] {
                    Object::Road(None) => {
                        let new_cost = cost0 + 1;
                        self.map[index] = Object::Road(Some(new_cost));
                        q.push_back(QItem{p, d, cost: new_cost});
                    }
                    Object::Road(Some(cost)) => {
                        let new_cost = cost0 + 1;
                        if cost > new_cost {
                            self.map[index] = Object::Road(Some(new_cost));
                            q.push_back(QItem{p, d, cost: new_cost});
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn to_index((x, y): Point, width: usize) -> usize {
        x + y * width
    }

    #[allow(unused)]
    fn dump(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let index = Board::to_index((c, r), self.width);

                match self.map.get(index) {
                    Some(Object::Wall) => eprint!("[##]"),
                    Some(Object::Road(_)) => eprint!("[..]"),
                    _ => {},
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn read_file<P>(path: P) -> Result<Vec<Point>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let mut map = vec![];

    for line in reader.lines() {
        let line = line?;

        let p = line.split(",")
            .map(|p| p.parse::<usize>())
            .collect::<Result<Vec<_>, _>>()?
        ;
        map.push((p[0], p[1]));
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let walls = read_file("./aoc_input_example.txt")?;
        assert_eq!(Some(24), solve_internal(&walls, (7, 7), 20));
        assert_eq!(None, solve_internal(&walls, (7, 7), 21));
        assert_eq!(Some((6, 1)), solve("./aoc_input_example.txt", (7, 7))?);
        Ok(())
    }
}
