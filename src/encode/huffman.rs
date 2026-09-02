use crate::LeafNode;
//
// Find better names for functions
//
pub fn build_encoded_data(tree: &LeafNode, arr: &mut Vec<String>, curr: &mut String) {
    if tree.left.is_none() && tree.right.is_none() {
        let value = curr.clone();
        arr.push(value);
    }

    if let Some(node) = tree.left.as_ref() {
        curr.push('0');
        build_encoded_data(node, arr, curr);
        curr.pop();
    }

    if let Some(node) = tree.right.as_ref() {
        curr.push('1');
        build_encoded_data(node, arr, curr);
        curr.pop();
    }
}

pub fn build_tree_bytes(tree: &LeafNode, bytes_storage: &mut Vec<u8>) {
    if tree.left.is_none() && tree.right.is_none() {
        bytes_storage.push(tree.char.as_bytes()[0]);
    } else {
        bytes_storage.push(0x0);
    }

    if let Some(node) = tree.left.as_ref() {
        build_tree_bytes(node, bytes_storage);
    }

    if let Some(node) = tree.right.as_ref() {
        build_tree_bytes(node, bytes_storage);
    }
}
