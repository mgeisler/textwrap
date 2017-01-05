extern crate hyphenation;
extern crate textwrap;

use hyphenation::Language;
use textwrap::Wrapper;

fn main() {
    let corpus = hyphenation::load(Language::English_US).unwrap();
    let example = "
Memory safety without garbage collection.
Concurrency without data races.
Zero-cost abstractions.
";
    let mut prev_lines = vec![];
    let mut wrapper = Wrapper::new(0);
    wrapper.corpus = Some(&corpus);
    for width in 15..60 {
        wrapper.width = width;
        let lines = wrapper.wrap(example);
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
