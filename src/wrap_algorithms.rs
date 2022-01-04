//! Word wrapping algorithms.
//!
//! After a text has been broken into words (or [`Fragment`]s), one
//! now has to decide how to break the fragments into lines. The
//! simplest algorithm for this is implemented by [`wrap_first_fit`]:
//! it uses no look-ahead and simply adds fragments to the line as
//! long as they fit. However, this can lead to poor line breaks if a
//! large fragment almost-but-not-quite fits on a line. When that
//! happens, the fragment is moved to the next line and it will leave
//! behind a large gap. A more advanced algorithm, implemented by
//! [`wrap_optimal_fit`], will take this into account. The optimal-fit
//! algorithm considers all possible line breaks and will attempt to
//! minimize the gaps left behind by overly short lines.
//!
//! While both algorithms run in linear time, the first-fit algorithm
//! is about 4 times faster than the optimal-fit algorithm.

#[cfg(feature = "smawk")]
mod optimal_fit;
#[cfg(feature = "smawk")]
pub use optimal_fit::{wrap_optimal_fit, OptimalFit, OverflowError};

use crate::core::{Fragment, Word};

/// Describes how to wrap words into lines.
///
/// The simplest approach is to wrap words one word at a time. This is
/// implemented by [`FirstFit`]. If the `smawk` Cargo feature is
/// enabled, a more complex algorithm is available, implemented by
/// [`OptimalFit`], which will look at an entire paragraph at a time
/// in order to find optimal line breaks.
pub trait WrapAlgorithm: WrapAlgorithmClone + std::fmt::Debug {
    /// Wrap words according to line widths.
    ///
    /// The `line_widths` slice gives the target line width for each
    /// line (the last slice element is repeated as necessary). This
    /// can be used to implement hanging indentation.
    ///
    /// Please see the implementors of the trait for examples.
    fn wrap<'a, 'b>(&self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]>;
}

// The internal `WrapAlgorithmClone` trait is allows us to implement
// `Clone` for `Box<dyn WrapAlgorithm>`. This in used in the
// `From<&Options<'_, WrapAlgo, WordSep, WordSplit>> for Options<'a,
// WrapAlgo, WordSep, WordSplit>` implementation.
#[doc(hidden)]
pub trait WrapAlgorithmClone {
    fn clone_box(&self) -> Box<dyn WrapAlgorithm>;
}

impl<T: WrapAlgorithm + Clone + 'static> WrapAlgorithmClone for T {
    fn clone_box(&self) -> Box<dyn WrapAlgorithm> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn WrapAlgorithm> {
    fn clone(&self) -> Box<dyn WrapAlgorithm> {
        use std::ops::Deref;
        self.deref().clone_box()
    }
}

impl WrapAlgorithm for Box<dyn WrapAlgorithm> {
    fn wrap<'a, 'b>(&self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]> {
        use std::ops::Deref;
        self.deref().wrap(words, line_widths)
    }
}

/// Wrap words using a fast and simple algorithm.
///
/// This algorithm uses no look-ahead when finding line breaks.
/// Implemented by [`wrap_first_fit`], please see that function for
/// details and examples.
#[derive(Clone, Copy, Debug)]
pub struct FirstFit;

impl FirstFit {
    /// Create a new empty struct.
    pub const fn new() -> Self {
        FirstFit
    }
}

impl Default for FirstFit {
    fn default() -> Self {
        Self::new()
    }
}

impl WrapAlgorithm for FirstFit {
    #[inline]
    fn wrap<'a, 'b>(&self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]> {
        // Every integer up to 2u64.pow(f64::MANTISSA_DIGITS) = 2**53
        // = 9_007_199_254_740_992 can be represented without loss by
        // a f64. Larger line widths will be rounded to the nearest
        // representable number.
        let f64_line_widths = line_widths.iter().map(|w| *w as f64).collect::<Vec<_>>();
        wrap_first_fit(words, &f64_line_widths)
    }
}

