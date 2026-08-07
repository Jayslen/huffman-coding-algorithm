use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Read;

//enum node_value

#[derive(Debug)]
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
    let heap: Vec<LeafNode> = Vec::new();
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
    build_heap(&hash_map);
    //println!("{:?}", hash_map);
}
