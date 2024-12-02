use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = BufReader::new(File::open("./aoc_input_example.txt")?);
    let mut reader = BufReader::new(File::open("./aoc_input.txt")?);

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


    println!("total: {}", total);

    Ok(())
}
