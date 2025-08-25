//! Here are all the functions that use combinatorics to calculate all numbers that have digits sum of 13

pub struct Combinatorics;

impl crate::traits::SumSequencer for Combinatorics {
    fn get_ints(&self) -> impl Iterator<Item = u64> + use<> {
        todo!() as <Vec<_> as IntoIterator>::IntoIter
    }
}

// The realization is [Jerome Kelleher](https://jeromekelleher.net/generating-integer-partitions.html)'s one

/// Right now it's experimental and useless since it's not digit-only.
struct PartitionIterator {
    buffer: Vec<u64>,
    number: u64,
    k: usize,
}

impl PartitionIterator {
    fn new(number: u64) -> Self {
        let mut val = Self {
            buffer: vec![0; number as usize + 1],
            number,
            k: 1,
        };

        val.buffer[1] = val.number;

        val
    }
}

impl Iterator for PartitionIterator {
    type Item = Vec<u64>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k == 0 {
            return None;
        }

        let x = self.buffer[self.k - 1] + 1;
        let mut y = self.buffer[self.k] - 1;

        self.k -= 1;

        while x <= y {
            self.buffer[self.k] = x;
            y -= x;
            self.k += 1
        }

        self.buffer[self.k] = x + y;

        Some(self.buffer[..self.k + 1].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::combinatorics::PartitionIterator;

    #[test]
    fn test_partition_iterator() {
        let iter = PartitionIterator::new(4);
        let mut expected_result = HashSet::new();
        let result = iter.map(|val| dbg!(val)).collect::<HashSet<_>>();

        expected_result.insert(vec![1, 1, 1, 1]);
        expected_result.insert(vec![1, 1, 2]);
        expected_result.insert(vec![2, 2]);
        expected_result.insert(vec![1, 3]);
        expected_result.insert(vec![4]);

        assert_eq!(result, expected_result);
    }
}
