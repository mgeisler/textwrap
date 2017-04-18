#[cfg(feature = "hyphenation")]
extern crate hyphenation;
extern crate textwrap;

#[cfg(feature = "hyphenation")]
use hyphenation::Language;
use textwrap::Wrapper;


#[cfg(not(feature = "hyphenation"))]
fn new_wrapper() -> Wrapper {
    Wrapper::new(0)
}

#[cfg(feature = "hyphenation")]
fn new_wrapper() -> Wrapper {
    let mut wrapper = Wrapper::new(0);
    wrapper.splitter = Box::new(hyphenation::load(Language::English_US).unwrap());
    wrapper
}

fn main() {
    let example = "
Memory safety without garbage collection.
Concurrency without data races.
Zero-cost abstractions.
";
    let mut prev_lines = vec![];
    let mut wrapper = new_wrapper();
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
