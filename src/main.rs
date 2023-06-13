use beltex::{Solver};

fn main() {
    let mut solver = Solver::new(&[1, 2, 3, 4, 5, 6]);

    solver.iterate_until_get(42);

    let lisps = solver.get_lisp(42);

    for lisp in lisps {
        println!("42 = {}", lisp);
    }
}
