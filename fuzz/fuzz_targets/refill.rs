#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    if input.0.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    let _ = textwrap::refill(&input.0, input.1);
});
