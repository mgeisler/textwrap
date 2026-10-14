//! Wrap each line from standard input and write it to standard output.
//!
//! Run with `cargo run --example stdin -- 40 < input.txt` to use 40 columns.
//! The width defaults to 80 columns. Existing line breaks and blank lines are
//! preserved, and each output line ends with a newline.

use std::env;
use std::io::{self, BufRead, Write};

fn run() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let width = match args.next() {
        Some(arg) => arg.parse::<usize>().ok().filter(|&width| width > 0),
        None => Some(80),
    };
    let width = width.filter(|_| args.next().is_none()).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "usage: stdin [positive-width]")
    })?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut output = io::BufWriter::new(stdout.lock());
    for line in stdin.lock().lines() {
        for wrapped in textwrap::wrap(&line?, width) {
            writeln!(output, "{wrapped}")?;
        }
    }
    output.flush()
}

fn main() -> io::Result<()> {
    match run() {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result,
    }
}
