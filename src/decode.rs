use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
struct LeafNode {
    pub char: char,
    pub left: Option<Box<LeafNode>>,
    pub right: Option<Box<LeafNode>>,
}

pub fn uncompress(file: &mut File) {
    let mut sizes: [u8; 3] = [0; 3];
    file.read_exact(&mut sizes);

    println!("Size of tree: {}", sizes[0]);
    println!("Size of data: {}", sizes[1]);
    println!("Size of padding: {}", sizes[2]);
    let has_padding = sizes[2] > 0;

    let mut tree: Vec<u8> = vec![0; sizes[0] as usize];

    file.read_exact(&mut tree);

    let mut index: usize = 0;
    let root = build_tree(&tree, &mut index);

    // println!("{:#?}", root);

    let mut encoded_data: Vec<u8> = Vec::new();

    file.read_to_end(&mut encoded_data).unwrap();
    // // println!("Encoded Data: {:?}", encoded_data);
    // // println!("Tree: {:?}", tree_e);

    // // let mut curr_byte = encoded_data[0];

    // // println!("Current Byte: {:08b}", curr_byte);

    // // curr_byte = curr_byte >> 6 & 0x01;

    // // println!("Current Byte: {:08b}", curr_byte);
    let data: String = String::new();
    let mut i = 0;
    let mut current_leaf = &root;
    //println!("Current Leaf: {:#?}", root);

    // let mut first = encoded_data[0].clone();
    // let mask = 0b11111111;
    // println!("{:08b}", first);
    // first = first >> 7 & 0x01;
    // println!("{:}", first);
    // println!("{:#?}", root);
    //
    while i < encoded_data.len() {
        let is_last_byte = i + 1 == encoded_data.len();
        let curr_byte = encoded_data[i];

        for cursor in (0..8).rev() {
            if has_padding && is_last_byte && cursor < (sizes[2] as usize) {
                break;
            }
            let bit = curr_byte >> cursor & 0x01;
            // println!("{}", bit);
            if current_leaf.char != '#' {
                print!("{:}", current_leaf.char);
                current_leaf = &root;
            }
            // println!("bit: {:08b}", bit);

            if bit == 0 {
                current_leaf = current_leaf.left.as_ref().unwrap();
            } else {
                current_leaf = current_leaf.right.as_ref().unwrap();
            }
        }
        i += 1;
    }
}

fn build_tree(bytes: &[u8], index: &mut usize) -> LeafNode {
    let byte = bytes[*index];
    *index += 1;

    if byte != 0 {
        // Leaf
        return LeafNode {
            char: (byte as char),
            left: None,
            right: None,
        };
    }

    // Internal node
    let left = build_tree(bytes, index);
    let right = build_tree(bytes, index);

    LeafNode {
        char: '#',
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    }
}
