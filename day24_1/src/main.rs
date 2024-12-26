use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<u64, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (circuit, mut outputs) = read_file(path)?;
    run(&circuit, &mut outputs);

    let output = outputs.into_iter()
        .fold(0u64, |acc, output| {
            match output.value {
                Some(value) => {
                    let value = (value as u64) << output.order;
                    acc + value
                }
                None => acc
            }
        })
    ;

    Ok(output)
}

#[derive(Debug)]
enum PatternError {
    UnexpectedToken(String),
}
impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
        }
    }
}
impl std::error::Error for PatternError {}

#[derive(PartialEq, Clone, Debug)]
enum Op {
    And, Or, XOr
}
impl Op {
    fn try_from(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        match token {
            "AND" => Ok(Op::And),
            "OR" => Ok(Op::Or),
            "XOR" => Ok(Op::XOr),
            _ => Err(Box::new(PatternError::UnexpectedToken(token.into()))),
        }
    }

    fn evaluate(&self, lhs: u8, rhs: u8) -> Option<u8> {
        match self {
            Op::And => Some(lhs & rhs),
            Op::Or => Some(lhs | rhs),
            Op::XOr => Some(lhs ^ rhs),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Gate {
    Logic(String, Op, Vec<String>),
    Input(String, u8),
}

#[derive(PartialEq, Debug)]
struct Output {
    name: String,
    order: u8,
    value: Option<u8>,
}

fn read_file<P>(path: P) -> Result<(Vec<Gate>, Vec<Output>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut circuit = vec![];
    let mut outputs = vec![];

    read_input(&mut reader, &mut circuit)?;
    read_logic(&mut reader, &mut circuit, &mut outputs)?;

    Ok((circuit, outputs))
}

fn read_input(reader: &mut impl BufRead, circuit: &mut Vec<Gate>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim();
        if s.len() == 0 { break; }

        let pair = s.split(":").collect::<Vec<_>>();
        circuit.push(Gate::Input(pair[0].to_string(), pair[1].trim().parse::<u8>()?));

        buf.clear();
    }

    Ok(())
}

fn read_logic(reader: &mut impl BufRead, circuit: &mut Vec<Gate>, outputs: &mut Vec<Output>) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {
        let s = buf.trim();
        if s.len() == 0 { break; }
        
        let tokens = s.split_ascii_whitespace().collect::<Vec<_>>();
        circuit.push(Gate::Logic(tokens[4].into(), Op::try_from(tokens[1])?, vec![tokens[0].into(), tokens[2].into()]));

        if tokens[4].starts_with('z') {
            outputs.push(Output{ name: tokens[4].into(), order: tokens[4][1..].parse::<u8>()?, value: None })
        }

        buf.clear();
    }
    Ok(())
}

fn run(circuit: &[Gate], outputs: &mut [Output]) {
    let mut gates = circuit.iter()
        .map(|gate| match gate {
            Gate::Logic(name, _, _) => (name.clone(), gate),
            Gate::Input(name, _) => (name.clone(), gate),
        })
        .collect::<HashMap<_,_>>()
    ;

    let mut cache = HashMap::new();

    for output in outputs {
        output.value = run_internal(&mut gates, &output.name, &mut cache);
    }
}

fn run_internal(gates: &mut HashMap<String, &Gate>, name: &str, cache: &mut HashMap<String, u8>) -> Option<u8> {
    let Some(gate) = gates.get_mut(name) else {
        return None;
    };

    match gate {
        Gate::Logic(name, op, operands) => {
            if let Some(value) = cache.get(name) {
                return Some(*value);
            }

            let Some(lhs) = run_internal(gates, &operands[0], cache) else { return None; };
            let Some(rhs) = run_internal(gates, &operands[1], cache) else { return None; };
            
            match op.evaluate(lhs, rhs) {
                Some(value) => {
                    cache.insert(name.clone(), value);
                    Some(value)
                }
                None => None,
            }
        }
        Gate::Input(_, value) => Some(*value),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(4, solve("./aoc_input_example_1.txt")?);
        assert_eq!(2024, solve("./aoc_input_example_2.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (circuit, outputs) = read_file("./aoc_input_example_1.txt")?;

        let expect_circuit = vec![
            Gate::Input("x00".into(), 1), Gate::Input("x01".into(), 1), Gate::Input("x02".into(), 1), 
            Gate::Input("y00".into(), 0), Gate::Input("y01".into(), 1), Gate::Input("y02".into(), 0), 
            Gate::Logic("z00".into(), Op::And, vec!["x00".into(), "y00".into()]),
            Gate::Logic("z01".into(), Op::XOr, vec!["x01".into(), "y01".into()]),
            Gate::Logic("z02".into(), Op::Or, vec!["x02".into(), "y02".into()]),
        ];
        assert_eq!(expect_circuit, circuit);

        let expect_outputs = vec![
            Output{name:"z00".into(), order: 0, value: None },
            Output{name:"z01".into(), order: 1, value: None },
            Output{name:"z02".into(), order: 2, value: None },
        ];
        assert_eq!(expect_outputs, outputs);

        Ok(())
    }

    #[test]
    fn run_example() -> Result<(), Box<dyn std::error::Error>> {
        let (circuit, mut outputs) = read_file("./aoc_input_example_1.txt")?;

        let expect_outputs = vec![
            Output{name:"z00".into(), order: 0, value: Some(0) },
            Output{name:"z01".into(), order: 1, value: Some(0) },
            Output{name:"z02".into(), order: 2, value: Some(1) },        
        ];

        run(&circuit, &mut outputs);

        assert_eq!(expect_outputs, outputs);

        Ok(())
    }
}
