use hyphenation::{Language, Load, Standard};

fn main() {
    let text = "textwrap: a small library for wrapping text.";
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let options = textwrap::Options::new(18).word_splitter(dictionary);
    println!("{}", textwrap::fill(text, &options));
}
