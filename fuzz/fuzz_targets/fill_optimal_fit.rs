#![no_main]
use libfuzzer_sys::fuzz_target;
use textwrap::core::WrapAlgorithm::OptimalFit;
use textwrap::Options;

fuzz_target!(|input: (String, usize)| {
    let options = Options::new(input.1).wrap_algorithm(OptimalFit);
    let _ = textwrap::fill(&input.0, &options);
});
