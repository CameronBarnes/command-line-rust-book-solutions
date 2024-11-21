use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use clap::Parser;
use itertools::Itertools;
use thiserror::Error;

/// concatenate files and print on the standard output
#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Rust cat", long_about = None, author = "Cameron Barnes <cameron_barnes@outlook.com>")]
pub struct Args {
    /// Number all output lines
    #[arg(short, long, conflicts_with = "number_nonblank")]
    number: bool,
    /// Number non-empty output lines, overrides '-n'
    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank: bool,
    /// The FILE(s) to print. With no FILE, or when FILE is -, read standard input.
    #[arg(default_values_t = ["-".to_string()])]
    file: Vec<String>,
}

#[derive(Error, Debug)]
pub enum CatrError {
    #[error(transparent)]
    IOEror(#[from] std::io::Error),
}

pub type CatrResult<T> = Result<T, CatrError>;

pub fn get_args() -> CatrResult<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> CatrResult<()> {
    for file in &args.file {
        match open(file) {
            Ok(reader) => print_from_reader(
                reader,
                args.number || args.number_nonblank,
                !args.number_nonblank,
            ),
            Err(err) => {
                eprintln!("Failed to open {file}: {err}");
                Ok(())
            }
        }?
    }
    Ok(())
}

fn open(filename: &str) -> CatrResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn print_from_reader(
    mut reader: Box<dyn BufRead>,
    numbers: bool,
    blank_lines: bool,
) -> CatrResult<()> {
    let mut out = std::io::stdout();
    let mut line_index = 1usize;
    loop {
        let buf = reader.fill_buf()?;
        let length = buf.len();
        if buf.is_empty() {
            break;
        }
        // out.write_all(b"`loop`")?;
        if numbers {
            let mut iter = buf.split(|b| *b == b'\n').peekable();
            while let Some(line) = iter.next() {
                // out.write_all(b"`num-loop`")?;
                if !blank_lines
                    && line
                        .iter()
                        .all(|b| *b == b' ' || *b == b'\t' || *b == b'\n')
                {
                    if iter.peek().is_some() {
                        // out.write_all(b"`num-loop-blank`")?;
                        out.write_all(b"\n")?;
                    }
                } else {
                    // out.write_all(b"`num-loop-not`")?;
                    if iter.peek().is_none() && line.is_empty() {
                        continue;
                    }
                    out.write_all(&format!("     {line_index}\t").bytes().collect_vec())?;
                    out.write_all(line)?;
                    out.write_all(b"\n")?;
                    line_index += 1;
                }
            }
        } else {
            out.write_all(buf)?;
        }
        reader.consume(length);
    }
    out.flush()?;
    Ok(())
}
