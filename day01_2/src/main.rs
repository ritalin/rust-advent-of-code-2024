use std::io::BufRead;
use std::path::Path;
use std::{fs::File, io::BufReader};
use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct GroupItem {
    left_value: i32,
    left_count: usize,
    right_count: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<i32, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);

    let mut buf = String::new();
    let mut entries = vec![];
    
    while reader.read_line(&mut buf)? > 0 {
        let mut iter = buf.split_ascii_whitespace();

        if let (Some(lhs), Some(rhs)) = (iter.next(), iter.next()) {
            entries.push((lhs.parse::<i32>()?, rhs.parse::<i32>()?));
        }
        buf.clear();
    }

    let (left_entries, right_entries): (Vec<i32>, Vec<i32>) = entries.into_iter().unzip();

    for (key, g) in &left_entries.iter().chunk_by(|v| *v) {
        eprintln!("left/ k: {:?}, g: {:?}", *key, g.map(|x| *x).collect::<Vec<i32>>());
    }

    let mut groups = <HashMap::<i32, GroupItem>>::new();

    for (key, g) in &left_entries.into_iter().chunk_by(|v| *v) {
        match groups.get_mut(&key) {
            Some(e) => {
                e.left_count += g.count();
            }
            None => {
                groups.insert(key, GroupItem { left_value: key, left_count: g.count(), right_count: 0 });
            }
        }
    }

    for v in right_entries {
        if let Some(e) = groups.get_mut(&v) {
            e.right_count += 1;
        }
    }

    for (_, e) in &groups {
        eprintln!("{:?}", e);
    }

    let total: i32 = groups.values()
        .map(|e| e.left_value * (e.left_count as i32) * (e.right_count as i32))
        .sum()
    ;

    Ok(total)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(31, actual);
        Ok(())
    }
}
