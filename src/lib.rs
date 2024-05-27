use std::{error::Error, fmt::format, io::{self, BufRead, BufReader}};
use std::fs::File;
use clap::Parser;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Head implemented in Rust", long_about = "Long about", author = "Izak Hudnik Zajec <hudnik.izak@gmail.com>")]
struct Cli {
    filename: String,

    #[arg(short, long, default_value_t = 10)]
    lines: i32,
}

fn open_file(filename: &str) -> Result<Box< dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run() {
    let cli = Cli::parse();

    let file_lines = open_file(&cli.filename).unwrap().lines().enumerate();
    let mut lines_count = 0;

    let mut final_line_index : i32 = 0;
    let mut final_lines : Vec<String> = vec![];

    for (line_num,  line) in file_lines {
        lines_count = line_num;
        final_lines.push(line.unwrap());
    }

    println!("Final line index of whole file: {:?}", lines_count);

    if cli.lines < 0 {
        final_line_index = lines_count.try_into().unwrap();
        final_line_index = final_line_index + cli.lines + 1;
    } else {
        final_line_index = cli.lines; 
    }

    println!("Stopping at line {:}", final_line_index);

    let mut line_num = 1;
    for line in final_lines {
        if line_num > final_line_index {
            break;
        }
        println!("{}", line);
        line_num = line_num + 1;
    }
}
