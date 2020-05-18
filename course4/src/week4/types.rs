use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use num::abs;
use rand::seq::{IteratorRandom, SliceRandom};

/// Wrapper that represents operations on a set of bits.
pub trait BitSet {
    fn new(n_bits: usize) -> Self;
    fn is_set(&self, i: usize) -> bool;
    fn flip(&mut self, i: usize) -> bool;
}

type BitString = Vec<bool>;

/// Data type to use to refer to a bounded collection of bits.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BitVec {
    n_bits: usize,
    bits: BitString,
}

impl BitSet for BitVec {
    fn new(n_bits: usize) -> BitVec {
        let bits: BitString = (1..=n_bits).map(|_| rand::random::<bool>()).collect();
        BitVec { n_bits, bits }
    }

    fn is_set(&self, i: usize) -> bool {
        if i <= self.n_bits {
            self.bits[i - 1]
        } else {
            panic!(
                "Index out of range: attempted to read bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }

    fn flip(&mut self, i: usize) -> bool {
        if i <= self.n_bits {
            self.bits[i - 1] ^= true;
            self.bits[i - 1]
        } else {
            panic!(
                "Index out of range: attempted to flip bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }
}

/// This struct represents the logic expression of a single variable.
/// It is represented in the test cases as a signed integer:
/// - The absolute value of it represents the index of the bit vector to be used as input of the expression.
/// - The sign represents whether the expression evaluates to the indexed bit (if the sign is positive)
///   or its complement (if the sign is negative).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ExprVal {
    expr: i64,
}

/// This struct represents the evaluation of 2 single expressions, or'ed together.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ExprTerm {
    expr: Vec<ExprVal>,
}

/// This struct represents the evaluation of multiple expression terms, and'ed together.
#[derive(Debug)]
pub struct ExprFull {
    terms: HashSet<ExprTerm>,
    var_to_exprs: Vec<HashSet<ExprTerm>>,
    unsat_count: Option<usize>,
    unsat_exprs: Option<HashSet<ExprTerm>>,
}

/// This function takes in a 2-SAT expression and removes those terms for which
/// a variable's expression is the same.
///
/// Terms are expressed as an OR operation between 2 operands, either of which could be negated or
/// not.
/// If a given variable is always negated or never negated across all relevant terms of a 2-SAT
/// expression, then such terms can be removed from the whole expression without affecting the
/// satisfiable condition of the expression.
///
/// Furthermore, if by removing such terms the condition is met for other variables, the terms in which
/// such variables participate can also be removed.
/// This process can be iterated over and over until there are no variables that meet the condition
/// described above.
fn simplify(expr: &mut ExprFull) -> () {
    let mut still_going = true;
    while still_going {
        still_going = false;
        for (i, es) in expr.var_to_exprs.iter().enumerate() {
            let mut has_neg = false;
            let mut has_pos = false;
            for e in es.iter() {
                if expr.terms.contains(e) {
                    let is_neg = e.vars_neg().get(&(&i + 1)).unwrap().clone();
                    if is_neg {
                        has_neg = true;
                    } else {
                        has_pos = true;
                    }
                }
            }
            if has_neg ^ has_pos {
                still_going = true;
                for e in es {
                    expr.terms.remove(e);
                }
            }
        }
    }
}

/// Represents a logical expression that can be evaluated against an input bit vector.
pub trait Evaluable<T> {
    fn new(input: T) -> Self;
    fn eval(&self, input: &BitVec) -> bool;
    fn vars(&self) -> Vec<usize>;
    fn vars_neg(&self) -> HashMap<usize, bool>;
    fn is_neg(&self) -> bool;
}

/// Represents a logical expression that can be evaluated against an input bit vector.
pub trait EvaluableMut<T> {
    fn new(input: T) -> Self;
    fn eval(&mut self, input: &mut BitVec) -> bool;
}

impl Evaluable<i64> for ExprVal {
    fn new(input: i64) -> Self {
        ExprVal { expr: input }
    }

    fn eval(&self, input: &BitVec) -> bool {
        let index = abs(self.expr) as usize;
        self.is_neg() ^ input.is_set(index)
    }

    fn vars(&self) -> Vec<usize> {
        let index = abs(self.expr) as usize;
        vec![index]
    }

    fn vars_neg(&self) -> HashMap<usize, bool> {
        let index = abs(self.expr) as usize;
        let is_neg = self.is_neg();
        vec![(index, is_neg)].into_iter().collect()
    }

    fn is_neg(&self) -> bool {
        self.expr < 0
    }
}

impl Evaluable<&Vec<i64>> for ExprTerm {
    fn new(input: &Vec<i64>) -> Self {
        let expr = vec![ExprVal::new(input[0]), ExprVal::new(input[1])];
        ExprTerm { expr }
    }

    fn eval(&self, input: &BitVec) -> bool {
        &self.expr[0].eval(input) | &self.expr[1].eval(input)
    }

    fn vars(&self) -> Vec<usize> {
        self.expr.iter().flat_map(|e| e.vars()).collect()
    }

    fn vars_neg(&self) -> HashMap<usize, bool> {
        self.expr.iter().flat_map(|e| e.vars_neg()).collect()
    }

    fn is_neg(&self) -> bool {
        false
    }
}

impl EvaluableMut<&Vec<Vec<i64>>> for ExprFull {
    fn new(input: &Vec<Vec<i64>>) -> Self {
        let terms: HashSet<ExprTerm> = input.iter().map(|i| ExprTerm::new(i)).collect();
        let n_vars = input.len();

        let mut var_to_exprs: Vec<HashSet<ExprTerm>> =
            (0..n_vars).map(|_| HashSet::new()).collect();
        for e in &terms {
            let vars = e.vars();
            for v in vars {
                var_to_exprs[v - 1].insert(e.clone());
            }
        }

        let mut expr = ExprFull {
            terms,
            var_to_exprs,
            unsat_count: None,
            unsat_exprs: None,
        };
        simplify(&mut expr);
        expr
    }

    fn eval(&mut self, input: &mut BitVec) -> bool {
        if self.unsat_count.is_none() {
            let unsat_exprs: HashSet<ExprTerm> = HashSet::from_iter(
                self.terms
                    .iter()
                    .filter(|e| !e.eval(input))
                    .map(|e| e.clone()),
            );
            let unsat_count = unsat_exprs.len();

            self.unsat_exprs = Some(unsat_exprs);
            self.unsat_count = Some(unsat_count);
        }
        self.unsat_count.unwrap() == 0
    }
}

pub trait Satisfiable<T>: Evaluable<T> {
    fn satisfy_term_randomly(&self, input: &mut BitVec) -> ();
}

pub trait SatisfiableMut<T>: EvaluableMut<T> {
    fn satisfy_term_randomly(&mut self, input: &mut BitVec) -> ();
}

impl Satisfiable<i64> for ExprVal {
    fn satisfy_term_randomly(&self, input: &mut BitVec) -> () {
        if !self.eval(input) {
            let index = abs(self.expr) as usize;
            input.flip(index);
        }
    }
}

impl Satisfiable<&Vec<i64>> for ExprTerm {
    fn satisfy_term_randomly(&self, input: &mut BitVec) -> () {
        if !self.eval(input) {
            let mut rng = rand::thread_rng();
            self.expr
                .choose(&mut rng)
                .unwrap()
                .satisfy_term_randomly(input);
        }
    }
}

impl SatisfiableMut<&Vec<Vec<i64>>> for ExprFull {
    fn satisfy_term_randomly(&mut self, input: &mut BitVec) -> () {
        if !self.eval(input) {
            // We randomly choose a term in the expression whose evaluation is false.
            let mut rng = rand::thread_rng();
            let unsat_pick = self.unsat_exprs.as_ref().unwrap().iter().choose(&mut rng);

            match unsat_pick {
                // If no unsatisfied term is found, we set the internal count of unsatisfied
                // terms to 0.
                None => {
                    self.unsat_count = Some(0);
                }
                Some(unsat_term) => {
                    // We extract the variables that are part of the randomly chosen unsatisfied
                    // term, and compute all the terms in which those variables participate.
                    let vars: HashSet<usize> = HashSet::from_iter(unsat_term.vars());
                    let affected_exprs: HashSet<ExprTerm> = vars
                        .iter()
                        .map(|v| &self.var_to_exprs[v - 1])
                        .fold(HashSet::new(), |acc, exprs| {
                            acc.union(&exprs).map(|e| e.clone()).collect()
                        });

                    // We satisfy the term by randomly flipping the value of one of its operands.
                    unsat_term.satisfy_term_randomly(input);

                    // We compute the current unsatisfied terms in the expression by re-evaluating
                    // the terms in which the relevant operands participate.
                    // We subtract from the global count those terms that are now satisfied with the
                    // new value, and add those terms that are now unsatisfied.
                    let to_remove: HashSet<&ExprTerm> =
                        HashSet::from_iter(affected_exprs.iter().filter(|e| e.eval(input)));
                    let to_add: HashSet<ExprTerm> = HashSet::from_iter(
                        affected_exprs
                            .iter()
                            .filter(|e| !e.eval(input))
                            .map(|e| e.clone()),
                    );
                    let with_new_items: HashSet<_> =
                        HashSet::from_iter(self.unsat_exprs.as_ref().unwrap().union(&to_add));
                    let without_old_items: HashSet<_> = HashSet::from_iter(
                        with_new_items.difference(&to_remove).map(|&e| e.clone()),
                    );

                    let unsat_count = without_old_items.len().clone();
                    self.unsat_exprs = Some(without_old_items);
                    self.unsat_count = Some(unsat_count);
                }
            }
        }
    }
}
