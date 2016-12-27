extern crate textwrap;

use textwrap::wrap;

fn main() {
    let example = "
Memory safety without garbage collection.
Concurrency without data races.
Zero-cost abstractions.
";
    let mut prev_lines = vec![];
    for width in 15..60 {
        let lines = wrap(example, width);
        if lines != prev_lines {
            let title = format!(" Width: {} ", width);
            println!(".{:-^1$}.", title, width + 2);
            for line in &lines {
                println!("| {:1$} |", line, width);
            }
            prev_lines = lines;
        }
    }
}
