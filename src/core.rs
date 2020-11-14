//! Building blocks for advanced wrapping functionality.
//!
//! The functions and structs in this module can be used to implement
//! advanced wrapping functionality when the `wrap` and `fill`
//! function don't do what you want.

use crate::{Options, WordSplitter};
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// The CSI or "Control Sequence Introducer" introduces an ANSI escape
/// sequence. This is typically used for colored text and will be
/// ignored when computing the text width.
const CSI: (char, char) = ('\x1b', '[');
/// The final bytes of an ANSI escape sequence must be in this range.
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char> = '\x40'..='\x7e';

/// Skip ANSI escape sequences. The `ch` is the current `char`, the
/// `chars` provide the following characters. The `chars` will be
/// modified if `ch` is the start of an ANSI escape sequence.
fn skip_ansi_escape_sequence<I: Iterator<Item = char>>(ch: char, chars: &mut I) -> bool {
    if ch == CSI.0 && chars.next() == Some(CSI.1) {
        // We have found the start of an ANSI escape code, typically
        // used for colored terminal text. We skip until we find a
        // "final byte" in the range 0x40–0x7E.
        for ch in chars {
            if ANSI_FINAL_BYTE.contains(&ch) {
                return true;
            }
        }
    }
    false
}

/// A (text) fragment denotes the unit which we wrap into lines.
///
/// Fragments represent an abstract _word_ plus the _whitespace_
/// following the word. In case the word falls at the end of the line,
/// the whitespace is dropped and a so-called _penalty_ is inserted
/// instead (typically `"-"` if the word was hyphenated).
///
/// For wrapping purposes, the precise content of the word, the
/// whitespace, and the penalty is irrelevant. All we need to know is
/// the displayed width of each part, which this trait provides.
pub trait Fragment: std::fmt::Debug {
    /// Displayed width of word represented by this fragment.
    fn width(&self) -> usize;

    /// Displayed width of the whitespace that must follow the word
    /// when the word is not at the end of a line.
    fn whitespace_width(&self) -> usize;

    /// Displayed width of the penalty that must be inserted if the
    /// word falls at the end of a line.
    fn penalty_width(&self) -> usize;
}

/// A piece of wrappable text, including any trailing whitespace.
///
/// A `Word` is an example of a `Fragment`, so it has a width,
/// trailing whitespace, and potentially a penalty item.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    word: &'a str,
    width: usize,
    pub(crate) whitespace: &'a str,
    pub(crate) penalty: &'a str,
}

impl std::ops::Deref for Word<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.word
    }
}

impl<'a> Word<'a> {
    /// Construct a new `Word`.
    ///
    /// A trailing strech of `' '` is automatically taken to be the
    /// whitespace part of the word.
    pub fn from(word: &str) -> Word<'_> {
        let trimmed = word.trim_end_matches(' ');
        let mut chars = trimmed.chars();
        let mut width = 0;
        while let Some(ch) = chars.next() {
            if skip_ansi_escape_sequence(ch, &mut chars) {
                continue;
            };
            width += ch.width().unwrap_or(0);
        }

        Word {
            word: trimmed,
            width: width,
            whitespace: &word[trimmed.len()..],
            penalty: "",
        }
    }

    /// Break this word into smaller words with a width of at most
    /// `line_width`. The whitespace and penalty from this `Word` is
    /// added to the last piece.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::core::Word;
    /// assert_eq!(Word::from("Hello!  ").break_apart(3).collect::<Vec<_>>(),
    ///            vec![Word::from("Hel"), Word::from("lo!  ")]);
    /// ```
    pub fn break_apart<'b>(&'b self, line_width: usize) -> impl Iterator<Item = Word<'a>> + 'b {
        let mut char_indices = self.word.char_indices();
        let mut offset = 0;
        let mut width = 0;

        std::iter::from_fn(move || {
            while let Some((idx, ch)) = char_indices.next() {
                if skip_ansi_escape_sequence(ch, &mut char_indices.by_ref().map(|(_, ch)| ch)) {
                    continue;
                }

                let ch_width = ch.width().unwrap_or(0);
                if width > 0 && width + ch_width > line_width {
                    let word = Word {
                        word: &self.word[offset..idx],
                        width: width,
                        whitespace: "",
                        penalty: "",
                    };
                    offset = idx;
                    width = ch_width;
                    return Some(word);
                }

                width += ch_width;
            }

            if offset < self.word.len() {
                let word = Word {
                    word: &self.word[offset..],
                    width: width,
                    whitespace: self.whitespace,
                    penalty: self.penalty,
                };
                offset = self.word.len();
                return Some(word);
            }

            None
        })
    }
}

