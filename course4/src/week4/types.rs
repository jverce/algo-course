use num::abs;

/// Wrapper that represents operations on a set of bits.
pub trait BitSet {
    fn new(n_bits: usize) -> Self;
    fn is_set(&self, i: usize) -> bool;
    fn set(&mut self, i: usize) -> ();
    fn clear(&mut self, i: usize) -> ();
}

type BitString = Vec<bool>;

/// Data type to use to refer to a bounded collection of bits.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    fn set(&mut self, i: usize) -> () {
        if i <= self.n_bits {
            self.bits[i - 1] = true;
        } else {
            panic!(
                "Index out of range: attempted to set bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }

    fn clear(&mut self, i: usize) -> () {
        if i <= self.n_bits {
            self.bits[i - 1] = false;
        } else {
            panic!(
                "Index out of range: attempted to clear bit {} of a bit vector of size {}",
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
#[derive(Debug)]
pub struct ExprVal {
    expr: i64,
}

/// This struct represents the evaluation of 2 single expressions, or'ed together.
#[derive(Debug)]
pub struct ExprTerm {
    expr: (ExprVal, ExprVal),
}

/// This struct represents the evaluation of multiple expression terms, and'ed together.
#[derive(Debug)]
pub struct ExprFull {
    expr: Vec<ExprTerm>,
}

/// Represents a logical expression that can be evaluated against an input bit vector.
pub trait Evaluable<T> {
    fn new(input: T) -> Self;
    fn eval(&self, input: &BitVec) -> bool;
}

impl Evaluable<i64> for ExprVal {
    fn new(input: i64) -> Self {
        ExprVal { expr: input }
    }

    fn eval(&self, input: &BitVec) -> bool {
        let index = abs(self.expr) as usize;
        (self.expr < 0) ^ input.is_set(index)
    }
}

impl Evaluable<&Vec<i64>> for ExprTerm {
    fn new(input: &Vec<i64>) -> Self {
        let expr = (ExprVal::new(input[0]), ExprVal::new(input[1]));
        ExprTerm { expr }
    }

    fn eval(&self, input: &BitVec) -> bool {
        let (a, b) = &self.expr;
        a.eval(input) | b.eval(input)
    }
}

impl Evaluable<&Vec<Vec<i64>>> for ExprFull {
    fn new(input: &Vec<Vec<i64>>) -> Self {
        let expr = input.iter().map(|i| ExprTerm::new(i)).collect();
        ExprFull { expr }
    }

    fn eval(&self, input: &BitVec) -> bool {
        let result = self.expr.iter().find(|e| !e.eval(input));
        result.is_none()
    }
}
