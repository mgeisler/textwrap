#[cfg(feature = "textwrap")]
use textwrap::fill;

#[cfg(not(feature = "textwrap"))]
/// Quick-and-dirty fill implementation.
///
/// Assumes single space between words, assumes 1 column per Unicode
/// character (no emoji handling) and assumes that the longest word
/// fit on the line (no handling of hyphens or over-long words).
fn fill(text: &str, width: usize) -> String {
    let mut result = String::with_capacity(text.len());
    let mut line_width = 0;
    for word in text.split_whitespace() {
        if line_width + 1 + word.len() > width {
            result.push('\n');
            line_width = 0;
        }

        result.push_str(word);
        result.push(' ');
        line_width += word.len() + 1;
    }

    // Remove final ' '.
    result.truncate(result.len() - 1);
    result
}

fn main() {
    let text = "Hello, welcome to a world with beautifully wrapped \
                text in your command-line programs. This includes \
                non-ASCII text such as Açai, Jalapeño, Frappé";
    for line in fill(text, 18).lines() {
        println!("│ {:18} │", line);
    }
}
