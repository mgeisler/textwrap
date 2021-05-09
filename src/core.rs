//! Building blocks for advanced wrapping functionality.
//!
//! The functions and structs in this module can be used to implement
//! advanced wrapping functionality when the [`wrap`](super::wrap) and
//! [`fill`](super::fill) function don't do what you want.
//!
//! In general, you want to follow these steps when wrapping
//! something:
//!
//! 1. Split your input into [`Fragment`]s. These are abstract blocks
//!    of text or content which can be wrapped into lines. See
//!    [`WordSeparator`](crate::WordSeparator) for how to do this for
//!    text.
//!
//! 2. Potentially split your fragments into smaller pieces. This
//!    allows you to implement things like hyphenation. If wrapping
//!    text, [`split_words`] can help you do this.
//!
//! 3. Potentially break apart fragments that are still too large to
//!    fit on a single line. This is implemented in [`break_words`].
//!
//! 4. Finally take your fragments and put them into lines. There are
//!    two algorithms for this: [`wrap_optimal_fit`] and
//!    [`wrap_first_fit`]. The former produces better line breaks, the
//!    latter is faster.
//!
//! 5. Iterate through the slices returned by the wrapping functions
//!    and construct your lines of output.
//!
//! Please [open an issue](https://github.com/mgeisler/textwrap/) if
//! the functionality here is not sufficient or if you have ideas for
//! improving it. We would love to hear from you!

use crate::{Options, WordSplitter};

#[cfg(feature = "smawk")]
mod optimal_fit;
#[cfg(feature = "smawk")]
pub use optimal_fit::wrap_optimal_fit;

/// The CSI or “Control Sequence Introducer” introduces an ANSI escape
/// sequence. This is typically used for colored text and will be
/// ignored when computing the text width.
const CSI: (char, char) = ('\x1b', '[');
/// The final bytes of an ANSI escape sequence must be in this range.
const ANSI_FINAL_BYTE: std::ops::RangeInclusive<char> = '\x40'..='\x7e';

/// Skip ANSI escape sequences. The `ch` is the current `char`, the
/// `chars` provide the following characters. The `chars` will be
/// modified if `ch` is the start of an ANSI escape sequence.
#[inline]
pub(crate) fn skip_ansi_escape_sequence<I: Iterator<Item = char>>(ch: char, chars: &mut I) -> bool {
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

#[cfg(feature = "unicode-width")]
#[inline]
fn ch_width(ch: char) -> usize {
    unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0)
}

/// First character which [`ch_width`] will classify as double-width.
/// Please see [`display_width`].
#[cfg(not(feature = "unicode-width"))]
const DOUBLE_WIDTH_CUTOFF: char = '\u{1100}';

#[cfg(not(feature = "unicode-width"))]
#[inline]
fn ch_width(ch: char) -> usize {
    if ch < DOUBLE_WIDTH_CUTOFF {
        1
    } else {
        2
    }
}

