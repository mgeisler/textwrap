use std::cell::RefCell;

use crate::core::{MeasuredFragment, Word};
use crate::wrap_algorithms::WrapAlgorithm;

/// Wrap words using an advanced algorithm with look-ahead.
///
/// This wrapping algorithm considers the entire paragraph to find
/// optimal line breaks. When wrapping text, "penalties" are assigned
/// to line breaks based on the gaps left at the end of lines. The
/// penalties are given by this struct, with [`OptimalFit::default`]
/// assigning penalties that work well for monospace text.
///
/// If you are wrapping proportional text, you are advised to assign
/// your own penalties according to your font size. See the individual
/// penalties below for details.
///
/// The underlying wrapping algorithm is implemented by
/// [`wrap_optimal_fit`], please see that function for examples.
///
/// **Note:** Only available when the `smawk` Cargo feature is
/// enabled.
#[derive(Clone, Copy, Debug)]
pub struct OptimalFit {
    /// Per-line penalty. This is added for every line, which makes it
    /// expensive to output more lines than the minimum required.
    pub nline_penalty: i32,

    /// Per-character cost for lines that overflow the target line width.
    ///
    /// With a default value of 50², every single character costs as
    /// much as leaving a gap of 50 characters behind. This is because
    /// we assign as cost of `gap * gap` to a short line. When
    /// wrapping monospace text, we can overflow the line by 1
    /// character in extreme cases:
    ///
    /// ```
    /// use textwrap::wrap_algorithms::{wrap_optimal_fit, OptimalFit};
    /// use textwrap::core::Word;
    ///
    /// let short = "foo ";
    /// let long = "x".repeat(50);
    /// let fragments = vec![Word::from(short), Word::from(&long)];
    /// let penalties = OptimalFit::new();
    ///
    /// // Perfect fit, both words are on a single line with no overflow.
    /// let wrapped = wrap_optimal_fit(&fragments, &[short.len() + long.len()], &penalties);
    /// assert_eq!(wrapped, vec![&[Word::from(short), Word::from(&long)]]);
    ///
    /// // The words no longer fit, yet we get a single line back. While
    /// // the cost of overflow (`1 * 2500`) is the same as the cost of the
    /// // gap (`50 * 50 = 2500`), the tie is broken by `nline_penalty`
    /// // which makes it cheaper to overflow than to use two lines.
    /// let wrapped = wrap_optimal_fit(&fragments, &[short.len() + long.len() - 1], &penalties);
    /// assert_eq!(wrapped, vec![&[Word::from(short), Word::from(&long)]]);
    ///
    /// // The cost of overflow would be 2 * 2500, whereas the cost of
    /// // the gap is only `49 * 49 + nline_penalty = 2401 + 1000 =
    /// // 3401`. We therefore get two lines.
    /// let wrapped = wrap_optimal_fit(&fragments, &[short.len() + long.len() - 2], &penalties);
    /// assert_eq!(wrapped, vec![&[Word::from(short)],
    ///                          &[Word::from(&long)]]);
    /// ```
    ///
    /// This only happens if the overflowing word is 50 characters
    /// long _and_ if the word overflows the line by exactly one
    /// character. If it overflows by more than one character, the
    /// overflow penalty will quickly outgrow the cost of the gap, as
    /// seen above.
    pub overflow_penalty: i32,

    /// When should the a single word on the last line be considered
    /// "too short"?
    ///
    /// If the last line of the text consist of a single word and if
    /// this word is shorter than `1 / short_last_line_fraction` of
    /// the line width, then the final line will be considered "short"
    /// and `short_last_line_penalty` is added as an extra penalty.
    ///
    /// The effect of this is to avoid a final line consisting of a
    /// single small word. For example, with a
    /// `short_last_line_penalty` of 25 (the default), a gap of up to
    /// 5 columns will be seen as more desirable than having a final
    /// short line.
    ///
    /// ## Examples
    ///
    ///
    ///
    ///
    /// ```
    /// use textwrap::{wrap, wrap_algorithms, Options};
    ///
    /// let text = "This is a demo of the short last line penalty.";
    ///
    /// // The first-fit algorithm leaves a single short word on the last line:
    /// assert_eq!(wrap(text, Options::new(37).wrap_algorithm(wrap_algorithms::FirstFit::new())),
    ///            vec!["This is a demo of the short last line",
    ///                 "penalty."]);
    ///
    /// #[cfg(feature = "smawk")] {
    /// let mut wrap_algorithm = wrap_algorithms::OptimalFit::new();
    ///
    /// // Since "penalty." is shorter than 25% of the line width, the
    /// // optimal-fit algorithm adds a penalty of 25. This is enough
    /// // to move "line " down:
    /// assert_eq!(wrap(text, Options::new(37).wrap_algorithm(wrap_algorithm)),
    ///            vec!["This is a demo of the short last",
    ///                 "line penalty."]);
    ///
    /// // We can change the meaning of "short" lines. Here, only words
    /// // shorter than 1/10th of the line width will be considered short:
    /// wrap_algorithm.short_last_line_fraction = 10;
    /// assert_eq!(wrap(text, Options::new(37).wrap_algorithm(wrap_algorithm)),
    ///            vec!["This is a demo of the short last line",
    ///                 "penalty."]);
    ///
    /// // If desired, the penalty can also be disabled:
    /// wrap_algorithm.short_last_line_fraction = 4;
    /// wrap_algorithm.short_last_line_penalty = 0;
    /// assert_eq!(wrap(text, Options::new(37).wrap_algorithm(wrap_algorithm)),
    ///            vec!["This is a demo of the short last line",
    ///                 "penalty."]);
    /// }
    /// ```
    pub short_last_line_fraction: usize,

