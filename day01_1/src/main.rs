use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<i32, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);

    let mut buf = String::new();
    let mut left_entries = vec![];
    let mut right_entries = vec![];

    while reader.read_line(&mut buf)? > 0 {
        let mut iter = buf.split_ascii_whitespace();

        if let (Some(lhs), Some(rhs)) = (iter.next(), iter.next()) {
            left_entries.push(lhs.parse::<i32>()?);
            right_entries.push(rhs.parse::<i32>()?);
        }
        buf.clear();
    }

    left_entries.sort();
    right_entries.sort();

    let total = left_entries.into_iter()
        .zip(right_entries)
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum::<i32>()
    ;

    Ok(total)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(11, actual);
        Ok(())
    }
}