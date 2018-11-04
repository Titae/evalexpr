mod eval_expr;

fn print_usage() -> i32 {
    println!("USAGE:\n\t./fun_evalexpr expr\n\nDESCRIPTION:\n\texpr\tmathematical expression to be evaluated");
    84
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let return_code = match args.len() {
        2 => {
            let exp = &args[1];
            match eval_expr::evaluate(exp) {
                Err(_) => {
		    println!("\"{}\" is unvalid!", exp);
		    84
		},
                Ok(res) => {
		    println!("{}", res);
		    0
		}
            }
        },
        //wrong argument number case
        _ => print_usage()
    };
    std::process::exit(return_code);
}
