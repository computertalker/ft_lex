use std::env; // standard library
use std::process;
use std::fs::File;

mod i_struct;

fn ft_error(content: &str, attachement: &str) -> ! // fn never come back
{
    eprintln!("ft_lex: {} {}", content, attachement);
    process::exit(1);
}

fn parse_opt(args: &mut Vec<String>, s_opt: &mut i_struct::Opt) -> usize
{ 
    let mut index = 0;
    for (i, arg) in args.iter().enumerate() { 
        index = i;
        match arg.as_str() {
            "-v" => s_opt.setv(),
            "-n" => s_opt.setn(),
            "-t" => s_opt.sett(),
            "-"  => {
                s_opt.setstdin();
                return i;
            }
            _ => return i,
        }
    }
    index
}

fn parse_file(args: &mut Vec<String>, s_opt: &mut i_struct::Opt, i: usize) -> Vec<String> {
    let mut index = i;
    if !s_opt.is_none() {
        index += 1;
    }
    println!("i:{}", i);
    let mut new_args: Vec<String> = Vec::new();
    for (loop_i, _arg) in args.iter().enumerate().skip(index) {
        if loop_i >= i {
            new_args.push(args[loop_i].clone());
        }
    }
    new_args
}

fn main()
{
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Using stdin");
        process::exit(0);
    }
    
    args.remove(0);
    let mut s_opt = i_struct::Opt::default();
    
    let res_opt = parse_opt(&mut args, &mut s_opt);
    if res_opt == 0 {
        println!("No opt, i = {}", res_opt);
        if s_opt.is_none() {
            println!("file waiting for review");
        }
        else {
            println!("use stdin and look the option");
        }
    }

    let _files: Vec<String> = parse_file(&mut args, &mut s_opt, res_opt);
         
    for arg in _files.iter() {
        println!("'{}'", arg); 
    }
    // NOW NEED TO CHECK FILES
    //
    //
    let mut _file = match File::open(&args[0]) {
        Ok (file) => file,
        Err(_) => {
            ft_error("can't open", &args[0]);
        }
    };
}
