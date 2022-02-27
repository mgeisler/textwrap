#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::wrap_first_fit;

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

fuzz_target!(|input: (f64, Vec<Word>)| {
    let width = input.0;
    let words = input.1;
    let _ = wrap_first_fit(&words, &[width]);
});
