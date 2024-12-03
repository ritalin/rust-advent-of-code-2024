use std::{fs::File, io::{BufRead, BufReader}, mem::discriminant, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {    
    println!("Safes: {}", solve("./aoc_input.txt")?);
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug)]
enum Comparison {
    Inv,
    Inc(i32),
    Dec(i32),
}

fn solve<P>(path: P) -> Result<i32, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);

    let mut buf = String::new();
    let mut safe_count = 0;
    
    while reader.read_line(&mut buf)? > 0 {
        let adjacents = buf.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>()
            .windows(2)
            .map(|w| (w[0], w[1]))
            .collect::<Vec<_>>()
        ;

        let report = adjacents.iter()
            .map(|(lhs, rhs)| match lhs - rhs {
                v if (1..=3).contains(&v) => Comparison::Dec(v),
                v if (-3..=-1).contains(&v) => Comparison::Inc(v.abs()),
                _ => Comparison::Inv,
            })
            .collect::<Vec<_>>()
        ;
        
        if let Some(rep0) = report.first() {
            let desc = discriminant(rep0);
            match report.iter().skip(1).all(|rep| discriminant(rep) == desc) {
                true if desc != discriminant(&Comparison::Inv) => {
                    eprintln!("Safe: {:?} /judge: {:?}", adjacents, report);
                    safe_count += 1;
                }
                _ => {
                    eprintln!("Unsafe: {:?} /judge: {:?}", adjacents, report);
                }
            }
        }

        buf.clear();
    }

    Ok(safe_count)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(2, actual);
        Ok(())
    }
}
