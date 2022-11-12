#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    if input.0.len() > 100_000 {
        return; // Avoid timeouts in OSS-Fuzz.
    }

    let (_, options) = textwrap::unfill(&input.0);
    if options.subsequent_indent.len() > 10_000 {
        // Avoid out of memory in OSS-fuzz. The indentation is added
        // on every line of the output, meaning that is can make the
        // memory usage explode.
        return;
    }

    let _ = textwrap::refill(&input.0, input.1);
});
