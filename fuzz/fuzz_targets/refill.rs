#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    let _ = textwrap::refill(&input.0, input.1);
});
