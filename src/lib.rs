use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Addition(i32, i32),
    Substraction(i32, i32),
    Multiplication(i32, i32),
    Identity(i32),
}

impl Operation {
    fn symbol(&self) -> char {
        match self {
            Operation::Addition(_, _) => '+',
            Operation::Substraction(_, _) => '-',
            Operation::Multiplication(_, _) => '*',
            Operation::Identity(_) => ' ',
        }
    }
}

struct CacheData {
    complexity: u32,
    operations: Vec<Operation>,
}

pub struct Solver {
    cache: HashMap<i32, CacheData>,
    inputs: Vec<i32>,
    previous_inputs_length: usize,
}

impl Solver {
    pub fn new(base: &[i32]) -> Solver {
        let inputs = base.to_vec();
        let cache = inputs
            .iter()
            .map(|&x| {
                (
                    x,
                    CacheData {
                        complexity: 0,
                        operations: vec![Operation::Identity(x)],
                    },
                )
            })
            .collect();

        Solver {
            cache,
            inputs,
            previous_inputs_length: 0,
        }
    }

    pub fn get(&self, value: i32) -> Option<&Vec<Operation>> {
        self.cache.get(&value).map(|data| &data.operations)
    }

    pub fn get_lisp(&self, value: i32) -> Vec<String> {
        self.cache
            .get(&value)
            .unwrap_or(&CacheData {
                complexity: 0,
                operations: vec![],
            })
            .operations
            .iter()
            .flat_map(|operation| match operation {
                Operation::Identity(value) => vec![value.to_string()],
                Operation::Addition(left, right)
                | Operation::Substraction(left, right)
                | Operation::Multiplication(left, right) => self
                    .get_lisp(*left)
                    .iter()
                    .cartesian_product(self.get_lisp(*right).iter())
                    .map(|(x, y)| format!("({} {} {})", x, operation.symbol(), y))
                    .collect(),
            })
            .collect()
    }

    pub fn iterate(&mut self) {
        let length = self.inputs.len();

        for i in 0..length {
            for j in max(self.previous_inputs_length, i)..length {
                let a = self.inputs[i];
                let b = self.inputs[j];
                let complexity = self.cache[&a].complexity + self.cache[&b].complexity + 1;

                for (result, operation) in [
                    (a.checked_add(b), Operation::Addition(a, b)),
                    (a.checked_sub(b), Operation::Substraction(a, b)),
                    (b.checked_sub(a), Operation::Substraction(b, a)),
                    (a.checked_mul(b), Operation::Multiplication(a, b)),
                ] {
                    match result {
                        Some(result) => {
                            self.cache
                                .entry(result)
                                .and_modify(|data| match data.complexity.cmp(&complexity) {
                                    std::cmp::Ordering::Greater => {
                                        data.complexity = complexity;
                                        data.operations = vec![operation];
                                    }
                                    std::cmp::Ordering::Equal => {
                                        data.operations.push(operation);
                                    }
                                    _ => (),
                                })
                                .or_insert_with(|| {
                                    self.inputs.push(result);
                                    CacheData {
                                        complexity,
                                        operations: vec![operation],
                                    }
                                });
                        }
                        None => continue,
                    }
                }
            }
        }

        self.previous_inputs_length = length;
    }

    pub fn iterate_until_get(&mut self, value: i32) {
        while !self.cache.contains_key(&value) {
            self.iterate();
        }
    }
}
