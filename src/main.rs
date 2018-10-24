mod eval_expr;

fn print_usage() -> () {
    println!("USAGE:\n\t./fun_evalexpr expr\n\nDESCRIPTION:\n\texpr\tmathematical expression to be evaluated")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            let exp = &args[1];
            println!("{}", eval_expr::evaluate(exp));
        },
        //wrong argument number case
        _ => {
            print_usage();
        }
    }
}
