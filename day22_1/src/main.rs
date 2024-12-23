use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let innitial_values = read_file(path)?;

    let total = innitial_values.into_iter()
        .map(|v| generate_secret(v, 2000))
        .sum::<u64>()
    ;

    Ok(total)
}

fn read_file<P>(path: P) -> Result<Vec<u64>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let values = reader.lines()
        .map(|s| s.unwrap().parse::<u64>())
        .collect::<Result<_, _>>()?
    ;

    Ok(values)
}

fn generate_secret(mut value: u64, repeat: usize) -> u64 {
    for _ in 0..repeat {
        value = next_secret(value);
    }

    value
}

fn next_secret(value: u64) -> u64 {
    let modulo = 16777216;

    let value = ((value * 64) ^ value) % modulo;
    let value = ((value / 32) ^ value) % modulo;
    let value = ((value * 2048) ^ value) % modulo;

    value
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(37327623, solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let values = read_file("./aoc_input_example.txt")?;
        let expect_values = vec![
            1, 10, 100, 2024
        ];

        assert_eq!(expect_values, values);
        Ok(())
    }

    #[test]
    fn next_secret_example() -> Result<(), Box<dyn std::error::Error>> {
        let secret = next_secret(123);
        assert_eq!(15887950, secret);

        let secret = next_secret(secret);
        assert_eq!(16495136, secret);

        let secret = next_secret(secret);
        assert_eq!(527345, secret);
        let secret = next_secret(secret);
        assert_eq!(704524, secret);
        let secret = next_secret(secret);
        assert_eq!(1553684, secret);
        let secret = next_secret(secret);
        assert_eq!(12683156, secret);
        let secret = next_secret(secret);
        assert_eq!(11100544, secret);
        let secret = next_secret(secret);
        assert_eq!(12249484, secret);
        let secret = next_secret(secret);
        assert_eq!(7753432, secret);
        let secret = next_secret(secret);
        assert_eq!(5908254, secret);

        Ok(())
    }

    #[test]
    fn generate_secret_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(8685429, generate_secret(1, 2000));
        assert_eq!(4700978, generate_secret(10, 2000));
        assert_eq!(15273692, generate_secret(100, 2000));
        assert_eq!(8667524, generate_secret(2024, 2000));

        Ok(())
    }
}
