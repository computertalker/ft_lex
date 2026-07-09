use crate::structs::opt;

fn parse_opt(args: &mut Vec<String>, s_opt: &mut opt::Opt) -> usize
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

fn parse_file(args: &mut Vec<String>, s_opt: &mut opt::Opt, i: usize) -> Vec<String> {
    let mut index = i;
    if !s_opt.is_none() {
        index += 1;
    }
    let mut new_args: Vec<String> = Vec::new();
    for (loop_i, arg) in args.iter().enumerate().skip(index) {
        if loop_i >= i {
            new_args.push(arg.clone());
        }
    }
    new_args
}

pub fn handle_args(mut args: &mut Vec<String>, s_opt: &mut opt::Opt) -> Vec<String>
{
    args.remove(0);
    let res_opt = parse_opt(args, s_opt);
    let temp: Vec<String> = parse_file(&mut args, s_opt, res_opt);
    temp
}
