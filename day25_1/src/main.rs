use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (locks, keys) = read_file(path)?;
    let total = match_key(&locks, &keys).into_iter()
        .count()
    ;

    Ok(total)
}

fn read_file<P>(path: P) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let width: usize = 5;
    let height: usize = 7;

    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::with_capacity(width + 1);
    let mut lines = String::with_capacity(width * height);

    let mut locks = vec![];
    let mut keys = vec![];

    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim_end();
        if (s.len() == 0) && (lines.len() > 0) {
            match lines.starts_with("#####") {
                true => locks.push(parse_lines(&lines, width)?),
                false => keys.push(parse_lines(&lines, width)?),
            }
            
            lines.clear();
        }

        lines.push_str(&s);
        buf.clear();
    }

    Ok((locks, keys))
}

fn parse_lines(lines: &str, width: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let lines: Vec<char> = lines.chars().collect();
    let mut key = vec![0u8; width];

    lines.chunks(width)
        .skip(1)
        .take(5)
        .for_each(|row| {
            row.into_iter().enumerate().for_each(|(i, ch)| match ch {
                '#' => key[i] += 1,
                _ => {},
            })
        })
    ;

    Ok(key)
}

fn match_key(locks: &[Vec<u8>], keys: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut matched = vec![];

    for i in 0..locks.len() {
        for j in 0..keys.len() {
            matched.push(match_key_internal(&locks[i], &keys[j], i, j));
        }
    }

    matched.into_iter().filter_map(std::convert::identity).collect()
}

fn match_key_internal(lock: &[u8], key: &[u8], lock_index: usize, key_index: usize) -> Option<(usize, usize)> {
   let matched =  lock.iter().zip(key)
        .map(|(lhs, rhs)| lhs + rhs)
        .all(|m| m < 6)
    ;

    matched.then(|| (lock_index, key_index))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[ignore = "reason"]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(4, solve("./aoc_input_example_1.txt")?);
        assert_eq!(2024, solve("./aoc_input_example_2.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (locks, keys) = read_file("./aoc_input_example.txt")?;
        let expect_locks = vec![
            vec![0, 5, 3, 4, 3],
            vec![1, 2, 0, 5, 3],
        ];
        let expect_keys = vec![
            vec![5, 0, 2, 1, 3],
            vec![4, 3, 4, 0, 2],
            vec![3, 0, 2, 0, 1],
        ];
        assert_eq!(expect_locks, locks);
        assert_eq!(expect_keys, keys);
        Ok(())
    }

    #[test]
    fn match_key_example() -> Result<(), Box<dyn std::error::Error>> {
        let locks = vec![
            vec![0,5,3,4,3],
            vec![1,2,0,5,3],
        ];
        let keys = vec![
            vec![5,0,2,1,3],
            vec![4,3,4,0,2],
            vec![3,0,2,0,1],
        ];

        assert_eq!(vec![(0, 2), (1, 1), (1, 2)], match_key(&locks, &keys));
        Ok(())
    }
}