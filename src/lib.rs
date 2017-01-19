//! `textwrap` provides functions for word wrapping and filling text.
//!
//! This can be very useful in commandline programs where you want to
//! format dynamic output nicely so it looks good in a terminal.
//!
//! To wrap text, one must know the width of each word so can know
//! when to break lines. This library measures the width of text using
//! the [displayed width][unicode-width], not the size in bytes.
//!
//! This is important for non-ASCII text. ASCII characters such as `a`
//! and `!` are simple: the displayed with is the same as the number
//! of bytes used in their UTF-8 encoding (one ASCII character takes
//! up one byte in UTF-8). However, non-ASCII characters and symbols
//! take up more than one byte: `é` is `0xc3 0xa9` and `⚙` is `0xe2
//! 0x9a 0x99` in UTF-8, respectively.
//!
//! This is why we take care to use the displayed width instead of the
//! byte count when computing line lengths. All functions in this
//! library handle Unicode characters like this.
//!
//! [unicode-width]: https://unicode-rs.github.io/unicode-width/unicode_width/index.html


extern crate unicode_width;
extern crate hyphenation;

use unicode_width::UnicodeWidthStr;
use hyphenation::Hyphenation;
use hyphenation::Corpus;

/// A Wrapper holds settings for wrapping and filling text.
///
/// The algorithm used by the `wrap` method works by doing a single
/// scan over words in the input string and splitting them into one or
/// more lines. The time and memory complexity is O(*n*) where *n* is
/// the length of the input string.
pub struct Wrapper<'a> {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// The hyphenation corpus (if any) used for automatic
    /// hyphenation.
    pub corpus: Option<&'a Corpus>,
}

impl<'a> Wrapper<'a> {
    /// Create a new Wrapper for wrapping at the specified width.
    pub fn new(width: usize) -> Wrapper<'a> {
        Wrapper::<'a> {
            width: width,
            corpus: None,
        }
    }

    /// Fill a line of text at `self.width` characters. Strings are
    /// wrapped based on their displayed width, not their size in
    /// bytes.
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
    ///
    /// This method simply joins the lines produced by `wrap`. As
    /// such, it inherits the O(*n*) time and memory complexity where
    /// *n* is the input string length.
    pub fn fill(&self, s: &str) -> String {
        self.wrap(&s).join("\n")
    }

    /// Wrap  a line of  text at `self.width` characters.  Strings are
    ///  wrapped based  on their  displayed width,  not their  size in
    /// bytes.
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
    ///
    /// This method does a single scan over the input string, it has
    /// an O(*n*) time and memory complexity where *n* is the input
    /// string length.
    pub fn wrap(&self, s: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut line = String::new();
        let mut remaining = self.width;

        for mut word in s.split_whitespace() {
            while !word.is_empty() {
                let splits = self.split_word(&word);
                let (smallest, hyphen, longest) = splits[0];
                let min_width = smallest.width() + hyphen.len();

                // Add a new line if even the smallest split doesn't
                // fit.
                if !line.is_empty() && 1 + min_width > remaining {
                    result.push(line);
                    line = String::new();
                    remaining = self.width;
                }

                let space = if line.is_empty() { 0 } else { 1 };

                // Find a split that fits on the current line.
                for &(head, hyphen, tail) in splits.iter().rev() {
                    if space + head.width() + hyphen.len() <= remaining {
                        if !line.is_empty() {
                            line.push(' ');
                        }
                        line.push_str(head);
                        line.push_str(hyphen);
                        remaining -= space + head.width() + hyphen.len();
                        word = tail;
                        break;
                    }
                }

                // If nothing got added, we forcibly add the smallest
                // split and continue with the longest tail.
                if line.is_empty() {
                    result.push(String::from(smallest) + hyphen);
                    remaining = self.width;
                    word = longest;
                }
            }
        }
        if !line.is_empty() {
            result.push(line);
        }
        return result;
    }

    /// Split word into all possible parts (head, tail). Word must be
    /// non-empty. The returned vector will always be non-empty.
    fn split_word<'b>(&self, word: &'b str) -> Vec<(&'b str, &'b str, &'b str)> {
        let mut result = Vec::new();

        // Split on hyphens or use the language corpus.
        match self.corpus {
            None => {
                // Split on hyphens, smallest split first.
                for (n, _) in word.match_indices('-') {
                    let (head, tail) = word.split_at(n + 1);
                    result.push((head, "", tail));
                }
            }
            Some(corpus) => {
                // Find splits based on language corpus. This includes
                // the splits that would have been found above.
                for n in word.opportunities(corpus) {
                    let (head, tail) = word.split_at(n);
                    let mut hyphen = "-";
                    if head.as_bytes()[head.len() - 1] == b'-' {
                        hyphen = "";
                    }
                    result.push((head, hyphen, tail));
                }
            }
        }

        // Finally option is no split at all.
        result.push((word, "", ""));

        return result;
    }
}

