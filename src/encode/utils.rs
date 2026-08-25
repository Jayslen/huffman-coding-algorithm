use crate::LeafNode;

pub fn heapify(heap: &mut Vec<LeafNode>, node_idx: usize, heap_len: usize) {
    let left = (node_idx * 2) + 1;
    let right = (node_idx * 2) + 2;
    let mut smallest = node_idx;

    if left < heap_len && heap[left].frequency < heap[smallest].frequency {
        smallest = left;
    }

    if right < heap_len && heap[right].frequency < heap[smallest].frequency {
        smallest = right;
    }

    if smallest != node_idx {
        heap.swap(node_idx, smallest);
        heapify(heap, smallest, heap_len);
    }
}

pub fn delete(heap: &mut Vec<LeafNode>, value: &String) {
    // Find index of value to be deleted
    let mut index: i32 = -1;
    for i in 0..heap.len() {
        if heap[i].char == *value {
            index = i as i32;
            break;
        }
    }

    if index == -1 {
        return;
    }

    let len = heap.len() - 1;

    heap.swap(index as usize, len);
    heap.pop();

    heapify(heap, index as usize, len);
}

// fn level_trasverse(tree: &mut LeafNode) {
//     let mut queqe: VecDeque<&LeafNode> = VecDeque::new();
//     let mut result: Vec<Vec<usize>> = Vec::new();
//     queqe.push_back(tree);
//     let mut level = 0;

//     while queqe.len() > 0 {
//         result.push(Vec::new());

//         for _ in 0..queqe.len() {
//             let curr_node = queqe.pop_front().unwrap();
//             result[level].push(curr_node.frequency);

//             if let Some(node) = curr_node.left.as_ref() {
//                 queqe.push_back(node);
//             }

//             if let Some(node) = curr_node.right.as_ref() {
//                 queqe.push_back(node);
//             }
//         }
//         level += 1;
//     }

//     println!("{:?}", result)
// }
