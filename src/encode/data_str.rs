use std::collections::HashMap;

use crate::{
    LeafNode, MinHeap, Tree,
    encode::utils::{delete, heapify},
};

pub fn build_heap(map: &HashMap<String, usize>) -> MinHeap {
    let mut heap: MinHeap = Vec::new();

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
    heap
}

pub fn build_tree(mut heap: MinHeap) -> Tree {
    while heap.len() > 1 {
        //println!("Current Heap{:#?}", heap);
        let root = heap[0].clone();
        let smallest = if heap.len() == 2 {
            1
        } else if &heap[1].frequency < &heap[2].frequency {
            1
        } else {
            2
        };

        let second = heap[smallest].clone();

        // println!("Selected:");
        // println!("{:#?}", root);
        // println!("{:#?}", second);

        delete(&mut heap, &root.char);
        delete(&mut heap, &second.char);

        let internal_node = LeafNode {
            frequency: root.frequency + second.frequency,
            char: format!("{}{}", root.char, second.char),
            left: Some(Box::new(root)),
            right: Some(Box::new(second)),
        };

        heap.push(internal_node);
    }
    let tree = heap[0].clone();
    drop(heap);
    tree
}
