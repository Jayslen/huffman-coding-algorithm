use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

use std::collections::VecDeque;
use std::os::unix::fs::MetadataExt;

#[derive(Debug, Clone)]
struct LeafNode {
    frequency: usize,
    char: String,
    left: Option<Box<LeafNode>>,
    right: Option<Box<LeafNode>>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut file = fs::File::open(file_path).unwrap();

    // 1. Hashmap to track chars' frecuency
    let mut hash_map: HashMap<String, usize> = HashMap::new();

    let mut buffer: [u8; 20480] = [0; 20480];

    let size = fs::metadata(file_path).unwrap().size();
    let mut total: usize = 0;

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        total += bytes_read;
        let chunk = &buffer[..bytes_read];

        print!("\rRead {} of {:?}", total, size);

        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        match std::str::from_utf8(chunk) {
            Ok(text) => {
                let s: Vec<_> = text.split("").collect();
                for char in s {
                    if char.len() == 0 {
                        continue;
                    }
                    let value = char.to_string();
                    if let Some(v) = hash_map.get(&value) {
                        hash_map.insert(value, v + 1);
                    } else {
                        hash_map.insert(value, 1);
                    }
                }
            }
            Err(_) => println!("{:?}", chunk),
        }
    }

    let mut prefixes: Vec<String> = Vec::new();
    let mut curr_prefix = String::new();
    let heap = build_heap(&hash_map);

    // here we moved the the value of heap into this variable
    // i decided to do this beacues it makes sense since we are builind a tree from the min-heap
    let mut tree = build_tree(heap);

    transverse(&mut tree[0], &mut prefixes, &mut curr_prefix);
}

fn build_heap(map: &HashMap<String, usize>) -> Vec<LeafNode> {
    let mut heap: Vec<LeafNode> = Vec::new();

    for i in map {
        heap.push(LeafNode {
            frequency: *i.1,
            char: i.0.to_string(),
            left: None,
            right: None,
        });
    }

    let len = heap.len();
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

fn heapify(heap: &mut Vec<LeafNode>, node_idx: usize, n: usize) {
    let left = (node_idx * 2) + 1;
    let right = (node_idx * 2) + 2;
    let mut smallest = node_idx;

    if left < n && heap[left].frequency < heap[smallest].frequency {
        smallest = left;
    }

    if right < n && heap[right].frequency < heap[smallest].frequency {
        smallest = right;
    }

    if smallest != node_idx {
        heap.swap(node_idx, smallest);
        heapify(heap, smallest, n);
    }
}

fn build_tree(mut heap: Vec<LeafNode>) -> Vec<LeafNode> {
    while heap.len() > 1 {
        let root = heap[0].clone();
        let smallest = if heap.len() == 2 {
            1
        } else if &heap[(0 * 2) + 1].frequency > &heap[(0 * 2) + 2].frequency {
            (0 * 2) + 1
        } else {
            (0 * 2) + 2
        };

        let second = heap[smallest].clone();

        delete(&mut heap, &root.char);
        delete(&mut heap, &second.char);

        let internal_node = LeafNode {
            frequency: root.frequency + second.frequency,
            char: format!("{}{}", root.char, second.char),
            left: Some(Box::new(root)),
            right: Some(Box::new(second)),
        };

        heap.push(internal_node);
        // Loop
        // Identify first 2
        // Sume them and add them to the heap
        // Merge them into a new node and add it to the heap
        // Remove the first 2 nodes from the heap
        // Repeat until there is only 1 node left in the heap
        //
    }
    heap
}

fn delete(heap: &mut Vec<LeafNode>, value: &String) {
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

fn level_trasverse(tree: &mut LeafNode) {
    let mut queqe: VecDeque<&LeafNode> = VecDeque::new();
    let mut result: Vec<Vec<usize>> = Vec::new();
    queqe.push_back(tree);
    let mut level = 0;

    while queqe.len() > 0 {
        result.push(Vec::new());

        for _ in 0..queqe.len() {
            let curr_node = queqe.pop_front().unwrap();
            result[level].push(curr_node.frequency);

            if let Some(node) = curr_node.left.as_ref() {
                queqe.push_back(node);
            }

            if let Some(node) = curr_node.right.as_ref() {
                queqe.push_back(node);
            }
        }
        level += 1;
    }

    println!("{:?}", result)
}

fn transverse(tree: &LeafNode, arr: &mut Vec<String>, curr: &mut String) {
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
