//! Methods of calculating the width of plaintext.

use super::Width;

/// Get the width of a string using [`unicode-width`]. This is accurate for most characters on most
/// terminals, however some terminals like iTerm2 will display something like
/// "👨‍👨‍👧‍👦" (a family emoji) in two columns instead of eight ("👨👨👧👦").
///
/// The only reliable way to support every single terminal is to print out the character and query
/// the cursor's position before and after, but using this approximation works _most_ of the time.
///
/// # Examples
///
/// ```
/// use textwrap::plain::{width, Width};
///
/// let width = width::Unicode::default();
/// assert_eq!(width.width_str("Hello World!"), 12);
/// assert_eq!(width.width_str("😊"), 2);
/// assert_eq!(width.width_str("👨‍👨‍👧‍👦"), 8);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Unicode {
    /// Whether to treat characters in the Ambiguous category as 2 columns wide. By default this is
    /// `false`, in accordance with recommendations for non-CJK contexts or when the context cannot
    /// be reliably determined.
    pub cjk: bool,
}

impl Unicode {
    /// Create a new `Unicode` using default settings.
    #[must_use]
    pub const fn new() -> Self {
        Self { cjk: false }
    }
    /// Treat characters in the Ambiguous category as 2 columns wide, as recommended for CJK
    /// contexts.
    #[must_use]
    pub const fn cjk(self) -> Self {
        Self { cjk: true }
    }
}

impl Width for Unicode {
    fn width_char(&self, c: char) -> usize {
        if self.cjk {
            unicode_width::UnicodeWidthChar::width_cjk(c)
        } else {
            unicode_width::UnicodeWidthChar::width(c)
        }
        .unwrap_or(0)
    }
}
