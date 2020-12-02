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
//!    of text or content which can be wrapped into lines. You can use
//!    [`find_words`] to do this for text.
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
use std::cell::RefCell;
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

/// The CSI or “Control Sequence Introducer” introduces an ANSI escape
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
/// A `Word` is an example of a [`Fragment`], so it has a width,
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
    #[inline]
    fn width(&self) -> usize {
        self.width
    }

    // We assume the whitespace consist of ' ' only. This allows us to
    // compute the display width in constant time.
    #[inline]
    fn whitespace_width(&self) -> usize {
        self.whitespace.len()
    }

    // We assume the penalty is `""` or `"-"`. This allows us to
    // compute the display width in constant time.
    #[inline]
    fn penalty_width(&self) -> usize {
        self.penalty.len()
    }
}

/// Split line into words separated by regions of `' '` characters.
///
/// # Examples
///
/// ```
/// use textwrap::core::{find_words, Fragment, Word};
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
pub fn split_words<'a, I, S, Opt>(words: I, options: Opt) -> impl Iterator<Item = Word<'a>>
where
    I: IntoIterator<Item = Word<'a>>,
    S: WordSplitter,
    Opt: Into<Options<'a, S>>,
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
/// use textwrap::core::{find_words, wrap_first_fit, wrap_optimal_fit, Word};
///
/// // Helper to convert wrapped lines to a Vec<String>.
/// fn lines_to_strings(lines: Vec<&[Word<'_>]>) -> Vec<String> {
///     lines.iter().map(|line| {
///         line.iter().map(|word| &**word).collect::<Vec<_>>().join(" ")
///     }).collect::<Vec<_>>()
/// }
///
/// let text = "These few words will unfortunately not wrap nicely.";
/// let words = find_words(text).collect::<Vec<_>>();
/// assert_eq!(lines_to_strings(wrap_first_fit(&words, |_| 15)),
///            vec!["These few words",
///                 "will",  // <-- short line
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
///
/// // We can avoid the short line if we look ahead:
/// assert_eq!(lines_to_strings(wrap_optimal_fit(&words, |_| 15)),
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
/// use textwrap::core::{wrap_first_fit, Fragment};
///
/// #[derive(Debug)]
/// struct Task<'a> {
///     name: &'a str,
///     hours: usize,   // Time needed to complete task.
///     sweep: usize,   // Time needed for a quick sweep after task during the day.
///     cleanup: usize, // Time needed to cleanup after task at end of day.
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
///     for day in wrap_first_fit(&tasks, |i| day_length) {
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

/// Cache for line numbers. This is necessary to avoid a O(n**2)
/// behavior when computing line numbers in [`wrap_optimal_fit`].
struct LineNumbers {
    line_numbers: RefCell<Vec<usize>>,
}

impl LineNumbers {
    fn new(size: usize) -> Self {
        let mut line_numbers = Vec::with_capacity(size);
        line_numbers.push(0);
        LineNumbers {
            line_numbers: RefCell::new(line_numbers),
        }
    }

    fn get(&self, i: usize, minima: &[(usize, i32)]) -> usize {
        while self.line_numbers.borrow_mut().len() < i + 1 {
            let pos = self.line_numbers.borrow().len();
            let line_number = 1 + self.get(minima[pos].0, &minima);
            self.line_numbers.borrow_mut().push(line_number);
        }

        self.line_numbers.borrow()[i]
    }
}

/// Per-line penalty. This is added for every line, which makes it
/// expensive to output more lines than the minimum required.
const NLINE_PENALTY: i32 = 1000;

/// Per-character cost for lines that overflow the target line width.
///
/// With a value of 50², every single character costs as much as
/// leaving a gap of 50 characters behind. This is becuase we assign
/// as cost of `gap * gap` to a short line. This means that we can
/// overflow the line by 1 character in extreme cases:
///
/// ```
/// use textwrap::core::{wrap_optimal_fit, Word};
///
/// let short = "foo ";
/// let long = "x".repeat(50);
/// let fragments = vec![Word::from(short), Word::from(&long)];
///
/// // Perfect fit, both words are on a single line with no overflow.
/// let wrapped = wrap_optimal_fit(&fragments, |_| short.len() + long.len());
/// assert_eq!(wrapped, vec![&[Word::from(short), Word::from(&long)]]);
///
/// // The words no longer fit, yet we get a single line back. While
/// // the cost of overflow (`1 * 2500`) is the same as the cost of the
/// // gap (`50 * 50 = 2500`), the tie is broken by `NLINE_PENALTY`
/// // which makes it cheaper to overflow than to use two lines.
/// let wrapped = wrap_optimal_fit(&fragments, |_| short.len() + long.len() - 1);
/// assert_eq!(wrapped, vec![&[Word::from(short), Word::from(&long)]]);
///
/// // The cost of overflow would be 2 * 2500, whereas the cost of the
/// // gap is only `49 * 49 + NLINE_PENALTY = 2401 + 1000 = 3401`. We
/// // therefore get two lines.
/// let wrapped = wrap_optimal_fit(&fragments, |_| short.len() + long.len() - 2);
/// assert_eq!(wrapped, vec![&[Word::from(short)],
///                          &[Word::from(&long)]]);
/// ```
///
/// This only happens if the overflowing word is 50 characters long
/// _and_ if it happens to overflow the line by exactly one character.
/// If it overflows by more than one character, the overflow penalty
/// will quickly outgrow the cost of the gap, as seen above.
const OVERFLOW_PENALTY: i32 = 50 * 50;

