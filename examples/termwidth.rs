#[cfg(feature = "terminal_size")]
use textwrap::Wrapper;

#[cfg(not(feature = "terminal_size"))]
fn main() {
    println!("Please enable the terminal_size feature to run this example.");
}

#[cfg(feature = "terminal_size")]
fn main() {
    let example = "Memory safety without garbage collection. \
                   Concurrency without data races. \
                   Zero-cost abstractions.";

    #[cfg(not(feature = "hyphenation"))]
    let (msg, wrapper) = ("without hyphenation", Wrapper::with_termwidth());

    #[cfg(feature = "hyphenation")]
    use hyphenation::Load;

    #[cfg(feature = "hyphenation")]
    let (msg, wrapper) = (
        "with hyphenation",
        Wrapper::with_termwidth().splitter(Box::new(
            hyphenation::Standard::from_embedded(hyphenation::Language::EnglishUS).unwrap(),
        )),
    );

    println!("Formatted {} in {} columns:", msg, wrapper.width);
    println!("----");
    println!("{}", wrapper.fill(example));
    println!("----");
}
