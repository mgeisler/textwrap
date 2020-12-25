#![no_main]
use libfuzzer_sys::{arbitrary, fuzz_target};
use textwrap::core;
use textwrap::core::Fragment;

#[derive(arbitrary::Arbitrary, Debug, Eq, PartialEq, Clone)]
struct BoxGluePenalty(usize, usize, usize);

#[rustfmt::skip]
impl core::Fragment for BoxGluePenalty {
    fn width(&self) -> usize { self.0 }
    fn whitespace_width(&self) -> usize { self.1 }
    fn penalty_width(&self) -> usize { self.2 }
}

fuzz_target!(|input: (Vec<BoxGluePenalty>, u64)| {
    let line_width = input.1 as usize;
    let fragments = input.0.clone();

    let total_width: Option<usize> = fragments.iter().fold(Some(0), |sum, f| {
        sum.and_then(|sum| sum.checked_add(f.width()))
            .and_then(|sum| sum.checked_add(f.whitespace_width()))
            .and_then(|sum| sum.checked_add(f.penalty_width()))
    });
    if total_width.is_none() {
        return;
    }

    let _ = core::wrap_optimal_fit(&fragments, &|_| line_width);
});
