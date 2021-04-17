//! Line breaking functionality.

use crate::core::Word;

/// Describes where a line break can occur.
///
/// The simplest approach is say that a line can end after one or more
/// ASCII spaces (`' '`). This works for Western languages without
/// emojis.
///
/// The line breaks occur between words, please see the
/// [`WordSplitter`](crate::WordSplitter) trait for options of how
/// to handle hyphenation of individual words.
///
/// # Examples
///
/// ```
/// use textwrap::{WordSeparator, AsciiSpace};
/// use textwrap::core::Word;
/// let words = AsciiSpace.find_words("Hello World!").collect::<Vec<_>>();
/// assert_eq!(words, vec![Word::from("Hello "), Word::from("World!")]);
/// ```
pub trait WordSeparator: WordSeparatorClone + std::fmt::Debug {
    // This trait should really return impl Iterator<Item = Word>, but
    // this isn't possible until Rust supports higher-kinded types:
    // https://github.com/rust-lang/rfcs/blob/master/text/1522-conservative-impl-trait.md
    /// Find all words in `line`.
    fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>> + 'a>;
}

// The internal `WordSeparatorClone` trait is allows us to implement
// `Clone` for `Box<dyn WordSeparator>`. This in used in the
// `From<&Options<'_, R, S>> for Options<'a, R, S>` implementation.
pub trait WordSeparatorClone {
    fn clone_box(&self) -> Box<dyn WordSeparator>;
}

impl<T: WordSeparator + Clone + 'static> WordSeparatorClone for T {
    fn clone_box(&self) -> Box<dyn WordSeparator> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn WordSeparator> {
    fn clone(&self) -> Box<dyn WordSeparator> {
        use std::ops::Deref;
        self.deref().clone_box()
    }
}

impl WordSeparator for Box<dyn WordSeparator> {
    fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>> + 'a> {
        use std::ops::Deref;
        self.deref().find_words(line)
    }
}

/// Find line breaks by regions of `' '` characters.
#[derive(Clone, Copy, Debug, Default)]
pub struct AsciiSpace;

/// Split `line` into words separated by regions of `' '` characters.
///
/// # Examples
///
/// ```
/// use textwrap::core::Word;
/// use textwrap::{AsciiSpace, WordSeparator};
///
/// let words = AsciiSpace.find_words("Hello   World!").collect::<Vec<_>>();
/// assert_eq!(words, vec![Word::from("Hello   "),
///                        Word::from("World!")]);
/// ```
impl WordSeparator for AsciiSpace {
    fn find_words<'a>(&self, line: &'a str) -> Box<dyn Iterator<Item = Word<'a>> + 'a> {
        let mut start = 0;
        let mut in_whitespace = false;
        let mut char_indices = line.char_indices();

        Box::new(std::iter::from_fn(move || {
            // for (idx, ch) in char_indices does not work, gives this
            // error:
            //
            // > cannot move out of `char_indices`, a captured variable in
            // > an `FnMut` closure
            #[allow(clippy::while_let_on_iterator)]
            while let Some((idx, ch)) = char_indices.next() {
                if in_whitespace && ch != ' ' {
                    let word = Word::from(&line[start..idx]);
                    start = idx;
                    in_whitespace = ch == ' ';
                    return Some(word);
                }

                in_whitespace = ch == ' ';
            }

            if start < line.len() {
                let word = Word::from(&line[start..]);
                start = line.len();
                return Some(word);
            }

            None
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Like assert_eq!, but the left expression is an iterator.
    macro_rules! assert_iter_eq {
        ($left:expr, $right:expr) => {
            assert_eq!($left.collect::<Vec<_>>(), $right);
        };
    }

    #[test]
    fn ascii_space_empty() {
        assert_iter_eq!(AsciiSpace.find_words(""), vec![]);
    }

    #[test]
    fn ascii_space_single_word() {
        assert_iter_eq!(AsciiSpace.find_words("foo"), vec![Word::from("foo")]);
    }

    #[test]
    fn ascii_space_two_words() {
        assert_iter_eq!(
            AsciiSpace.find_words("foo bar"),
            vec![Word::from("foo "), Word::from("bar")]
        );
    }

    #[test]
    fn ascii_space_multiple_words() {
        assert_iter_eq!(
            AsciiSpace.find_words("foo bar baz"),
            vec![Word::from("foo "), Word::from("bar "), Word::from("baz")]
        );
    }

    #[test]
    fn ascii_space_only_whitespace() {
        assert_iter_eq!(AsciiSpace.find_words("    "), vec![Word::from("    ")]);
    }

    #[test]
    fn ascii_space_inter_word_whitespace() {
        assert_iter_eq!(
            AsciiSpace.find_words("foo   bar"),
            vec![Word::from("foo   "), Word::from("bar")]
        )
    }

    #[test]
    fn ascii_space_trailing_whitespace() {
        assert_iter_eq!(AsciiSpace.find_words("foo   "), vec![Word::from("foo   ")]);
    }

    #[test]
    fn ascii_space_leading_whitespace() {
        assert_iter_eq!(
            AsciiSpace.find_words("   foo"),
            vec![Word::from("   "), Word::from("foo")]
        );
    }

    #[test]
    fn ascii_space_multi_column_char() {
        assert_iter_eq!(
            AsciiSpace.find_words("\u{1f920}"), // cowboy emoji 🤠
            vec![Word::from("\u{1f920}")]
        );
    }

    #[test]
    fn ascii_space_hyphens() {
        assert_iter_eq!(
            AsciiSpace.find_words("foo-bar"),
            vec![Word::from("foo-bar")]
        );
        assert_iter_eq!(
            AsciiSpace.find_words("foo- bar"),
            vec![Word::from("foo- "), Word::from("bar")]
        );
        assert_iter_eq!(
            AsciiSpace.find_words("foo - bar"),
            vec![Word::from("foo "), Word::from("- "), Word::from("bar")]
        );
        assert_iter_eq!(
            AsciiSpace.find_words("foo -bar"),
            vec![Word::from("foo "), Word::from("-bar")]
        );
    }

    #[test]
    #[cfg(unix)]
    fn ascii_space_colored_text() {
        use termion::color::{Blue, Fg, Green, Reset};

        let green_hello = format!("{}Hello{} ", Fg(Green), Fg(Reset));
        let blue_world = format!("{}World!{}", Fg(Blue), Fg(Reset));
        assert_iter_eq!(
            AsciiSpace.find_words(&format!("{}{}", green_hello, blue_world)),
            vec![Word::from(&green_hello), Word::from(&blue_world)]
        );
    }

    #[test]
    fn ascii_space_color_inside_word() {
        let text = "foo\u{1b}[0m\u{1b}[32mbar\u{1b}[0mbaz";
        assert_iter_eq!(AsciiSpace.find_words(&text), vec![Word::from(text)]);
    }
}
