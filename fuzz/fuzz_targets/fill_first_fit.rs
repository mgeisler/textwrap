#![no_main]
use libfuzzer_sys::fuzz_target;
use textwrap::{Options, WrapAlgorithm};

fuzz_target!(|input: (String, usize)| {
    if input.0.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    let options = Options::new(input.1).wrap_algorithm(WrapAlgorithm::FirstFit);
    let _ = textwrap::fill(&input.0, &options);
});
