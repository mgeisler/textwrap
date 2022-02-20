#![no_main]
use libfuzzer_sys::fuzz_target;
use textwrap::Options;
use textwrap::WrapAlgorithm;

fuzz_target!(|input: (String, usize)| {
    let options = Options::new(input.1).wrap_algorithm(WrapAlgorithm::new_optimal_fit());
    let _ = textwrap::fill(&input.0, &options);
});
