use std::{fs::File, io::{BufRead, BufReader}, ops::RangeInclusive, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = Board {width: 101, height: 103};
    println!("total: {:?}", solve("./aoc_input.txt", board)?);
    Ok(())
}

fn solve<P>(path: P, board: Board) -> Result<Option<usize>, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let robots = read_file(path)?;
    
    Ok(move_robot(&board, robots))
}

struct Board {
    width: isize,
    height: isize,
}

impl Board {
    fn to_index(&self, (x, y): (isize, isize)) -> usize {
        (x + y * self.width) as usize
    }
}

#[derive(PartialEq, Debug)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn read_file<P>(path: P) -> Result<Vec<Robot>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    
    reader.lines()
        .map(|s| parse_line(&s?))
        .collect()
}

fn parse_line(s: &str) -> Result<Robot, Box<dyn std::error::Error>>  {
    let parts = s.split_ascii_whitespace().collect::<Vec<_>>();

    let p = parse_line_internal(&parts[0][2..])?;
    let v = parse_line_internal(&parts[1][2..])?;

    Ok(Robot { position: (p[0], p[1]), velocity: (v[0], v[1]) })
}

fn parse_line_internal(pair: &str) -> Result<Vec<isize>, Box<dyn std::error::Error>>  {
    Ok(pair.split(',').map(|x| x.parse::<isize>()).collect::<Result<Vec<_>, _>>()?)
}

fn move_robot(board: &Board, mut robots: Vec<Robot>) -> Option<usize> {
    for t in 1..10000 {
        let mut traces: String = std::iter::repeat('.').take(board.width as usize * board.height as usize).collect();

        robots = robots.into_iter()
            .map(|robot| move_robot_internal(board, robot, 1))
            .map(|robot| {
                let index = board.to_index(robot.position);
                traces.replace_range(index..=index, "#");
                robot
            })
            .collect()
        ;

        if is_tree_drawn(board, &traces) {
            dump_trace(board, &traces, t);
        }
    }

    None
}

fn move_robot_internal(board: &Board, Robot {position: (p_x, p_y), velocity: (v_x, v_y)}: Robot, times: usize) -> Robot {
    Robot {
        position: (
            (p_x + (v_x + board.width) * (times as isize)) % board.width,
            (p_y + (v_y + board.height) * (times as isize)) % board.height,
        ),
        velocity: (v_x, v_y),
    }
}

fn is_tree_drawn(board: &Board, traces: &str) -> bool {
    let max_len: usize = 5;

    let (w, h) = (board.width as usize, board.height as usize);

    for r in 0..h {
        let line = &traces[(r * w)..((r+1) * w)];
        if let Some (from) = line.find(&format!("{}", "#".repeat(max_len))) {
            if is_tree_drawn_internal(traces, r, 1..=(max_len / 2), w, from, max_len) {
                return true;
            }
        }
    }

    false
}

fn is_tree_drawn_internal(traces: &str, row: usize, range: RangeInclusive<usize>, width: usize, offset: usize, len: usize) -> bool {
    if row < *range.end() { return false; }

    for n in range {
        if !(&traces[((row - n) * width)..((row - n + 1) * width)][offset+n..].starts_with(&format!("{}", "#".repeat(len - n * 2)))) {
            return false;
        }
    }

    true
}

fn dump_trace(board: &Board, traces: &str, time: usize) {
    let w = board.width as usize;

    eprintln!("\n#{} >>>", time);    

    for r in 0..w {
        eprintln!("{}", &traces[(r * w)..((r + 1) * w)]);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn is_tree_drawn_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board{ width: 8, height: 7};
        let pattern = format!("{}{}{}{}{}{}{}",
            "........",
            "........",
            "........",
            "....#...",
            "...###..",
            "..#####.",
            ".#######",
        );
        assert_eq!(true, is_tree_drawn(&board, &pattern));

        let pattern = format!("{}{}{}{}{}{}{}",
            "........",
            "........",
            "........",
            "....#...",
            "...###..",
            "....#...",
            "...###..",
        );
        assert_eq!(false, is_tree_drawn(&board, &pattern));

        Ok(())
    }
}