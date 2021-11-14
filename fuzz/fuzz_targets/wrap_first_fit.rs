#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::wrap_first_fit;

#[derive(Arbitrary, Debug, Eq, PartialEq)]
struct Word {
    width: u16,
    whitespace_width: u16,
    penalty_width: u16,
}

#[rustfmt::skip]
impl core::Fragment for Word {
    fn width(&self) -> u16 { self.width }
    fn whitespace_width(&self) -> u16 { self.whitespace_width }
    fn penalty_width(&self) -> u16 { self.penalty_width }
}

fuzz_target!(|input: (u16, Vec<Word>)| {
    let width = input.0;
    let words = input.1;
    let _ = wrap_first_fit(&words, &[width]);
});
