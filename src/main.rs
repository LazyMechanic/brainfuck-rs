use std::io;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut state: std::vec::Vec<usize> = std::vec::Vec::new();
    let mut cur_index: usize = 0;

    let mut prog_text: String = String::new();
    io::stdin().read_to_string(&mut prog_text)?;
    let prog_text_chars: std::vec::Vec<char> = prog_text.chars().collect();

    let mut i = 0;

    while i < prog_text_chars.len() {
        let ch = prog_text_chars[i];
        match ch {
            '>' => cur_index += 1,
            '<' => cur_index -= 1,
            '.' => print!("{}", memory[cur_index] as char),
            ',' => {
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                memory[cur_index] = input.parse()?
            }
            '+' => memory[cur_index] += 1,
            '-' => memory[cur_index] -= 1,
            '[' => {
                if memory[cur_index] == 0 {
                    while i < prog_text_chars.len() {
                        if prog_text_chars[i] == ']' {
                            break;
                        }

                        i += 1;
                    }
                } else {
                    state.push(i)
                }
            }
            ']' => {
                i = match state.pop() {
                    Some(x) => x,
                    None => {
                        return Err("Invalid syntax".into());
                    }
                };

                if i == 0 {
                    continue;
                } else {
                    i -= 1;
                }
            }
            _ => (),
        }

        i += 1;
    }

    Ok(())
}
