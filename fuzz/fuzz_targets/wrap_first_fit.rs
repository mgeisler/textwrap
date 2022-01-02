#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::wrap_first_fit;

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

fuzz_target!(|input: (usize, Vec<Word>)| {
    let width = input.0;
    let words = input.1;
    let _ = wrap_first_fit(&words, &[width]);
});
