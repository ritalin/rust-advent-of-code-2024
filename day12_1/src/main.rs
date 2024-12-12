use std::{fs::File, io::{BufRead, BufReader}, path::Path};

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

    fn neighbor(&self, (x0, y0): Point, d: Direction) -> Option<Point> {
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
}

#[derive(PartialEq, Debug)]
struct Region {
    area: usize,
    fence: usize,
}

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
    let mut fence: usize = 0;

    eval_region_rec(board, board.from_index(p0), trace, &mut area, &mut fence);

    Region { area, fence }
}

fn eval_region_rec(board: &Board, p0: Point, trace: &mut Vec<bool>, acc_area: &mut usize, acc_fence: &mut usize) {
    let index = board.to_index(p0);

    let ch = board.map[index];
    trace[index] = true;
    *acc_area += 1;

    for d in Direction::iter() {
        match board.neighbor(p0, d) {
            Some(next) if (ch == board.map[board.to_index(next)]) => {
                if !trace[board.to_index(next)] {
                    eval_region_rec(board, next, trace, acc_area, acc_fence);
                }
            }
            _ => {
                *acc_fence += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(140, solve("./aoc_input_example_1.txt")?);
        assert_eq!(772, solve("./aoc_input_example_2.txt")?);
        assert_eq!(1930, solve("./aoc_input_example_3.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_1.txt")?;
        assert_eq!(4, board.width);
        assert_eq!(4, board.height);
        assert_eq!("AAAABBCDBBCCEEEC".chars().collect::<Vec<_>>(), board.map);
        Ok(())
    }

    #[test]
    fn eval_fence_example_1() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_1.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 4, fence: 10},
            Region{area: 4, fence: 8},
            Region{area: 4, fence:  10},
            Region{area: 1, fence: 4},
            Region{area: 3, fence: 8},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }

    #[test]
    fn eval_fence_example_2() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_2.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 21, fence: 36},
            Region{area: 1, fence: 4},
            Region{area: 1, fence: 4},
            Region{area: 1, fence: 4},
            Region{area: 1, fence: 4},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }

    #[test]
    fn eval_fence_example_3() -> Result<(), Box<dyn std::error::Error>> {
        let board = read_file("./aoc_input_example_3.txt")?;
        let regions = eval_region(board);
        let expect_regions = vec![
            Region{area: 12, fence: 18},
            Region{area: 4, fence: 8},
            Region{area: 14, fence: 28},
            Region{area: 10, fence: 18},
            Region{area: 13, fence: 20},
            Region{area: 11, fence: 20},
            Region{area: 1, fence: 4},
            Region{area: 13, fence: 18},
            Region{area: 14, fence: 22},
            Region{area: 5, fence: 12},
            Region{area: 3, fence: 8},
        ];
        assert_eq!(expect_regions, regions);
        Ok(())
    }
}