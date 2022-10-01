#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (String, usize)| {
    // Filter out multi-line input.
    if input.0.contains('\n') {
        return;
    }

    let mut fast = Vec::new();
    let mut slow = Vec::new();

    let options = textwrap::Options::new(input.1);
    textwrap::fuzzing::wrap_single_line(&input.0, &options, &mut fast);
    textwrap::fuzzing::wrap_single_line_slow_path(&input.0, &options, &mut slow);
    assert_eq!(fast, slow);
});
