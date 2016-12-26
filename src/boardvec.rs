const ARRAY_SIZE: usize = 2;
const U64_LOG: usize = 6;

/// Fixed length bit vector used to store board positions
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct BoardVec {
    array: [u64; ARRAY_SIZE],
}

impl BoardVec {
    pub fn new() -> Self {
        BoardVec { array: [0; ARRAY_SIZE] }
    }

    pub fn size() -> usize {
        ARRAY_SIZE << U64_LOG
    }

    pub fn set(&mut self, mut idx: usize) {
        let self_index = idx >> U64_LOG;
        idx -= self_index << U64_LOG;
        self.array[self_index] = self.array[self_index] | (1 << idx);
    }

    pub fn get(&self, mut idx: usize) -> bool {
        let self_index = idx >> U64_LOG;
        idx -= self_index << U64_LOG;
        (self.array[self_index] & (1 << idx)) != 0
    }

    /// Whether this vector has any 1 bits in common with `other`
    pub fn intersects(&self, other: &BoardVec) -> bool {
        for i in 0..ARRAY_SIZE {
            if (self.array[i] & other.array[i]) != 0 {
                return true;
            }
        }
        false
    }

    /// Whether all bits set to 1 in `other` are also 1 in this vector
    pub fn contains(&self, other: &BoardVec) -> bool {
        for i in 0..ARRAY_SIZE {
            if (self.array[i] & other.array[i]) != other.array[i] {
                return false;
            }
        }
        true
    }
}