/// Compute the display width of `text` while skipping over ANSI
/// escape sequences.
///
/// # Examples
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("Café Plain"), 10);
/// assert_eq!(display_width("\u{1b}[31mCafé Rouge\u{1b}[0m"), 10);
/// ```
///
/// **Note:** When the `unicode-width` Cargo feature is disabled, the
/// width of a `char` is determined by a crude approximation which
/// simply counts chars below U+1100 as 1 column wide, and all other
/// characters as 2 columns wide. With the feature enabled, function
/// will correctly deal with [combining characters] in their
/// decomposed form (see [Unicode equivalence]).
///
/// An example of a decomposed character is “é”, which can be
/// decomposed into: “e” followed by a combining acute accent: “◌́”.
/// Without the `unicode-width` Cargo feature, every `char` below
/// U+1100 has a width of 1. This includes the combining accent:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("Cafe Plain"), 10);
/// #[cfg(feature = "unicode-width")]
/// assert_eq!(display_width("Cafe\u{301} Plain"), 10);
/// #[cfg(not(feature = "unicode-width"))]
/// assert_eq!(display_width("Cafe\u{301} Plain"), 11);
/// ```
///
/// ## Emojis and CJK Characters
///
/// Characters such as emojis and [CJK characters] used in the
/// Chinese, Japanese, and Korean langauges are seen as double-width,
/// even if the `unicode-width` feature is disabled:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
/// assert_eq!(display_width("你好"), 4);  // “Nǐ hǎo” or “Hello” in Chinese
/// ```
///
/// # Limitations
///
/// The displayed width of a string cannot always be computed from the
/// string alone. This is because the width depends on the rendering
/// engine used. This is particularly visible with [emoji modifier
/// sequences] where a base emoji is modified with, e.g., skin tone or
/// hair color modifiers. It is up to the rendering engine to detect
/// this and to produce a suitable emoji.
///
/// A simple example is “❤️”, which consists of “❤” (U+2764: Black
/// Heart Symbol) followed by U+FE0F (Variation Selector-16). By
/// itself, “❤” is a black heart, but if you follow it with the
/// variant selector, you may get a wider red heart.
///
/// A more complex example would be “👨‍🦰” which should depict a man
/// with red hair. Here the computed width is too large — and the
/// width differs depending on the use of the `unicode-width` feature:
///
/// ```
/// use textwrap::core::display_width;
///
/// assert_eq!("👨‍🦰".chars().collect::<Vec<char>>(), ['\u{1f468}', '\u{200d}', '\u{1f9b0}']);
/// #[cfg(feature = "unicode-width")]
/// assert_eq!(display_width("👨‍🦰"), 4);
/// #[cfg(not(feature = "unicode-width"))]
/// assert_eq!(display_width("👨‍🦰"), 6);
/// ```
///
/// This happens because the grapheme consists of three code points:
/// “👨” (U+1F468: Man), Zero Width Joiner (U+200D), and “🦰”
/// (U+1F9B0: Red Hair). You can see them above in the test. With
/// `unicode-width` enabled, the ZWJ is correctly seen as having zero
/// width, without it is counted as a double-width character.
///
/// ## Terminal Support
///
/// Modern browsers typically do a great job at combining characters
/// as shown above, but terminals often struggle more. As an example,
/// Gnome Terminal version 3.38.1, shows “❤️” as a big red heart, but
/// shows "👨‍🦰" as “👨🦰”.
///
/// [combining characters]: https://en.wikipedia.org/wiki/Combining_character
/// [Unicode equivalence]: https://en.wikipedia.org/wiki/Unicode_equivalence
/// [CJK characters]: https://en.wikipedia.org/wiki/CJK_characters
/// [emoji modifier sequences]: https://unicode.org/emoji/charts/full-emoji-modifiers.html
pub fn display_width(text: &str) -> usize {
    let mut chars = text.chars();
    let mut width = 0;
    while let Some(ch) = chars.next() {
        if skip_ansi_escape_sequence(ch, &mut chars) {
            continue;
        }
        width += ch_width(ch);
    }
    width
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

/// The string following a word
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PostFix<'a> {
    /// Whitespace to insert, if the word does not fall at the end of a line.
    WhiteSpace(&'a str),
    /// Penalty string to insert, if the word falls at the end of a line or ends on a hyphen.
    Penalty(&'a str),
}

impl<'a> PostFix<'a> {
    /// Creates a penalty for the given `word`.
    /// # Returns
    /// Returns an empty penalty if the word ends on a hyphen `'-'`anyway.
    /// Returns a hyphen penalty otherwise
    pub fn new_penalty(word: &str) -> Self {
        let need_hyphen = !word.ends_with('-');
        PostFix::Penalty(if need_hyphen { "-" } else { "" })
    }

    /// Returns the length of the white space or `0`, if the postfix is penalty
    pub fn whitespace_len(&self) -> usize {
        match self {
            PostFix::WhiteSpace(w) => w.len(),
            PostFix::Penalty(_) => 0,
        }
    }

    /// Returns the length of the penalty string or `0`, if the postfix is a white space
    pub fn penalty_len(&self) -> usize {
        match self {
            PostFix::WhiteSpace(_) => 0,
            PostFix::Penalty(p) => p.len(),
        }
    }

    /// Returns `Some` with the penalty string, if the postfix is a non-empty string.
    /// Returns `None` otherwise
    pub fn try_penalty(&self) -> Option<&'a str> {
        match self {
            PostFix::WhiteSpace(_) => None,
            PostFix::Penalty(p) => {
                if p.is_empty() {
                    None
                } else {
                    Some(*p)
                }
            }
        }
    }

    /// Returns `true`, if the postfix is a non-empty penalty
    pub fn is_penalty(&self) -> bool {
        matches!(self, PostFix::Penalty(p) if !p.is_empty())
    }

    /// Returns `true`, if the postfix is a whitespace
    pub fn is_whitespace(&self) -> bool {
        matches!(self, PostFix::WhiteSpace(_))
    }
}

/// A piece of wrappable text, including any trailing whitespace.
///
/// A `Word` is an example of a [`Fragment`], so it has a width,
/// trailing whitespace, and potentially a penalty item.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Word<'a> {
    /// Word content.
    pub word: &'a str,
    /// String to insert, depending on whether the word does or doesn't fall at the end of a line.
    pub post_fix: PostFix<'a>,
    // Cached width in columns.
    width: usize,
}

