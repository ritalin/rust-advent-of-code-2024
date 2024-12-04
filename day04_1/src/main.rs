use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let mut results = HashSet::<(i32, Direction)>::new();

    let (lines, widths): (Vec<String>, Vec<usize>) = reader.lines()
        .filter_map(|s| s.ok())
        .map(|s| s.trim_end().to_string())
        .map(|s| (s.to_string(), s.len()))
        .unzip()
    ;

    let buf = lines.into_iter().flat_map(|s| s.into_bytes()).collect::<Vec<_>>();
    let width = widths.into_iter().max().unwrap() as i32;
    let height = (buf.len() as i32) / width;

    eprintln!("width: {}", width);

    let mut i: i32 = 0;

    for c in &buf {
        if *c == b'X' { 
            solve_internal(&mut results, &buf, width, height, i);
        }
        i += 1;
    }

    Ok(results.len() / 2)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction {
    N, S, W, E,
    NW, NE, SW, SE,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [Direction::N, Direction::S, Direction::W, Direction::E, Direction::NW, Direction::NE, Direction::SW, Direction::SE].into_iter()
    }

    pub fn rev(self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::S => Direction::N,
            Direction::W => Direction::E, 
            Direction::E => Direction::W,
            Direction::NW => Direction::SE,
            Direction::NE => Direction::SW,
            Direction::SW => Direction::NE,
            Direction::SE => Direction::NW,        
        }
    }

    pub fn judge(self, buf: &[u8], width: i32, height: i32, p0: i32, needle: &str) -> Option<i32> {
        let mut p = (p0 % width, p0 / width);
        let mut index = p0 as usize;

        for c0 in needle.as_bytes().iter().skip(1) {
            match self {
                Direction::N => p.1 -= 1,
                Direction::S => p.1 += 1,
                Direction::W => p.0 -= 1, 
                Direction::E => p.0 += 1,
                Direction::NW => p = (p.0 - 1, p.1 - 1),
                Direction::NE => p = (p.0 + 1, p.1 - 1),
                Direction::SW => p = (p.0 - 1, p.1 + 1),
                Direction::SE => p = (p.0 + 1, p.1 + 1),
            }
            if (p.0 < 0) || (p.1 < 0) || (p.0 >= width) || (p.1 >= height) { return None; }
            
            index = (p.1 * width + p.0) as usize;

            if let Some(c) = buf.get(index) {
                if c != c0 { return None; }
            }
        }

        Some(index as i32)
    }
}

fn solve_internal(results: &mut HashSet::<(i32, Direction)>, buf: &[u8], width: i32, height: i32, p0: i32) {
    for d in Direction::iter() {
        if results.contains(&(p0, d)) {
            continue;
        }

        if let Some(end) = d.judge(buf, width, height, p0, "XMAS") {
            results.insert((p0, d));
            results.insert((end, d.rev()));

            eprintln!("({}, {}) - ({}, {}) [{:?}], {}", p0 % width, p0 / width, end % width, end / width, d, p0);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(18, actual);
        Ok(())
    }
}
