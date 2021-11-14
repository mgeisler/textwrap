#![no_main]
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use textwrap::core;
use textwrap::wrap_algorithms::wrap_first_fit;

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

fuzz_target!(|input: (u32, Vec<Word>)| {
    let width = input.0;
    let words = input.1;
    let _ = wrap_first_fit(&words, &[width]);
});
