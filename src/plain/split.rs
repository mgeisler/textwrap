//! Methods of splitting plaintext into [`Span`]s.

use std::iter::{self, FusedIterator};

use super::Span;

/// Split a string into [`Span`]s by naïvely splitting on whitespace.
///
/// This is fast, but only applicable for English text, other languages will easily break.
///
/// # Examples
///
/// ```
/// use textwrap::plain::{Span, split};
///
/// assert_eq!(
///     split::space("Hello World!").collect::<Vec<_>>(),
///     vec![
///         Span::with_glue("Hello", " "),
///         Span::with_glue("World!", ""),
///     ],
/// );
/// // Does not work for French text:
/// assert_eq!(
///     split::space("bonjour le monde !").collect::<Vec<_>>(),
///     vec![
///         Span::with_glue("bonjour", " "),
///         Span::with_glue("le", " "),
///         Span::with_glue("monde", " "),
///         Span::new("!"),
///     ],
/// );
/// ```
#[must_use]
pub fn space(s: &str) -> Space<'_> {
    Space { s }
}

/// Iterator for the [`space`] function.
#[derive(Debug, Clone)]
pub struct Space<'a> {
    s: &'a str,
}

impl<'a> Iterator for Space<'a> {
    type Item = Span<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        let (text, rest) = self.s.split_at(
            self.s
                .find(char::is_whitespace)
                .unwrap_or_else(|| self.s.len()),
        );
        let (whitespace, rest) = rest.split_at(
            rest.find(|c: char| !c.is_whitespace())
                .unwrap_or_else(|| rest.len()),
        );
        self.s = rest;
        Some(Span::with_glue(text, whitespace))
    }
}
impl<'a> DoubleEndedIterator for Space<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        let (rest, whitespace) = self.s.split_at(
            self.s
                .char_indices()
                .rev()
                .take_while(|(_, c)| c.is_whitespace())
                .last()
                .map(|(i, _)| i)
                .unwrap_or_else(|| self.s.len()),
        );
        let (rest, text) = rest.split_at(
            rest.char_indices()
                .rev()
                .take_while(|(_, c)| !c.is_whitespace())
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0),
        );
        self.s = rest;
        Some(Span::with_glue(text, whitespace))
    }
}
impl<'a> FusedIterator for Space<'a> {}

#[test]
fn test_space() {
    let s = "  Hello World\t!  ";

    let mut parts = vec![
        Span::with_glue("", "  "),
        Span::with_glue("Hello", " "),
        Span::with_glue("World", "\t"),
        Span::with_glue("!", "  "),
    ];
    assert_eq!(space(s).collect::<Vec<_>>(), parts);
    parts.reverse();
    assert_eq!(space(s).rev().collect::<Vec<_>>(), parts);

    let s = "Hello   \n   World!";
    let mut parts = vec![Span::with_glue("Hello", "   \n   "), Span::new("World!")];
    assert_eq!(space(s).collect::<Vec<_>>(), parts);
    parts.reverse();
    assert_eq!(space(s).rev().collect::<Vec<_>>(), parts);
}

/// Further split a span by splitting on soft hyphens (U+AD, written `\u{AD}` inside a Rust
/// string) and hard hyphens (which are never omitted).
///
/// Takes the glue of the outer span, which will be added to the glue of the last span.
///
/// # Examples
///
/// ```
/// use textwrap::plain::{split, Span};
///
/// assert_eq!(
///     split::manual_hyphens("hy\u{AD}phen\u{AD}a\u{AD}tion", " ").collect::<Vec<_>>(),
///     vec![
///         Span::with_penalty("hy", "-"),
///         Span::with_penalty("phen", "-"),
///         Span::with_penalty("a", "-"),
///         Span::with_glue("tion", " "),
///     ],
/// );
/// // Hard hyphens will not be removed.
/// assert_eq!(
///     split::manual_hyphens("hy-phen-a-tion", "").collect::<Vec<_>>(),
///     vec![
///         Span::new("hy-"),
///         Span::new("phen-"),
///         Span::new("a-"),
///         Span::new("tion"),
///     ],
/// )
/// ```
#[must_use]
pub fn manual_hyphens<'a>(s: &'a str, outer_span_glue: &'a str) -> ManualHyphens<'a> {
    ManualHyphens {
        s,
        outer_span_glue: Some(outer_span_glue),
    }
}

