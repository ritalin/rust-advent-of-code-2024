use std::{cmp::Ordering, collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u32, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (rules, pages): (HashSet<OrderingRule>, Vec<Page>) = read_file(path)?;

    let total = pages.into_iter()
        .filter_map(|p| {
            let mut sorted_numbers = p.clone();
            sorted_numbers.sort_by(|lhs, rhs| {
                if rules.contains(&(*lhs, *rhs)) {
                    return Ordering::Less;
                }
                if rules.contains(&(*rhs, *lhs)) {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });

            match p != sorted_numbers {
                true => Some(sorted_numbers),
                false => None,
            }
        })
        .map(|p| p[p.len() / 2])
        .sum::<u32>()
    ;

    Ok(total)
}

type OrderingRule = (u32, u32);
type Page  = Vec<u32>;

fn read_file<P>(path: P) -> Result<(HashSet<OrderingRule>, Vec<Page>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    let mut rules = HashSet::<OrderingRule>::new();

    while reader.read_line(&mut buf)? > 0 {
        if buf == "\n" { 
            buf.clear();
            break; 
        }
        
        let pair: Vec<String> = buf.trim_end().split('|').map(String::from).collect();
        let rule = pair.into_iter()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect::<Vec<_>>()
        ;

        rules.insert((rule[0], rule[1]));
        buf.clear();
    }

    let mut pages = vec![];

    while reader.read_line(&mut buf)? > 0 {
        let numbers: Vec<u32> = buf.trim_end().split(",")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect()
        ;
        pages.push(numbers);

        buf.clear();
    }

    Ok((rules, pages))
}