impl Fragment for Word<'_> {
    fn width(&self) -> usize {
        self.width
    }

    // We assume the whitespace consist of ' ' only. This allows us to
    // compute the display width in constant time.
    fn whitespace_width(&self) -> usize {
        self.whitespace.len()
    }

    // We assume the penalty is `""` or `"-"`. This allows us to
    // compute the display width in constant time.
    fn penalty_width(&self) -> usize {
        self.penalty.len()
    }
}

/// Split line into words separated by regions of `' '` characters.
///
/// # Examples
///
/// ```
/// use textwrap::core::{Fragment, Word, find_words};
/// let words = find_words("Hello World!").collect::<Vec<_>>();
/// assert_eq!(words, vec![Word::from("Hello "), Word::from("World!")]);
/// assert_eq!(words[0].width(), 5);
/// assert_eq!(words[0].whitespace_width(), 1);
/// assert_eq!(words[0].penalty_width(), 0);
/// ```
pub fn find_words(line: &str) -> impl Iterator<Item = Word> {
    let mut start = 0;
    let mut in_whitespace = false;
    let mut char_indices = line.char_indices();

    std::iter::from_fn(move || {
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
    })
}

/// Split words into smaller words according to the split points given
/// by `options`.
///
/// Note that we split all words, regardless of their length. This is
/// to more cleanly separate the business of splitting (including
/// automatic hyphenation) from the business of word wrapping.
///
/// # Examples
///
/// ```
/// use textwrap::{Options, NoHyphenation};
/// use textwrap::core::{Word, split_words};
///
/// // The default splitter is HyphenSplitter:
/// let options = Options::new(80);
/// assert_eq!(
///     split_words(vec![Word::from("foo-bar")], &options).collect::<Vec<_>>(),
///     vec![Word::from("foo-"), Word::from("bar")]
/// );
///
/// // The NoHyphenation splitter ignores the '-':
/// let options = Options::new(80).splitter(NoHyphenation);
/// assert_eq!(
///     split_words(vec![Word::from("foo-bar")], &options).collect::<Vec<_>>(),
///     vec![Word::from("foo-bar")]
/// );
/// ```
pub fn split_words<'a, I, S: WordSplitter, T: Into<crate::Options<'a, S>>>(
    words: I,
    options: T,
) -> impl Iterator<Item = Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>,
{
    let options = options.into();

    words.into_iter().flat_map(move |word| {
        let mut prev = 0;
        let mut split_points = options.splitter.split_points(&word).into_iter();
        std::iter::from_fn(move || {
            if let Some(idx) = split_points.next() {
                let need_hyphen = !word[..idx].ends_with('-');
                let w = Word {
                    word: &word.word[prev..idx],
                    width: word[prev..idx].width(),
                    whitespace: "",
                    penalty: if need_hyphen { "-" } else { "" },
                };
                prev = idx;
                return Some(w);
            }

            if prev < word.word.len() || prev == 0 {
                let w = Word {
                    word: &word.word[prev..],
                    width: word[prev..].width(),
                    whitespace: word.whitespace,
                    penalty: word.penalty,
                };
                prev = word.word.len() + 1;
                return Some(w);
            }

            None
        })
    })
}

/// Forcibly break words wider than `line_width` into smaller words.
///
/// This simply calls `Word::break_apart` on words that are too wide.
/// This means that no extra `'-'` is inserted, the word is simply
/// broken into smaller pieces.
pub fn break_words<'a, I>(words: I, line_width: usize) -> Vec<Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>,
{
    let mut shortened_words = Vec::new();
    for word in words {
        if word.width() > line_width {
            shortened_words.extend(word.break_apart(line_width));
        } else {
            shortened_words.push(word);
        }
    }
    shortened_words
}