impl std::ops::Deref for Word<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.word
    }
}

impl<'a> Word<'a> {
    /// Construct a `Word` from a string.
    ///
    /// A trailing stretch of `' '` is automatically taken to be the
    /// whitespace part of the word.
    pub fn from(word: &str) -> Word<'_> {
        let trimmed = word.trim_end_matches(' ');
        let post_fix = if trimmed.len() == word.len() {
            if word.ends_with('-') {
                PostFix::Penalty("")
            } else {
                PostFix::WhiteSpace("")
            }
        } else {
            PostFix::WhiteSpace(&word[trimmed.len()..])
        };
        Word {
            word: trimmed,
            width: display_width(&trimmed),
            post_fix,
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
    /// assert_eq!(
    ///     Word::from("Hello!  ").break_apart(3).collect::<Vec<_>>(),
    ///     vec![Word::from("Hel"), Word::from("lo!  ")]
    /// );
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

                if width > 0 && width + ch_width(ch) > line_width {
                    let word_segment = &self.word[offset..idx];
                    let word = Word {
                        word: word_segment,
                        width,
                        post_fix: if word_segment.ends_with('-') {
                            PostFix::Penalty("")
                        } else {
                            PostFix::WhiteSpace("")
                        },
                    };
                    offset = idx;
                    width = ch_width(ch);
                    return Some(word);
                }

                width += ch_width(ch);
            }

            if offset < self.word.len() {
                let word = Word {
                    word: &self.word[offset..],
                    width,
                    post_fix: self.post_fix,
                };
                offset = self.word.len();
                return Some(word);
            }

            None
        })
    }
}

