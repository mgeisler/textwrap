use criterion::{Criterion, criterion_group, criterion_main};
use lipsum::lipsum_words_with_rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

pub fn benchmark(c: &mut Criterion) {
    // Generate a piece of text with some empty lines.
    let words_per_line = [
        5, 10, 15, 5, 0, 10, 5, 0, 5, 10, // 10 lines
        10, 10, 5, 5, 5, 5, 15, 10, 5, 0, // 20 lines
        10, 5, 0, 0, 15, 10, 10, 5, 5, 5, // 30 lines
        15, 5, 0, 10, 5, 0, 0, 15, 5, 10, // 40 lines
        5, 15, 0, 5, 15, 0, 10, 10, 5, 5, // 50 lines
    ];
    let mut text = String::new();
    for (line_no, word_count) in words_per_line.iter().enumerate() {
        let rng = ChaCha20Rng::seed_from_u64(line_no as u64);
        text.push_str(&lipsum_words_with_rng(rng, *word_count));
        text.push('\n');
    }
    assert_eq!(text.len(), 2304); // The size for reference.

    c.bench_function("indent", |b| b.iter(|| textwrap::indent(&text, "    ")));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
