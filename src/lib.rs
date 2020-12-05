//! The textwrap library provides functions for word wrapping and
//! filling text.
//!
//! Wrapping text can be very useful in commandline programs where you
//! want to format dynamic output nicely so it looks good in a
//! terminal. A quick example:
//!
//! ```
//! use textwrap::plain::{self, split, width};
//!
//! fn main() {
//!     let text = "textwrap: a small library for wrapping text.";
//!     // Split the text into parts by whitespace.
//!     let parts = split::space(text);
//!     // Calculate the width of each part using its Unicode width.
//!     let fragments = parts.map(|s| s.width(width::Unicode::default()));
//!     // Wrap the fragments using a simple greedy algorithm.
//!     println!("{}", plain::concat(textwrap::wrap_greedy(fragments, std::iter::repeat(18))));
//! }
//! ```
//!
//! When you run this program, it will display the following output:
//!
//! ```text
//! textwrap: a small
//! library for
//! wrapping text.
//! ```
//!
//! # Wrapping Strings at Compile Time
//!
//! If your strings are known at compile time, please take a look at
//! the procedural macros from the [textwrap-macros] crate.
//!
//! # Architecture
//!
//! `textwrap` is designed to be highly modular. The typical usage of wrapping plaintext involves
//! several steps:
//!
//! - First, the text must be split into [`Span`](plain::Span)s. There are various methods of doing
//! this, such as [splitting on whitespace](plain::split::space) or [splitting on manual hyphens
//! too](plain::split::space_manual_hyphens).
//! - Next, each span must be converted into a [`Fragment`](plain::Fragment). This involves choosing
//! an implementor of [`plain::Width`] that can calculate the width of each span. Once again, there
//! are various methods of doing this, such as using its [Unicode width](plain::width::Unicode) - at
//! this point a graphical application will use their font.
//! - Then the actual wrapping can take place. Various algorithms can be used, currently this
//! library only provides [`wrap_greedy`] which is fast and simple but doesn't always produce the
//! most aesthetically pleasing outcome.
//! - Finally, this all must be converted into whatever output format you like, for example
//! [`lines`](plain::lines) will produce an iterator over each line, and [`concat`](plain::concat)
//! will concatenate each line into a single string separated by newlines.
//!
//! # Cargo Features
//!
//! The textwrap library has two optional features:
//!
//! * `terminal_size`: enables automatic detection of the terminal
//!   width via the [terminal_size] crate. See the
//!   [`Options::with_termwidth`] constructor for details.
//!
//! [textwrap-macros]: https://docs.rs/textwrap-macros/

#![doc(html_root_url = "https://docs.rs/textwrap/0.12.1")]
#![forbid(unsafe_code)] // See https://github.com/mgeisler/textwrap/issues/210
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![allow(clippy::redundant_field_names)]

use std::cmp;
use std::fmt::{self, Debug, Formatter};
use std::iter::{self, FusedIterator};

pub mod plain;

/// A fragment that can be placed on a line.
pub trait Fragment: Sized {
    /// Get the width of the main content of the fragment.
    #[must_use]
    fn width(&self) -> usize;
    /// Get the width of the glue of the fragment. This is displayed only when the fragment is
    /// followed by another fragment of the same width.
    ///
    /// When the fragment represents text, this is typically whitespace following each word.
    #[must_use]
    fn glue_width(&self) -> usize;
    /// Get the width of the penalty of the fragment. This is displayed only when the fragment is at
    /// the end of the line.
    ///
    /// When the fragment represents text, this is typically a hyphen.
    #[must_use]
    fn penalty_width(&self) -> usize;
    /// Attempt to forcibly break the fragment's content into two at the given width. If
    /// successful, the sum of the content and penalty width of the first fragment must be <= the
    /// width given.
    ///
    /// The default implementation fails.
    ///
    /// # Errors
    ///
    /// Fails and returns the fragment unchanged if the fragment cannot be broken.
    fn try_break(self, _total_width: usize) -> Result<(Self, Self), Self> {
        Err(self)
    }
}

