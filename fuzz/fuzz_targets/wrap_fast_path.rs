#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    if input.0.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    if input.0.contains('\n') {
        return; // Filter out multi-line input.
    }

    let options = textwrap::Options::new(input.1);
    let mut fast = Vec::new();
    let mut slow = Vec::new();
    textwrap::fuzzing::wrap_single_line(&input.0, &options, &mut fast);
    textwrap::fuzzing::wrap_single_line_slow_path(&input.0, &options, &mut slow);
    assert_eq!(fast, slow);
});
