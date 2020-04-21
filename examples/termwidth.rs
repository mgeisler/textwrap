#[cfg(feature = "hyphenation")]
use hyphenation::{Language, Load, Standard};
#[cfg(feature = "terminal_size")]
use textwrap::Wrapper;

#[cfg(not(feature = "terminal_size"))]
fn main() {
    println!("Please enable the terminal_size feature to run this example.");
}

#[cfg(feature = "terminal_size")]
fn main() {
    #[cfg(not(feature = "hyphenation"))]
    fn new_wrapper<'a>() -> (&'static str, Wrapper<'a, textwrap::HyphenSplitter>) {
        ("without hyphenation", Wrapper::with_termwidth())
    }

    #[cfg(feature = "hyphenation")]
    fn new_wrapper<'a>() -> (&'static str, Wrapper<'a, Standard>) {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        (
            "with hyphenation",
            Wrapper::with_splitter(textwrap::termwidth(), dictionary),
        )
    }

    let example = "Memory safety without garbage collection. \
                   Concurrency without data races. \
                   Zero-cost abstractions.";
    // Create a new Wrapper -- automatically set the width to the
    // current terminal width.
    let (msg, wrapper) = new_wrapper();
    println!("Formatted {} in {} columns:", msg, wrapper.width);
    println!("----");
    println!("{}", wrapper.fill(example));
    println!("----");
}
