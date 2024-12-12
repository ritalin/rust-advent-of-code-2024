use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let board = read_file(path)?;
    let total = eval_region(board).into_iter()
        .map(|r| r.area * r.fence)
        .sum::<usize>()
    ;

    Ok(total)
}

type Point = (usize, usize);
struct Board {
    width: usize,
    height: usize,
    map: Vec<char>,
}

impl Board {
    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }

    fn from_index(&self, p: usize) -> Point {
        (p % self.width, p / self.width)
    }

    fn neighbor(&self, (x0, y0): Point, d: &Direction) -> Option<Point> {
        let x = x0 as isize;
        let y = y0 as isize;

        let (next_x, next_y) = match d {
            Direction::N => (x, y - 1), 
            Direction::S => (x, y + 1), 
            Direction::W => (x -1, y), 
            Direction::E => (x + 1, y),
        };

        if next_x < 0 { return None; }
        if next_x >= self.width as isize { return None; }
        if next_y < 0 { return None; }
        if next_y >= self.height as isize { return None; }

        Some((next_x as usize, next_y as usize))
    }

    fn edge_end(&self, (x0, y0): Point, d: &Direction) -> Option<Point> {
        let x = x0 as isize;
        let y = y0 as isize;

        let (next_x, next_y) = match d {
            Direction::N => (x, y - 1), 
            Direction::S => (x, y + 1), 
            Direction::W => (x -1, y), 
            Direction::E => (x + 1, y),
        };

        if next_x < 0 { return None; }
        if next_x > self.width as isize { return None; }
        if next_y < 0 { return None; }
        if next_y > self.height as isize { return None; }

        Some((next_x as usize, next_y as usize))
    }

    fn count_edge(&self, p0: Point, fences: &HashSet<(Point, Point)>, directions: impl Iterator<Item = Direction>) -> usize {
        directions
            .filter_map(|d| self.edge_end(p0, &d))
            .filter(|p| fences.contains(&(p0, *p)) || fences.contains(&(*p, p0)))
            .count()
    }

    fn is_corner(&self, p0: Point, fences: &HashSet<(Point, Point)>) -> bool {
        let horizontal = self.count_edge(p0, fences, [Direction::W, Direction::E].into_iter());
        let vertical = self.count_edge(p0, fences, [Direction::N, Direction::S].into_iter());

        (horizontal > 0) && (vertical > 0)
    }
}

#[derive(PartialEq, Debug)]
struct Region {
    area: usize,
    fence: usize,
}

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    N, E, S, W,
}
impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::N, Direction::S, Direction::W, Direction::E].into_iter()
    }
}

fn read_file<P>(path: P) -> Result<Board, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let (widths, map): (Vec<usize>, Vec<String>) = reader.lines()
        .map(|row| row.map(|s| (s.len(), s.clone())).unwrap())
        .unzip()
    ;

    Ok(Board {
        width: *widths.first().unwrap(),
        height: map.len(),
        map: map.into_iter().flat_map(|row| row.chars().collect::<Vec<_>>()).collect(),
    })
}

fn eval_region(board: Board) -> Vec<Region> {
    let mut trace = std::iter::repeat(false).take(board.width * board.height).collect::<Vec<_>>();

    board.map.iter().enumerate()
        .filter_map(|(i, _)| match trace[i] {
            true => None,
            false => Some(eval_region_internal(&board, i, &mut trace))
        })
        .collect()
}

fn eval_region_internal(board: &Board, p0: usize, trace: &mut Vec<bool>) -> Region {
    let mut area: usize = 0;
    let mut fences = HashSet::<(Point, Point)>::new();
    let p = board.from_index(p0);

    eval_region_rec(board, p, trace, &mut area, &mut fences);

    let mut left_fences: HashSet<(Point, Point)> = fences.iter().map(|x| x.clone()).collect();

    let mut fence = eval_fence(board, p, &fences, &mut left_fences);

    while let Some((((x0, y0), (x1, _)), mut rest_fences)) = pick_fence(board, left_fences) {
        let dirs = match x0 == x1 { 
            false => [Direction::W, Direction::E],
            true => [Direction::N, Direction::S],
        };

        for d in dirs {
            eval_fence_rec(board, (x0, y0), (x0, y0), &fences, &mut rest_fences, &d, &mut fence);
        }
        left_fences = rest_fences;
    }

    Region { area, fence }
}

