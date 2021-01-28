use std::env;
use std::process::exit;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

const SUCCESS: i32 = 0;
const ERR_WRONG_AMOUNT_ARGS: i32 = 1;
const ERR_UNKNOWN_MODE: i32 = 2;
const ERR_FILE_NOT_FOUND: i32 = 3;
const ERR_FILE_IS_DIR: i32 = 4;
const ERR_FILE_OPEN: i32 = 5;
const ERR_READ_FILE: i32 = 6;

const BUF_SIZE: usize = 256;

enum Mode {
    Bin,
    Oct,
    Hex,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        usage();
        exit(SUCCESS);
    }

    /* Wrong amount of Args -> Print Usage */
    if args.len() != 3 {
        usage();
        exit(ERR_WRONG_AMOUNT_ARGS);
    }

    let mode = match args.get(1).unwrap().as_str() {
        "bin" => Mode::Bin,
        "oct" => Mode::Oct,
        "hex" => Mode::Hex,
        arg => {
            println!("Unknown mode: {}", arg);
            usage();
            exit(ERR_UNKNOWN_MODE);
        }
    };

    let path = PathBuf::from(args.get(2).unwrap());

    /* File not Found */
    if !path.exists() {
        println!("File not found: {}", args.get(2).unwrap());
        exit(ERR_FILE_NOT_FOUND);
    }

    /* Path is Directory */
    if path.is_dir() {
        println!("Path is a directory!");
        exit(ERR_FILE_IS_DIR);
    }

    dump(&path, mode);

    exit(SUCCESS);
}

fn dump(path: &PathBuf, mode: Mode) {
    let mut reader = match File::open(path) {
        Ok(file) => {
            BufReader::with_capacity(BUF_SIZE, file)
        }
        Err(err) => {
            println!("Error opening File: {}", err);
            exit(ERR_FILE_OPEN);
        }
    };

    let mut counter: u64 = 0;
    let col = match &mode {
        Mode::Bin => 4,
        Mode::Oct => 8,
        Mode::Hex => 16,
    };

    loop {
        let buffer = match reader.fill_buf() {
            Ok(buf) => {
                buf
            }
            Err(err) => {
                println!("Error reading file: {}", err);
                exit(ERR_READ_FILE);
            }
        };

        if buffer.is_empty() {
            print!("\n");
            return;
        }

        for byte in buffer {
            if counter % col == 0 {
                print!("\n{:08x}  ", counter);
            }

            match &mode {
                Mode::Bin => print!("{:08b}  ", byte),
                Mode::Oct => print!("{:03o}  ", byte),
                Mode::Hex => print!("{:02x}  ", byte),
            }

            counter += 1;
        }

        // Consume read bytes
        let length = buffer.len();
        reader.consume(length);
    }
}

fn usage() {
    println!("Usage: dump [bin|oct|hex] FILE");
}
