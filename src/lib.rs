//! `textwrap` provides functions for word wrapping and filling text.

extern crate unicode_width;

use std::iter;
use unicode_width::UnicodeWidthStr;

/// A Wrapper holds settings for wrapping text.
pub struct Wrapper {
    width: usize,
}

impl Wrapper {
    /// Create a new Wrapper for wrapping at the specified width.
    pub fn new(width: usize) -> Wrapper {
        Wrapper { width: width }
    }

    /// Fill a line of text at `self.width` bytes.
    ///
    /// The result is a string with newlines between each line. Use
    /// the `wrap` method if you need access to the individual lines.
    ///
    /// ```
    /// use textwrap::Wrapper;
    ///
    /// let wrapper = Wrapper::new(15);
    /// assert_eq!(wrapper.fill("Memory safety without garbage collection."),
    ///            "Memory safety\nwithout garbage\ncollection.");
    /// ```
    pub fn fill(&self, s: &str) -> String {
        self.wrap(&s).join("\n")
    }

    /// Wrap a line of text at `self.width` bytes and return a vector
    /// of lines.
    ///
    /// ```
    /// use textwrap::Wrapper;
    ///
    /// let wrap15 = Wrapper::new(15);
    /// assert_eq!(wrap15.wrap("Concurrency without data races."),
    ///            vec!["Concurrency",
    ///                 "without data",
    ///                 "races."]);
    ///
    /// let wrap20 = Wrapper::new(20);
    /// assert_eq!(wrap20.wrap("Concurrency without data races."),
    ///            vec!["Concurrency without",
    ///                 "data races."]);
    /// ```
    pub fn wrap(&self, s: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut line = Vec::new();
        let mut line_width = 0;

        for mut word in s.split_whitespace() {
            while !word.is_empty() {
                let splits = split_word(&word);
                let (smallest, longest) = splits[0];
                let min_width = smallest.width();

                // Add a new line if even the smallest split doesn't
                // fit.
                if !line.is_empty() && line_width + line.len() + min_width > self.width {
                    result.push(line.join(" "));
                    line = Vec::new();
                    line_width = 0;
                }

                // Find a split that fits on the current line.
                for &(head, tail) in splits.iter().rev() {
                    if line_width + line.len() + head.width() <= self.width {
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
}

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
///
/// This function creates a Wrapper on the fly. If you need to wrap
/// many strings, it can be more efficient to create a single Wrapper
/// and call its [`fill` method](struct.Wrapper.html#method.fill).
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
///
/// This function creates a Wrapper on the fly. If you need to wrap
/// many strings, it can be more efficient to create a single Wrapper
/// and call its [`wrap` method](struct.Wrapper.html#method.wrap).
pub fn wrap(s: &str, width: usize) -> Vec<String> {
    Wrapper::new(width).wrap(s)
}

/// Split word into all possible parts (head, tail). Word must be
/// non-empty. The returned vector will always be non-empty.
fn split_word(word: &str) -> Vec<(&str, &str)> {
    let hyphens = word.match_indices('-');
    let word_end = iter::once((word.len() - 1, ""));
    return hyphens.chain(word_end)
        .map(|(n, _)| word.split_at(n + 1))
        .collect();
}

/// Add prefix to each non-empty line.
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent("Foo\nBar\n", "  "), "  Foo\n  Bar\n");
/// ```
///
/// Empty lines (lines consisting only of whitespace) are not indented
/// and the whitespace is replaced by a single newline (`\n`):
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent("Foo\n\nBar\n  \t  \nBaz\n", "  "),
///            "  Foo\n\n  Bar\n\n  Baz\n");
/// ```
///
/// Leading and trailing whitespace on non-empty lines is kept
/// unchanged:
///
/// ```
/// use textwrap::indent;
///
/// assert_eq!(indent(" \t  Foo   ", "  "), "   \t  Foo   \n");
/// ```
pub fn indent(s: &str, prefix: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        if line.chars().any(|c| !c.is_whitespace()) {
            result.push_str(prefix);
            result.push_str(line);
        }
        result.push('\n');
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Add newlines. Ensures that the final line in the vector also
    /// has a newline.
    fn add_nl(lines: &Vec<&str>) -> String {
        lines.join("\n") + "\n"
    }

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
    fn hyphens() {
        assert_eq!(wrap("foo-bar", 5), vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        assert_eq!(wrap("foobar-", 5), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        assert_eq!(wrap("foobar-baz", 5), vec!["foobar-", "baz"]);
    }

    #[test]
    fn test_fill() {
        assert_eq!(fill("foo bar baz", 10), "foo bar\nbaz");
    }

    #[test]
    fn test_indent_empty() {
        assert_eq!(indent("\n", "  "), "\n");
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_indent_nonempty() {
        let x = vec!["  foo",
                     "bar",
                     "  baz"];
        let y = vec!["//  foo",
                     "//bar",
                     "//  baz"];
        assert_eq!(indent(&add_nl(&x), "//"), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_indent_empty_line() {
        let x = vec!["  foo",
                     "bar",
                     "",
                     "  baz"];
        let y = vec!["//  foo",
                     "//bar",
                     "",
                     "//  baz"];
        assert_eq!(indent(&add_nl(&x), "//"), add_nl(&y));
    }
}
