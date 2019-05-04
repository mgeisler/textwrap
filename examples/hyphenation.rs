#[cfg(feature = "hyphenation")]
use hyphenation::{Language, Load, Standard};
#[cfg(feature = "hyphenation")]
use textwrap::Wrapper;

#[cfg(not(feature = "hyphenation"))]
fn main() {
    println!("Please run this example as");
    println!();
    println!("  cargo run --example hyphenation --features hyphenation");
}

#[cfg(feature = "hyphenation")]
fn main() {
    let text = "textwrap: a small library for wrapping text.";
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let wrapper = Wrapper::with_splitter(18, dictionary);
    println!("{}", wrapper.fill(text));
}
