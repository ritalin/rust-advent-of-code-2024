use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<Option<String>, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (regs, programs) = read_file(path)?;

    let computer = Computer::new(regs, &programs);

    Ok(solve_internal(&computer).map(|a| a.to_string()))
}

fn solve_internal(computer: &Computer) -> Option<u64> {
    let opcodes = computer.clone().into_iter()
        .take_while(|op| match op {
            Opcode::Jnz(_) => false,
            _ => true,
        })
        .collect::<Vec<_>>()
    ;

    let output = computer.programs.iter()
        .rev()
        .map(|x| *x)
        .collect::<Vec<_>>()
    ;
    
    let regs = Registers { reg_a: 0, reg_b: 0, reg_c: 0 };
    Analyzer::analyze(&regs, &opcodes, &output, 0)
}

struct Analyzer {}

impl Analyzer {
    fn analyze(regs: &Registers, opcodes: &[Opcode], outputs: &[u8], index: usize) -> Option<u64> {
        if index >= outputs.len() {
            return Some(regs.reg_a);
        }

        for a in 0..8 {
            if let Some(regs) = Analyzer::analyze_internal(regs, opcodes, outputs[index] as u64, a) {
                if let Some(reg_a) = Analyzer::analyze(&regs, opcodes, outputs, index + 1) {
                    return Some(reg_a);
                }
            }
        }

        None
    }

    fn analyze_internal(regs: &Registers, opcodes: &[Opcode], output: u64, a: u64) -> Option<Registers> {
        let mut tmp = Registers{ reg_a: (regs.reg_a << 3) + a, ..*regs};

        for op in opcodes {
            match op {
                Opcode::Adv(operand) => Analyzer::analyze_adv(&mut tmp, *operand),
                Opcode::Bxl(operand) => Analyzer::analyze_bxl(&mut tmp, *operand),
                Opcode::Bst(operand) => Analyzer::analyze_bst(&mut tmp, *operand),
                Opcode::Jnz(_) => todo!(),
                Opcode::Bxc(_) => Analyzer::analyze_bxc(&mut tmp),
                Opcode::Out(operand) => {
                    if Analyzer::analyze_out(&mut tmp, *operand, output) {
                        return Some(Registers{ reg_a: (regs.reg_a << 3) + a, ..*regs});
                    }
                }
                Opcode::Bdv(_) => todo!(),
                Opcode::Cdv(operand) => Analyzer::analyze_cdv(&mut tmp, *operand),
                Opcode::Hlt => {},
            }
        }

        None
    }

    fn analyze_adv(regs: &mut Registers, operand: u64) {
        let operand = match operand {
            0..=3 | 7 => operand,
            4 => regs.reg_a,
            5 => regs.reg_b,
            6 => regs.reg_c,
            _ => return,
        };
        regs.reg_a >>= operand;
    }

    fn analyze_bxl(regs: &mut Registers, operand: u64) {
        regs.reg_b ^= operand;
    }

    fn analyze_bst(regs: &mut Registers, operand: u64) {
        let operand = match operand {
            0..=3 | 7 => operand,
            4 => regs.reg_a,
            5 => regs.reg_b,
            6 => regs.reg_c,
            _ => return,
        };
        regs.reg_b = operand % 8;
    }

    fn analyze_bxc(regs: &mut Registers) {
        regs.reg_b ^= regs.reg_c;
    }

    fn analyze_out(regs: &mut Registers, operand: u64, output: u64) -> bool {
        let operand = match operand {
            0..=3 | 7 => operand,
            4 => regs.reg_a,
            5 => regs.reg_b,
            6 => regs.reg_c,
            _ => return false,
        };

        operand % 8 == output
    }