/// Wrap fragments with a simple greedy algorithm.
///
/// This takes an iterator of the fragments to place and an iterator over the line widths, and
/// returns an iterator over fragments and whether they occur at the end of the line.
///
/// # Examples
///
/// ```
/// use textwrap::plain;
///
/// let fragments = plain::split::space("Lorem ipsum dolor sit amet")
///     .map(|s| s.width(plain::width::Unicode::default()));
/// for (fragment, eol) in textwrap::wrap_greedy(fragments, std::iter::repeat(11)) {
///     print!("{}", fragment.span().content);
///     if eol {
///         println!("{}", fragment.span().penalty);
///     } else {
///         print!("{}", fragment.span().glue);
///     }
/// }
/// ```
///
/// The above should output:
///
/// ```text
/// Lorem ipsum
/// dolor sit
/// amet
/// ```
#[must_use]
pub fn wrap_greedy<F, W>(fragments: F, widths: W) -> WrapGreedy<F::IntoIter, W::IntoIter>
where
    F: IntoIterator,
    <F as IntoIterator>::Item: Fragment,
    W: IntoIterator<Item = usize>,
{
    WrapGreedy {
        inner: WrapGreedyInner {
            broken_fragment: None,
            fragments: fragments.into_iter(),
            widths: widths.into_iter(),
            glue: 0,
            remaining_width: None,
        }
        .peekable(),
    }
}

#[test]
fn test_wrap_greedy() {
    let fragments = plain::split::space("Lorem ipsum dolor sit amet")
        .map(|s| s.width(plain::width::Unicode::default()));
    let s = wrap_greedy(fragments, std::iter::repeat(11))
        .flat_map(|(fragment, eol)| {
            iter::once(fragment.span().content)
                .chain(std::iter::once(if eol {
                    fragment.span().penalty
                } else {
                    fragment.span().glue
                }))
                .chain(std::iter::once(if eol { "\n" } else { "" }))
        })
        .collect::<String>();
    assert_eq!(
        s,
        "\
Lorem ipsum
dolor sit
amet
"
    );
}

/// Iterator for [`wrap_greedy`].
pub struct WrapGreedy<F, W>
where
    F: Iterator,
    <F as Iterator>::Item: Fragment,
    W: Iterator<Item = usize>,
{
    inner: iter::Peekable<WrapGreedyInner<F, W>>,
}

impl<F, W> Iterator for WrapGreedy<F, W>
where
    F: Iterator,
    <F as Iterator>::Item: Fragment,
    W: Iterator<Item = usize>,
{
    type Item = (<F as Iterator>::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let (_, fragment) = self.inner.next()?;
        let eol = self
            .inner
            .peek()
            .map(|&(newline, _)| newline)
            .unwrap_or(true);
        Some((fragment, eol))
    }
}
impl<F, W> FusedIterator for WrapGreedy<F, W>
where
    F: FusedIterator,
    <F as Iterator>::Item: Fragment,
    W: FusedIterator<Item = usize>,
{
}

// Manual Debug impl so that `<F as Iterator>::Item: Debug`.
impl<F, W> Debug for WrapGreedy<F, W>
where
    F: Iterator + Debug,
    <F as Iterator>::Item: Fragment + Debug,
    W: Iterator<Item = usize> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("WrapGreedy")
            .field("inner", &self.inner)
            .finish()
    }
}

// Manual Clone impl so that `<F as Iterator>::Item: Clone`.
impl<F, W> Clone for WrapGreedy<F, W>
where
    F: Iterator + Clone,
    <F as Iterator>::Item: Fragment + Clone,
    W: Iterator<Item = usize> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

/// Inner iterator used in [`WrapGreedy`].
///
/// This iterates over tuples of whether the fragment starts on a new line, and the fragment
/// itself.
struct WrapGreedyInner<F, W>
where
    F: Iterator,
    <F as Iterator>::Item: Fragment,
    W: Iterator<Item = usize>,
{
    /// The latter half of the fragment if it was broken.
    broken_fragment: Option<<F as Iterator>::Item>,
    fragments: F,
    widths: W,
    /// The glue of the previous fragment on the line.
    glue: usize,
    /// The remaining width in the current line. `None` if the line overflows, so not even a
    /// zero-width fragment can fit on it (unlike `Some(0)` where it can).
    remaining_width: Option<usize>,
}

