pub(crate) mod macros;

use std::io::{self, Read};

pub struct CantSeeFuckInterpreter {
    memory: Vec<u8>,
    pointer: usize,
}

impl Default for CantSeeFuckInterpreter {
    fn default() -> Self {
        CantSeeFuckInterpreter {
            pointer: 0,
            memory: vec![0; 30_000],
        }
    }
}

impl CantSeeFuckInterpreter {
    pub fn interpret(&mut self, code: &str) -> io::Result<()> {
        let mut pc = 0; // Program counter
        let mut loop_stack: Vec<usize> = Vec::new(); // Stack for loop positions
        let code_chars: Vec<char> = code.chars().collect();

        while pc < code_chars.len() {
            match code_chars[pc] {
                ' ' => {
                    self.pointer += 1;
                    if self.pointer >= self.memory.len() {
                        self.pointer = 0; // Wrap around
                    }
                }
                '\t' => {
                    if self.pointer == 0 {
                        self.pointer = self.memory.len() - 1; // Wrap around
                    } else {
                        self.pointer -= 1;
                    }
                }
                '\n' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                '\u{2063}' => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                '\r' => println!("{}", self.memory[self.pointer] as char),
                '\x0B' => {
                    let mut input = [0];
                    io::stdin().read_exact(&mut input)?;
                    self.memory[self.pointer] = input[0];
                }
                '\u{00A0}' => {
                    if self.memory[self.pointer] == 0 {
                        let mut loop_counter = 1;
                        while loop_counter > 0 {
                            pc += 1;
                            if pc >= code_chars.len() {
                                return Err(io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    "Unmatched '\u{00A0}' in code",
                                ));
                            }
                            if code_chars[pc] == '\u{00A0}' {
                                loop_counter += 1;
                            } else if code_chars[pc] == '\u{2007}' {
                                loop_counter -= 1;
                            }
                        }
                    } else {
                        loop_stack.push(pc);
                    }
                }
                '\u{2007}' => {
                    if let Some(&loop_start) = loop_stack.last() {
                        if self.memory[self.pointer] != 0 {
                            pc = loop_start;
                        } else {
                            loop_stack.pop();
                        }
                    } else {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Unmatched '\u{2007}' in code",
                        ));
                    }
                }
                _ => (), // Ignore invalid characters
            }
            pc += 1;
        }

        if !loop_stack.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unmatched '\u{00A0}' in code",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    pub fn simple_test() -> io::Result<()> {
        let mut interpreter = CantSeeFuckInterpreter::default();
        let program = std::fs::read_to_string("test_resources/simple_program.csf").unwrap();
        assert_stdout_eq!(interpreter.interpret(&program)?, "1\n6\n5\n");
        Ok(())
    }
}