    fn analyze_cdv(regs: &mut Registers, operand: u64) {
        let operand = match operand {
            0..=3 | 7 => operand,
            4 => regs.reg_a,
            5 => regs.reg_b,
            6 => regs.reg_c,
            _ => return,
        };

        regs.reg_c = regs.reg_a >> operand;
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Registers {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
}

#[derive(PartialEq, Debug)]
enum Opcode {
    Adv(u64),
    Bxl(u64),
    Bst(u64),
    Jnz(u64),
    Bxc(u64),
    Out(u64),
    Bdv(u64),
    Cdv(u64),
    Hlt,
}

#[derive(Clone)]
struct Computer {
    programs: Vec<u8>,
    pc: usize,
}

impl Computer {
    fn new(_: Registers, programs: &[u8]) -> Self {
        Self {
            programs: programs.to_vec(),
            pc: 0,
        }
    }

    fn decode(&mut self) -> Option<Opcode> {
        if self.pc >= self.programs.len() {
            return None;
        }

        let instruction = &self.programs[self.pc..][..2];

        let opcode = instruction[0];
        let operand = instruction[1] as u64;

        let opcode = match opcode {
            0 => Opcode::Adv(operand),
            1 => Opcode::Bxl(operand),
            2 => Opcode::Bst(operand),
            3 => Opcode::Jnz(operand),
            4 => Opcode::Bxc(operand),
            5 => Opcode::Out(operand),
            6 => Opcode::Bdv(operand),
            7 => Opcode::Cdv(operand),
            _ => Opcode::Hlt,
        };

        match opcode {
            Opcode::Jnz(_) => Some(opcode),
            Opcode::Hlt => None,
            _ => {
                self.pc += 2;
                Some(opcode)
            }
        }
    }
}

impl Iterator for Computer {
    type Item = Opcode;

    fn next(&mut self) -> Option<Self::Item> {
        self.decode()
    }
}

#[derive(Debug)]
enum PatternError {
    RegNotFound(String),
    RegInvalid(String),
    ProgramInvalid(String),
}
impl std::fmt::Display for PatternError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternError::RegNotFound(msg) => write!(f, "PatternError: {}", msg),
            PatternError::RegInvalid(msg) => write!(f, "PatternError: {}", msg),
            PatternError::ProgramInvalid(msg) => write!(f, "PatternError: {}", msg),
        }
    }
}
impl std::error::Error for PatternError {}

fn read_file<P>(path: P) -> Result<(Registers, Vec<u8>), Box<dyn std::error::Error>> 
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    let regs = Registers {
        reg_a: read_reg(&mut reader, "A")?,
        reg_b: read_reg(&mut reader, "B")?,
        reg_c: read_reg(&mut reader, "C")?,
    };
    let _ = reader.read_line(&mut buf);

    let programs = read_program(&mut reader)?;

    Ok((regs, programs))
}

fn read_reg(reader: &mut impl BufRead, name: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let mut buf = String::new();

    let len = reader.read_line(&mut buf)?;
    if len == 0 {
        return Err(Box::new(PatternError::RegNotFound(name.into())));
    }
    
    let Some(value) = buf.split(":").last() else {
        return Err(Box::new(PatternError::RegInvalid(name.into())));
    };

    Ok(value.trim().parse::<u64>()?)
}

fn read_program(reader: &mut impl BufRead) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buf = String::new();

    let len = reader.read_line(&mut buf)?;
    if len == 0 {
        return Err(Box::new(PatternError::ProgramInvalid("Not found".into())));
    }
    
    let Some(codes) = buf.split(":").last() else {
        return Err(Box::new(PatternError::ProgramInvalid("Empty".into())));
    };

    let programs = codes.trim().split(",")
        .map(|op| op.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?
    ;

    Ok(programs)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(Some("117440".into()), solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn solve_input() -> Result<(), Box<dyn std::error::Error>> {
        let (regs, programs) = read_file("./aoc_input.txt")?;

        let computer = Computer::new(regs, &programs);
        if let Some(reg_a) = solve_internal(&computer) {  
            eprintln!("{:?}", reg_a);
        }

        Ok(())
    }
}