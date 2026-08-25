use std::collections::HashMap;

use crate::{LeafNode, encode::utils::heapify};

pub struct MinHeap {
    pub heap: Vec<LeafNode>,
}

impl MinHeap {
    pub fn build_heap(map: &HashMap<String, usize>) -> Self {
        let mut min_heap = MinHeap { heap: Vec::new() };

        let mut heap = &mut min_heap.heap;
        for i in map {
            heap.push(LeafNode {
                frequency: *i.1,
                char: i.0.to_string(),
                left: None,
                right: None,
            });
        }

        let len = heap.len();
        // find the last non-leaf node
        let mut i = (len / 2) as isize - 1;

        while i >= 0 {
            heapify(&mut heap, i as usize, len);
            if i == 0 {
                break;
            }
            i -= 1;
        }
        min_heap
    }
}
