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
        .filter_map(std::convert::identity)
        .collect::<HashSet<Point>>()
    ;

    Ok(antinodes.len())
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

fn put_antinode(a1: Point, a2: Point, board: &Board) -> Vec<Option<Point>> {
    let distance = (a2.0 as isize - a1.0 as isize, a2.1 as isize - a1.1 as isize);

    vec![
        // a1 -> a2
        board.try_put((a2.0 as isize + distance.0, a2.1 as isize + distance.1)),
        // a2 -> a1
        board.try_put((a1.0 as isize - distance.0, a1.1 as isize - distance.1)),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(14, actual);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (board, anntenas) = read_file("./aoc_input_example.txt")?;

        //12 x 12
        assert_eq!(12, board.width);
        assert_eq!(12, board.height);

        let expect_anntena_0: Vec<Point> = vec![(8, 1), (5, 2), (7, 3), (4, 4)];
        let expect_anntena_a: Vec<Point> = vec![(6, 5), (8, 8), (9, 9)];

        assert_eq!(2, anntenas.len());
        assert_eq!(expect_anntena_0, anntenas[&'0']);
        assert_eq!(expect_anntena_a, anntenas[&'A']);

        Ok(())
    }

    #[test]
    fn make_anneta_pair_example() -> Result<(), Box<dyn std::error::Error>> {
        let anntenas: Vec<Point> = vec![(8, 1), (5, 2), (7, 3), (4, 4)];
        let pairs = make_anntena_pair(&anntenas);

        let expect_pairs = HashSet::<(Point, Point)>::from([
            ((8, 1), (5, 2)), ((8, 1), (7, 3)), ((8, 1), (4, 4)), 
            ((5, 2), (7, 3)), ((5, 2), (4, 4)),
            ((7, 3), (4, 4))
        ]);

        assert_eq!(expect_pairs, pairs);

        Ok(())
    }

    #[test]
    fn put_antinode_example() -> Result<(), Box<dyn std::error::Error>> {
        let pairs = vec![
            ((8, 1), (5, 2)), 
            ((8, 1), (7, 3)), 
            ((8, 1), (4, 4)), 
        ];
        let board = Board { width: 12, height: 12};
        
        let expect_antinodes: Vec<Point> = vec![
            (11, 0), (2, 3), 
            (6, 5), // (9, -1)
            (0, 7), // (12, -2), 
        ];

        assert_eq!(
            HashSet::<Option<Point>>::from([Some(expect_antinodes[0]), Some(expect_antinodes[1])]), 
            HashSet::<Option<Point>>::from_iter(put_antinode(pairs[0].0, pairs[0].1, &board))
        );
        assert_eq!(
            HashSet::<Option<Point>>::from([Some(expect_antinodes[2]), None]), 
            HashSet::<Option<Point>>::from_iter(put_antinode(pairs[1].0, pairs[1].1, &board))
        );
        assert_eq!(
            HashSet::<Option<Point>>::from([Some(expect_antinodes[3]), None]), 
            HashSet::<Option<Point>>::from_iter(put_antinode(pairs[2].0, pairs[2].1, &board))
        );

        Ok(())
    }
}