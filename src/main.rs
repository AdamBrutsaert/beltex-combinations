use beltex::Solver;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        required = true,
        help = "The value for which the combinations must be found"
    )]
    target: i32,

    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "1,2,3,4,5,6,7,8,9",
        help = "The base numbers to use"
    )]
    base: Vec<i32>,
}

fn main() {
    let args = Args::parse();
    let mut solver = Solver::new(args.base.as_slice());

    solver.iterate_until_get(args.target);
    for lisp in solver.get_lisp(args.target) {
        println!("{} = {lisp}", args.target);
    }
}
