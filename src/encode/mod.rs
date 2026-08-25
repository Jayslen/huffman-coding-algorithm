mod heap;
mod tree;
mod utils;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

use crate::encode::heap::MinHeap;
use crate::encode::tree::{build_tree, transverse};

pub fn compress(file_path: &str, file: &mut File) {
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
            Err(_) => println!("Error while reading chunk as UTF-8 string"),
        }
    }

    let mut prefixes: Vec<String> = Vec::new();
    let mut curr_prefix = String::new();

    let heap = MinHeap::build_heap(&hash_map);
    // here we moved the the value of heap into this variable
    // i decided to do this beacues it makes sense since we are builind a tree from the min-heap

    let mut tree = build_tree(heap.heap);

    transverse(&mut tree[0], &mut prefixes, &mut curr_prefix);
    //println!("{:#?}", tree);
    println!("Prefixes: {:#?}", prefixes);
}
