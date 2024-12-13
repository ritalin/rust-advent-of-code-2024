use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<usize, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let questions = read_file(path)?;

    let fewest = questions.into_iter()
        .map(|q| serve_cost(q))
        .filter(|costs| !costs.is_empty())
        .filter_map(|costs| {
            let accepted = costs.iter().all(|cost| match cost {
                Cost::A(x) | Cost::B(x) if *x > 100 => true,
                _ => false
            });
            if accepted { Some(costs) } else { None }
        })
        .flat_map(std::convert::identity)
        .map(|cost| cost.token())
        .sum::<usize>()
    ;
    Ok(fewest)
}

fn serve_cost(Question { a: (a_x, a_y), b: (b_x, b_y), prize: (p_x, p_y) }: Question) -> Vec<Cost> {
    let min_count_x = p_x / usize::max(a_x, b_x);
    let min_count_y = p_y / usize::max(a_y, b_y);

    let (count_a, count_b) =
        if min_count_x < min_count_y {
            if a_x < b_x {
                let Some((count_b, count_a)) = serve_cost_internal(min_count_x, b_x, b_y, (a_x, p_x), (a_y, p_y)) else { return vec![] };
                (count_a, count_b)
            } 
            else {
                let Some((count_a, count_b)) = serve_cost_internal(min_count_x, a_x, a_y, (b_x, p_x), (b_y, p_y)) else { return vec![] };
                (count_a, count_b)
            }
        }
        else {
            if a_y < b_y {
                let Some((count_b, count_a)) = serve_cost_internal(min_count_y, b_y, b_x, (a_y, p_y), (a_x, p_x)) else { return vec![] };
                (count_a, count_b)
            } 
            else {
                let Some((count_a, count_b)) = serve_cost_internal(min_count_y, a_y, a_x, (b_y, p_y), (b_x, p_x)) else { return vec![] }; 
                (count_a, count_b)
            }
        }
    ;

    vec![Cost::A(count_a), Cost::B(count_b)]
}

fn serve_cost_internal(count_limit: usize, value_1: usize, other_value_1: usize, (value_2, prize) : (usize, usize), (other_value_2, other_prize): (usize, usize)) -> Option<Point> {
    let mut left: usize = 1;
    let mut right: usize = count_limit;

    while left <= right {
        let mid = (left + right) / 2;

        let cost = other_prize - other_value_1 * mid;

        if cost % other_value_2 != 0 {
            match value_1 * mid + value_2 * (cost / other_value_2) <= prize {
                true => left = mid + 1,
                false => right = mid - 1,
            }
            continue;
        }

        let count = cost / other_value_2;

        match (value_2 * count + value_1 * mid == prize, other_value_2 * count + other_value_1 * mid == other_prize, ) {
            (true, true) => return Some((mid, count)),
            _ => {}
        }

        match value_1 * mid + value_2 * count < prize {
            true => left = mid + 1,
            false => right = mid - 1,
        }
    }

    None
}

type Point = (usize, usize);

#[derive(PartialEq, Debug)]
struct Question {
    a: Point,
    b: Point,
    prize: Point,
}

#[derive(PartialEq, Clone, Debug)]
enum Cost {
    A(usize),
    B(usize),
}

impl Cost {
    fn token(self) -> usize {
        match self {
            Cost::A(count) => count * 3,
            Cost::B(count) => count,
        }
    }
}

fn read_file<P>(path: P) -> Result<Vec<Question>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut questions = vec![];

    loop {
        match (read_line(&mut reader)?, read_line(&mut reader)?, read_line(&mut reader)?) {
            (Some(s_a), Some(s_b), Some(s_prize)) => {
                questions.push(Question {
                    a: parse_button(&s_a.trim_end())?,
                    b: parse_button(&s_b.trim_end())?,
                    prize: parse_prize(&s_prize.trim_end(), 10000000000000)?,
                });
            }
            _ => {
                break;
            }
        }
        let _ = read_line(&mut reader)?;
    }

    Ok(questions)
}

fn read_line(reader: &mut impl BufRead) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut buf = String::new();

    match reader.read_line(&mut buf) {
        Ok(len) if len > 0 => {
            Ok(Some(buf.clone()))
        }
        Ok(_) => {
            Ok(None)
        },
        Err(err) => {
            Err(String::into(err.to_string()))
        },
    }
}

#[derive(Debug)]
enum PatternError {
    InvalidPattern(String),
}

impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::InvalidPattern(msg) => write!(f, "PatternError: {}", msg),
        }
    }
}

impl std::error::Error for PatternError {}

fn parse_button(s: &str) -> Result<Point, Box<dyn std::error::Error>> {
    let Some(s) = s.split(':').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Need collon separator".to_string())));
    };
    let &[s_x, s_y] = s.split(',').collect::<Vec<_>>().as_slice() else {
        return Err(Box::new(PatternError::InvalidPattern("Need comma separator".to_string())));
    };

    let Some(x) = s_x.split('+').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Invalid point formmat (X)".to_string())));
    };
    let Some(y) = s_y.split('+').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Invalid point formmat (Y)".to_string())));
    };

    Ok((x.parse::<usize>()?, y.parse::<usize>()?))
}

fn parse_prize(s: &str, offset: usize) -> Result<Point, Box<dyn std::error::Error>> {
    let Some(s) = s.split(':').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Need collon separator".to_string())));
    };
    let &[s_x, s_y] = s.split(',').collect::<Vec<_>>().as_slice() else {
        return Err(Box::new(PatternError::InvalidPattern("Need comma separator".to_string())));
    };

    let Some(x) = s_x.split('=').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Invalid point formmat (X)".to_string())));
    };
    let Some(y) = s_y.split('=').last() else {
        return Err(Box::new(PatternError::InvalidPattern("Invalid point formmat (Y)".to_string())));
    };

    Ok((x.parse::<usize>()? + offset, y.parse::<usize>()? + offset))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let questions = read_file("./aoc_input_example.txt")?;

        let expect_questions = vec![
            Question { a: (94, 34), b: (22, 67), prize: (10000000008400, 10000000005400) },
            Question { a: (26, 66), b: (67, 21), prize: (10000000012748, 10000000012176) },
            Question { a: (17, 86), b: (84, 37), prize: (10000000007870, 10000000006450) },
            Question { a: (69, 23), b: (27, 71), prize: (10000000018641, 10000000010279) },
        ];

        assert_eq!(expect_questions, questions);
        Ok(())
    }

    #[test]
    fn solve_cost_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Vec::<Cost>::new(), serve_cost(Question { a: (94, 34), b: (22, 67), prize: (10000000008400, 10000000005400) }));
        assert_eq!(vec![Cost::A(118679050709), Cost::B(103199174542)], serve_cost(Question { a: (26, 66), b: (67, 21), prize: (10000000012748, 10000000012176) }));
        assert_eq!(Vec::<Cost>::new(), serve_cost(Question { a: (17, 86), b: (84, 37), prize: (10000000007870, 10000000006450) }));
        assert_eq!(vec![Cost::A(102851800151), Cost::B(107526881786)], serve_cost(Question { a: (69, 23), b: (27, 71), prize: (10000000018641, 10000000010279) }));
        Ok(())
    }
}
