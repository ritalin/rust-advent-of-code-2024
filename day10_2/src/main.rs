use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let board = read_file(path)?;
    
    let total = board.map.iter().enumerate()
        .filter_map(|(i, x)| match *x {
            0 => Some(board.trail(board.from_index(i))),
            _ => None,
        })
        .sum::<usize>()
    ;

    Ok(total)
}

type Point = (usize, usize);
struct Board {
    width: usize,
    height: usize,
    map: Vec<u8>,
}

impl Board {
    fn move_to(&self, (x0, y0): Point, direction: Direction) -> Option<Point> {
        let (diff_x, diff_y) = match direction {
            Direction::N => (0, -1),
            Direction::E => (1, 0),
            Direction::S => (0, 1),
            Direction::W => (-1, 0),
        };

        let x = x0 as isize + diff_x;
        let y = y0 as isize + diff_y;

        if x < 0 { return None; }
        if y < 0 { return None; }
        if x as usize >= self.width { return None; }
        if y as usize >= self.height { return None; }
        
        Some((x as usize, y as usize))
    }

    fn trail(&self, p0: Point) -> usize {
        let mut total = 0;
        trail_internal(self, 9, p0, 0, &mut total);

        total
    }

    fn peek(&self, index: usize) -> u8 {
        self.map[index]
    }

    fn from_index(&self, index: usize) -> Point {
        (index % self.width, index / self.width)
    }

    fn to_index(&self, (x, y): Point) -> usize {
        x + y * self.width
    }
}

fn trail_internal(board: &Board, top_peak: u8, p0: Point, current_peak: u8, total: &mut usize) {
    for d in Direction::iter() {
        if let Some(p) = board.move_to(p0, d) {
            match board.peek(board.to_index(p)) {
                peek if (peek == top_peak) && (peek == current_peak + 1) => {
                    *total += 1;
                } 
                peek if peek == current_peak + 1 => {
                    trail_internal(board, top_peak, p, current_peak + 1, total);
                }
                _ => {}
            }
        }
    }
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

    let (widths, map): (Vec<usize>, Vec<Vec<u8>>) = reader.lines()
        .map(|row| {(
            row.as_ref().unwrap().len(), 
            row.unwrap().chars().map(|ch| (ch as u8 - '0' as u8)).collect::<Vec<_>>()
        )})
        .unzip()
    ;

    Ok(Board {
        height: widths.len(),
        width: *widths.first().unwrap(),
        map: map.into_iter().flat_map(std::convert::identity).collect(),
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(81, actual);
        Ok(())
    }

    #[test]
    fn trail_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board {
            width: 8,
            height: 8,
            map: vec![
                8, 9, 0, 1, 0, 1, 2, 3, 
                7, 8, 1, 2, 1, 8, 7, 4, 
                8, 7, 4, 3, 0, 9, 6, 5, 
                9, 6, 5, 4, 9, 8, 7, 4, 
                4, 5, 6, 7, 8, 9, 0, 3, 
                3, 2, 0, 1, 9, 0, 1, 2, 
                0, 1, 3, 2, 9, 8, 0, 1, 
                1, 0, 4, 5, 6, 7, 3, 2, 
            ],
        };

        assert_eq!(20, board.trail((2, 0)));
        assert_eq!(24, board.trail((4, 0)));
        assert_eq!(10, board.trail((4, 2)));
        assert_eq!(4, board.trail((6, 4)));
        assert_eq!(1, board.trail((2, 5)));
        assert_eq!(4, board.trail((5, 5)));
        assert_eq!(5, board.trail((0, 6)));
        assert_eq!(8, board.trail((6, 6)));
        assert_eq!(5, board.trail((1, 7)));
        Ok(())
    }
}
