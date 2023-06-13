use beltex::{Solver};
use clap;

fn main() {
    let command = clap::Command::new("prog")
        .arg(clap::Arg::new("base")
            .short('b')
            .long("base")
            .value_delimiter(',')
            .default_value("1,2,3,4,5,6,7,8,9"))
        .arg(clap::Arg::new("target"))
        .get_matches();

    let base = command.get_many::<String>("base").unwrap().map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
    let target = command.get_one::<String>("target").unwrap().parse::<i32>().unwrap();

    let mut solver = Solver::new(base.as_slice());
    solver.iterate_until_get(target);
    for lisp in solver.get_lisp(target) {
        println!("{target} = {lisp}");
    }
}
