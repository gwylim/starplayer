const ARRAY_SIZE: usize = 2;
const U64_LOG: usize = 6;

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
}
