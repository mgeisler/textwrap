#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::{wrap_optimal_fit, OptimalFit};

#[derive(Arbitrary, Debug)]
struct Penalties {
    nline_penalty: u32,
    overflow_penalty: u32,
    short_last_line_fraction: u32,
    short_last_line_penalty: u32,
    hyphen_penalty: u32,
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
    width: u32,
    whitespace_width: u32,
    penalty_width: u32,
}

#[rustfmt::skip]
impl core::Fragment for Word {
    fn width(&self) -> u32 { self.width }
    fn whitespace_width(&self) -> u32 { self.whitespace_width }
    fn penalty_width(&self) -> u32 { self.penalty_width }
}

fuzz_target!(|input: (u32, Vec<Word>, Penalties)| {
    let width = input.0;
    let words = input.1;
    let penalties = input.2.into();
    let _ = wrap_optimal_fit(&words, &[width], &penalties);
});