/// Iterator for [`manual_hyphens`].
#[derive(Debug, Clone)]
pub struct ManualHyphens<'a> {
    s: &'a str,
    outer_span_glue: Option<&'a str>,
}
impl<'a> Iterator for ManualHyphens<'a> {
    type Item = Span<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        let (text, rest) = self.s.split_at(
            self.s
                .char_indices()
                .find(|&(_, c)| c == '-' || c == '\u{AD}')
                .map(|(i, c)| i + c.len_utf8())
                .unwrap_or_else(|| self.s.len()),
        );
        self.s = rest;

        let glue = if rest.is_empty() {
            self.outer_span_glue.take()
        } else {
            None
        }
        .unwrap_or_default();

        let (content, penalty) = if let Some(text) = text.strip_suffix('\u{AD}') {
            // Only insert soft hyphens as a penalty
            (text, "-")
        } else {
            (text, "")
        };

        Some(Span {
            content,
            glue,
            penalty,
        })
    }
}
impl<'a> DoubleEndedIterator for ManualHyphens<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.s.is_empty() {
            return None;
        }

        let (rest, text) = self.s.split_at(
            self.s
                .char_indices()
                .rev()
                .take_while(|&(_, c)| c != '-' && c != '\u{AD}')
                .last()
                .map(|(i, _)| i)
                .unwrap_or(0),
        );
        self.s = rest;

        let glue = self.outer_span_glue.take().unwrap_or_default();

        let (content, penalty) = if let Some(text) = text.strip_suffix('\u{AD}') {
            // Only insert soft hyphens as a penalty
            (text, "-")
        } else {
            (text, "")
        };

        Some(Span {
            content,
            glue,
            penalty,
        })
    }
}
impl<'a> FusedIterator for ManualHyphens<'a> {}

#[test]
fn test_manual_hyphens() {
    let s = "some-text  - separated\u{AD}  by -hyphens-\u{AD}-.";
    let spans: Vec<_> = manual_hyphens(s, "glue").collect();
    assert_eq!(spans.len(), 8);

    for span in &spans[..7] {
        assert_eq!(span.glue, "");
    }
    assert_eq!(spans[7].glue, "glue");

    let assert_frag = |i: usize, text: &str, penalty: bool| {
        assert_eq!(spans[i].content, text);
        assert_eq!(spans[i].penalty, if penalty { "-" } else { "" });
    };
    assert_frag(0, "some-", false);
    assert_frag(1, "text  -", false);
    assert_frag(2, " separated", true);
    assert_frag(3, "  by -", false);
    assert_frag(4, "hyphens-", false);
    assert_frag(5, "", true);
    assert_frag(6, "-", false);
    assert_frag(7, ".", false);
}

/// Split a string into [`Span`]s by naïvely splitting on whitespace, and then on soft hyphens
/// (U+AD, written `\u{AD}` inside a Rust string) and hard hyphens (which are never omitted).
///
/// # Examples
///
/// ```
/// use textwrap::plain::{split, Span};
///
/// assert_eq!(
///     split::space_manual_hyphens(
///         "Ex\u{AD}ceptur sint oc\u{AD}caecat cu\u{AD}pidatat non proid\u{AD}ent",
///     )
///     .collect::<Vec<_>>(),
///     vec![
///         Span::with_penalty("Ex", "-"),
///         Span::with_glue("ceptur", " "),
///         Span::with_glue("sint", " "),
///         Span::with_penalty("oc", "-"),
///         Span::with_glue("caecat", " "),
///         Span::with_penalty("cu", "-"),
///         Span::with_glue("pidatat", " "),
///         Span::with_glue("non", " "),
///         Span::with_penalty("proid", "-"),
///         Span::new("ent"),
///     ]
/// )
/// ```
#[must_use]
pub fn space_manual_hyphens(s: &str) -> SpaceManualHyphens<'_> {
    SpaceManualHyphens {
        inner: space(s).flat_map(|span| manual_hyphens(span.content, span.glue)),
    }
}

type SpaceManualHyphensInner<'a> =
    iter::FlatMap<Space<'a>, ManualHyphens<'a>, fn(Span<'a>) -> ManualHyphens<'a>>;

/// Iterator for the [`space_manual_hyphens`] function.
#[derive(Debug, Clone)]
pub struct SpaceManualHyphens<'a> {
    inner: SpaceManualHyphensInner<'a>,
}

impl<'a> Iterator for SpaceManualHyphens<'a> {
    type Item = Span<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}
impl<'a> DoubleEndedIterator for SpaceManualHyphens<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}
impl<'a> FusedIterator for SpaceManualHyphens<'a> {}