impl<F, W> Iterator for WrapGreedyInner<F, W>
where
    F: Iterator,
    <F as Iterator>::Item: Fragment,
    W: Iterator<Item = usize>,
{
    type Item = (bool, <F as Iterator>::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let fragment = self
            .broken_fragment
            .take()
            .or_else(|| self.fragments.next())?;

        let new_remaining_width = self
            .remaining_width
            .and_then(|width| width.checked_sub(self.glue))
            .and_then(|width| width.checked_sub(fragment.width()))
            // Require the penalty to fit on the line; although in theory this can provide
            // suboptimal line wrapping, it avoids infinite lookahead and backtracking.
            .filter(|&width| width >= fragment.penalty_width());

        self.glue = fragment.glue_width();

        Some(match new_remaining_width {
            // The line has not overflowed.
            Some(remaining_width) => {
                self.remaining_width = Some(remaining_width);
                (false, fragment)
            }
            // The line has overflowed; move the fragment to the next line.
            None => {
                let line_width = match self.widths.next() {
                    Some(width) => width,
                    None => {
                        self.remaining_width = None;
                        return None;
                    }
                };

                let (new_remaining_width, fragment) = match line_width
                    .checked_sub(fragment.width())
                    .filter(|&width| width >= fragment.penalty_width())
                {
                    // The fragment fits on the next line.
                    Some(remaining_width) => (Some(remaining_width), fragment),
                    // The fragment doesn't fit on the next line, try to break it.
                    None => match fragment.try_break(line_width) {
                        // Breaking it was successful!
                        Ok((left, right)) => {
                            let width = line_width - left.width();
                            debug_assert!(width >= left.penalty_width());
                            self.broken_fragment = Some(right);
                            (Some(width), left)
                        }
                        // Failed to break it, have the line overflow.
                        Err(fragment) => (None, fragment),
                    },
                };
                self.remaining_width = new_remaining_width;
                (true, fragment)
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min_fragments, _) = self.fragments.size_hint();
        let (min_extra_lines, max_extra_lines) = self.widths.size_hint();
        (
            // In the worst-case (i.e. fewest fragments yielded) scenario each fragment will take up
            // one line.
            cmp::min(min_extra_lines, min_fragments),
            if max_extra_lines == Some(0) && self.remaining_width.is_none() {
                // If there are no more lines and there is no more space on the current line, we
                // know that the iterator is finished.
                Some(0)
            } else {
                // Otherwise, there is the possibility of splitting the next fragments infinitely.
                None
            },
        )
    }
}
impl<F, W> FusedIterator for WrapGreedyInner<F, W>
where
    F: FusedIterator,
    <F as Iterator>::Item: Fragment,
    W: FusedIterator<Item = usize>,
{
}

// Manual Debug impl so that `<F as Iterator>::Item: Debug`.
impl<F, W> Debug for WrapGreedyInner<F, W>
where
    F: Iterator + Debug,
    <F as Iterator>::Item: Fragment + Debug,
    W: Iterator<Item = usize> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("WrapGreedy")
            .field("broken_fragment", &self.broken_fragment)
            .field("fragments", &self.fragments)
            .field("widths", &self.widths)
            .field("glue", &self.glue)
            .field("remaining_width", &self.remaining_width)
            .finish()
    }
}
// Manual Clone impl so that `<F as Iterator>::Item: Clone`.
impl<F, W> Clone for WrapGreedyInner<F, W>
where
    F: Iterator + Clone,
    <F as Iterator>::Item: Fragment + Clone,
    W: Iterator<Item = usize> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            broken_fragment: self.broken_fragment.clone(),
            fragments: self.fragments.clone(),
            widths: self.widths.clone(),
            glue: self.glue,
            remaining_width: self.remaining_width,
        }
    }
}

/// Return the current terminal width. If the terminal width cannot be
/// determined (typically because the standard output is not connected
/// to a terminal), a default width of 80 characters will be used.
///
/// **Note:** Only available when the `terminal_size` feature is
/// enabled.
///
/// # Examples
///
/// Create an iterator that repeats the terminal width for passing into [`wrap_greedy`]:
///
/// ```
/// let widths = std::iter::repeat(textwrap::termwidth());
/// # drop(widths);
/// ```
#[cfg(feature = "terminal_size")]
#[must_use]
pub fn termwidth() -> usize {
    terminal_size::terminal_size().map_or(80, |(terminal_size::Width(w), _)| w.into())
}
