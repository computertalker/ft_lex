use crate::structs::tokens;
use std::iter::Peekable;
use std::str::Chars;
use std::fs;

fn skip_line(chars: &mut Peekable<Chars>) {
    while let Some(&c) = chars.peek() {
        if c == '\n' {
            break;
        }
        chars.next();
    }
}

pub fn lexer_loop(path: &str)
{
    let mut first_part = false;
    let mut second_part = false;
    let mut line = 1;
    let content = fs::read_to_string(path).unwrap();
    let mut chars: Peekable<Chars> = content.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                line+=1;
            }
            '%' => {
                if chars.peek() == Some(&'{') {
                    first_part = true;
                }
            }
            _ => {
                if !first_part && !second_part {
                    if c.is_alphabetic() {
                        eprintln!("{}:{}: incomplete name definition", path, line);
                        skip_line(&mut chars);
                        line+=1;
                    }
                    else {
                        eprintln!("{}:{}: bad character: {}", path, line, c)
                    }
                }
            }
        }
    }
}
