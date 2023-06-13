use beltex::{Solver};

fn main() {
    let mut solver = Solver::new(&[1, 2, 3, 4, 5, 6]);

    solver.iterate_until_get(26);

    let lisps = solver.get_lisp(26);

    for lisp in lisps {
        println!("26 = {}", lisp);
    }
}
