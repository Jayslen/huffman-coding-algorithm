use std::collections::HashMap;

use crate::LeafNode;
//
// Find better names for functions
//
pub fn huffman_codes(tree: &LeafNode, arr: &mut HashMap<String, String>, curr: &mut String) {
    if tree.left.is_none() && tree.right.is_none() {
        let mut value = curr.clone();
        if value.is_empty() {
            value = "0".to_string();
        }
        arr.insert(tree.char.to_string(), value);
    }

    if let Some(node) = tree.left.as_ref() {
        curr.push('0');
        huffman_codes(node, arr, curr);
        curr.pop();
    }

    if let Some(node) = tree.right.as_ref() {
        curr.push('1');
        huffman_codes(node, arr, curr);
        curr.pop();
    }
}

pub fn build_encode_tree_ouput(tree: &LeafNode, bytes_storage: &mut Vec<u8>) {
    if tree.left.is_none() && tree.right.is_none() {
        bytes_storage.push(tree.char.as_bytes()[0]);
    } else {
        bytes_storage.push(0x0);
    }

    if let Some(node) = tree.left.as_ref() {
        build_encode_tree_ouput(node, bytes_storage);
    }

    if let Some(node) = tree.right.as_ref() {
        build_encode_tree_ouput(node, bytes_storage);
    }
}
