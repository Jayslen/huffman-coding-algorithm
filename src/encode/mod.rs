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

        //        print!("\rRead {} of {:?}", total, size);

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
    prepare_result(&prefixes);
}

fn prepare_result(result: &Vec<String>) {
    let mut bytes: Vec<u8> = Vec::new();
    let codes = result.clone().join("");

    println!("codes: {}", codes);
    let mut codes_iter = codes.chars();

    let bytes_need = codes.len() / 8 + if codes.len() % 8 > 0 { 1 } else { 0 };
    println!("\n");
    for _ in 0..bytes_need {
        let mut pack: u8 = 0;
        for j in (0..8).rev() {
            // println!("j: {}", j);
            // println!("pack {:08b}", pack);
            if let Some(value) = codes_iter.next() {
                // println!("value: {}", value);
                let mut byte = value.to_string().parse::<u8>().unwrap();
                byte = byte << j;
                // println!("c: {:08b}", c);
                pack |= byte;
                //println!("pack: {:08b}", pack);
            } else {
                break;
            }
        }
        bytes.push(pack);
    }

    fs::write("output", &bytes).expect("Unable to write file");
    bytes.iter().for_each(|f| println!("{:08b}", f));
}
