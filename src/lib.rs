//! `textwrap` provides functions for word wrapping and filling text.

extern crate unicode_width;

use unicode_width::UnicodeWidthStr;

/// Fill a line of text at `width` bytes.
///
/// The result is a string with newlines between each line. Use `wrap`
/// if you need access to the individual lines.
///
/// ```
/// use textwrap::fill;
///
/// assert_eq!(fill("Memory safety without garbage collection.", 15),
///            "Memory safety\nwithout garbage\ncollection.");
/// ```
pub fn fill(s: &str, width: usize) -> String {
    wrap(s, width).join("\n")
}

/// Wrap a line of text at `width` bytes and return a vector of lines.
///
/// ```
/// use textwrap::wrap;
///
/// assert_eq!(wrap("Concurrency without data races.", 15),
///            vec!["Concurrency",
///                 "without data",
///                 "races."]);
///
/// assert_eq!(wrap("Concurrency without data races.", 20),
///            vec!["Concurrency without",
///                 "data races."]);
/// ```
pub fn wrap(s: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    let mut line = Vec::new();
    let mut line_width = 0;

    for mut word in s.split_whitespace() {
        while !word.is_empty() {
            let splits = split_word(&word);
            let (smallest, longest) = splits[0];
            let min_width = smallest.width();

            // Add a new line if even the smallest split doesn't fit.
            if !line.is_empty() && line_width + line.len() + min_width > width {
                result.push(line.join(" "));
                line = Vec::new();
                line_width = 0;
            }

            // Find a split that fits on the current line.
            for &(head, tail) in splits.iter().rev() {
                if line_width + line.len() + head.width() <= width {
                    line.push(head);
                    line_width += head.width();
                    word = tail;
                    break;
                }
            }

            // If nothing got added, we forcibly add the smallest
            // split and continue with the longest tail.
            if line_width == 0 {
                result.push(String::from(smallest));
                line_width = 0;
                word = longest;
            }
        }
    }
    if !line.is_empty() {
        result.push(line.join(" "));
    }
    return result;
}

/// Split word into all possible parts (head, tail). Must return a
/// non-empty vector.
fn split_word(word: &str) -> Vec<(&str, &str)> {
    return vec![(word, "")];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_wrap() {
        assert_eq!(wrap("foo", 10), vec!["foo"]);
    }

    #[test]
    fn simple() {
        assert_eq!(wrap("foo bar baz", 5), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn multi_word_on_line() {
        assert_eq!(wrap("foo bar baz", 10), vec!["foo bar", "baz"]);
    }

    #[test]
    fn long_word() {
        assert_eq!(wrap("foo", 0), vec!["foo"]);
    }

    #[test]
    fn long_words() {
        assert_eq!(wrap("foo bar", 0), vec!["foo", "bar"]);
    }

    #[test]
    fn whitespace_is_squeezed() {
        assert_eq!(wrap(" foo \t  bar  ", 10), vec!["foo bar"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_eq!(wrap("Hello, World!", 15), vec!["Hello, World!"]);
        assert_eq!(wrap("Ｈｅｌｌｏ, Ｗｏｒｌｄ!", 15),
                   vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]);
    }

    #[test]
    fn test_fill() {
        assert_eq!(fill("foo bar baz", 10), "foo bar\nbaz");
    }
}
