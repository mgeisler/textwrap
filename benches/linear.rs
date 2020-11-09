use criterion::BenchmarkId;
use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

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
    let mut group = c.benchmark_group("String lengths");
    for length in [100, 200, 400, 800, 1600].iter() {
        let text = lorem_ipsum(*length);
        let options = textwrap::Options::new(LINE_LENGTH);
        group.bench_with_input(BenchmarkId::new("fill", length), &text, |b, text| {
            b.iter(|| textwrap::fill(text, &options));
        });

        let options: textwrap::Options = options.splitter(Box::new(textwrap::HyphenSplitter));
        group.bench_with_input(BenchmarkId::new("fill_boxed", length), &text, |b, text| {
            b.iter(|| textwrap::fill(text, &options));
        });

        group.bench_with_input(BenchmarkId::new("fill_usize", length), &text, |b, text| {
            b.iter(|| textwrap::fill(text, LINE_LENGTH));
        });

        #[cfg(feature = "hyphenation")]
        {
            use hyphenation::{Language, Load, Standard};
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("benches")
                .join("la.standard.bincode");
            let dictionary = Standard::from_path(Language::Latin, &path).unwrap();
            let options = options.splitter(dictionary);
            group.bench_with_input(BenchmarkId::new("hyphenation", length), &text, |b, text| {
                b.iter(|| textwrap::fill(text, &options));
            });
        };
    }
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
