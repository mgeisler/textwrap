#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    if input.0.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    let options = textwrap::Options::new(input.1);
    let fast = textwrap::fill(&input.0, &options);
    let slow = textwrap::fuzzing::fill_slow_path(&input.0, options);
    assert_eq!(fast, slow);
});
