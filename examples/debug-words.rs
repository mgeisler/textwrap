use textwrap::WordSeparator;

fn main() {
    #[cfg(feature = "unicode-linebreak")]
    let word_separator = WordSeparator::UnicodeBreakProperties;
    #[cfg(not(feature = "unicode-linebreak"))]
    let word_separator = WordSeparator::AsciiSpace;

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let text = args.join(" ");
    let words = word_separator.find_words(&text).collect::<Vec<_>>();

    println!("word_separator = {:?}", word_separator);
    println!("text = {:?}", text);
    println!("words = {:#?}", words);
}
