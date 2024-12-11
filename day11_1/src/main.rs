use std::{collections::VecDeque, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let mut stones = read_file(path)?;
    
    for _ in 0..25 {
        stones = blink(stones);
    }

    Ok(stones.len())
}

fn read_file<P>(path: P) -> Result<VecDeque<String>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    reader.read_line(&mut buf)?;
    let q = buf.split_ascii_whitespace()
        .map(|n| n.trim_end().to_string())
        .collect::<VecDeque<String>>()
    ;

    Ok(q)
}

fn blink(mut queue: VecDeque<String>) -> VecDeque<String> {
    let mut next_queue = VecDeque::<String>::new();

    while let Some(stone) = queue.pop_front() {
        if stone == "0" {
            next_queue.push_back("1".to_string())
        }
        else if stone.len() % 2 == 0 {
            let half_len = stone.len() / 2;

            next_queue.push_back(stone[0..half_len].parse::<u64>().unwrap().to_string());
            next_queue.push_back(stone[half_len..].parse::<u64>().unwrap().to_string());
        }
        else {
            next_queue.push_back((stone.parse::<u64>().unwrap() * 2024).to_string());
        }
    }

    next_queue
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(55312, actual);
        Ok(())
    }

    #[test]
    fn reaf_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let q = read_file("./aoc_input_example.txt")?;

        let expect = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);

        assert_eq!(expect, q);
        Ok(())
    }

    #[test]
    fn blink_example() -> Result<(), Box<dyn std::error::Error>> {
        let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);

        let q = {
            let next_queue = blink(q);
            let expect = vec!["253000", "1", "7"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };
        let q = {
            let next_queue = blink(q);
            let expect = vec!["253", "0", "2024", "14168"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };
        let q = {
            let next_queue = blink(q);
            let expect = vec!["512072", "1", "20", "24", "28676032"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };
        let q = {
            let next_queue = blink(q);
            let expect = vec!["512", "72", "2024", "2", "0", "2", "4", "2867", "6032"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };
        let q = {
            let next_queue = blink(q);
            let expect = vec!["1036288", "7", "2", "20", "24", "4048", "1", "4048", "8096", "28", "67", "60", "32"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };
        let _ = {
            let next_queue = blink(q);
            let expect = vec!["2097446912", "14168", "4048", "2", "0", "2", "4", "40", "48", "2024", "40", "48", "80", "96", "2", "8", "6", "7", "6", "0", "3", "2"];
            assert_eq!(expect.into_iter().map(|s| s.to_string()).collect::<VecDeque<_>>(), next_queue);
            next_queue
        };

        Ok(())
    }
}
