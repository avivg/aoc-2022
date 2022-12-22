use std::ops::Index;

struct Node {
    val: i64,
    next: usize,
    prev: usize,
}

pub struct SwapVec {
    nodes: Vec<Node>,
}

impl SwapVec {
    pub fn new(nums: impl Iterator<Item = i64>) -> Self {
        let mut sv = Self {
            nodes: nums
                .map(|n| Node {
                    val: n,
                    next: 0,
                    prev: 0,
                })
                .collect(),
        };
        let len = sv.nodes.len();
        for i in 0..len {
            sv.nodes[i].next = (i + 1) % len;
            sv.nodes[i].prev = (i + len - 1) % len;
        }
        sv
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    fn contains(&self, num: i64) -> bool {
        self.nodes.iter().any(|n| n.val == num)
    }

    // Swaps the element in original order 'idx' with the element pointed by nodes[idx].next
    pub fn swap_with_next(&mut self, idx: usize) {
        let next = self.nodes[idx].next;
        self.remove(idx);
        self.insert_after(next, idx);
    }

    pub fn swap_with_prev(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        self.swap_with_next(prev);
    }

    pub fn mixed_iter<'a>(&'a self) -> impl Iterator<Item = i64> + 'a {
        SwapVecIter::from(self)
    }

    fn remove(&mut self, idx: usize) {
        let prev = self.nodes[idx].prev;
        let next = self.nodes[idx].next;

        self.nodes[next].prev = prev;
        self.nodes[prev].next = next;
    }

    fn insert_after(&mut self, after_idx: usize, inserted_idx: usize) {
        let next = self.nodes[after_idx].next;

        // Set new inserted node's next and prev
        self.nodes[inserted_idx].next = next;
        self.nodes[inserted_idx].prev = after_idx;

        // Fix existing nodes' pointers
        self.nodes[after_idx].next = inserted_idx;
        self.nodes[next].prev = inserted_idx;
    }
}

struct SwapVecIter<'a> {
    cur: usize,
    sv: &'a SwapVec,
}

impl<'a> From<&'a SwapVec> for SwapVecIter<'a> {
    fn from(swapvec: &'a SwapVec) -> Self {
        Self {
            cur: 0,
            sv: swapvec,
        }
    }
}

impl<'a> Iterator for SwapVecIter<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let cur_node = &self.sv.nodes[self.cur];
        self.cur = cur_node.next;
        Some(cur_node.val)
    }
}

impl Index<usize> for SwapVec {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index].val
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swapvec_from_ended_iterator_should_be_created_empty() {
        let sv = SwapVec::new([].into_iter());
        assert_eq!(0, sv.len());
    }

    #[test]
    fn swapvec_from_iterator_with_items_should_contain_the_items() {
        let sv = SwapVec::new(1..=3);
        assert_eq!(3, sv.len());
        assert!(sv.contains(1));
        assert!(sv.contains(2));
        assert!(sv.contains(3));
        assert!(!sv.contains(4));
    }

    #[test]
    fn swapvec_iterator_should_be_cyclical() {
        let sv = SwapVec::new(1..=3);
        let mut miter = sv.mixed_iter();
        for i in 0..10 {
            assert_eq!(Some(i % 3 + 1), miter.next());
        }
    }

    #[test]
    fn swapvec_should_allow_swapping_next_adjacent_cell() {
        let mut sv = SwapVec::new(1..=3);
        sv.swap_with_next(0);
        let mut miter = sv.mixed_iter();
        assert_eq!(Some(1), miter.next());
        assert_eq!(Some(3), miter.next());
        assert_eq!(Some(2), miter.next());
        assert_eq!(Some(1), miter.next());
    }

    #[test]
    fn swapvec_should_allow_swapping_prev_adjacent_cell() {
        let mut sv = SwapVec::new(1..=3);
        sv.swap_with_prev(1);
        let mut miter = sv.mixed_iter();
        assert_eq!(Some(1), miter.next());
        assert_eq!(Some(3), miter.next());
        assert_eq!(Some(2), miter.next());
        assert_eq!(Some(1), miter.next());
    }

    #[test]
    fn swapvec_should_be_indexed_using_original_order() {
        let mut sv = SwapVec::new(1..=10);
        sv.swap_with_next(3);
        sv.swap_with_prev(7);
        for i in 1..=10 {
            assert_eq!(i as i64, sv[i - 1]);
        }
    }

    #[test]
    fn verify_forward_wraparound() {
        let mut sv1 = SwapVec::new(0..7);
        let mut sv2 = SwapVec::new(0..7);

        for _ in 0..3 {
            sv1.swap_with_next(2);
        }
        let sv1res = sv1.mixed_iter().take(7).collect::<Vec<_>>();

        let m = 3 + (6 * 21);
        for _ in 0..m {
            sv2.swap_with_next(2);
        }
        assert_eq!(sv1res, sv2.mixed_iter().take(7).collect::<Vec<_>>());
    }

    #[test]
    fn verify_backward_wraparound() {
        let mut sv1 = SwapVec::new(0..7);
        let mut sv2 = SwapVec::new(0..7);

        for _ in 0..3 {
            sv1.swap_with_prev(5);
        }
        let sv1res = sv1.mixed_iter().take(7).collect::<Vec<_>>();

        let m = 3 + (6 * 21);
        for _ in 0..m {
            sv2.swap_with_prev(5);
        }
        assert_eq!(sv1res, sv2.mixed_iter().take(7).collect::<Vec<_>>());
    }
}
