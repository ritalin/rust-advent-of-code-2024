use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

struct Point {
    x: i32,
    y: i32,
    width: i32,
}

impl Point {
    pub fn from(index: i32, width: i32) -> Self {
        Self {
            x: index % width,
            y: index / width,
            width,
        }
    }

    pub fn index(&self) -> i32 {
        self.x + self.y * self.width
    }

    pub fn offset(&self, diff_x: i32, diff_y: i32) -> Self {
        Self {
            x: self.x + diff_x,
            y: self.y + diff_y,
            width: self.width,
        }
    }

    pub fn clone(&self) -> Self {
        self.offset(0, 0)
    }

    pub fn out_of_bound(&self, height: i32) -> bool {
        self.x < 0 || self.x >= self.width || self.y < 0 || self.y >= height
    }
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);

    let (lines, widths): (Vec<String>, Vec<usize>) = reader.lines()
        .filter_map(|s| s.ok())
        .map(|s| s.trim_end().to_string())
        .map(|s| (s.to_string(), s.len()))
        .unzip()
    ;

    let buf = lines.into_iter().flat_map(|s| s.into_bytes()).collect::<Vec<_>>();

    let width = widths.into_iter().max().unwrap() as i32;
    let height = (buf.len() as i32) / width;
    let mut results = HashSet::<i32>::new();

    let mut i: i32 = 0;

    for c in &buf {
        if *c == b'A' { 
            solve_internal(&mut results, &buf, height, &Point::from(i, width));
        }
        i += 1;
    }

    Ok(results.len())
}

fn solve_internal(results: &mut HashSet::<i32>, buf: &[u8], height: i32, p0: &Point) {
    let j1 = judge(buf, height, p0.offset(-1, -1), p0.clone(), p0.offset(1, 1));
    let j2 = judge(buf, height, p0.offset(1, -1), p0.clone(), p0.offset(-1, 1));

    if j1 && j2 {
        results.insert(p0.index());
    }
}

fn judge(buf: &[u8], height: i32, p0: Point, p1: Point, p2: Point) -> bool {
    if p0.out_of_bound(height) || p2.out_of_bound(height) { return false; }

    let s = [buf[p0.index() as usize], buf[p1.index() as usize], buf[p2.index() as usize]];
    
    s.iter().eq("MAS".as_bytes()) || s.iter().rev().eq("MAS".as_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(9, actual);
        Ok(())
    }
}
