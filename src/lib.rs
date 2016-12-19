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
    let mut line = String::new();
    let mut line_width = 0;

    for word in s.split_whitespace() {
        let word_width = word.width();
        if line_width > 0 && line_width + 1 + word_width > width {
            result.push(line);
            line = String::new();
            line_width = 0;
        }
        if line_width > 0 {
            line.push(' ');
            line_width += 1;
        }
        line.push_str(word);
        line_width += word_width;
    }
    if line_width > 0 {
        result.push(line);
    }
    return result;
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
