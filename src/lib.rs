use std::{error::Error, io::{self, BufRead, BufReader}};
use std::fs::File;
use clap::Parser;
use crate::styles::get_styles;

pub mod styles;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Head implemented in Rust", long_about = "Long about", author = "Izak Hudnik Zajec <hudnik.izak@gmail.com>", styles=get_styles())]
struct Cli {
    #[arg(
        name = "File name"
        )]
    filename: String,

    #[arg(
        short = 'n',
        long,
        default_value_t = 10,
        help = "DEFAUL OPTION.\nSpecify how many lines we should print (+) or how many lines should be cut from the end (-)."
        )]
    lines: i32,

    #[arg(
        short = 'c',
        long = "bytes",
        required = false,
        help = "Specify how many bytes we should print (+) or how many bytes should be cut from the end (-)."
        )]
    bytes: Option<i32>,
}

fn open_file(filename: &str) -> Result<Box< dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_lines(lines_vec: Vec<String>, lines_num: i32) {
    let mut final_line_index : i32;
    // Calculate the last line number based on the specified lines arg
    if lines_num < 0 {
        final_line_index = lines_vec.len().try_into().unwrap();
        final_line_index = final_line_index + lines_num;
    } else {
        final_line_index = lines_num;
    }

    // Print the appropriate number of lines
    let mut line_num = 1;
    for line in lines_vec {
        if line_num > final_line_index {
            break;
        }
        println!("{}", line);
        line_num = line_num + 1;
    }
}

fn print_bytes(byte_vec: Vec<char>, bytes_num: i32) {
    let mut final_byte_index : i32;
    // Calculate the last line number based on the specified lines arg
    if bytes_num < 0 {
        final_byte_index = byte_vec.len().try_into().unwrap();
        final_byte_index = final_byte_index + bytes_num;
    } else {
        final_byte_index = bytes_num;
    }

    let mut count = 1;

    for char in byte_vec {
        if count > final_byte_index {
            break;
        }
        print!("{}", char);
        count = count + 1;
    }
}

pub fn run() {
    let cli = Cli::parse();

    match open_file(&cli.filename) {
        Err(err) => eprint!("Failed to open file: '{}' - {}", &cli.filename, err),
        Ok(reader) => {
            let mut final_lines : Vec<String> = vec![];
            let mut final_chars : Vec<char> = vec![];

            // Count the lines from the Buffer
            for line in reader.lines() {
                let unwraped_line = line.unwrap();
                final_lines.push(unwraped_line.clone());
                for char in unwraped_line.clone().chars() {
                    final_chars.push(char);
                }
                final_chars.push('\n');
            }
            if let Some(bytes_num) = cli.bytes {
                print_bytes(final_chars, bytes_num);
            } else {
                print_lines(final_lines, cli.lines);
            }
        }
    }

}
