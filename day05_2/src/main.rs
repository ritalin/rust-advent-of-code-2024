use std::{collections::{HashMap, HashSet, VecDeque}, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u32, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (rules, pages): (HashSet<OrderingRule>, Vec<Page>) = read_file(path)?;

    let page_pair = |p: &Vec<u32>| {
        p.windows(2).map(|x| (x[0], x[1])).collect::<Vec<_>>()
    };

    let total = pages.into_iter()
        .filter_map(|p| {
            match page_pair(&p).iter().all(|pair| rules.contains(&pair)) {
                true => {
                    None
                },
                false => {
                    Permutation::new(p, &rules).next()
                },
            }
        })
        .map(|p| {
            eprintln!("{:?}", p);
            p[p.len() / 2]
        })
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

struct Permutation {
    source: Vec<u32>,
    lookup: HashMap<u32, Vec<OrderingRule>>,
    cache: VecDeque<Vec<u32>>,
    index: usize,
}

impl Permutation {
    pub fn new(source: Vec<u32>, rules: &HashSet<OrderingRule>) -> Self {
        Self {
            source: source.clone(),
            lookup: Self::init_lookup(&source, rules),
            cache: VecDeque::<Vec<u32>>::new(),
            index: 0,
        }
    }

    fn init_lookup(source: &[u32], rules: &HashSet<OrderingRule>) -> HashMap<u32, Vec<OrderingRule>> {
        let mut lookup = source.iter().map(|x| (*x, vec![])).collect::<HashMap<u32, Vec<OrderingRule>>>();

        for i in 0..(source.len() - 1) {
            for j in (i+1)..source.len() {
                let item = (source[i], source[j]);
                if rules.contains(&item) {
                    if let Some (values) = lookup.get_mut(&item.0) {
                        values.push(item);
                    }
                    continue;
                }

                let item = (item.1, item.0);
                if rules.contains(&item) {
                    if let Some (values) = lookup.get_mut(&item.0) {
                        values.push(item);
                    }
                }
            }
        }

        lookup
    }

    pub fn next(&mut self) -> Option<Vec<u32>> {
        let mut acc = Vec::<u32>::with_capacity(self.source.len());

        while self.index < self.source.len() {
            let candidate = self.source[self.index];

            acc.push(candidate);
            let _ = Self::collect_next_internal(&mut self.cache, &self.lookup, candidate, self.source.len()-1, &mut vec![candidate]);
            acc.pop();
            
            if let Some(result) = self.cache.pop_front() { 
                return Some(result); 
            }
            self.index += 1;
        }

        None
    }

    fn collect_next_internal(cache: &mut VecDeque<Vec<u32>>, lookup: &HashMap<u32, Vec<OrderingRule>>, current: u32, left: usize, acc: &mut Vec<u32>) -> bool {
        if left == 0 {
            cache.push_back(acc.clone());
            return true;
        }

        for pair in lookup.get(&current).unwrap() {
            let candidate = pair.1;
            if acc.contains(&candidate) { continue; }

            acc.push(candidate);
            if Self::collect_next_internal(cache, lookup, candidate, left-1, acc) {
                return true;
            }
            let _ = acc.pop();
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(123, actual);
        Ok(())
    }

    #[test]
    fn test_permutation() -> Result<(), Box<dyn std::error::Error>> {
        let (rules, _): (HashSet<OrderingRule>, Vec<Page>) = read_file("./aoc_input_example.txt")?;

        let mut perm = Permutation::new(vec![97,13,75,29,47], &rules);

        assert_eq!(perm.next(), Some(vec![97, 75, 47, 29, 13]));
        assert_eq!(perm.next(), None);
        Ok(())
    }

    #[test]
    fn test_permutation_2() -> Result<(), Box<dyn std::error::Error>> {
        let (rules, _): (HashSet<OrderingRule>, Vec<Page>) = read_file("./aoc_input.txt")?;

        let mut perm = Permutation::new(vec![53,75,31,49,73,14,77,11,21,26,76,72,86,87,46,13,16,78,69,37,12,99,66], &rules);

        assert_eq!(perm.next(), Some(vec![76, 86, 14, 72, 26, 12, 99, 37, 53, 49, 13, 78, 77, 31, 69, 87, 66, 21, 11, 16, 75, 73, 46]));
        assert_eq!(perm.next(), None);
        Ok(())
    }
}

