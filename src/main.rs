use beltex::{Solver};
use clap::{Command, Arg};

fn main() {
    let matches = Command::new("")
        .version("1.0")
        .author("Adam Brutsaert <brutsaertadam@yahoo.fr>")
        .about("A project made in Rust that searches each and every possible way to get to a number in the least amount of steps with certains constraints.")
        .arg(Arg::new("base")
            .short('b')
            .long("base")
            .value_delimiter(',')
            .default_value("1,2,3,4,5,6,7,8,9"))
        .arg(Arg::new("target").required(true))
        .get_matches();

    let base = matches.get_many::<String>("base").unwrap().map(|v| v.parse().unwrap()).collect::<Vec<i32>>();
    let target = matches.get_one::<String>("target").unwrap().parse::<i32>().unwrap();

    let mut solver = Solver::new(base.as_slice());
    solver.iterate_until_get(target);
    for lisp in solver.get_lisp(target) {
        println!("{target} = {lisp}");
    }
}
