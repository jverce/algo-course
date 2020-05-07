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
        if i < self.n_bits {
            self.bits[i]
        } else {
            panic!(
                "Overflow: attempted to read bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }

    fn set(&mut self, i: usize) -> () {
        if i < self.n_bits {
            self.bits[i] = true;
        } else {
            panic!(
                "Overflow: attempted to set bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }

    fn clear(&mut self, i: usize) -> () {
        if i < self.n_bits {
            self.bits[i] = false;
        } else {
            panic!(
                "Overflow: attempted to clear bit {} of a bit vector of size {}",
                i, self.n_bits
            );
        }
    }
}