/// Wrap abstract fragments into lines with a first-fit algorithm.
///
/// The `line_widths` slice gives the target line width for each line
/// (the last slice element is repeated as necessary). This can be
/// used to implement hanging indentation.
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
/// use textwrap::core::Word;
/// use textwrap::wrap_algorithms::wrap_first_fit;
/// use textwrap::word_separators::{AsciiSpace, WordSeparator};
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
/// assert_eq!(lines_to_strings(wrap_first_fit(&words, &[15.0])),
///            vec!["These few words",
///                 "will",  // <-- short line
///                 "unfortunately",
///                 "not wrap",
///                 "nicely."]);
///
/// // We can avoid the short line if we look ahead:
/// #[cfg(feature = "smawk")]
/// use textwrap::wrap_algorithms::{wrap_optimal_fit, OptimalFit};
/// #[cfg(feature = "smawk")]
/// assert_eq!(lines_to_strings(wrap_optimal_fit(&words, &[15.0], &OptimalFit::new()).unwrap()),
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
/// use textwrap::wrap_algorithms::wrap_first_fit;
/// use textwrap::core::{Fragment, Word};
///
/// #[derive(Debug)]
/// struct Task<'a> {
///     name: &'a str,
///     hours: f64,   // Time needed to complete task.
///     sweep: f64,   // Time needed for a quick sweep after task during the day.
///     cleanup: f64, // Time needed for full cleanup if day ends with this task.
/// }
///
/// impl Fragment for Task<'_> {
///     fn width(&self) -> f64 { self.hours }
///     fn whitespace_width(&self) -> f64 { self.sweep }
///     fn penalty_width(&self) -> f64 { self.cleanup }
/// }
///
/// // The morning tasks
/// let tasks = vec![
///     Task { name: "Foundation",  hours: 4.0, sweep: 2.0, cleanup: 3.0 },
///     Task { name: "Framing",     hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Plumbing",    hours: 2.0, sweep: 2.0, cleanup: 2.0 },
///     Task { name: "Electrical",  hours: 2.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Insulation",  hours: 2.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Drywall",     hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Floors",      hours: 3.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Countertops", hours: 1.0, sweep: 1.0, cleanup: 2.0 },
///     Task { name: "Bathrooms",   hours: 2.0, sweep: 1.0, cleanup: 2.0 },
/// ];
///
/// // Fill tasks into days, taking `day_length` into account. The
/// // output shows the hours worked per day along with the names of
/// // the tasks for that day.
/// fn assign_days<'a>(tasks: &[Task<'a>], day_length: f64) -> Vec<(f64, Vec<&'a str>)> {
///     let mut days = Vec::new();
///     // Assign tasks to days. The assignment is a vector of slices,
///     // with a slice per day.
///     let assigned_days: Vec<&[Task<'a>]> = wrap_first_fit(&tasks, &[day_length]);
///     for day in assigned_days.iter() {
///         let last = day.last().unwrap();
///         let work_hours: f64 = day.iter().map(|t| t.hours + t.sweep).sum();
///         let names = day.iter().map(|t| t.name).collect::<Vec<_>>();
///         days.push((work_hours - last.sweep + last.cleanup, names));
///     }
///     days
/// }
///
/// // With a single crew working 8 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 8.0),
///     [
///         (7.0, vec!["Foundation"]),
///         (8.0, vec!["Framing", "Plumbing"]),
///         (7.0, vec!["Electrical", "Insulation"]),
///         (5.0, vec!["Drywall"]),
///         (7.0, vec!["Floors", "Countertops"]),
///         (4.0, vec!["Bathrooms"]),
///     ]
/// );
///
/// // With two crews working in shifts, 16 hours a day:
/// assert_eq!(
///     assign_days(&tasks, 16.0),
///     [
///         (14.0, vec!["Foundation", "Framing", "Plumbing"]),
///         (15.0, vec!["Electrical", "Insulation", "Drywall", "Floors"]),
///         (6.0, vec!["Countertops", "Bathrooms"]),
///     ]
/// );
/// ```
///
/// Apologies to anyone who actually knows how to build a house and
/// knows how long each step takes :-)
pub fn wrap_first_fit<'a, 'b, T: Fragment>(
    fragments: &'a [T],
    line_widths: &'b [f64],
) -> Vec<&'a [T]> {
    // The final line width is used for all remaining lines.
    let default_line_width = line_widths.last().copied().unwrap_or(0.0);
    let mut lines = Vec::new();
    let mut start = 0;
    let mut width = 0.0;

    for (idx, fragment) in fragments.iter().enumerate() {
        let line_width = line_widths
            .get(lines.len())
            .copied()
            .unwrap_or(default_line_width);
        if width + fragment.width() + fragment.penalty_width() > line_width && idx > start {
            lines.push(&fragments[start..idx]);
            start = idx;
            width = 0.0;
        }
        width += fragment.width() + fragment.whitespace_width();
    }
    lines.push(&fragments[start..]);
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Word(f64);

    #[rustfmt::skip]
    impl Fragment for Word {
        fn width(&self) -> f64 { self.0 }
        fn whitespace_width(&self) -> f64 { 1.0 }
        fn penalty_width(&self) -> f64 { 0.0 }
    }

    #[test]
    fn wrap_string_longer_than_f64() {
        let words = vec![
            Word(1e307),
            Word(2e307),
            Word(3e307),
            Word(4e307),
            Word(5e307),
            Word(6e307),
        ];
        // Wrap at just under f64::MAX (~19e307). The tiny
        // whitespace_widths disappear because of loss of precision.
        assert_eq!(
            wrap_first_fit(&words, &[15e307]),
            &[
                vec![
                    Word(1e307),
                    Word(2e307),
                    Word(3e307),
                    Word(4e307),
                    Word(5e307)
                ],
                vec![Word(6e307)]
            ]
        );
    }
}
