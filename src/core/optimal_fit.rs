use crate::core::Fragment;
use std::cell::{Cell, RefCell};

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

    fn get<T>(&self, i: usize, minima: &[(usize, T)]) -> usize {
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
const NLINE_PENALTY: usize = 1000;

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
const OVERFLOW_PENALTY: usize = 50 * 50;

/// The last line is short if it is less than 1/4 of the target width.
const SHORT_LINE_FRACTION: usize = 4;

/// Penalize a short last line.
const SHORT_LAST_LINE_PENALTY: usize = 25;

/// Penalty for lines ending with a hyphen.
const HYPHEN_PENALTY: usize = 25;

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
/// for only 10 characters. The [greedy
/// algorithm](super::wrap_first_fit) will produce these lines, each
/// annotated with the corresponding penalty:
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
/// the number of words. Compared to
/// [`wrap_first_fit`](super::wrap_first_fit), this function is about
/// 4 times slower.
///
/// The optimization of per-line costs over the entire paragraph is
/// inspired by the line breaking algorithm used in TeX, as described
/// in the 1981 article [_Breaking Paragraphs into
/// Lines_](http://www.eprg.org/G53DOC/pdfs/knuth-plass-breaking.pdf)
/// by Knuth and Plass. The implementation here is based on [Python
/// code by David
/// Eppstein](https://github.com/jfinkels/PADS/blob/master/pads/wrap.py).
///
/// # Panics
///
/// The total width of all fragments must fit inside an `usize`
/// (including the whitespace and penalty widths).
///
/// **Note:** Only available when the `smawk` Cargo feature is
/// enabled.
pub fn wrap_optimal_fit<'a, T: Fragment, F: Fn(usize) -> usize>(
    fragments: &'a [T],
    line_widths: F,
) -> Vec<&'a [T]> {
    let mut min_idx = 0;
    let mut max_idx = fragments.len();

    let mut result = Vec::new();

    // We call wrap_optimal_fit_checked on smaller and smaller slices
    // until we either end up with a single fragment or we find a
    // slice which can be wrapped without overflow. In either case, we
    // advance min_idx which ensures that we make progress.
    loop {
        match wrap_optimal_fit_checked(&fragments[min_idx..max_idx], &line_widths) {
            Some(lines) => {
                let partial_last_line = lines.len() > 1;
                result.extend(lines);
                if max_idx == fragments.len() {
                    return result; // All done!
                }

                min_idx = max_idx;
                max_idx = fragments.len();

                // We assume that the last wrapped line is incomplete
                // and needs to be re-wrapped.
                if partial_last_line {
                    let last_line = result.pop().unwrap();
                    min_idx -= last_line.len();
                }
            }
            None => {
                if max_idx - min_idx == 1 {
                    // This single fragment is causing an overflow, so
                    // we put on its own line.
                    result.push(&fragments[min_idx..max_idx]);
                    if max_idx == fragments.len() {
                        return result; // All done!
                    }

                    min_idx = max_idx;
                    max_idx = fragments.len();
                } else {
                    max_idx = min_idx + (max_idx - min_idx) / 2;
                }
            }
        }
    }
}

