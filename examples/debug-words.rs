use textwrap::word_separators::WordSeparator;

fn main() {
    #[cfg(feature = "unicode-linebreak")]
    let word_separator = textwrap::word_separators::UnicodeBreakProperties;
    #[cfg(not(feature = "unicode-linebreak"))]
    let word_separator = textwrap::word_separators::AsciiSpace;

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let text = args.join(" ");
    let words = word_separator.find_words(&text).collect::<Vec<_>>();

    println!("word_separator = {:?}", word_separator);
    println!("text = {:?}", text);
    println!("words = {:#?}", words);
}
