mod data_str;
mod huffman;
mod utils;

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::os::unix::fs::MetadataExt;

use crate::encode::{
    data_str::{build_heap, build_tree},
    huffman::{build_encode_tree_ouput, huffman_codes},
};

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

    let heap = build_heap(&hash_map);
    // Here we moved the the value of heap into this variable
    // I decided to do this beacues it makes sense since we are builind a tree from the min-heap
    let tree = build_tree(heap);
    // println!("{:#?}", tree);
    //println!("\n\nTree: {:#?}", tree);
    let mut prefixes: HashMap<String, String> = HashMap::new();
    let mut curr_prefix = String::new();

    huffman_codes(&tree, &mut prefixes, &mut curr_prefix);
    drop(curr_prefix);
    // println!("{:#?}", prefixes);
    //println!("\n\nPrefixes: {:?}", prefixes);
    let mut tree_encoded: Vec<u8> = Vec::new();
    build_encode_tree_ouput(&tree, &mut tree_encoded);
    // println!("{:#?}", prefixes);
    // println!("{:#?}", hash_map);

    // This need to be optimized because it reads the entire file again.
    // It just here to test the output.
    // Later it will be improved.
    let original_input = fs::read_to_string(file_path).expect("Unable to read file");
    let output: Vec<String> = original_input
        .chars()
        .map(|c| prefixes.get(&c.to_string()).unwrap().to_string())
        .collect();

    let mut padding_bits: u8 = 0;
    let encoded_data = pack_encoded_output(&output, &mut padding_bits);

    let mut result: Vec<u8> = Vec::from([
        tree_encoded.len() as u8,
        encoded_data.len() as u8,
        padding_bits,
    ]);

    for b in tree_encoded.iter() {
        result.push(*b);
    }

    for b in encoded_data.iter() {
        result.push(*b);
    }
    fs::write("output", &result).expect("Unable to write file");
}

fn pack_encoded_output(result: &Vec<String>, padding_bits: &mut u8) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    let codes = result.clone().join("");

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
                *padding_bits = 8 - j as u8;
                break;
            }
        }
        bytes.push(pack);
    }

    bytes
}
