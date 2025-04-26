pub(crate) mod macros;

use std::collections::HashMap;
use std::io::{self, Read};

pub struct CantSeeFuckInterpreter {
    memory: Vec<u8>,
    pointer: usize,
    code_map: HashMap<char, char>,
}

impl Default for CantSeeFuckInterpreter {
    fn default() -> Self {
        let mut code_map = HashMap::new();
        code_map.insert(' ', '>'); // Map space to '>'
        code_map.insert('\t', '<'); // Map tab to '<'
        code_map.insert('\n', '+'); // Map newline to '+'
        code_map.insert('\u{2063}', '-'); // Map invisible separator to '-'
        code_map.insert('\r', '.'); // Map carriage return to '.'
        code_map.insert('\x0B', ','); // Map vertical tab to ','
        code_map.insert('\u{00A0}', '['); // Map non-breaking space to '['
        code_map.insert('\u{2007}', ']'); // Map figure space to ']'
        CantSeeFuckInterpreter {
            memory: vec![0; 30_000],
            pointer: 0,
            code_map,
        }
    }
}

impl CantSeeFuckInterpreter {
    pub fn interpret(&mut self, code: &str) -> io::Result<()> {
        let mut pc = 0; // Program counter
        let mut loop_stack: Vec<usize> = Vec::new(); // Stack for loop positions
        let code_chars: Vec<char> = code.chars().collect();

        while pc < code_chars.len() {
            match self.code_map.get(&code_chars[pc]) {
                Some('>') => {
                    self.pointer += 1;
                    if self.pointer >= self.memory.len() {
                        self.pointer = 0; // Wrap around
                    }
                }
                Some('<') => {
                    if self.pointer == 0 {
                        self.pointer = self.memory.len() - 1; // Wrap around
                    } else {
                        self.pointer -= 1;
                    }
                }
                Some('+') => self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1),
                Some('-') => self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1),
                Some('.') => println!("{}", self.memory[self.pointer] as char),
                Some(',') => {
                    let mut input = [0];
                    io::stdin().read_exact(&mut input)?;
                    self.memory[self.pointer] = input[0];
                }
                Some('[') => {
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
                            if let Some(&c) = self.code_map.get(&code_chars[pc]) {
                                if c == '[' {
                                    loop_counter += 1;
                                } else if c == ']' {
                                    loop_counter -= 1;
                                }
                            }
                        }
                    } else {
                        loop_stack.push(pc);
                    }
                }
                Some(']') => {
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
