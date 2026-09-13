use compression::{decode, encode};
use std::{
    env::{self, Args},
    fs::{self, File},
    io::ErrorKind,
};

enum Action {
    Compress,
    Decompress,
    None,
}

struct CLI {
    pattern: Action,
    file_path: std::path::PathBuf,
    destination_file: Option<std::path::PathBuf>,
}

impl Action {
    fn from_str(s: &str) -> Action {
        match s {
            "compress" => Action::Compress,
            "decompress" => Action::Decompress,
            _ => Action::None,
        }
    }
}

impl CLI {
    fn new(args: Args) -> Self {
        let args: Vec<String> = args.collect();
        if args.len() < 3 {
            eprintln!("Must use: {} <compress|decompress> <file_path>", args[0]);
            std::process::exit(1);
        }

        let pattern = Action::from_str(&args[1]);
        let file_path = std::path::PathBuf::from(&args[2]);

        let destination_file = if args.len() > 3 {
            Some(std::path::PathBuf::from(&args[3]))
        } else {
            None
        };

        CLI {
            pattern,
            file_path,
            destination_file,
        }
    }
}

fn main() {
    let cli = CLI::new(env::args());
    match cli.pattern {
        Action::Compress => {
            let mut file = fs::File::open(&cli.file_path).unwrap();

            let (mut destination_file, destination) = create_destination_file(&cli);
            match encode::compress(&mut file, &mut destination_file) {
                Ok(_) => println!(
                    "File compressed successfully to '{}'",
                    destination.display()
                ),
                Err(err) => {
                    eprintln!("Error during compression: {}", err);
                    remove_file(&destination);
                }
            }
        }
        Action::Decompress => {
            let mut file = fs::File::open(&cli.file_path).unwrap();
            let (mut destination_file, path) = create_destination_file(&cli);
            match decode::uncompress(&mut file, &mut destination_file) {
                Ok(_) => println!("File decompressed successfully to '{}'", path.display()),
                Err(err) => {
                    eprintln!("Error during decompression: {}", err);
                    remove_file(&path);
                }
            }
            // Decode module not ready
        }
        Action::None => {
            println!("Invalid action. Use 'compress' or 'decompress'.");
        }
    }
}

fn remove_file(path: &std::path::PathBuf) {
    fs::remove_file(path).unwrap();
}

fn create_destination_file(cli: &CLI) -> (File, std::path::PathBuf) {
    let file_extension = match &cli.pattern {
        &Action::Compress => "huffman",
        &Action::Decompress => "txt",
        _ => "",
    };

    let destination = match &cli.destination_file {
        Some(path) => path.clone(),
        None => {
            let mut default_path = cli.file_path.clone();
            default_path.set_extension(file_extension);
            default_path
        }
    };

    let file =fs::File::create_new(&destination).unwrap_or_else(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            eprintln!("Destination file '{}' already exists. Please specify a different destination file.",
                destination.display());
            std::process::exit(1);
        } else {
            panic!("Failed to create destination file");
        }});

    return (file, destination);
}
