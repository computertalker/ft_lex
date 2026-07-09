use std::env;
use std::process;
use std::fs::File;

mod parse;
mod structs;
mod lexer;
use parse::parse_opt;
use structs::opt::Opt;
use lexer::lexer::lexer_loop;

fn _ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        process::exit(0);
    }

    let mut s_opt: Opt = Opt::default();
    let files: Vec<String> = parse_opt::handle_args(&mut args, &mut s_opt);

    for (index, file) in files.iter().enumerate() {
        if index == 0 && s_opt.getstdin() {
          // Lex from stdin
            // Lex from iterations
        }
        lexer_loop(file);
    }
    /*
    let mut _file = match File::open(&args[0]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[0]);
        }
    };
    */
}
