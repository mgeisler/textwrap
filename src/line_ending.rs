//! Line ending detection and conversion.

use std::fmt::{Debug, Formatter};
use std::str::FromStr;

/// Supported line endings. Like in the Rust's standard library, two
/// line endings are supported: `\r\n` and `\n`
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineEnding {
    /// _Carriage return and line feed_ – a line ending sequence
    /// historically used in _Windows_. Corresponds to the sequence
    /// of ASCII control characters `0x0D 0x0A` or `\r\n`
    CRLF,
    /// _Line feed_ – a line ending historically used in _Unix_.
    ///  Corresponds to the ASCII control character `0x0A` or `\n`
    LF,
}

/// Returned when attempted creating [`LineEnding`] value from an
/// unsupported `&str` value
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnsupportedLineEnding;

impl std::fmt::Display for UnsupportedLineEnding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "unsupported line ending sequence")
    }
}

impl std::error::Error for UnsupportedLineEnding {}

impl LineEnding {
    /// Turns this [`LineEnding`] value into its ASCII representation.
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CRLF => "\r\n",
            Self::LF => "\n",
        }
    }
}

impl FromStr for LineEnding {
    type Err = UnsupportedLineEnding;

    #[inline]
    fn from_str(s: &str) -> Result<LineEnding, UnsupportedLineEnding> {
        match s {
            "\u{000D}\u{000A}" => Ok(LineEnding::CRLF),
            "\u{000A}" => Ok(LineEnding::LF),
            _ => Err(UnsupportedLineEnding),
        }
    }
}

/// An iterator over the lines of a string, as tuples of string slice
/// and [`LineEnding`] value; it only emits non-empty lines (i.e. having
/// some content before the terminating `\r\n` or `\n`).
///
/// This struct is used internally by the library.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NonEmptyLines<'a>(pub &'a str);

impl<'a> Iterator for NonEmptyLines<'a> {
    type Item = (&'a str, Option<LineEnding>);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(lf) = self.0.find('\n') {
            if lf == 0 || (lf == 1 && self.0.as_bytes()[lf - 1] == b'\r') {
                self.0 = &self.0[(lf + 1)..];
                continue;
            }
            let trimmed = match self.0.as_bytes()[lf - 1] {
                b'\r' => (&self.0[..(lf - 1)], Some(LineEnding::CRLF)),
                _ => (&self.0[..lf], Some(LineEnding::LF)),
            };
            self.0 = &self.0[(lf + 1)..];
            return Some(trimmed);
        }
        if self.0.len() > 0 {
            let result = Some((self.0, None));
            self.0 = "";
            return result;
        } else {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::line_ending::NonEmptyLines;
    use crate::LineEnding;

    #[test]
    fn non_empty_lines_full_case() {
        assert_eq!(
            NonEmptyLines("LF\nCRLF\r\n\r\n\nunterminated")
                .collect::<Vec<(&str, Option<LineEnding>)>>(),
            vec![
                ("LF", Some(LineEnding::LF)),
                ("CRLF", Some(LineEnding::CRLF)),
                ("unterminated", None),
            ]
        );
    }

    #[test]
    fn non_empty_lines_new_lines_only() {
        assert_eq!(NonEmptyLines("\r\n\n\n\r\n").next(), None);
    }

    #[test]
    fn non_empty_lines_no_input() {
        assert_eq!(NonEmptyLines("").next(), None);
    }
}
