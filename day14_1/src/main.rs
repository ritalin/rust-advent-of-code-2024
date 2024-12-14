use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = Board {width: 101, height: 103};
    println!("total: {}", solve("./aoc_input.txt", board, 100)?);
    Ok(())
}

fn solve<P>(path: P, board: Board, times: usize) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let robots = read_file(path)?;
    let positions = move_robot(&board, robots, times);
    
    Ok(count_robots(&board, positions).into_iter().product())
}

#[derive(PartialEq, Eq, Hash)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

struct Board {
    width: isize,
    height: isize,
}

impl Board {
    fn quadrant(&self, (x, y): (isize, isize)) -> Option<Quadrant> {
        let h_margin = ((self.width - 1) / 2)..((self.width + 1) / 2);
        let v_margin = ((self.height - 1) / 2)..((self.height + 1) / 2);

        if (0..h_margin.start).contains(&x) && (0..v_margin.start).contains(&y) {
            return Some(Quadrant::First);
        }
        if (h_margin.end..self.width).contains(&x) && (0..v_margin.start).contains(&y) {
            return Some(Quadrant::Second);
        }
        if (0..h_margin.start).contains(&x) && (v_margin.end..self.height).contains(&y) {
            return Some(Quadrant::Third);
        }
        if (h_margin.end..self.width).contains(&x) && (v_margin.end..self.height).contains(&y) {
            return Some(Quadrant::Fourth);
        }

        None
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

fn move_robot(board: &Board, robots: Vec<Robot>, times: usize) -> Vec<(isize, isize)> {
    robots.into_iter()
        .map(|robot| move_robot_internal(board, robot, times))
        .map(|robot| robot.position)
        .collect()
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

fn count_robots(board: &Board, positions: Vec<(isize, isize)>) -> Vec<usize> {
    let counts = HashMap::<Quadrant, usize>::from_iter(
        vec![(Quadrant::First, 0), (Quadrant::Second, 0), (Quadrant::Third, 0), (Quadrant::Fourth, 0)].into_iter()
    );

    let counts = positions.into_iter()
        .fold(counts, |mut map, p| {
            match board.quadrant(p) {
                Some(q) => {
                    if let Some(v) = map.get_mut(&q) {
                       *v += 1;
                    }
                }
                None => {},
            }
            map
        })
    ;
    
    counts.values().map(|c| *c).collect()
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board {width: 11, height: 7};
        assert_eq!(12, solve("./aoc_input_example.txt", board, 100)?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let robots = read_file("./aoc_input_example.txt")?;
        let expect_robots = vec![
            Robot{ position: (0,4), velocity: (3,-3) },
            Robot{ position: (6,3), velocity: (-1,-3) },
            Robot{ position: (10,3), velocity: (-1,2) },
            Robot{ position: (2,0), velocity: (2,-1) },
            Robot{ position: (0,0), velocity: (1,3) },
            Robot{ position: (3,0), velocity: (-2,-2) },
            Robot{ position: (7,6), velocity: (-1,-3) },
            Robot{ position: (3,0), velocity: (-1,-2) },
            Robot{ position: (9,3), velocity: (2,3) },
            Robot{ position: (7,3), velocity: (-1,2) },
            Robot{ position: (2,4), velocity: (2,-3) },
            Robot{ position: (9,5), velocity: (-3,-3) },
        ];
        assert_eq!(expect_robots, robots);
        Ok(())
    }

    #[test]
    fn move_robot_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board { width: 11, height: 7 };
        let mut robot = Robot { position: (2, 4), velocity: (2, -3) };

        robot = move_robot_internal(&board, robot, 1);
        assert_eq!(Robot { position: (4, 1), velocity: (2, -3) }, robot);
        robot = move_robot_internal(&board, robot, 1);
        assert_eq!(Robot { position: (6, 5), velocity: (2, -3) }, robot);
        robot = move_robot_internal(&board, robot, 1);
        assert_eq!(Robot { position: (8, 2), velocity: (2, -3) }, robot);
        robot = move_robot_internal(&board, robot, 1);
        assert_eq!(Robot { position: (10, 6), velocity: (2, -3) }, robot);
        robot = move_robot_internal(&board, robot, 1);
        assert_eq!(Robot { position: (1, 3), velocity: (2, -3) }, robot);
        Ok(())
    }

    #[test]
    fn move_robot_100_times_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board { width: 11, height: 7 };
        let robots = vec![
            Robot{ position: (0,4), velocity: (3,-3) },
            Robot{ position: (6,3), velocity: (-1,-3) },
            Robot{ position: (10,3), velocity: (-1,2) },
            Robot{ position: (2,0), velocity: (2,-1) },
            Robot{ position: (0,0), velocity: (1,3) },
            Robot{ position: (3,0), velocity: (-2,-2) },
            Robot{ position: (7,6), velocity: (-1,-3) },
            Robot{ position: (3,0), velocity: (-1,-2) },
            Robot{ position: (9,3), velocity: (2,3) },
            Robot{ position: (7,3), velocity: (-1,2) },
            Robot{ position: (2,4), velocity: (2,-3) },
            Robot{ position: (9,5), velocity: (-3,-3) },
        ];
        let expect_robot_positions: Vec<(isize, isize)> = vec![
            (6, 0), (6, 0), (9, 0),
            (0, 2),
            (1, 3), (2, 3),
            (5, 4), 
            (3, 5), (4, 5), (4, 5), 
            (1, 6), (6, 6), 
        ];

        assert_eq_unordered!(expect_robot_positions, move_robot(&board, robots, 100));
        Ok(())
    }

    #[test]
    fn count_robot_example() -> Result<(), Box<dyn std::error::Error>> {
        let board = Board { width: 11, height: 7 };
        let robot_positions: Vec<(isize, isize)> = vec![
            (6, 0), (6, 0), (9, 0),
            (0, 2),
            (1, 3), (2, 3),
            (5, 4), 
            (3, 5), (4, 5), (4, 5), 
            (1, 6), (6, 6), 
        ];
        let expect_counts = vec![1, 4, 3, 1];

        assert_eq_unordered!(expect_counts, count_robots(&board, robot_positions));
        Ok(())
    }
}

