#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: String| {
    let _ = textwrap::unfill(&input);
});
