//! TODO

use std::str::FromStr;

/// TODO doc
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LineEnding {
    /// TODO
    CRLF,
    /// TODO
    LF,
}

impl LineEnding {
    /// TODO
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::CRLF => "\r\n",
            Self::LF => "\n",
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
            _ => Result::Err(()),
        }
    }
}

/// TODO
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
