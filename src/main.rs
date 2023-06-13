use beltex::{Solver};

fn main() {
    let mut solver = Solver::new(&[1, 2, 3, 4, 5, 6]);

    solver.iterate();
    solver.iterate();

    println!("{:#?}", solver.get_lisp(14))
}
