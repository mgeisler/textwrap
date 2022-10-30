#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: String| {
    if input.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    let _ = textwrap::unfill(&input);
});
