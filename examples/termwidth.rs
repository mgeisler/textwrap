use textwrap::{fill, Options, WordSplitter};

fn main() {
    let example = "Memory safety without garbage collection. \
                   Concurrency without data races. \
                   Zero-cost abstractions.";

    #[cfg(not(feature = "hyphenation"))]
    let (msg, options) = ("without hyphenation", Options::with_termwidth());

    #[cfg(feature = "hyphenation")]
    use hyphenation::Load;

    #[cfg(feature = "hyphenation")]
    let (msg, options) = (
        "with hyphenation",
        Options::with_termwidth().word_splitter(WordSplitter::Hyphenation(
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS).unwrap(),
        )),
    );

    println!("Formatted {} in {} columns:", msg, options.width);
    println!("----");
    println!("{}", fill(example, &options));
    println!("----");
}
