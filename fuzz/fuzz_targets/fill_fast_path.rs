#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    let options = textwrap::Options::new(input.1);
    let fast = textwrap::fill(&input.0, &options);
    # fill_slow_path_for_fuzzing is exposed due to --cfg fuzzing.
    let slow = textwrap::fill_slow_path_for_fuzzing(&input.0, options);
    assert_eq!(fast, slow);
});
