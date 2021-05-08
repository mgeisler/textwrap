#![no_main]
use libfuzzer_sys::fuzz_target;
use textwrap::wrap_algorithms;
use textwrap::Options;

fuzz_target!(|input: (String, usize)| {
    let options = Options::new(input.1).wrap_algorithm(wrap_algorithms::OptimalFit);
    let _ = textwrap::fill(&input.0, &options);
});
