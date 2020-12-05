//! Utilities for wrapping on plaintext.

use std::iter::FusedIterator;

pub mod split;
pub mod width;

/// A span of text with content, glue and a penalty.
///
/// This type is intentionally agnostic over the algorithm used to calculate the width of each
/// string, and so does not implement [`Fragment`](super::Fragment).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span<'a> {
    /// The content of the span.
    pub content: &'a str,
    /// The glue of the span. This is displayed when another span follows it on the same line, and
    /// is typically whitespace.
    pub glue: &'a str,
    /// The penalty of the span. This is displayed when the span is on the end of the line, and is
    /// typically a hyphen.
    pub penalty: &'a str,
}

impl<'a> Span<'a> {
    /// Create a new span of text from its content.
    #[must_use]
    pub const fn new(content: &'a str) -> Self {
        Self {
            content,
            glue: "",
            penalty: "",
        }
    }
    /// Create a new span of text from its content and glue.
    #[must_use]
    pub const fn with_glue(content: &'a str, glue: &'a str) -> Self {
        Self {
            content,
            glue,
            penalty: "",
        }
    }
    /// Create a new span of text from its content and a penalty.
    #[must_use]
    pub const fn with_penalty(content: &'a str, penalty: &'a str) -> Self {
        Self {
            content,
            glue: "",
            penalty,
        }
    }
    /// Convert this `Span` into a [`Fragment`] using a type to get the text's width.
    #[must_use]
    pub fn width<W: Width>(self, calculator: W) -> Fragment<'a, W> {
        Fragment {
            span: self,
            content_width: calculator.width_str(self.content),
            glue_width: calculator.width_str(self.glue),
            penalty_width: calculator.width_str(self.penalty),
            calculator,
            allow_break: true,
        }
    }
}

/// A method of calculating the width of text.
pub trait Width {
    /// Get the width of a character.
    fn width_char(&self, c: char) -> usize;

    /// Get the width of a string.
    fn width_str(&self, s: &str) -> usize {
        s.chars().map(|c| self.width_char(c)).sum()
    }

    /// Get a tuple of the index and width up to which the string's width is below `max_width`.
    fn width_up_to<'a>(&self, string: &'a str, max_width: usize) -> (usize, usize) {
        let mut width = 0;
        for (i, c) in string.char_indices() {
            let new_width = width + self.width_char(c);
            if new_width > max_width {
                return (i, width);
            }
            width = new_width;
        }
        (string.len(), width)
    }
}

/// A text [`Fragment`](super::Fragment); a [`Span`] combined with a [`Width`].
///
/// Created by [`Span::width`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fragment<'a, W> {
    span: Span<'a>,
    content_width: usize,
    glue_width: usize,
    penalty_width: usize,
    calculator: W,
    /// Whether to allow the force breaking of fragments longer than the line width. By default
    /// this is `true`.
    pub allow_break: bool,
}

impl<'a, W> Fragment<'a, W> {
    /// Get the span of this fragment.
    #[must_use]
    pub fn span(&self) -> Span<'a> {
        self.span
    }

    /// Disallow force breaking fragments longer than the line width.
    #[must_use]
    pub fn no_break(self) -> Self {
        Self {
            allow_break: false,
            ..self
        }
    }
}

impl<'a, W: Width + Copy> super::Fragment for Fragment<'a, W> {
    fn width(&self) -> usize {
        self.content_width
    }
    fn glue_width(&self) -> usize {
        self.glue_width
    }
    fn penalty_width(&self) -> usize {
        self.penalty_width
    }
    fn try_break(self, total_width: usize) -> Result<(Self, Self), Self> {
        if self.allow_break {
            let (i, left_width) = self.calculator.width_up_to(self.span.content, total_width);
            if i > 0 {
                let (left, right) = self.span.content.split_at(i);

                return Ok((
                    Self {
                        span: Span::new(left),
                        content_width: left_width,
                        glue_width: 0,
                        penalty_width: 0,
                        ..self
                    },
                    Self {
                        span: Span {
                            content: right,
                            ..self.span
                        },
                        content_width: self.content_width - left_width,
                        ..self
                    },
                ));
            }
        }
        Err(self)
    }
}