impl Fragment for Word<'_> {
    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    // We assume the whitespace consist of ' ' only. This allows us to
    // compute the display width in constant time.
    #[inline]
    fn whitespace_width(&self) -> usize {
        self.post_fix.whitespace_len()
    }

    // We assume the penalty is `""` or `"-"`. This allows us to
    // compute the display width in constant time.
    #[inline]
    fn penalty_width(&self) -> usize {
        self.post_fix.penalty_len()
    }
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
/// use textwrap::core::{split_words, Word};
/// use textwrap::{NoHyphenation, Options};
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
pub fn split_words<'a, I, R, S>(
    words: I,
    options: &'a Options<'a, R, S>,
) -> impl Iterator<Item = Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>,
    S: WordSplitter,
{
    words.into_iter().flat_map(move |word| {
        let mut prev = 0;
        let mut split_points = options.splitter.split_points(&word).into_iter();
        std::iter::from_fn(move || {
            if let Some(idx) = split_points.next() {
                let w = Word {
                    word: &word.word[prev..idx],
                    width: display_width(&word[prev..idx]),
                    post_fix: PostFix::new_penalty(&word[..idx]),
                };
                prev = idx;
                return Some(w);
            }

            if prev < word.word.len() || prev == 0 {
                let w = Word {
                    word: &word.word[prev..],
                    width: display_width(&word[prev..]),
                    post_fix: word.post_fix,
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
/// This simply calls [`Word::break_apart`] on words that are too
/// wide. This means that no extra `'-'` is inserted, the word is
/// simply broken into smaller pieces.
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

/// Wrapping algorithms.
///
/// After a text has been broken into [`Fragment`]s, the one now has
/// to decide how to break the fragments into lines. The simplest
/// algorithm for this is implemented by [`wrap_first_fit`]: it uses
/// no look-ahead and simply adds fragments to the line as long as
/// they fit. However, this can lead to poor line breaks if a large
/// fragment almost-but-not-quite fits on a line. When that happens,
/// the fragment is moved to the next line and it will leave behind a
/// large gap. A more advanced algorithm, implemented by
/// [`wrap_optimal_fit`], will take this into account. The optimal-fit
/// algorithm considers all possible line breaks and will attempt to
/// minimize the gaps left behind by overly short lines.
///
/// While both algorithms run in linear time, the first-fit algorithm
/// is about 4 times faster than the optimal-fit algorithm.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WrapAlgorithm {
    /// Use an advanced algorithm which considers the entire paragraph
    /// to find optimal line breaks. Implemented by
    /// [`wrap_optimal_fit`].
    ///
    /// **Note:** Only available when the `smawk` Cargo feature is
    /// enabled.
    #[cfg(feature = "smawk")]
    OptimalFit,
    /// Use a fast and simple algorithm with no look-ahead to find
    /// line breaks. Implemented by [`wrap_first_fit`].
    FirstFit,
}

/// Wrap abstract fragments into lines with a first-fit algorithm.
///
/// The `line_widths` map line numbers (starting from 0) to a target
/// line width. This can be used to implement hanging indentation.
///
/// The fragments must already have been split into the desired
/// widths, this function will not (and cannot) attempt to split them
/// further when arranging them into lines.
///
/// # First-Fit Algorithm
///
/// This implements a simple “greedy” algorithm: accumulate fragments
/// one by one and when a fragment no longer fits, start a new line.
/// There is no look-ahead, we simply take first fit of the fragments
/// we find.
///
/// While fast and predictable, this algorithm can produce poor line
/// breaks when a long fragment is moved to a new line, leaving behind
/// a large gap:
///
/// ```
/// use textwrap::core::{wrap_first_fit, Word};
/// use textwrap::{AsciiSpace, WordSeparator};
///
/// // Helper to convert wrapped lines to a Vec<String>.
/// fn lines_to_strings(lines: Vec<&[Word<'_>]>) -> Vec<String> {
///     lines.iter().map(|line| {
///         line.iter().map(|word| &**word).collect::<Vec<_>>().join(" ")
///     }).collect::<Vec<_>>()
/// }
///
/// let text = "These few words will unfortunately not wrap nicely.";
/// let words = AsciiSpace.find_words(text).collect::<Vec<_>>();
/// assert_eq!(lines_to_strings(wrap_first_fit(&words, |_| 15)),
///            vec!["These few words",
///                 "will",  // <-- short line
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
///
/// // We can avoid the short line if we look ahead:
/// #[cfg(feature = "smawk")]
/// assert_eq!(lines_to_strings(textwrap::core::wrap_optimal_fit(&words, |_| 15)),
///            vec!["These few",
///                 "words will",
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
/// ```
///
/// The [`wrap_optimal_fit`] function was used above to get better
/// line breaks. It uses an advanced algorithm which tries to avoid
/// short lines. This function is about 4 times faster than
/// [`wrap_optimal_fit`].
///
/// # Examples
///
/// Imagine you're building a house site and you have a number of
/// tasks you need to execute. Things like pour foundation, complete
/// framing, install plumbing, electric cabling, install insulation.
///
/// The construction workers can only work during daytime, so they
/// need to pack up everything at night. Because they need to secure
/// their tools and move machines back to the garage, this process
/// takes much more time than the time it would take them to simply
/// switch to another task.
///
/// You would like to make a list of tasks to execute every day based
/// on your estimates. You can model this with a program like this:
///
/// ```
/// use textwrap::core::{wrap_first_fit, Fragment};
///
/// #[derive(Debug)]
/// struct Task<'a> {
///     name: &'a str,
///     hours: usize,   // Time needed to complete task.
///     sweep: usize,   // Time needed for a quick sweep after task during the day.
///     cleanup: usize, // Time needed for full cleanup if day ends with this task.
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
/// // Fill tasks into days, taking `day_length` into account. The
/// // output shows the hours worked per day along with the names of
/// // the tasks for that day.
/// fn assign_days<'a>(tasks: &[Task<'a>], day_length: usize) -> Vec<(usize, Vec<&'a str>)> {
///     let mut days = Vec::new();
///     // Assign tasks to days. The assignment is a vector of slices,
///     // with a slice per day.
///     let assigned_days: Vec<&[Task<'a>]> = wrap_first_fit(&tasks, |i| day_length);
///     for day in assigned_days.iter() {
///         let last = day.last().unwrap();
///         let work_hours: usize = day.iter().map(|t| t.hours + t.sweep).sum();
///         let names = day.iter().map(|t| t.name).collect::<Vec<_>>();
///         days.push((work_hours - last.sweep + last.cleanup, names));
///     }
///     days
/// }
///
/// // With a single crew working 8 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 8),
///     [
///         (7, vec!["Foundation"]),
///         (8, vec!["Framing", "Plumbing"]),
///         (7, vec!["Electrical", "Insulation"]),
///         (5, vec!["Drywall"]),
///         (7, vec!["Floors", "Countertops"]),
///         (4, vec!["Bathrooms"]),
///     ]
/// );
///
/// // With two crews working in shifts, 16 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 16),
///     [
///         (14, vec!["Foundation", "Framing", "Plumbing"]),
///         (15, vec!["Electrical", "Insulation", "Drywall", "Floors"]),
///         (6, vec!["Countertops", "Bathrooms"]),
///     ]
/// );
/// ```
///
/// Apologies to anyone who actually knows how to build a house and
/// knows how long each step takes :-)
pub fn wrap_first_fit<T: Fragment, F: Fn(usize) -> usize>(
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

    #[cfg(feature = "unicode-width")]
    use unicode_width::UnicodeWidthChar;

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
    fn emojis_have_correct_width() {
        use unic_emoji_char::is_emoji;

        // Emojis in the Basic Latin (ASCII) and Latin-1 Supplement
        // blocks all have a width of 1 column. This includes
        // characters such as '#' and '©'.
        for ch in '\u{1}'..'\u{FF}' {
            if is_emoji(ch) {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);

                #[cfg(feature = "unicode-width")]
                assert_eq!(ch.width().unwrap(), 1, "char: {}", desc);

                #[cfg(not(feature = "unicode-width"))]
                assert_eq!(ch_width(ch), 1, "char: {}", desc);
            }
        }

        // Emojis in the remaining blocks of the Basic Multilingual
        // Plane (BMP), in the Supplementary Multilingual Plane (SMP),
        // and in the Supplementary Ideographic Plane (SIP), are all 1
        // or 2 columns wide when unicode-width is used, and always 2
        // columns wide otherwise. This includes all of our favorite
        // emojis such as 😊.
        for ch in '\u{FF}'..'\u{2FFFF}' {
            if is_emoji(ch) {
                let desc = format!("{:?} U+{:04X}", ch, ch as u32);

                #[cfg(feature = "unicode-width")]
                assert!(ch.width().unwrap() <= 2, "char: {}", desc);

                #[cfg(not(feature = "unicode-width"))]
                assert_eq!(ch_width(ch), 2, "char: {}", desc);
            }
        }

        // The remaining planes contain almost no assigned code points
        // and thus also no emojis.
    }

    #[test]
    fn display_width_works() {
        assert_eq!("Café Plain".len(), 11); // “é” is two bytes
        assert_eq!(display_width("Café Plain"), 10);
        assert_eq!(display_width("\u{1b}[31mCafé Rouge\u{1b}[0m"), 10);
    }

    #[test]
    fn display_width_narrow_emojis() {
        #[cfg(feature = "unicode-width")]
        assert_eq!(display_width("⁉"), 1);

        // The ⁉ character is above DOUBLE_WIDTH_CUTOFF.
        #[cfg(not(feature = "unicode-width"))]
        assert_eq!(display_width("⁉"), 2);
    }

    #[test]
    fn display_width_narrow_emojis_variant_selector() {
        #[cfg(feature = "unicode-width")]
        assert_eq!(display_width("⁉\u{fe0f}"), 1);

        // The variant selector-16 is also counted.
        #[cfg(not(feature = "unicode-width"))]
        assert_eq!(display_width("⁉\u{fe0f}"), 4);
    }

    #[test]
    fn display_width_emojis() {
        assert_eq!(display_width("😂😭🥺🤣✨😍🙏🥰😊🔥"), 20);
    }

    #[test]
    fn split_words_no_words() {
        assert_iter_eq!(split_words(vec![], &Options::new(80)), vec![]);
    }

    #[test]
    fn split_words_empty_word() {
        assert_iter_eq!(
            split_words(vec![Word::from("   ")], &Options::new(80)),
            vec![Word::from("   ")]
        );
    }

    #[test]
    fn split_words_hyphen_splitter() {
        assert_iter_eq!(
            split_words(vec![Word::from("foo-bar")], &Options::new(80)),
            vec![Word::from("foo-"), Word::from("bar")]
        );
    }

    #[test]
    fn split_words_short_line() {
        // Note that `split_words` does not take the line width into
        // account, that is the job of `break_words`.
        assert_iter_eq!(
            split_words(vec![Word::from("foobar")], &Options::new(3)),
            vec![Word::from("foobar")]
        );
    }

    #[test]
    fn split_words_adds_penalty() {
        #[derive(Clone, Debug)]
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
                    post_fix: PostFix::Penalty("-")
                },
                Word {
                    word: "bar",
                    width: 3,
                    post_fix: PostFix::WhiteSpace("")
                }
            ]
        );

        assert_iter_eq!(
            split_words(vec![Word::from("fo-bar")].into_iter(), &options),
            vec![
                Word {
                    word: "fo-",
                    width: 3,
                    post_fix: PostFix::Penalty("")
                },
                Word {
                    word: "bar",
                    width: 3,
                    post_fix: PostFix::WhiteSpace("")
                }
            ]
        );
    }
}
