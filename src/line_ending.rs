//! TODO

use std::str::FromStr;

/// TODO doc
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineEnding {
    /// TODO
    CR,
    /// TODO
    CRLF,
    /// TODO
    LF,
}

impl LineEnding {
    /// TODO
    #[inline]
    pub const fn len_chars(&self) -> usize {
        match self {
            Self::CRLF => 2,
            _ => 1,
        }
    }

    /// TODO
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CRLF => "\u{000D}\u{000A}",
            Self::LF => "\u{000A}",
            Self::CR => "\u{000D}",
        }
    }
}

impl FromStr for LineEnding {
    // TODO add a descriptive error
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<LineEnding, ()> {
        match s {
            "\u{000D}\u{000A}" => Result::Ok(LineEnding::CRLF),
            "\u{000A}" => Result::Ok(LineEnding::LF),
            "\u{000D}" => Result::Ok(LineEnding::CR),
            _ => Result::Err(()),
        }
    }
}
