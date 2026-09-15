use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone)]
struct LeafNode {
    pub char: Option<char>,
    pub left: Option<Box<LeafNode>>,
    pub right: Option<Box<LeafNode>>,
}

pub fn uncompress(file: &mut File, destination_file: &mut File) -> Result<(), std::io::Error> {
    let mut header: [u8; 2] = [0; 2];
    let _ = file.read_exact(&mut header)?;

    if header[0] != 0x0A {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "File was not compressed with this tool",
        ));
    }

    let mut tree: Vec<u8> = vec![0; header[1] as usize];
    let _ = file.read_exact(&mut tree)?;

    let mut index: usize = 0;
    let root = build_tree(&tree, &mut index);

    let mut buffer: [u8; 2024] = [0; 2024];

    let mut current_leaf = &root;

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            return Ok(());
        }

        let mut i = 0;
        while i < bytes_read {
            let curr_byte = buffer[i];

            for cursor in (0..8).rev() {
                let bit = curr_byte >> cursor & 0x01;
                if let Some(c) = current_leaf.char {
                    destination_file.write_all(&[c as u8])?;
                    current_leaf = &root;
                }

                if bit == 0 {
                    current_leaf = current_leaf.left.as_ref().unwrap();
                } else {
                    current_leaf = current_leaf.right.as_ref().unwrap();
                }
            }
            i += 1;
        }
    }
}

fn build_tree(bytes: &[u8], index: &mut usize) -> LeafNode {
    let byte = bytes[*index];
    *index += 1;

    if byte != 0 {
        // Leaf
        return LeafNode {
            char: Some(byte as char),
            left: None,
            right: None,
        };
    }

    // Internal node
    let left = build_tree(bytes, index);
    let right = build_tree(bytes, index);

    LeafNode {
        char: None,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}
