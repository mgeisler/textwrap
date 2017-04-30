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
extern crate term_size;
#[cfg(feature = "hyphenation")]
extern crate hyphenation;

use unicode_width::UnicodeWidthStr;
use unicode_width::UnicodeWidthChar;
#[cfg(feature = "hyphenation")]
use hyphenation::{Hyphenation, Corpus};

pub trait WordSplitter {
    /// Return all possible splits of word. Each split is a triple
    /// with a head, a hyphen, and a tail where `head + &hyphen +
    /// &tail == word`. The hyphen can be empty if there is already a
    /// hyphen in the head.
    ///
    /// The splits should go from smallest to longest and should
    /// include no split at all. So the word "technology" could be
    /// split into
    ///
    /// ```
    /// vec![("tech", "-", "nology"),
    ///      ("technol", "-", "ogy"),
    ///      ("technolo", "-", "gy"),
    ///      ("technology", "", "")];
    /// ```
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)>;
}

pub struct HyphenSplitter;

/// HyphenSplitter is the default WordSplitter. As the name says, it
/// will split words on any existing hyphens in the word.
///
/// It will only use hyphens that are surrounded by alphanumeric
/// characters, which prevents a word like "--foo-bar" from being
/// split on the first or second hyphen.
impl WordSplitter for HyphenSplitter {
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        let mut triples = Vec::new();
        // Split on hyphens, smallest split first. We only use hyphens
        // that are surrounded by alphanumeric characters. This is to
        // avoid splitting on repeated hyphens, such as those found in
        // --foo-bar.
        let char_indices = word.char_indices().collect::<Vec<_>>();
        for w in char_indices.windows(3) {
            let ((_, prev), (n, c), (_, next)) = (w[0], w[1], w[2]);
            if prev.is_alphanumeric() && c == '-' && next.is_alphanumeric() {
                let (head, tail) = word.split_at(n + 1);
                triples.push((head, "", tail));
            }
        }

        // Finally option is no split at all.
        triples.push((word, "", ""));

        triples
    }
}

/// A hyphenation Corpus can be used to do language-specific
/// hyphenation using patterns from the hyphenation crate.
#[cfg(feature = "hyphenation")]
impl WordSplitter for Corpus {
    fn split<'w>(&self, word: &'w str) -> Vec<(&'w str, &'w str, &'w str)> {
        // Find splits based on language corpus.
        let mut triples = Vec::new();
        for n in word.opportunities(&self) {
            let (head, tail) = word.split_at(n);
            let hyphen = if head.ends_with('-') { "" } else { "-" };
            triples.push((head, hyphen, tail));
        }
        // Finally option is no split at all.
        triples.push((word, "", ""));

        triples
    }
}

/// A Wrapper holds settings for wrapping and filling text.
///
/// The algorithm used by the `wrap` method works by doing a single
/// scan over words in the input string and splitting them into one or
/// more lines. The time and memory complexity is O(*n*) where *n* is
/// the length of the input string.
pub struct Wrapper {
    /// The width in columns at which the text will be wrapped.
    pub width: usize,
    /// Allow long words to be broken if they cannot fit on a line.
    /// When set to false, some lines be being longer than self.width.
    pub break_words: bool,
    /// The method for splitting words, if any.
    pub splitter: Box<WordSplitter>,
}

impl Wrapper {
    /// Create a new Wrapper for wrapping at the specified width. By
    /// default, we allow words longer than `width` to be broken. No
    /// hyphenation corpus is loaded by default.
    pub fn new(width: usize) -> Wrapper {
        Wrapper {
            width: width,
            break_words: true,
            splitter: Box::new(HyphenSplitter {}),
        }
    }

    /// Crate a new Wrapper for wrapping text at the current terminal
    /// width. If the terminal width cannot be determined (typically
    /// because the standard input and output is not connected to a
    /// terminal), a width of 80 characters will be used. Other
    /// settings use the same defaults as `Wrapper::new`.
    pub fn with_termwidth() -> Wrapper {
        Wrapper::new(term_size::dimensions_stdout().map_or(80, |(w, _)| w))
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
        self.wrap(s).join("\n")
    }

    /// Wrap a line of text at `self.width` characters. Strings are
    /// wrapped based on their displayed width, not their size in
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
        let mut lines = Vec::with_capacity(s.len() / (self.width + 1));
        let mut line = String::with_capacity(self.width);
        let mut remaining = self.width;
        const NBSP: char = '\u{a0}'; // non-breaking space

