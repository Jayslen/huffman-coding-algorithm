use std::env;
use std::io::{BufRead, Read};
use std::{fs, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut file = fs::File::open(file_path).unwrap();

    let mut buffer: [u8; 2024] = [0; 2024];

    loop {
        let bytes_read = file.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let chunk = &buffer[..bytes_read];

        match std::str::from_utf8(chunk) {
            Ok(text) => print!("{}", text),
            Err(_) => println!("{:?}", chunk),
        }
        break;
    }
}
