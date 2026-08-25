use crate::{LeafNode, encode::utils::delete};

pub fn build_tree(mut heap: Vec<LeafNode>) -> Vec<LeafNode> {
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
    heap
}

pub fn transverse(tree: &LeafNode, arr: &mut Vec<String>, curr: &mut String) {
    if tree.left.is_none() && tree.right.is_none() {
        let value = curr.clone();
        arr.push(value);
    }

    if let Some(node) = tree.left.as_ref() {
        curr.push('0');
        transverse(node, arr, curr);
        curr.pop();
    }

    if let Some(node) = tree.right.as_ref() {
        curr.push('1');
        transverse(node, arr, curr);
        curr.pop();
    }
}