        for mut word in s.split(|c: char| c.is_whitespace() && c != NBSP) {
            // Skip over adjacent whitespace characters.
            if word.is_empty() {
                continue;
            }

            // Attempt to fit the word without any splitting.
            if self.fit_part(word, "", &mut remaining, &mut line) {
                continue;
            }

            // If that failed, loop until nothing remains to be added.
            while !word.is_empty() {
                let splits = self.splitter.split(word);
                let (smallest, hyphen, longest) = splits[0];
                let min_width = smallest.width() + hyphen.len();

                // Add a new line if even the smallest split doesn't
                // fit.
                if !line.is_empty() && 1 + min_width > remaining {
                    lines.push(line);
                    line = String::with_capacity(self.width);
                    remaining = self.width;
                }

                // Find a split that fits on the current line.
                for &(head, hyphen, tail) in splits.iter().rev() {
                    if self.fit_part(head, hyphen, &mut remaining, &mut line) {
                        word = tail;
                        break;
                    }
                }

                // If even the smallest split doesn't fit on the line,
                // we might have to break the word.
                if line.is_empty() {
                    if self.break_words && self.width > 1 {
                        // Break word on a character boundary as close
                        // to self.width as possible. Characters are
                        // at most 2 columns wide, so we will chop off
                        // at least one character.
                        let mut head_width = 0;
                        for (idx, c) in word.char_indices() {
                            head_width += c.width().unwrap_or(0);
                            if head_width > self.width {
                                let (head, tail) = word.split_at(idx);
                                lines.push(String::from(head));
                                word = tail;
                                break;
                            }
                        }
                    } else {
                        // We forcibly add the smallest split and
                        // continue with the longest tail. This will
                        // result in a line longer than self.width.
                        lines.push(String::from(smallest) + hyphen);
                        remaining = self.width;
                        word = longest;
                    }
                }
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
        lines
    }

    /// Try to fit a word (or part of a word) onto a line. The line
    /// and the remaining width is updated as appropriate if the word
    /// or part fits.
    fn fit_part<'b>(&self,
                    part: &'b str,
                    hyphen: &'b str,
                    remaining: &mut usize,
                    line: &mut String)
                    -> bool {
        let space = if line.is_empty() { 0 } else { 1 };
        let fits_in_line = space + part.width() + hyphen.len() <= *remaining;
        if fits_in_line {
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(part);
            line.push_str(hyphen);
            *remaining -= space + part.width() + hyphen.len();
        }

        fits_in_line
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
    result
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
    result
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "hyphenation")]
    extern crate hyphenation;

    #[cfg(feature = "hyphenation")]
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
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("foobar-"), vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(wrap("foo-bar-baz", 5), vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("The --foo-bar flag."),
                   vec!["The", "--foo-", "bar", "flag."]);
    }

    #[test]
    fn repeated_hyphens() {
        let mut wrapper = Wrapper::new(4);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("foo--bar"), vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(wrap("Na2-CH4", 5), vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("foo(-)bar"), vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(wrap("foo-bar-baz", 9), vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("foobar-baz"), vec!["foobar-", "baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let mut wrapper = Wrapper::new(10);
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Internatio", "nalization"]);

        wrapper.splitter = Box::new(corpus);
        assert_eq!(wrapper.wrap("Internationalization"),
                   vec!["Interna-", "tionaliza-", "tion"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let corpus = hyphenation::load(Language::English_US).unwrap();
        let mut wrapper = Wrapper::new(8);
        wrapper.break_words = false;
        assert_eq!(wrapper.wrap("over-caffinated"), vec!["over-", "caffinated"]);

        wrapper.splitter = Box::new(corpus);
        assert_eq!(wrapper.wrap("over-caffinated"),
                   vec!["over-", "caffi-", "nated"]);
    }

    #[test]
    fn break_words() {
        assert_eq!(wrap("foobarbaz", 3), vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn break_words_wide_characters() {
        assert_eq!(wrap("Ｈｅｌｌｏ", 5), vec!["Ｈｅ", "ｌｌ", "ｏ"]);
    }

    #[test]
    fn break_words_zero_width() {
        assert_eq!(wrap("foobar", 0), vec!["foobar"]);
    }

    #[test]
    fn test_non_breaking_space() {
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.fill("foo bar baz"), "foo bar baz");
    }

    #[test]
    fn test_non_breaking_hyphen() {
        let mut wrapper = Wrapper::new(5);
        wrapper.break_words = false;
        assert_eq!(wrapper.fill("foo‑bar‑baz"), "foo‑bar‑baz");
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
