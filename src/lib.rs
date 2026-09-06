#[derive(Debug, Clone)]
pub struct LeafNode {
    frequency: usize,
    char: String,
    left: Option<Box<LeafNode>>,
    right: Option<Box<LeafNode>>,
}

pub type MinHeap = Vec<LeafNode>;
pub type Tree = LeafNode;

pub mod decode;
pub mod encode;

// #[cfg(test)]
// mod test {
//     use std::collections::HashMap;

//     use super::*;

//     #[test]
//     fn min_heap_root() {
//         let mut d = encode::MinHeap::new();
//         d.build_heap(&HashMap::from([
//             ("a".to_string(), 5),
//             ("b".to_string(), 3),
//             ("c".to_string(), 2),
//         ]));

//         let root = &d.heap[0];
//         assert_eq!(d.heap.len(), 3);

//         assert_eq!(root.char, "c");
//         assert_eq!(root.frequency, 2);
//     }
// }
