use textwrap::Wrapper;

fn main() {
    let example = "Memory safety without garbage collection. \
                   Concurrency without data races. \
                   Zero-cost abstractions.";
    let mut prev_lines = vec![];

    let mut wrapper = Wrapper::new(0);
    #[cfg(feature = "hyphenation")]
    {
        use hyphenation::Load;
        let language = hyphenation::Language::EnglishUS;
        let dictionary = hyphenation::Standard::from_embedded(language).unwrap();
        wrapper.splitter = Box::new(dictionary);
    }

    for width in 15..60 {
        wrapper.width = width;
        let lines = wrapper.wrap(example).collect::<Vec<_>>();
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
