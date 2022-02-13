use hyphenation::{Language, Load, Standard};
use textwrap::WordSplitter;

fn main() {
    let text = "textwrap: a small library for wrapping text.";
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options = textwrap::Options::new(18).word_splitter(WordSplitter::Hyphenation(dictionary));
    println!("{}", textwrap::fill(text, &options));
}
