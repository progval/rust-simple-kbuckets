use std::fmt::Debug;

/// Trait that table keys must implement.
pub trait Key: Debug + Clone + Ord + PartialEq {
    fn bitxor(&self, other: &Self) -> Self;
    fn is_zero(&self) -> bool;
    fn bits(&self) -> usize;
}

impl Key for u64 {
    fn bitxor(&self, other: &u64) -> u64 {
        self ^ other
    }
    fn is_zero(&self) -> bool {
        *self == 0
    }
    fn bits(&self) -> usize {
        (64 - self.leading_zeros()) as usize
    }
}

impl Key for Vec<u8> {
    fn bitxor(&self, other: &Vec<u8>) -> Vec<u8> {
        self.iter().zip(other.iter()).map(|(digit1, digit2)| digit1 ^ digit2).collect()
    }
    fn is_zero(&self) -> bool {
        self.iter().all(|digit| *digit == 0)
    }
    fn bits(&self) -> usize {
        let mut bits = self.len()*8;
        for digit in self {
            if *digit == 0 {
                bits -= 8;
            }
            else {
                return bits - digit.leading_zeros() as usize
            }
        }
        assert!(bits == 0);
        0
    }
}
