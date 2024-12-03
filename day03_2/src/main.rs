use std::{fs::File, io::{BufRead, BufReader}, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("total: {}", solve("./aoc_input.txt")?);
    Ok(())
}

fn solve<P>(path: P) -> Result<i32, Box<dyn std::error::Error>>
    where P: AsRef<Path>
{
    let mut reader = BufReader::new(File::open(path)?);
    let mut buf = String::new();

    while reader.read_line(&mut buf)? > 0 {}
    let total: i32 = TokenIterator::new(buf.clone()).sum::<i32>();

    Ok(total)
}

#[derive(Debug)]
enum State {
    Identifier,
    LBracket,
    LValue,
    Comma,
    RValue,
    RBracket,
    Eof,
}

struct TokenIterator {
    source: String,
    index: usize,
    next_state: State,
}

impl TokenIterator {
    pub fn new(source: String) -> Self {
        Self {
            source,
            index: 0,
            next_state: State::Identifier,
        }
    }
}

impl Iterator for TokenIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut lhs: Option<i32> = None;
        let mut rhs: Option<i32> = None;
        let mut disabled = false;

        while self.index < self.source.len() {
            if let Some(s) = self.source.chars().nth(self.index) {
                if s == ' ' {
                    self.index += 1;
                    continue;
                }
            }
            if self.source[self.index..].starts_with("do()") {
                disabled = false;
                self.index += "do()".len();
            }
            if self.source[self.index..].starts_with("don't()") {
                disabled = true;
                self.index += "don't()".len();
            }
            if disabled {
                self.index += 1;
                continue;
            }

            match self.next_state {
                State::Identifier => match self.source[self.index..].starts_with("mul") {
                    true => {
                        self.next_state = State::LBracket;
                        self.index += 3;
                    }
                    false => {
                        self.index += 1;
                    }
                }
                State::LBracket => match self.source[self.index..].starts_with("(") {
                    true => {
                        self.next_state = State::LValue;
                        self.index += 1;
                    }
                    false => {
                        self.next_state = State::Identifier;
                        self.index += 1;
                    }
                }
                State::LValue => match self.source[self.index..].find(|c: char| !c.is_ascii_digit()) {
                    Some(p) => {
                        lhs = self.source[self.index..(self.index+p)].parse().ok();
                        self.next_state = State::Comma;
                        self.index += p;
                    }
                    None => {
                        self.next_state = State::Identifier;
                        self.index += 1;
                    }
                }
                State::Comma => match self.source[self.index..].starts_with(",") {
                    true => {
                        self.next_state = State::RValue;
                        self.index += 1;
                    }
                    false => {
                        self.next_state = State::Identifier;
                        self.index += 1;
                    }
                }
                State::RValue => match self.source[self.index..].find(|c: char| !c.is_ascii_digit()) {
                    Some(p) => {
                        rhs = self.source[self.index..(self.index+p)].parse().ok();
                        self.next_state = State::RBracket;
                        self.index += p;
                    }
                    None => {
                        self.next_state = State::Identifier;
                        self.index += 1;
                    }
                }
                State::RBracket => {
                    match self.source[self.index..].starts_with(")") {
                        true if lhs.is_some() && rhs.is_some() => {
                            self.next_state = State::Identifier;
                            self.index += 1;
                            return lhs.zip(rhs).map(|(v1, v2)| v1 * v2);
                        }
                        _ => {
                            self.next_state = State::Identifier;
                            self.index += 1;
                        }
                    }
                }
                State::Eof => break,
            }
        }

        self.next_state = State::Eof;
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() -> Result<(), Box<dyn std::error::Error>> {
        let actual = crate::solve("./aoc_input_example.txt")?;
        assert_eq!(48, actual);
        Ok(())
    }
}
