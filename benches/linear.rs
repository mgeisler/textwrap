#![feature(test)]

// The benchmarks here verify that the complexity grows as O(*n*)
// where *n* is the number of characters in the text to be wrapped.

extern crate test;
#[cfg(feature = "hyphenation")]
extern crate hyphenation;
extern crate lipsum;
extern crate textwrap;

use test::Bencher;
#[cfg(feature = "hyphenation")]
use hyphenation::Language;
#[cfg(feature = "hyphenation")]
use textwrap::Wrapper;

const LINE_LENGTH: usize = 60;

/// Generate a lorem ipsum text with the given number of characters.
fn lorem_ipsum(length: usize) -> String {
    // The average word length in the lorem ipsum text is somewhere
    // between 6 and 7. So we conservatively divide by 5 to have a
    // long enough text that we can truncate below.
    let mut text = lipsum::lipsum(length / 5);
    text.truncate(length);
    text
}

#[bench]
fn lorem_100(b: &mut Bencher) {
    let text = &lorem_ipsum(100);
    b.iter(|| textwrap::fill(text, LINE_LENGTH))
}

#[bench]
fn lorem_200(b: &mut Bencher) {
    let text = &lorem_ipsum(200);
    b.iter(|| textwrap::fill(text, LINE_LENGTH))
}

#[bench]
fn lorem_400(b: &mut Bencher) {
    let text = &lorem_ipsum(400);
    b.iter(|| textwrap::fill(text, LINE_LENGTH))
}

#[bench]
fn lorem_800(b: &mut Bencher) {
    let text = &lorem_ipsum(800);
    b.iter(|| textwrap::fill(text, LINE_LENGTH))
}

#[bench]
#[cfg(feature = "hyphenation")]
fn hyphenation_lorem_100(b: &mut Bencher) {
    let text = &lorem_ipsum(100);
    let corpus = hyphenation::load(Language::Latin).unwrap();
    let wrapper = Wrapper::new(LINE_LENGTH).word_splitter(Box::new(corpus));
    b.iter(|| wrapper.fill(text))
}

#[bench]
#[cfg(feature = "hyphenation")]
fn hyphenation_lorem_200(b: &mut Bencher) {
    let text = &lorem_ipsum(200);
    let corpus = hyphenation::load(Language::Latin).unwrap();
    let wrapper = Wrapper::new(LINE_LENGTH).word_splitter(Box::new(corpus));
    b.iter(|| wrapper.fill(text))
}

#[bench]
#[cfg(feature = "hyphenation")]
fn hyphenation_lorem_400(b: &mut Bencher) {
    let text = &lorem_ipsum(400);
    let corpus = hyphenation::load(Language::Latin).unwrap();
    let wrapper = Wrapper::new(LINE_LENGTH).word_splitter(Box::new(corpus));
    b.iter(|| wrapper.fill(text))
}

#[bench]
#[cfg(feature = "hyphenation")]
fn hyphenation_lorem_800(b: &mut Bencher) {
    let text = &lorem_ipsum(800);
    let corpus = hyphenation::load(Language::Latin).unwrap();
    let wrapper = Wrapper::new(LINE_LENGTH).word_splitter(Box::new(corpus));
    b.iter(|| wrapper.fill(text))
}
