use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {:?}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<String, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let (regs, programs) = read_file(path)?;

    let mut computer = Computer::new(regs, &programs);
    let mut output = vec![];

    let _ = computer.exec(&mut output);

    let output = output.into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
    ;

    Ok(output)
}

#[derive(PartialEq, Debug)]
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

struct Computer {
    regs: Registers,
    programs: Vec<u8>,
    pc: usize,
}

impl Computer {
    fn new(regs: Registers, programs: &[u8]) -> Self {
        Self {
            regs,
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

        let operand = if (opcode == 1) || (opcode == 3) {
            operand
        }
        else {
            match operand {
                0_u64..=3_u64 | 7 => operand,
                4_u64 => self.regs.reg_a,
                5_u64 => self.regs.reg_b,
                6_u64 => self.regs.reg_c,
                _ => return None,
            }
        };

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

    fn exec(&mut self, output: &mut Vec<u64>) -> usize {
        let mut len: usize = 0;

        while let Some(op) = self.next() {
            match op {
                Opcode::Adv(operand) => self.exec_adv(operand),
                Opcode::Bxl(operand) => self.exec_bxl(operand),
                Opcode::Bst(operand) => self.exec_bst(operand),
                Opcode::Jnz(operand) => self.exec_jnz(operand),
                Opcode::Bxc(_) => self.exec_bxc(),
                Opcode::Out(operand) => {
                    len += self.exec_out(operand, output);
                },
                Opcode::Bdv(operand) => self.exec_bdv(operand),
                Opcode::Cdv(operand) => self.exec_cdv(operand),
                Opcode::Hlt => {},
            }
        }

        len
    } 

    fn exec_adv(&mut self, operand: u64) {
        let denomitor = u64::pow(2, operand as u32);
        self.regs.reg_a /= denomitor;
    }

    fn exec_bxl(&mut self, operand: u64) {
        self.regs.reg_b ^= operand;
    }

    fn exec_bst(&mut self, operand: u64) {
        self.regs.reg_b = operand % 8;
    }

    fn exec_jnz(&mut self, operand: u64) {
            if self.regs.reg_a == 0 {
            self.pc += 2;
        }
        else {
            self.pc = operand as usize;
        }
    }

    fn exec_bxc(&mut self) {
        self.regs.reg_b ^= self.regs.reg_c;
    }

    fn exec_out(&mut self, operand: u64, output: &mut Vec<u64>) -> usize {
        output.push(operand % 8);
        1
    }

    fn exec_bdv(&mut self, operand: u64) {
        let denomitor = u64::pow(2, operand as u32);
        self.regs.reg_b = self.regs.reg_a / denomitor;
    }

    fn exec_cdv(&mut self, operand: u64) {
        let denomitor = u64::pow(2, operand as u32);
        self.regs.reg_c = self.regs.reg_a / denomitor;
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
        assert_eq!("4,6,3,5,6,3,5,2,1,0", &solve("./aoc_input_example.txt")?);
        Ok(())
    }

    #[test]
    fn read_file_example() -> Result<(), Box<dyn std::error::Error>> {
        let (registers, programs) = read_file("aoc_input_example.txt")?;

        assert_eq!(Registers{ reg_a: 729, reg_b: 0, reg_c: 0 }, registers);
        assert_eq!(vec![0,1,5,4,3,0], programs);
        Ok(())
    }

    #[test]
    fn decode_example() -> Result<(), Box<dyn std::error::Error>> {
        let regs = Registers {
            reg_a: 1000,
            reg_b: 16,
            reg_c: 32,
        };
        let mut decoder = Computer::new(regs, &[0, 1, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 7, 5, 3, 4]);

        assert_eq!(Some(Opcode::Adv(1)), decoder.next());
        assert_eq!(Some(Opcode::Bxl(2)), decoder.next());
        assert_eq!(Some(Opcode::Bst(3)), decoder.next());
        assert_eq!(Some(Opcode::Bxc(1000)), decoder.next());
        assert_eq!(Some(Opcode::Out(32)), decoder.next());
        assert_eq!(Some(Opcode::Bdv(7)), decoder.next());
        assert_eq!(Some(Opcode::Cdv(16)), decoder.next());
        assert_eq!(Some(Opcode::Jnz(4)), decoder.next());
        Ok(())
    }

    #[test]
    fn exec_example() -> Result<(), Box<dyn std::error::Error>> {
        let mut output = Vec::<u64>::new();
        let mut write_len: usize = 0;

        write_len += {
            let mut computer = Computer::new(Registers{ reg_a: 0, reg_b: 0, reg_c: 9}, &[2, 6]);
            let len = computer.exec(&mut output);
            assert_eq!(0, len);
            assert_eq!(1, computer.regs.reg_b);
            assert_eq!(&Vec::<u64>::new(), &output[write_len..]);
            len
        };
        write_len += {
            let mut computer = Computer::new(Registers{ reg_a: 10, reg_b: 0, reg_c: 0}, &[5,0,5,1,5,4]);
            let len = computer.exec(&mut output);
            assert_eq!(3, len);
            assert_eq!(vec![0, 1, 2], output[write_len..]);
            len
        };
        let _ =  {
            let mut computer = Computer::new(Registers{ reg_a: 2024, reg_b: 0, reg_c: 0}, &[0,1,5,4,3,0]);
            let len = computer.exec(&mut output);
            assert_eq!(11, len);
            assert_eq!(vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], output[write_len..]);
            len
        };
        let _ = {
            let mut computer = Computer::new(Registers{ reg_a: 0, reg_b: 29, reg_c: 0}, &[1, 7]);
            let len = computer.exec(&mut output);
            assert_eq!(0, len);
            assert_eq!(26, computer.regs.reg_b);
            len
        };
        let _ = {
            let mut computer = Computer::new(Registers{ reg_a: 0, reg_b: 2024, reg_c: 43690}, &[4, 0]);
            let len = computer.exec(&mut output);
            assert_eq!(0, len);
            assert_eq!(44354, computer.regs.reg_b);
            len
        };
        let _ = {
            let mut computer = Computer::new(Registers{ reg_a: 999, reg_b: 0, reg_c: 0}, &[6, 1]);
            let len = computer.exec(&mut output);
            assert_eq!(0, len);
            assert_eq!(999, computer.regs.reg_a);
            assert_eq!(499, computer.regs.reg_b);
            assert_eq!(0, computer.regs.reg_c);
            len
        };
        let _ = {
            let mut computer = Computer::new(Registers{ reg_a: 999, reg_b: 0, reg_c: 0}, &[7, 1]);
            let len = computer.exec(&mut output);
            assert_eq!(0, len);
            assert_eq!(999, computer.regs.reg_a);
            assert_eq!(0, computer.regs.reg_b);
            assert_eq!(499, computer.regs.reg_c);
            len
        };

        Ok(())
    }

    #[test]
    fn exec_example_p2() -> Result<(), Box<dyn std::error::Error>> {
        let mut output = Vec::<u64>::new();

        let mut computer = Computer::new(Registers{ reg_a: 117440, reg_b: 0, reg_c: 0}, &[0, 3, 5, 4, 3, 0]);
        let len = computer.exec(&mut output);
        assert_eq!(6, len);
        assert_eq!(&vec![0, 3, 5, 4, 3, 0], &output);

        Ok(())
    }
}