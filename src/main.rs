use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

//enum node_value

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

    let mut hash_map: HashMap<String, usize> = HashMap::new();
    let mut heap: Vec<LeafNode> = Vec::new();
    let mut buffer: [u8; 2024] = [0; 2024];

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let chunk = &buffer[..bytes_read];

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
    build_heap(&hash_map, &mut heap);
    build_tree(&mut heap);
    println!("{:#?}", heap);
}

fn build_heap(map: &HashMap<String, usize>, heap: &mut Vec<LeafNode>) {
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
        heapify(heap, i as usize, len);
        if i == 0 {
            break;
        }
        i -= 1;
    }
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

fn build_tree(heap: &mut Vec<LeafNode>) {
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

        delete(heap, &root.char);
        delete(heap, &second.char);

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
}
