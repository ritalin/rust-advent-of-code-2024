use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<i64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let innitial_values = read_file(path)?;

    let mut map = HashMap::<Sequence, i64>::new();

    for value in innitial_values {
        let prices =  generate_price(value, 2000);
        let ballances = generate_ballance(&prices);
        let buyer_map = generate_buyer_map(&prices, &ballances);

        merge_buyer_map(&mut map, &buyer_map);
    }

    Ok(map.values().map(|p| *p).max().unwrap_or(0))
}

fn read_file<P>(path: P) -> Result<Vec<i64>, Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let reader = BufReader::new(File::open(path)?);
    let values = reader.lines()
        .map(|s| s.unwrap().parse::<i64>())
        .collect::<Result<_, _>>()?
    ;

    Ok(values)
}

type Sequence = Vec<i64>;

fn next_secret(value: i64) -> i64 {
    let modulo = 16777216;

    let value = ((value * 64) ^ value) % modulo;
    let value = ((value / 32) ^ value) % modulo;
    let value = ((value * 2048) ^ value) % modulo;

    value
}

fn generate_price(mut value: i64, rep: usize) -> Vec<i64> {
    let mut prices = vec![value % 10];

    for _ in 0..rep {
        value = next_secret(value);
        prices.push(value % 10);
    }

    prices
}

fn generate_ballance(prices: &[i64]) -> Vec<i64> {
    prices.windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect()
}

fn generate_buyer_map(prices: &[i64], ballances: &[i64]) -> HashMap<Sequence, i64> {
    let mut map = HashMap::<Sequence, i64>::new();

    prices[1..].windows(4)
        .zip(ballances.windows(4))
        .for_each(|(p0, seq)| {
            map.entry(seq.to_vec())
                .or_insert(p0[3])
            ;
        })
    ;

    map
}

fn merge_buyer_map(map: &mut HashMap<Sequence, i64>, buyer_map: &HashMap<Sequence, i64>) {
    buyer_map.iter()
        .for_each(|(seq, p0)| {
            map.entry(seq.to_vec())
                .and_modify(|p| *p += *p0)
                .or_insert(*p0)
            ;
        })
    ;
}

#[allow(unused)]
fn dump_buyer_map(map: &HashMap<Sequence, i64>) {
    map.into_iter()
        .for_each(|(seq, p)| {
            println!("{:?} {:?}", p,  &seq);
        })
    ;
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(23, solve("./aoc_input_example.txt")?);
        Ok(())
    }
}
