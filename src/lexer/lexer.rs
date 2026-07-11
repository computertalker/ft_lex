use crate::structs::tokens::TOK::FirstBlock;
use crate::structs::tokens::{self, Token};

use std::iter::Peekable;
use std::str::Chars;
use std::fs;
use std::process;

fn skip_line(chars: &mut Peekable<Chars>, line: &mut i32, column: &mut i32) {
    while let Some(&c) = chars.peek() {
        if c == '\n' {
            *line+=1;
            *column = 1;
            break;
        }
        chars.next();
    }
}

fn skip_char(chars: &mut Peekable<Chars>, c: char,  line: &mut i32, column: &mut i32) {
    if c != '\n' {
        chars.next();
        *column+=1;
    }
    else {
        chars.next();
        *line+=1;
        *column = 1;
    }
}

fn get_lex_def(path: &str, tok_list: &mut Vec<Token>, mut chars: &mut Peekable<Chars>) -> i32
{
    let (mut err, mut  line, mut column): (i32, i32, i32) = (0, 1, 0);
    let mut open_block: bool = false;
    let mut strbuf: String = Default::default();

    while let Some(c) = chars.next() {
        match c {
            '\n' => {
                if open_block {
                    let mut def_tok = Token::default();
                    def_tok.update(FirstBlock, strbuf.clone(), line, column);
                    tok_list.push(def_tok);
                    strbuf.clear();
                }
                skip_char(chars, c, &mut line, &mut column);
            }
            '%' => {
                if chars.peek() == Some(&'{') {
                    open_block = true;
                    skip_line(&mut chars, &mut line, &mut column);
                }
                if chars.peek() == Some(&'}') {
                    open_block = false;
                    skip_line(&mut chars, &mut line, &mut column);
                }
                if chars.peek() == Some(&'%')  {
                    return err;
                }
            }
            _ => {
                if open_block {
                    strbuf.push(c);
                }
                else if c.is_alphabetic() {
                    eprintln!("{}:{}: incomplete name definition", path, line);
                    skip_line(&mut chars, &mut line, &mut column);
                    err+=1;
                }
                else {
                    eprintln!("{}:{}: bad character: {}", path, line, c);
                    err+=1;
                }
            }
        }
    }
    if open_block {
        eprintln!("{}:{}: premature EOF", path, line); 
        err+=1;
    }
    err
}

pub fn lexer_loop(path: &str)
{
    let mut should_die: i32 = 0;
    let mut tok_def: Vec<Token> = vec![];
    let content = fs::read_to_string(path).unwrap();
    let mut chars: Peekable<Chars> = content.chars().peekable();

    should_die += get_lex_def(path, &mut tok_def, &mut chars);
    

    if should_die > 0 {
        process::exit(1);
    }
}