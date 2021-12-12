#![no_main]
use libfuzzer_sys::fuzz_target;
use textwrap::wrap_algorithms;
use textwrap::Options;

fuzz_target!(|input: (String, u16)| {
    let options = Options::new(input.1).wrap_algorithm(wrap_algorithms::OptimalFit::default());
    let _ = textwrap::fill(&input.0, &options);
});
