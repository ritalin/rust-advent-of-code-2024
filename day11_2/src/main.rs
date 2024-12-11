use std::{collections::{HashMap, VecDeque}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let stones = read_file(path)?;

    Ok(blink(stones, 75))
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

fn blink(mut stones: VecDeque<String>, counter_limit: usize) -> usize {
    let mut total = 0;
    let mut cache = HashMap::<(String, usize), usize>::new();

    while let Some(stone) = stones.pop_front() {
        blink_internal(stone, counter_limit, 0, &mut total, &mut cache);
    }

    total
}

fn blink_internal(stone: String, counter_limit: usize, counter: usize, acc: &mut usize, cache: &mut HashMap<(String, usize), usize>) {
    if counter == counter_limit { 
        *acc += 1;
        return;  
    }

    if let Some(sub_total) = cache.get(&(stone.clone(), counter)) {
        *acc += sub_total;
        return;
    }

    let prev = *acc;

    if stone == "0" {
        blink_internal("1".to_string(), counter_limit, counter + 1, acc, cache);
    }
    else if stone.len() % 2 == 0 {
        let half_len = stone.len() / 2;

        blink_internal(stone[0..half_len].parse::<u64>().unwrap().to_string(), counter_limit, counter + 1, acc, cache);
        blink_internal(stone[half_len..].parse::<u64>().unwrap().to_string(), counter_limit, counter + 1, acc, cache);
    }
    else {
        blink_internal((stone.parse::<u64>().unwrap() * 2024).to_string(), counter_limit, counter + 1, acc, cache);
    }
    cache.insert((stone, counter), *acc - prev);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn blink_example() -> Result<(), Box<dyn std::error::Error>> {
        {
            let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);
            assert_eq!(3, blink(q, 1));
        }
        {
            let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);
            assert_eq!(4, blink(q, 2));
        }
        {
            let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);
            assert_eq!(9, blink(q, 4));
        }
        {
            let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);
            assert_eq!(22, blink(q, 6));
        }
        {
            let q = VecDeque::<String>::from(vec!["125".to_string(), "17".to_string()]);
            assert_eq!(55312, blink(q, 25));
        }

        Ok(())
    }
}