/// Wrap abstract fragments into lines with an optimal-fit algorithm.
/// Returns `None` if an overflow occurs during the penalty
/// computations. See [`wrap_optimal_fit`].
fn wrap_optimal_fit_checked<'a, T: Fragment, F: Fn(usize) -> usize>(
    fragments: &'a [T],
    line_widths: F,
) -> Option<Vec<&'a [T]>> {
    let mut widths = Vec::with_capacity(fragments.len() + 1);
    let mut width = 0;
    widths.push(width);
    for fragment in fragments {
        width += fragment.width() + fragment.whitespace_width();
        widths.push(width);
    }

    if widths.last() < Some(&line_widths(0)) {
        return Some(vec![fragments]);
    }

    let line_numbers = LineNumbers::new(fragments.len());
    let detected_overflow = Cell::new(false);

    let cost_fn = |minima: &[(usize, usize)], i, j| -> Option<usize> {
        // Line number for fragment `i`.
        let line_number = line_numbers.get(i, &minima);
        let target_width = std::cmp::max(1, line_widths(line_number));

        // Compute the width of a line spanning fragments[i..j] in
        // constant time. We need to adjust widths[j] by subtracting
        // the whitespace of fragment[j-i] and then add the penalty.
        let last_fragment: &T = &fragments[j - 1];
        let line_width = widths[j] - widths[i] - last_fragment.whitespace_width()
            + last_fragment.penalty_width();

        // We compute cost of the line containing fragments[i..j]. We
        // start with values[i].1, which is the optimal cost for
        // breaking before fragments[i].
        //
        // First, every extra line cost NLINE_PENALTY.
        let mut cost = minima[i].1.checked_add(NLINE_PENALTY)?;

        // Next, we add a penalty depending on the line length.
        if line_width > target_width {
            // Lines that overflow get a hefty penalty.
            let overflow: usize = line_width - target_width;
            cost = cost.checked_add(overflow.checked_mul(OVERFLOW_PENALTY)?)?;
        } else if j < fragments.len() {
            // Other lines (except for the last line) get a milder
            // penalty which depend on the size of the gap.
            let gap: usize = target_width - line_width;
            cost = cost.checked_add(gap.checked_mul(gap)?)?;
        } else if i + 1 == j && line_width < target_width / SHORT_LINE_FRACTION {
            // The last line can have any size gap, but we do add a
            // penalty if the line is very short (typically because it
            // contains just a single word).
            cost = cost.checked_add(SHORT_LAST_LINE_PENALTY)?;
        }

        // Finally, we discourage hyphens.
        if fragments[j - 1].penalty_width() > 0 {
            // TODO: this should use a penalty value from the fragment
            // instead.
            cost = cost.checked_add(HYPHEN_PENALTY)?;
        }

        Some(cost)
    };

    let minima = smawk::online_column_minima(0, widths.len(), |minima: &[(usize, usize)], i, j| {
        if detected_overflow.get() {
            return 0;
        }
        match cost_fn(minima, i, j) {
            Some(cost) => cost,
            None => {
                detected_overflow.set(true);
                0
            }
        }
    });

    if detected_overflow.into_inner() {
        return None;
    }

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
    Some(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Eq, PartialEq)]
    struct BoxGluePenalty(usize);

    #[rustfmt::skip]
    impl Fragment for BoxGluePenalty {
        fn width(&self) -> usize { self.0 }
        fn whitespace_width(&self) -> usize { 1 }
        fn penalty_width(&self) -> usize { 0 }
    }

    #[test]
    fn optimal_fit_single_fragment_overflow() {
        let fragments = vec![BoxGluePenalty(2 << 60)];
        let line_widths = |_| 80;

        assert_eq!(wrap_optimal_fit_checked(&fragments, &line_widths), None);
        assert_eq!(
            wrap_optimal_fit(&fragments, &line_widths),
            vec![[BoxGluePenalty(2 << 60)]]
        );
    }

    #[test]
    fn optimal_fit_rewrapping_on_overflow() {
        let fragments = vec![
            BoxGluePenalty(1001),
            BoxGluePenalty(1002),
            BoxGluePenalty(1003),
            BoxGluePenalty(1004),
            BoxGluePenalty(105),     // small fragment
            BoxGluePenalty(2 << 60), // over-sized fragment
            BoxGluePenalty(1007),
            BoxGluePenalty(1008),
        ];
        let line_widths = |_| 2500; // Room for two big fragments.

        assert_eq!(wrap_optimal_fit_checked(&fragments, &line_widths), None);
        // First five fragments fit on two lines and the small 105
        // fragment is included on the second line:
        assert_eq!(
            wrap_optimal_fit_checked(&fragments[..5], &line_widths).unwrap(),
            vec![
                vec![BoxGluePenalty(1001), BoxGluePenalty(1002)],
                vec![
                    BoxGluePenalty(1003),
                    BoxGluePenalty(1004),
                    BoxGluePenalty(105)
                ]
            ]
        );
        assert_eq!(
            wrap_optimal_fit(&fragments, &line_widths),
            vec![
                vec![BoxGluePenalty(1001), BoxGluePenalty(1002)],
                vec![
                    BoxGluePenalty(1003),
                    BoxGluePenalty(1004),
                    BoxGluePenalty(105),
                ],
                vec![BoxGluePenalty(2 << 60)],
                vec![BoxGluePenalty(1007), BoxGluePenalty(1008)]
            ]
        );
    }
}
