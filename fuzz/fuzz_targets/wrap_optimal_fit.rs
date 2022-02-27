#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::wrap_algorithms::wrap_optimal_fit;
use textwrap::{core, wrap_algorithms};

#[derive(Arbitrary, Debug)]
struct Penalties {
    nline_penalty: usize,
    overflow_penalty: usize,
    short_last_line_fraction: usize,
    short_last_line_penalty: usize,
    hyphen_penalty: usize,
}

impl Into<wrap_algorithms::Penalties> for Penalties {
    fn into(self) -> wrap_algorithms::Penalties {
        wrap_algorithms::Penalties {
            nline_penalty: self.nline_penalty,
            overflow_penalty: self.overflow_penalty,
            short_last_line_fraction: std::cmp::max(1, self.short_last_line_fraction),
            short_last_line_penalty: self.short_last_line_penalty,
            hyphen_penalty: self.hyphen_penalty,
        }
    }
}

#[derive(Arbitrary, Debug, PartialEq)]
struct Word {
    width: f64,
    whitespace_width: f64,
    penalty_width: f64,
}

#[rustfmt::skip]
impl core::Fragment for Word {
    fn width(&self) -> f64 { self.width }
    fn whitespace_width(&self) -> f64 { self.whitespace_width }
    fn penalty_width(&self) -> f64 { self.penalty_width }
}

// Check wrapping fragments with mostly arbitrary widths. Infinite
// widths are not supported since they instantly trigger an overflow
// in the cost computation. Similarly for very large values: the 1e100
// bound used here is somewhat conservative, the real bound seems to
// be around 1e170.
fuzz_target!(|input: (usize, Vec<Word>, Penalties)| {
    let width = input.0;
    let words = input.1;
    let penalties = input.2.into();

    for word in &words {
        for width in [word.width, word.whitespace_width, word.penalty_width] {
            if !width.is_finite() || width.abs() > 1e100 {
                return;
            }
        }
    }

    let _ = wrap_optimal_fit(&words, &[width as f64], &penalties);
});
