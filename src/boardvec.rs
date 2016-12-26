const U64_LOG: usize = 6;

/// Fixed length bit vector used to store board positions
#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct BoardVec {
    word0: u64,
    word1: u64
}

impl BoardVec {
    pub fn new() -> Self {
        BoardVec { word0: 0, word1: 0 }
    }

    pub fn size() -> usize {
        2 << U64_LOG
    }

    pub fn set(&mut self, idx: usize) {
        if idx < 64 {
            self.word0 |= 1 << idx;
        } else {
            self.word1 |= 1 << (idx - 64);
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        if idx < 64 {
            (self.word0 & (1 << idx)) != 0
        } else {
            (self.word1 & (1 << (idx - 64))) != 0
        }
    }

    /// Whether this vector has any 1 bits in common with `other`
    pub fn intersects(&self, other: &BoardVec) -> bool {
        ((self.word0 & other.word0) != 0) || ((self.word1 & other.word1) != 0)
    }

    /// Whether all bits set to 1 in `other` are also 1 in this vector
    pub fn contains(&self, other: &BoardVec) -> bool {
        ((self.word0 & other.word0) == other.word0) && ((self.word1 & other.word1) == other.word1)
    }
}
