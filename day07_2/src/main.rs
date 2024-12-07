use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<i64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let equations = read_file(path)?;

    let total = equations.into_iter()
        .filter_map(|eq| solve_internal(eq.ans, &eq.numbers))
        .sum::<i64>()
    ;

    Ok(total)
}

fn solve_internal(ans: i64, numbers: &[i64]) -> Option<i64> {
    solve_rec(ans, numbers, numbers[0], 1)
}

fn solve_rec(ans: i64, numbers: &[i64], acc: i64, index: usize) -> Option<i64> {
    if index == numbers.len() {
        return Some(acc);
    }

    for op in &[Op::Add, Op::Mul, Op::Concat] {
        let result = match op {
            Op::Add => acc + numbers[index],
            Op::Mul => acc * numbers[index],
            Op::Concat => (acc.to_string() + &numbers[index].to_string()).parse::<i64>().unwrap(),
        };
        
        match solve_rec(ans, numbers, result, index + 1) {
            Some(res) if res == ans => {
                return Some(ans);
            }
            _ => {}
        }
    }

    None
}

#[derive(PartialEq, Eq, Debug)]
struct Equation {
    ans: i64,
    numbers: Vec<i64>,
}

enum Op { Add, Mul, Concat, }

fn read_file<P>(path: P) -> Result<Vec<Equation>, Box<dyn std::error::Error>> 
where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);

    let equations = reader.lines()
        .map(|s| s.unwrap().trim_end().split(':').map(String::from).collect::<Vec<_>>())
        .map(|x| {
            Equation {
                ans: x[0].parse::<i64>().unwrap(),
                numbers: x[1].split_ascii_whitespace().map(|xs| xs.parse::<i64>().unwrap()).collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>()
    ;

    Ok(equations)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = solve("./aoc_input_example.txt")?;
        assert_eq!(11387, actual);
        Ok(())
    }

    #[test]
    fn solve_equation() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Some(156), solve_internal(156, &[15, 6]));
        assert_eq!(Some(190), solve_internal(190, &[10, 19]));
        assert_eq!(None, solve_internal(83, &[17, 5]));
        assert_eq!(Some(292), solve_internal(292, &[11, 6, 16, 20]));
        assert_eq!(Some(3255967), solve_internal(3255967, &[4, 7, 6, 10, 17, 420, 5, 1, 6, 97]));

        Ok(())
    }
}

