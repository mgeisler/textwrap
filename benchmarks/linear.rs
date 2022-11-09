use std::time::Duration;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

// The benchmarks here verify that the complexity grows as O(*n*)
// where *n* is the number of characters in the text to be wrapped.

use lipsum::lipsum_words_from_seed;

const LINE_LENGTH: usize = 60;

/// Generate a lorem ipsum text with the given number of characters.
fn lorem_ipsum(length: usize) -> String {
    // The average word length in the lorem ipsum text is somewhere
    // between 6 and 7. So we conservatively divide by 5 to have a
    // long enough text that we can truncate below.
    let mut text = lipsum_words_from_seed(length / 5, 42);
    text.truncate(length);
    text
}

pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fill");
    let lengths = [
        0, 5, 10, 20, 30, 40, 50, 60, 80, 100, 200, 300, 400, 600, 800, 1200, 1600, 2400, 3200,
        4800, 6400,
    ];
    let wrap_algorithms = [
        (textwrap::WrapAlgorithm::new_optimal_fit(), "optimal_fit"),
        (textwrap::WrapAlgorithm::FirstFit, "first_fit"),
    ];
    let word_separators = [
        (textwrap::WordSeparator::UnicodeBreakProperties, "unicode"),
        (textwrap::WordSeparator::AsciiSpace, "ascii"),
    ];

    for length in lengths {
        let text = lorem_ipsum(length);
        let length_id = format!("{length:04}");

        for (algorithm, algorithm_name) in &wrap_algorithms {
            for (separator, separator_name) in &word_separators {
                let name = format!("{algorithm_name}_{separator_name}");
                let options = textwrap::Options::new(LINE_LENGTH)
                    .wrap_algorithm(*algorithm)
                    .word_separator(*separator);
                group.bench_with_input(BenchmarkId::new(&name, &length_id), &text, |b, text| {
                    b.iter(|| textwrap::fill(text, &options));
                });
            }
        }

        group.bench_function(BenchmarkId::new("inplace", &length_id), |b| {
            b.iter_batched(
                || text.clone(),
                |mut text| textwrap::fill_inplace(&mut text, LINE_LENGTH),
                criterion::BatchSize::SmallInput,
            );
        });

        use hyphenation::{Language, Load, Standard};
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("la.standard.bincode");
        let dictionary = Standard::from_path(Language::Latin, &path).unwrap();
        let options = textwrap::Options::new(LINE_LENGTH)
            .wrap_algorithm(textwrap::WrapAlgorithm::new_optimal_fit())
            .word_separator(textwrap::WordSeparator::AsciiSpace)
            .word_splitter(textwrap::WordSplitter::Hyphenation(dictionary));
        group.bench_with_input(
            BenchmarkId::new("optimal_fit_ascii_hyphenation", &length_id),
            &text,
            |b, text| {
                b.iter(|| textwrap::fill(text, &options));
            },
        );
    }
    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().warm_up_time(Duration::from_millis(500));
    targets = benchmark
);
criterion_main!(benches);