#[test]
fn fragment_try_break() {
    use super::Fragment as _;

    let w = width::Unicode::default();

    assert_eq!(
        Span::new("abc").width(w).try_break(2),
        Ok((Span::new("ab").width(w), Span::new("c").width(w)))
    );
    assert_eq!(
        Span::new("abcde").width(w).try_break(2),
        Ok((Span::new("ab").width(w), Span::new("cde").width(w)))
    );
    assert_eq!(
        Span::new("abc").width(w).try_break(3),
        Ok((Span::new("abc").width(w), Span::new("").width(w))),
    );
    assert_eq!(
        Span::new("a😊c").width(w).try_break(2),
        Ok((Span::new("a").width(w), Span::new("😊c").width(w))),
    );
    assert_eq!(
        Span::new("😊").width(w).try_break(1),
        Err(Span::new("😊").width(w)),
    );
    assert_eq!(
        Span {
            content: "abc",
            glue: " ",
            penalty: "-",
        }
        .width(w)
        .try_break(1),
        Ok((
            Span::new("a").width(w),
            Span {
                content: "bc",
                glue: " ",
                penalty: "-",
            }
            .width(w),
        ))
    );

    assert_eq!(
        Span::new("abc").width(w).no_break().try_break(2),
        Err(Span::new("abc").width(w).no_break()),
    );
}

/// Iterate over the lines of wrapped text.
///
/// # Examples
///
/// ```
/// use textwrap::plain::{self, width, split};
///
/// let parts = split::space("Lorem ipsum dolor sit amet");
/// let fragments = parts.map(|s| s.width(width::Unicode::default()));
/// let wrapped = textwrap::wrap_greedy(fragments, std::iter::repeat(11));
/// for line in plain::lines(wrapped) {
///     println!("{}", line);
/// }
/// ```
///
/// The above should output
///
/// ```text
/// Lorem ipsum
/// dolor sit
/// amet
/// ```
#[must_use]
pub fn lines<'a, W, I: IntoIterator<Item = (Fragment<'a, W>, bool)>>(
    iter: I,
) -> Lines<I::IntoIter> {
    Lines {
        iter: iter.into_iter(),
    }
}

/// Iterator for [`lines`].
#[derive(Debug, Clone)]
pub struct Lines<I> {
    iter: I,
}

impl<'a, W, I> Iterator for Lines<I>
where
    I: Iterator<Item = (Fragment<'a, W>, bool)>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();

        loop {
            let (fragment, eol) = self.iter.next()?;
            line.push_str(fragment.span().content);
            line.push_str(if eol {
                fragment.span().penalty
            } else {
                fragment.span().glue
            });
            if eol {
                break Some(line);
            }
        }
    }
}
impl<'a, W, I> FusedIterator for Lines<I> where I: FusedIterator<Item = (Fragment<'a, W>, bool)> {}

#[test]
fn test_lines() {
    let mut iter = lines(crate::wrap_greedy(
        split::space("Lorem ipsum dolor sit amet").map(|s| s.width(width::Unicode::default())),
        std::iter::repeat(11),
    ));

    assert_eq!(iter.next().unwrap(), "Lorem ipsum");
    assert_eq!(iter.next().unwrap(), "dolor sit");
    assert_eq!(iter.next().unwrap(), "amet");
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

/// Concatenate all the lines of wrapped text using newlines.
///
/// # Examples
///
/// ```
/// use textwrap::plain::{self, width, split};
///
/// let parts = split::space("Lorem ipsum dolor sit amet");
/// let fragments = parts.map(|s| s.width(width::Unicode::default()));
/// let wrapped = textwrap::wrap_greedy(fragments, std::iter::repeat(11));
/// assert_eq!(
///     plain::concat(wrapped),
///     "Lorem ipsum\ndolor sit\namet\n",
/// );
/// ```
#[must_use]
pub fn concat<'a, W, I: IntoIterator<Item = (Fragment<'a, W>, bool)>>(iter: I) -> String {
    let mut s = String::new();
    for (fragment, eol) in iter {
        s.push_str(fragment.span().content);
        if eol {
            s.push_str(fragment.span().penalty);
            s.push('\n');
        } else {
            s.push_str(fragment.span().glue);
        }
    }
    s
}