/// Wrap abstract fragments into lines of differnet widths.
///
/// The `line_widths` maps the line number to the desired width. This
/// can be used to implement hanging indentation.
///
/// The fragments must already have been split into the desired
/// widths, this function will not (and cannot) attempt to split them
/// further when arranging them into lines.
///
/// # Examples
///
/// Imagine you're building a house site and you have a number of
/// tasks you need to execute. Things like pour foundation, complete
/// framing, install plumbing, electric cabling, install insolutation.
///
/// The construction workers can only work during daytime, so they
/// need to pack up everything at night. Because they need to secure
/// their tools and move machines back to the garage, this process
/// takes much more time than the time it would take them to simply
/// switch to another task.
///
/// You would like to make a list of taks to execute every day based
/// on your estimates. You can model this with a program like this:
///
/// ```
/// use textwrap::core::{Fragment, wrap_fragments};
///
/// #[derive(Debug)]
/// struct Task<'a> {
///      name: &'a str,
///      hours: usize,    // Time needed to complete task.
///      sweep: usize,    // Time needed for a quick sweep after task during the day.
///      cleanup: usize,  // Time needed to cleanup after task at end of day.
/// }
///
/// impl Fragment for Task<'_> {
///     fn width(&self) -> usize { self.hours }
///     fn whitespace_width(&self) -> usize { self.sweep }
///     fn penalty_width(&self) -> usize { self.cleanup }
/// }
///
/// // The morning tasks
/// let tasks = vec![
///     Task { name: "Foundation",  hours: 4, sweep: 2, cleanup: 3 },
///     Task { name: "Framing",     hours: 3, sweep: 1, cleanup: 2 },
///     Task { name: "Plumbing",    hours: 2, sweep: 2, cleanup: 2 },
///     Task { name: "Electrical",  hours: 2, sweep: 1, cleanup: 2 },
///     Task { name: "Insulation",  hours: 2, sweep: 1, cleanup: 2 },
///     Task { name: "Drywall",     hours: 3, sweep: 1, cleanup: 2 },
///     Task { name: "Floors",      hours: 3, sweep: 1, cleanup: 2 },
///     Task { name: "Countertops", hours: 1, sweep: 1, cleanup: 2 },
///     Task { name: "Bathrooms",   hours: 2, sweep: 1, cleanup: 2 },
/// ];
///
/// fn assign_days<'a>(tasks: &[Task<'a>], day_length: usize) -> Vec<(usize, Vec<&'a str>)> {
///     let mut days = Vec::new();
///     for day in wrap_fragments(&tasks, |i| { day_length }) {
///         let last = day.last().unwrap();
///         let work_hours: usize = day.iter().map(|t| t.hours + t.sweep).sum();
///         let names = day.iter().map(|t| t.name).collect::<Vec<_>>();
///         days.push((work_hours - last.sweep + last.cleanup, names));
///     }
///     days
/// }
///
/// // With a single crew working 8 hours a day:
/// assert_eq!(assign_days(&tasks, 8), [
///     (7, vec!["Foundation"]),
///     (8, vec!["Framing", "Plumbing"]),
///     (7, vec!["Electrical", "Insulation"]),
///     (5, vec!["Drywall"]),
///     (7, vec!["Floors", "Countertops"]),
///     (4, vec!["Bathrooms"]),
/// ]);
///
/// // With two crews working in shifts, 16 hours a day:
/// assert_eq!(assign_days(&tasks, 16), [
///     (14, vec!["Foundation", "Framing", "Plumbing"]),
///     (15, vec!["Electrical", "Insulation", "Drywall", "Floors"]),
///      (6, vec!["Countertops", "Bathrooms"]),
/// ]);
/// ```
///
/// Apologies to anyone who actually knows how to build a house and
/// knows how long each step takes :-)
pub fn wrap_fragments<T: Fragment, F: Fn(usize) -> usize>(
    fragments: &[T],
    line_widths: F,
) -> Vec<&[T]> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut width = 0;

    for (idx, fragment) in fragments.iter().enumerate() {
        let line_width = line_widths(lines.len());
        if width + fragment.width() + fragment.penalty_width() > line_width && idx > start {
            lines.push(&fragments[start..idx]);
            start = idx;
            width = 0;
        }
        width += fragment.width() + fragment.whitespace_width();
    }
    lines.push(&fragments[start..]);
    lines
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
    fn skip_ansi_escape_sequence_works() {
        let blue_text = "\u{1b}[34mHello\u{1b}[0m";
        let mut chars = blue_text.chars();
        let ch = chars.next().unwrap();
        assert!(skip_ansi_escape_sequence(ch, &mut chars));
        assert_eq!(chars.next(), Some('H'));
    }

    #[test]
    fn find_words_empty() {
        assert_iter_eq!(find_words(""), vec![]);
    }

    #[test]
    fn find_words_single_word() {
        assert_iter_eq!(find_words("foo"), vec![Word::from("foo")]);
    }

    #[test]
    fn find_words_two_words() {
        assert_iter_eq!(
            find_words("foo bar"),
            vec![Word::from("foo "), Word::from("bar")]
        );
    }

    #[test]
    fn find_words_multiple_words() {
        assert_iter_eq!(
            find_words("foo bar baz"),
            vec![Word::from("foo "), Word::from("bar "), Word::from("baz")]
        );
    }

    #[test]
    fn find_words_whitespace() {
        assert_iter_eq!(find_words("    "), vec![Word::from("    ")]);
    }

    #[test]
    fn find_words_inter_word_whitespace() {
        assert_iter_eq!(
            find_words("foo   bar"),
            vec![Word::from("foo   "), Word::from("bar")]
        )
    }

    #[test]
    fn find_words_trailing_whitespace() {
        assert_iter_eq!(find_words("foo   "), vec![Word::from("foo   ")]);
    }

    #[test]
    fn find_words_leading_whitespace() {
        assert_iter_eq!(
            find_words("   foo"),
            vec![Word::from("   "), Word::from("foo")]
        );
    }

    #[test]
    fn find_words_multi_column_char() {
        assert_iter_eq!(
            find_words("\u{1f920}"), // cowboy emoji 🤠
            vec![Word::from("\u{1f920}")]
        );
    }

    #[test]
    fn find_words_hyphens() {
        assert_iter_eq!(find_words("foo-bar"), vec![Word::from("foo-bar")]);
        assert_iter_eq!(
            find_words("foo- bar"),
            vec![Word::from("foo- "), Word::from("bar")]
        );
        assert_iter_eq!(
            find_words("foo - bar"),
            vec![Word::from("foo "), Word::from("- "), Word::from("bar")]
        );
        assert_iter_eq!(
            find_words("foo -bar"),
            vec![Word::from("foo "), Word::from("-bar")]
        );
    }

    #[test]
    fn split_words_no_words() {
        assert_iter_eq!(split_words(vec![], 80), vec![]);
    }

    #[test]
    fn split_words_empty_word() {
        assert_iter_eq!(
            split_words(vec![Word::from("   ")], 80),
            vec![Word::from("   ")]
        );
    }

    #[test]
    fn split_words_hyphen_splitter() {
        assert_iter_eq!(
            split_words(vec![Word::from("foo-bar")], 80),
            vec![Word::from("foo-"), Word::from("bar")]
        );
    }

    #[test]
    fn split_words_short_line() {
        // Note that `split_words` does not take the line width into
        // account, that is the job of `break_words`.
        assert_iter_eq!(
            split_words(vec![Word::from("foobar")], 3),
            vec![Word::from("foobar")]
        );
    }

    #[test]
    fn split_words_adds_penalty() {
        #[derive(Debug)]
        struct FixedSplitPoint;
        impl WordSplitter for FixedSplitPoint {
            fn split_points(&self, _: &str) -> Vec<usize> {
                vec![3]
            }
        }

        let options = Options::new(80).splitter(FixedSplitPoint);
        assert_iter_eq!(
            split_words(vec![Word::from("foobar")].into_iter(), &options),
            vec![
                Word {
                    word: "foo",
                    width: 3,
                    whitespace: "",
                    penalty: "-"
                },
                Word {
                    word: "bar",
                    width: 3,
                    whitespace: "",
                    penalty: ""
                }
            ]
        );

        assert_iter_eq!(
            split_words(vec![Word::from("fo-bar")].into_iter(), &options),
            vec![
                Word {
                    word: "fo-",
                    width: 3,
                    whitespace: "",
                    penalty: ""
                },
                Word {
                    word: "bar",
                    width: 3,
                    whitespace: "",
                    penalty: ""
                }
            ]
        );
    }
}
