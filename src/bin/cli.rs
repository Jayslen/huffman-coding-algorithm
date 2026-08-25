use compression::encode;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    // let file_path = "/home/jayslen/Development/huffman-coding/text.txt";
    let mut file = fs::File::open(file_path).unwrap();

    encode::compress(file_path, &mut file);
}