fn eval_region_rec(board: &Board, p0: Point, trace: &mut Vec<bool>, acc_area: &mut usize, fences: &mut HashSet<(Point, Point)>) {
    let index = board.to_index(p0);

    let ch = board.map[index];
    trace[index] = true;
    *acc_area += 1;

    for d in Direction::iter() {
        match board.neighbor(p0, &d) {
            Some(next) if (ch == board.map[board.to_index(next)]) => {
                if !trace[board.to_index(next)] {
                    eval_region_rec(board, next, trace, acc_area, fences);
                }
            }
            _ => {
                let (x, y) = p0;
                let (edge0, edge1) = match d {
                    Direction::N => ((x, y), (x + 1, y)), 
                    Direction::S => ((x, y + 1), (x + 1, y + 1)), 
                    Direction::W => ((x, y), (x, y + 1)), 
                    Direction::E => ((x + 1, y), (x + 1, y + 1)),
                };
                fences.insert((edge0, edge1));
            }
        }
    }
}

fn eval_fence(board: &Board, edge0: Point, fences: &HashSet<(Point, Point)>, left_fences: &mut HashSet<(Point, Point)>) -> usize {
    let mut fence: usize = 0;
    eval_fence_rec(board, edge0, edge0, fences, left_fences, &Direction::E, &mut fence);

    fence
}

fn pick_fence(board: &Board, fences: HashSet<(Point, Point)>) -> Option<((Point, Point), HashSet<(Point, Point)>)> 
{
    let mut cornre_iter = fences.iter()
        .filter(|(p, _)| board.is_corner(*p, &fences))
    ;

    match cornre_iter.next() {
        Some(item) => Some((item.clone(), fences)),
        None => None,
    }
}

fn eval_fence_rec(board: &Board, p0: Point, p: Point, fences: &HashSet<(Point, Point)>, left_fences: &mut HashSet<(Point, Point)>, direction: &Direction, fence: &mut usize) {
    let mut edge0 = p;

    while let Some(edge) = board.edge_end(edge0, direction) {
        match (left_fences.contains(&(edge0, edge)), left_fences.contains(&(edge, edge0))) {
            (true, false) => {
                left_fences.remove(&(edge0, edge));
            }
            (false, true) => {
                left_fences.remove(&(edge, edge0));
            }
            _ => break,
        }
        edge0 = edge;

        if board.is_corner(edge0, fences) {
            break;
        }
    }

    if edge0 == p {
        return;
    }

    *fence += 1;

    if edge0 != p0 {
        let next_dirs = match *direction {
            Direction::N | Direction::S => [Direction::W, Direction::E],
            Direction::W | Direction::E => [Direction::N, Direction::S],
        };
        for d in next_dirs {
            eval_fence_rec(board, p0, edge0, fences, left_fences, &d, fence);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(80, solve("./aoc_input_example_1.txt")?);
        assert_eq!(236, solve("./aoc_input_example_2.txt")?);
        assert_eq!(368, solve("./aoc_input_example_3.txt")?);
        Ok(())
    }

    #[test]
    fn eval_fence_example_1() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_1.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 4, fence: 4},
            Region{area: 4, fence: 4},
            Region{area: 4, fence:  8},
            Region{area: 1, fence: 4},
            Region{area: 3, fence: 4},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }

    #[test]
    fn eval_fence_example_2() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_2.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 17, fence: 12},
            Region{area: 4, fence: 4},
            Region{area: 4, fence: 4},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }

    #[test]
    fn eval_fence_example_3() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_3.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 28, fence: 12},
            Region{area: 4, fence: 4},
            Region{area: 4, fence: 4},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }

    #[test]
    fn eval_fence_example_4() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_4.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 32, fence: 24},
            Region{area: 4, fence: 4},
            Region{area: 4, fence: 4},
            Region{area: 1, fence: 4},
            Region{area: 4, fence: 4},
            Region{area: 4, fence: 4},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }
}