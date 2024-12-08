use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (board, anntenas) = read_file(path)?;

    let antinodes = anntenas.values()
        .flat_map(|xs| {
            make_anntena_pair(xs).into_iter()
            .flat_map(|(a1, a2)| put_antinode(a1, a2, &board))
        })
        .collect::<HashSet<Point>>()
    ;
    
    Ok(antinodes.len())
}

#[allow(unused)]
fn result_board(board: &Board, antinodes: &[&Point]) {
    let mut result: Vec<u8> = std::iter::repeat(b'.').take(board.width * board.height).collect();

    for an in antinodes {
        result[an.0 + an.1 * board.width] = b'#';
    }

    for y in 0..board.height {
        eprintln!("{:?}", String::from_utf8(result[(y * board.width)..((y+1) * board.width)].to_vec()));
    }

    for an in antinodes {
        eprintln!("{:?}", an);
    }
}

type Point = (usize, usize);
struct Board { width: usize, height: usize }

impl Board {
    fn try_put(&self, antinode: (isize, isize)) -> Option<Point> {
        // left
        if antinode.0 < 0 { return None; }
        // top
        if antinode.1 < 0 { return None; }
        // right
        if antinode.0 >= self.width as isize { return None; }
        // bottom
        if antinode.1 >= self.height as isize { return None; }

        Some((antinode.0 as usize, antinode.1 as usize))
    }
}

fn read_file<P>(path: P) -> Result<(Board, HashMap<char, Vec<Point>>), Box<dyn std::error::Error>> 
where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let mut anntenas = HashMap::<char, Vec<Point>>::new();
    let mut board = Board{width: 0, height: 0};

    for (y, row) in reader.lines().enumerate() {
        let row = row?;
        board.width = row.len();
        board.height += 1;

        for (x, cell) in row.chars().into_iter().enumerate() {
            if cell != '.' {
                anntenas.entry(cell).or_insert_with(|| vec![]).push((x, y));
            }
        } 
    }

    Ok((board, anntenas))
}

fn make_anntena_pair(anntenas: &[Point]) -> HashSet<(Point, Point)> {
    let mut pairs = HashSet::<(Point, Point)>::new();

    for a1 in anntenas {
        for a2 in anntenas {
            if (a1 != a2) && (!pairs.contains(&(*a2, *a1))) {
                pairs.insert((*a1, *a2));
            }
        }
    }
    
    pairs
}

fn put_antinode(a1: Point, a2: Point, board: &Board) -> Vec<Point> {
    let distance = (a2.0 as isize - a1.0 as isize, a2.1 as isize - a1.1 as isize);

    let antinodes = [
        put_antinode_internal(a2, board, |an: Point| (an.0 as isize + distance.0, an.1 as isize + distance.1)),
        put_antinode_internal(a2, board, |an: Point| (an.0 as isize - distance.0, an.1 as isize - distance.1)),
        put_antinode_internal(a1, board, |an: Point| (an.0 as isize + distance.0, an.1 as isize + distance.1)),
        put_antinode_internal(a1, board, |an: Point| (an.0 as isize - distance.0, an.1 as isize - distance.1)),
    ];

    antinodes.into_iter().flat_map(std::convert::identity).collect()
}
fn put_antinode_internal<F>(anntena: Point, board: &Board, next_fn: F) -> Vec<Point>
    where F: Fn(Point) -> (isize, isize)
{
    let mut last_an = next_fn(anntena);

    std::iter::from_fn(move || {
        match board.try_put(last_an) {
            Some(an) => {
                last_an = next_fn(an);
                Some(an)
            }
            None => None,
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(34, actual);
        Ok(())
    }

    #[test]
    fn put_antinode_example() -> Result<(), Box<dyn std::error::Error>> {
        let pairs = vec![
            ((8, 1), (5, 2)), // (-3, 1)
            ((8, 1), (7, 3)), // (-1, 2)
            ((8, 1), (4, 4)), // (-4, 3)
        ];
        let board = Board { width: 12, height: 12};
        
        let expect_antinodes: Vec<Vec<Point>> = vec![
            vec![(11, 0), (5, 2), (2, 3), (8, 1)], 
            vec![(7, 3), (6, 5), (5, 7), (4, 9), (3, 11), (6, 5), (5, 7), (4, 9), (3, 11), (8, 1)], 
            vec![(4, 4), (0, 7), (0, 7), (8, 1)], 
        ];

        assert_eq!(
            HashSet::<Point>::from_iter(expect_antinodes[0].clone()), 
            HashSet::<Point>::from_iter(put_antinode(pairs[0].0, pairs[0].1, &board))
        );
        assert_eq!(
            HashSet::<Point>::from_iter(expect_antinodes[1].clone()), 
            HashSet::<Point>::from_iter(put_antinode(pairs[1].0, pairs[1].1, &board))
        );
        assert_eq!(
            HashSet::<Point>::from_iter(expect_antinodes[2].clone()), 
            HashSet::<Point>::from_iter(put_antinode(pairs[2].0, pairs[2].1, &board))
        );

        Ok(())
    }
}