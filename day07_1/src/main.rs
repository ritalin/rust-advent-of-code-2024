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

    for op in &[Op::Add, Op::Mul] {
        let result = match op {
            Op::Add => acc + numbers[index],
            Op::Mul => acc * numbers[index],
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

enum Op { Add, Mul, }

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
        assert_eq!(3749, actual);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let equations = read_file("./aoc_input_example.txt")?;

        let expect_eqs = vec![
            Equation { ans: 190, numbers: vec![10, 19] },
            Equation { ans: 3267, numbers: vec![81, 40, 27] },
            Equation { ans: 83, numbers: vec![17, 5] },
            Equation { ans: 156, numbers: vec![15, 6] },
            Equation { ans: 7290, numbers: vec![6, 8, 6, 15] },
            Equation { ans: 161011, numbers: vec![16, 10, 13] },
            Equation { ans: 192, numbers: vec![17, 8, 14] },
            Equation { ans: 21037, numbers: vec![9, 7, 18, 13] },
            Equation { ans: 292, numbers: vec![11, 6, 16, 20] },
        ];
        
        assert_eq!(expect_eqs, equations);

        Ok(())
    }

    #[test]
    fn solve_equation() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Some(190), solve_internal(190, &[10, 19]));
        assert_eq!(None, solve_internal(83, &[17, 5]));
        assert_eq!(Some(292), solve_internal(292, &[11, 6, 16, 20]));
        assert_eq!(Some(3255967), solve_internal(3255967, &[4, 7, 6, 10, 17, 420, 5, 1, 6, 97]));

        Ok(())
    }
}