/// The last line is short if it is less than 1/4 of the target width.
const SHORT_LINE_FRACTION: usize = 4;

/// Penalize a short last line.
const SHORT_LAST_LINE_PENALTY: i32 = 25;

/// Penalty for lines ending with a hyphen.
const HYPHEN_PENALTY: i32 = 25;

/// Wrap abstract fragments into lines with an optimal-fit algorithm.
///
/// The `line_widths` map line numbers (starting from 0) to a target
/// line width. This can be used to implement hanging indentation.
///
/// The fragments must already have been split into the desired
/// widths, this function will not (and cannot) attempt to split them
/// further when arranging them into lines.
///
/// # Optimal-Fit Algorithm
///
/// The algorithm considers all possible break points and picks the
/// breaks which minimizes the gaps at the end of each line. More
/// precisely, the algorithm assigns a cost or penalty to each break
/// point, determined by `cost = gap * gap` where `gap = target_width -
/// line_width`. Shorter lines are thus penalized more heavily since
/// they leave behind a larger gap.
///
/// We can illustrate this with the text “To be, or not to be: that is
/// the question”. We will be wrapping it in a narrow column with room
/// for only 10 characters. The [greedy algorithm](wrap_first_fit)
/// will produce these lines, each annotated with the corresponding
/// penalty:
///
/// ```text
/// "To be, or"   1² =  1
/// "not to be:"  0² =  0
/// "that is"     3² =  9
/// "the"         7² = 49
/// "question"    2² =  4
/// ```
///
/// We see that line four with “the” leaves a gap of 7 columns, which
/// gives it a penalty of 49. The sum of the penalties is 63.
///
/// There are 10 words, which means that there are `2_u32.pow(9)` or
/// 512 different ways to typeset it. We can compute
/// the sum of the penalties for each possible line break and search
/// for the one with the lowest sum:
///
/// ```text
/// "To be,"     4² = 16
/// "or not to"  1² =  1
/// "be: that"   2² =  4
/// "is the"     4² = 16
/// "question"   2² =  4
/// ```
///
/// The sum of the penalties is 41, which is better than what the
/// greedy algorithm produced.
///
/// Searching through all possible combinations would normally be
/// prohibitively slow. However, it turns out that the problem can be
/// formulated as the task of finding column minima in a cost matrix.
/// This matrix has a special form (totally monotone) which lets us
/// use a [linear-time algorithm called
/// SMAWK](https://lib.rs/crates/smawk) to find the optimal break
/// points.
///
/// This means that the time complexity remains O(_n_) where _n_ is
/// the number of words. Compared to [`wrap_first_fit`], this function
/// is about 4 times slower.
///
/// The use of penalties is inspired by the line breaking algorithm
/// used TeX, described in the 1981 article [_Breaking Paragraphs into
/// Lines_](http://www.eprg.org/G53DOC/pdfs/knuth-plass-breaking.pdf)
/// by Knuth and Plass. The implementation here is based on [Python
/// code by David
/// Eppstein](https://github.com/jfinkels/PADS/blob/master/pads/wrap.py).
pub fn wrap_optimal_fit<'a, T: Fragment, F: Fn(usize) -> usize>(
    fragments: &'a [T],
    line_widths: F,
) -> Vec<&'a [T]> {
    let mut widths = Vec::with_capacity(fragments.len() + 1);
    let mut width = 0;
    widths.push(width);
    for fragment in fragments {
        width += fragment.width() + fragment.whitespace_width();
        widths.push(width);
    }

    let line_numbers = LineNumbers::new(fragments.len());

    let minima = smawk::online_column_minima(0, widths.len(), |minima, i, j| {
        // Line number for fragment `i`.
        let line_number = line_numbers.get(i, &minima);
        let target_width = std::cmp::max(1, line_widths(line_number));

        // Compute the width of a line spanning fragments[i..j] in
        // constant time. We need to adjust widths[j] by subtracting
        // the whitespace of fragment[j-i] and then add the penalty.
        let line_width = widths[j] - widths[i] - fragments[j - 1].whitespace_width()
            + fragments[j - 1].penalty_width();

        // We compute cost of the line containing fragments[i..j]. We
        // start with values[i].1, which is the optimal cost for
        // breaking before fragments[i].
        //
        // First, every extra line cost NLINE_PENALTY.
        let mut cost = minima[i].1 + NLINE_PENALTY;

        // Next, we add a penalty depending on the line length.
        if line_width > target_width {
            // Lines that overflow get a hefty penalty.
            let overflow = (line_width - target_width) as i32;
            cost += overflow * OVERFLOW_PENALTY;
        } else if j < fragments.len() {
            // Other lines (except for the last line) get a milder
            // penalty which depend on the size of the gap.
            let gap = (target_width - line_width) as i32;
            cost += gap * gap;
        } else if i + 1 == j && line_width < target_width / SHORT_LINE_FRACTION {
            // The last line can have any size gap, but we do add a
            // penalty if the line is very short (typically because it
            // contains just a single word).
            cost += SHORT_LAST_LINE_PENALTY;
        }

        // Finally, we discourage hyphens.
        if fragments[j - 1].penalty_width() > 0 {
            // TODO: this should use a penalty value from the fragment
            // instead.
            cost += HYPHEN_PENALTY;
        }

        cost
    });

    let mut lines = Vec::with_capacity(line_numbers.get(fragments.len(), &minima));
    let mut pos = fragments.len();
    loop {
        let prev = minima[pos].0;
        lines.push(&fragments[prev..pos]);
        pos = prev;
        if pos == 0 {
            break;
        }
    }

    lines.reverse();
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
