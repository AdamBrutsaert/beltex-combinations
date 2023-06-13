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
                    Operation::Addition(left, right) => {
                        self.get_lisp(*left).iter().cartesian_product(self.get_lisp(*right).iter()).map(|(x, y)| {
                            format!("({} + {})", x, y)
                        }).collect()
                    },
                    Operation::Substraction(left, right) => {
                        self.get_lisp(*left).iter().cartesian_product(self.get_lisp(*right).iter()).map(|(x, y)| {
                            format!("({} - {})", x, y)
                        }).collect()
                    },
                    Operation::Multiplication(left, right) => {
                        self.get_lisp(*left).iter().cartesian_product(self.get_lisp(*right).iter()).map(|(x, y)| {
                            format!("({} * {})", x, y)
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

                // Addition
                match self.complexity.get(&(a + b)) {
                    Some(&res_complexity) => {
                        match res_complexity.cmp(&complexity) {
                            std::cmp::Ordering::Greater => {
                                self.complexity.insert(a + b, complexity);
                                self.cache.insert(a + b, vec![Operation::Addition(a, b)]);
                            },
                            std::cmp::Ordering::Equal => {
                                self.cache.get_mut(&(a + b)).unwrap().push(Operation::Addition(a, b));
                            },
                            _ =>()
                        }
                    },
                    None => {
                        self.complexity.insert(a + b, complexity);
                        self.cache.insert(a + b, vec![Operation::Addition(a, b)]);
                        self.inputs.push(a + b)
                    }
                }

                // Substraction
                match self.complexity.get(&(a - b)) {
                    Some(&res_complexity) => {
                        match res_complexity.cmp(&complexity) {
                            std::cmp::Ordering::Greater => {
                                self.complexity.insert(a - b, complexity);
                                self.cache.insert(a - b, vec![Operation::Substraction(a, b)]);
                            },
                            std::cmp::Ordering::Equal => {
                                self.cache.get_mut(&(a - b)).unwrap().push(Operation::Substraction(a, b));
                            },
                            _ =>()
                        }
                    },
                    None => {
                        self.complexity.insert(a - b, complexity);
                        self.cache.insert(a - b, vec![Operation::Substraction(a, b)]);
                        self.inputs.push(a - b)
                    }
                }

                match self.complexity.get(&(b - a)) {
                    Some(&res_complexity) => {
                        match res_complexity.cmp(&complexity) {
                            std::cmp::Ordering::Greater => {
                                self.complexity.insert(b - a, complexity);
                                self.cache.insert(b - a, vec![Operation::Substraction(b, a)]);
                            },
                            std::cmp::Ordering::Equal => {
                                self.cache.get_mut(&(b - a)).unwrap().push(Operation::Substraction(b, a));
                            },
                            _ =>()
                        }
                    },
                    None => {
                        self.complexity.insert(b - a, complexity);
                        self.cache.insert(b - a, vec![Operation::Substraction(b, a)]);
                        self.inputs.push(b - a)
                    }
                }

                // Multiplication
                match self.complexity.get(&(a * b)) {
                    Some(&res_complexity) => {
                        match res_complexity.cmp(&complexity) {
                            std::cmp::Ordering::Greater => {
                                self.complexity.insert(a * b, complexity);
                                self.cache.insert(a + b, vec![Operation::Multiplication(a, b)]);
                            },
                            std::cmp::Ordering::Equal => {
                                self.cache.get_mut(&(a * b)).unwrap().push(Operation::Multiplication(a, b));
                            },
                            _ =>()
                        }
                    },
                    None => {
                        self.complexity.insert(a * b, complexity);
                        self.cache.insert(a * b, vec![Operation::Multiplication(a, b)]);
                        self.inputs.push(a * b)
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