    /// Penalty for a last line with a single short word.
    ///
    /// Set this to zero if you do not want to penalize short last lines.
    pub short_last_line_penalty: i32,

    /// Penalty for lines ending with a hyphen.
    pub hyphen_penalty: i32,
}

impl OptimalFit {
    /// Default penalties for monospace text.
    ///
    /// The penalties here work well for monospace text. This is
    /// because they expect the gaps at the end of lines to be roughly
    /// in the range `0..100`. If the gaps are larger, the
    /// `overflow_penalty` and `hyphen_penalty` become insignificant.
    pub const fn new() -> Self {
        OptimalFit {
            nline_penalty: 1000,
            overflow_penalty: 50 * 50,
            short_last_line_fraction: 4,
            short_last_line_penalty: 25,
            hyphen_penalty: 25,
        }
    }
}

impl Default for OptimalFit {
    fn default() -> Self {
        Self::new()
    }
}

impl WrapAlgorithm for OptimalFit {
    #[inline]
    fn wrap<'a, 'b>(&self, words: &'b [Word<'a>], line_widths: &'b [usize]) -> Vec<&'b [Word<'a>]> {
        wrap_optimal_fit(words, line_widths, &self)
    }
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

    fn get<T>(&self, i: usize, minima: &[(usize, T)]) -> usize {
        while self.line_numbers.borrow_mut().len() < i + 1 {
            let pos = self.line_numbers.borrow().len();
            let line_number = 1 + self.get(minima[pos].0, &minima);
            self.line_numbers.borrow_mut().push(line_number);
        }

        self.line_numbers.borrow()[i]
    }
}

/// Wrap abstract fragments into lines with an optimal-fit algorithm.
///
/// The `line_widths` slice gives the target line width for each line
/// (the last slice element is repeated as necessary). This can be
/// used to implement hanging indentation.
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
/// **Note:** Only available when the `smawk` Cargo feature is
/// enabled.
pub fn wrap_optimal_fit<'a, 'b, T: MeasuredFragment>(
    fragments: &'a [T],
    line_widths: &'b [usize],
    penalties: &'b OptimalFit,
) -> Vec<&'a [T]> {
    // The final line width is used for all remaining lines.
    let default_line_width = line_widths.last().copied().unwrap_or(0);
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
        let line_width = line_widths
            .get(line_number)
            .copied()
            .unwrap_or(default_line_width);
        let target_width = std::cmp::max(1, line_width);

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
        let mut cost = minima[i].1 + penalties.nline_penalty;

        // Next, we add a penalty depending on the line length.
        if line_width > target_width {
            // Lines that overflow get a hefty penalty.
            let overflow = (line_width - target_width) as i32;
            cost += overflow * penalties.overflow_penalty;
        } else if j < fragments.len() {
            // Other lines (except for the last line) get a milder
            // penalty which depend on the size of the gap.
            let gap = (target_width - line_width) as i32;
            cost += gap * gap;
        } else if i + 1 == j && line_width < target_width / penalties.short_last_line_fraction {
            // The last line can have any size gap, but we do add a
            // penalty if the line is very short (typically because it
            // contains just a single word).
            cost += penalties.short_last_line_penalty;
        }

        // Finally, we discourage hyphens.
        if fragments[j - 1].penalty_width() > 0 {
            // TODO: this should use a penalty value from the fragment
            // instead.
            cost += penalties.hyphen_penalty;
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