/// Fill a line of text at `width` characters. Strings are wrapped
/// based on their displayed width, not their size in bytes.
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
/// This function creates a Wrapper on the fly with default settings.
/// If you need to set a language corpus for automatic hyphenation, or
/// need to fill many strings, then it is suggested to create Wrapper
/// and call its [`fill` method](struct.Wrapper.html#method.fill).
pub fn fill(s: &str, width: usize) -> String {
    wrap(s, width).join("\n")
}

/// Wrap a line of text at `width` characters. Strings are wrapped
/// based on their displayed width, not their size in bytes.
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
/// This function creates a Wrapper on the fly with default settings.
/// If you need to set a language corpus for automatic hyphenation, or
/// need to wrap many strings, then it is suggested to create Wrapper
/// and call its [`wrap` method](struct.Wrapper.html#method.wrap).
pub fn wrap(s: &str, width: usize) -> Vec<String> {
    Wrapper::new(width).wrap(s)
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

/// Removes common leading whitespace from each line.
///
/// This will look at each non-empty line and determine the maximum
/// amount of whitespace that can be removed from the line.
///
/// ```
/// use textwrap::dedent;
///
/// assert_eq!(dedent("  1st line\n  2nd line\n"),
///            "1st line\n2nd line\n");
/// ```
pub fn dedent(s: &str) -> String {
    let mut prefix = String::new();
    let mut lines = s.lines();

    // We first search for a non-empty line to find a prefix.
    for line in &mut lines {
        let whitespace = line.chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        // Check if the line had anything but whitespace
        if whitespace.len() < line.len() {
            prefix = whitespace;
            break;
        }
    }

    // We then continue looking through the remaining lines to
    // possibly shorten the prefix.
    for line in &mut lines {
        let whitespace = line.chars()
            .zip(prefix.chars())
            .take_while(|&(a, b)| a == b)
            .map(|(_, b)| b)
            .collect::<String>();
        // Check if we have found a shorter prefix
        if whitespace.len() < prefix.len() {
            prefix = whitespace;
        }
    }

    // We now go over the lines a second time to build the result.
    let mut result = String::new();
    for line in s.lines() {
        if line.starts_with(&prefix) && line.chars().any(|c| !c.is_whitespace()) {
            let (_, tail) = line.split_at(prefix.len());
            result.push_str(tail);
        }
        result.push('\n');
    }
    return result;
}

#[cfg(test)]
mod tests {
    extern crate hyphenation;

    use hyphenation::Language;
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
    fn auto_hyphenation() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let mut wrapper = Wrapper::new(10);
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Internationalization"]);

        wrapper.corpus = Some(&corpus);
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Interna-", "tionaliza-", "tion"]);
    }

    #[test]
    fn auto_hyphenation_with_hyphen() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let mut wrapper = Wrapper::new(8);
        assert_eq!(wrapper.wrap("over-caffinated"), vec!["over-", "caffinated"]);

        wrapper.corpus = Some(&corpus);
        assert_eq!(wrapper.wrap("over-caffinated"),
                   vec!["over-", "caffi-", "nated"]);
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

    #[test]
    fn test_dedent_empty() {
        assert_eq!(dedent(""), "");
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_multi_line() {
        let x = vec!["    foo",
                     "  bar",
                     "    baz"];
        let y = vec!["  foo",
                     "bar",
                     "  baz"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_empty_line() {
        let x = vec!["    foo",
                     "  bar",
                     "   ",
                     "    baz"];
        let y = vec!["  foo",
                     "bar",
                     "",
                     "  baz"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }

    #[test]
    #[cfg_attr(rustfmt, rustfmt_skip)]
    fn test_dedent_mixed_whitespace() {
        let x = vec!["\tfoo",
                     "  bar"];
        let y = vec!["\tfoo",
                     "  bar"];
        assert_eq!(dedent(&add_nl(&x)), add_nl(&y));
    }
}
