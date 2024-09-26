use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    /// The file to read.
    file: PathBuf,

    /// Number of octets per line. Default is 16. (-i: 12, -ps: 30).
    #[clap(short = 'c', long = "cols", default_value_t = 16)]
    cols: usize,

    /// Number of octets per group in normal output. Default is 2. (-e: 4).
    #[clap(short = 'g', long = "bytes", default_value_t = 2)]
    bytes: usize,

    /// use upper case hex letters.
    #[clap(short = 'u')]
    upper: bool,
}

fn main() {
    let args = Args::parse();

    match fs::read(&args.file) {
        Ok(content) => {
            let output = content
                .chunks(args.cols)
                .enumerate()
                .map(|(i, chunk)| {
                    let bytes: String = chunk
                        .iter()
                        .map(|&byte| {
                            if args.upper {
                                format!("{:02X}", byte)
                            } else {
                                format!("{:02x}", byte)
                            }
                        })
                        .collect::<Vec<String>>()
                        .chunks(args.bytes)
                        .map(|group| group.join(""))
                        .collect::<Vec<String>>()
                        .join(" ");

                    let ascii: String = chunk
                        .iter()
                        .map(|&byte| {
                            if byte.is_ascii_graphic() || byte == b' ' {
                                byte as char
                            } else {
                                '.'
                            }
                        })
                        .collect();

                    let width = 2 * args.cols + args.cols / args.bytes;
                    format!("{:08x}: {:width$} {}", i * args.cols, bytes, ascii)
                })
                .collect::<Vec<String>>()
                .join("\n");

            println!("{}", output);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
