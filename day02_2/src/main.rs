use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Comparison {
    Inv,
    Inc,
    Dec,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut reader = BufReader::new(File::open("./aoc_input_example.txt")?);
    let mut reader = BufReader::new(File::open("./aoc_input.txt")?);

    let mut buf = String::new();
    let mut safe_count = 0;
    
    while reader.read_line(&mut buf)? > 0 {
        let levels = buf.split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>()
        ;

        if judge(&levels) {
            safe_count += 1;
        }

        buf.clear();
    }
    
    println!("Safes: {}", safe_count);
    Ok(())
}

fn judge(levels: &Vec<i32>) -> bool {
    if judge_internal(levels) {
        eprintln!("Safe: {:?}", &levels);
        return true;
    }

    for i in 0..levels.len() {
        let new_levels =
            levels.iter().enumerate()
            .filter_map(|(j, lv)| match i != j {
                true => Some(*lv),
                false => None,
            })
            .collect::<Vec<_>>()
        ;

        if judge_internal(&new_levels) {
            eprintln!("Safe: {:?}", &new_levels);
            return true;
        }
    }

    eprintln!("Unsafe: {:?}", &levels);
    return false;
}

fn judge_internal(levels: &Vec<i32>) -> bool {
    let report = levels
        .windows(2)
        .map(|w| (w[0], w[1]))
        .map(|(lhs, rhs)| match lhs - rhs {
            v if (1..=3).contains(&v) => Comparison::Dec,
            v if (-3..=-1).contains(&v) => Comparison::Inc,
            _ => Comparison::Inv,
        })
        .collect::<Vec<_>>()
    ;

    if let Some(rep0) = report.first() {
        if *rep0 != Comparison::Inv { 
            return report.iter().skip(1).all(|rep| rep == rep0);
        }
    }

    false
}