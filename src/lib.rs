use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::max;

#[derive(Copy, Clone, Debug)]
pub enum Operation {
    Addition(i32, i32),
    Substraction(i32, i32),
    Multiplication(i32, i32),
    Identity(i32)
}

impl Operation {
    fn symbol(&self) -> char {
        match self {
            Operation::Addition(_, _) => '+',
            Operation::Substraction(_, _) => '-',
            Operation::Multiplication(_, _) => '*',
            Operation::Identity(_) => ' '
        }
    }
}

pub struct Solver {
    complexity: HashMap<i32, u32>,
    cache: HashMap<i32, Vec<Operation>>,
    inputs: Vec<i32>,
    previous_inputs_length: usize
}

impl Solver {
    pub fn new(base: &[i32]) -> Solver {
        let inputs = base.to_vec();
        let complexity = inputs.iter().map(|&x| (x, 0)).collect();
        let cache = inputs.iter().map(|&x| (x, vec![Operation::Identity(x)])).collect();

        Solver {
            complexity,
            cache,
            inputs,
            previous_inputs_length: 0
        }
    }

    pub fn get(&self, value: i32) -> Option<&Vec<Operation>> {
        self.cache.get(&value)
    }

    pub fn get_lisp(&self, value: i32) -> Vec<String> {
        if let Some(operations) = self.cache.get(&value) {
            operations.iter().flat_map(|operation| {
                match operation {
                    Operation::Identity(value) => vec![value.to_string()],
                    Operation::Addition(left, right) | Operation::Substraction(left, right) | Operation::Multiplication(left, right) => {
                        self.get_lisp(*left).iter().cartesian_product(self.get_lisp(*right).iter()).map(|(x, y)| {
                            format!("({} {} {})", x, operation.symbol(), y)
                        }).collect()
                    }
                }
            }).collect()
        } else {
            vec![]
        }
    }

    pub fn iterate(&mut self) {
        let length = self.inputs.len();
        for i in 0..length {
            for j in max(self.previous_inputs_length, i)..length {
                let a = self.inputs[i];
                let b = self.inputs[j];
                let complexity = self.complexity[&a] + self.complexity[&b] + 1;


                for (result, operation) in [(a + b, Operation::Addition(a, b)),
                                            (a - b, Operation::Substraction(a, b)),
                                            (b - a, Operation::Substraction(b, a)),
                                            (a * b, Operation::Multiplication(a, b))] {
                    match self.complexity.get(&result) {
                        Some(&res_complexity) => {
                            match res_complexity.cmp(&complexity) {
                                std::cmp::Ordering::Greater => {
                                    self.complexity.insert(result, complexity);
                                    self.cache.insert(result, vec![operation]);
                                },
                                std::cmp::Ordering::Equal => {
                                    self.cache.get_mut(&result).unwrap().push(operation);
                                },
                                _ =>()
                            }
                        },
                        None => {
                            self.complexity.insert(result, complexity);
                            self.cache.insert(result, vec![Operation::Addition(a, b)]);
                            self.inputs.push(result)
                        }
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