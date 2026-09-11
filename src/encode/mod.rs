mod data_str;
mod huffman;
mod utils;

use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::os::unix::fs::MetadataExt;
use std::{collections::HashMap, io::Seek};

use crate::encode::{
    data_str::{build_heap, build_tree},
    huffman::{build_encode_tree_ouput, huffman_codes},
};

pub fn compress(file_path: &str, file: &mut File) {
    let mut hash_map: HashMap<String, usize> = HashMap::new();

    let mut buffer: [u8; 500_024] = [0; 500_024];

    // let size = fs::metadata(file_path).unwrap().size();
    // let mut total: usize = 0;

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }

        // total += bytes_read;
        let chunk = &buffer[..bytes_read];

        // print!("\rRead {} of {:?}", total, size);

        // std::io::Write::flush(&mut std::io::stdout()).unwrap();

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

    file.seek(std::io::SeekFrom::Start(0))
        .expect("Unable to seek file");

    println!("\n\nEncoding file...");

    let mut curr_byte: u8 = 0x0;
    let mut curr_count: u8 = 7;

    let mut result: Vec<u8> = Vec::from([tree_encoded.len() as u8, 0x0, 0x0]);
    result.extend(tree_encoded);

    let mut output_file = fs::File::create_new("./output2").unwrap();
    output_file.write(&result).expect("Unable to write file");

    drop(result);

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
                    let code = prefixes.get(&value).unwrap();
                    code.chars().for_each(|c| {
                        curr_byte |= c.to_string().parse::<u8>().unwrap() << curr_count;
                        if curr_count == 0 {
                            output_file
                                .write(&[curr_byte])
                                .expect("Unable to write file");
                            curr_byte = 0x0;
                            curr_count = 7;
                        } else {
                            curr_count -= 1;
                        }
                    });
                    // println!("{:}", code);
                    // Verify if code is more or less than 8 bits, if it is more than 8 bits, we need to split it into multiple bytes if not, we must look for the next character and append it to the current byte until we have 8 bits, then we can push it to the output vector.
                }
            }
            Err(_) => println!("Error while reading chunk as UTF-8 string"),
        }
    }
}
