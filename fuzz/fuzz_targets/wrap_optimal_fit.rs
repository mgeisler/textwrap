#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::{wrap_optimal_fit, OptimalFit};

#[derive(Arbitrary, Debug)]
struct Penalties {
    nline_penalty: i32,
    overflow_penalty: i32,
    short_last_line_fraction: usize,
    short_last_line_penalty: i32,
    hyphen_penalty: i32,
}

impl Into<OptimalFit> for Penalties {
    fn into(self) -> OptimalFit {
        OptimalFit {
            nline_penalty: self.nline_penalty,
            overflow_penalty: self.overflow_penalty,
            short_last_line_fraction: std::cmp::max(1, self.short_last_line_fraction),
            short_last_line_penalty: self.short_last_line_penalty,
            hyphen_penalty: self.hyphen_penalty,
        }
    }
}

#[derive(Arbitrary, Debug, Eq, PartialEq)]
struct Word {
    width: usize,
    whitespace_width: usize,
    penalty_width: usize,
}

#[rustfmt::skip]
impl core::Fragment for Word {
    fn width(&self) -> usize { self.width }
    fn whitespace_width(&self) -> usize { self.whitespace_width }
    fn penalty_width(&self) -> usize { self.penalty_width }
}

fuzz_target!(|input: (usize, Vec<Word>, Penalties)| {
    let width = input.0;
    let words = input.1;
    let penalties = input.2.into();
    let _ = wrap_optimal_fit(&words, &[width], &penalties);
});
