use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u32, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (rules, pages): (HashSet<OrderingRule>, Vec<Page>) = read_file(path)?;
    let total = pages.into_iter()
        .filter(|p| {
            p.pages.iter().all(|pair| rules.contains(&pair))
        })
        .map(|p| p.middle)
        .sum::<u32>()
    ;

    Ok(total)
}

type OrderingRule = (u32, u32);

struct Page {
    pages: Vec<OrderingRule>,
    middle: u32,
}

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

        pages.push(Page {
            pages: numbers.windows(2).map(|x| (x[0], x[1])).collect::<Vec<OrderingRule>>(),
            middle: numbers[numbers.len() / 2],
        });

        buf.clear();
    }

    Ok((rules, pages))
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(143, actual);
        Ok(())
    }

    #[test]
    fn read_example_file() -> Result<(), Box<dyn std::error::Error>> {
        let (rules, pages): (HashSet<OrderingRule>, Vec<Page>) = crate::read_file("./aoc_input_example.txt")?;

        let expect_rules = vec![
            (47, 53), (97, 13), (97, 61), 
            (97, 47), (75, 29), (61, 13), 
            (75, 53), (29, 13), (97, 29), 
            (53, 29), (61, 53), (97, 53), 
            (61, 29), (47, 13), (75, 47), 
            (97, 75), (47, 61), (75, 61), 
            (47, 29), (75, 13), (53, 13), 
        ];
        let expect_pages = vec![
            vec![(75,47), (47,61), (61,53), (53,29)],
            vec![(97,61), (61,53), (53,29), (29,13)],
            vec![(75,29), (29,13)],
            vec![(75,97), (97,47), (47,61), (61,53)],
            vec![(61,13), (13,29)],
            vec![(97,13), (13,75), (75,29), (29,47)],
        ];
        let expect_middles = vec![61, 53, 29, 47, 13, 75];

        assert_eq!(expect_rules.len(), rules.len());
        assert_eq!(expect_rules.into_iter().collect::<HashSet<OrderingRule>>(), rules);

        assert_eq!(expect_pages.len(), pages.len());
        assert_eq!(expect_pages, pages.iter().map(|p| p.pages.clone()).collect::<Vec<_>>());
        assert_eq!(expect_middles, pages.iter().map(|p| p.middle).collect::<Vec<_>>());
        Ok(())
    }
}

