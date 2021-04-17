use textwrap::{AsciiSpace, HyphenSplitter, NoHyphenation, Options, WordSeparator, WordSplitter};

/// Cleaned up type name.
fn type_name<T: ?Sized>(_val: &T) -> String {
    std::any::type_name::<T>()
        .replace("alloc::boxed::Box", "Box")
        .replace("textwrap::word_separator", "textwrap")
        .replace("textwrap::splitting", "textwrap")
}

#[test]
fn static_hyphensplitter() {
    // Inferring the full type.
    let options = Options::new(10);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<textwrap::AsciiSpace, textwrap::HyphenSplitter>"
    );

    // Inferring part of the type.
    let options: Options<_, HyphenSplitter> = Options::new(10);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<textwrap::AsciiSpace, textwrap::HyphenSplitter>"
    );

    // Explicitly making all parameters inferred.
    let options: Options<'_, _, _> = Options::new(10);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<textwrap::AsciiSpace, textwrap::HyphenSplitter>"
    );
}

#[test]
fn box_static_nohyphenation() {
    // Inferred static type.
    let options = Options::new(10)
        .splitter(Box::new(NoHyphenation))
        .word_separator(Box::new(AsciiSpace));
    assert_eq!(
        type_name(&options),
        "textwrap::Options<Box<textwrap::AsciiSpace>, Box<textwrap::NoHyphenation>>"
    );
}

#[test]
fn box_dyn_wordsplitter() {
    // Inferred dynamic type due to default type parameter.
    let options = Options::new(10)
        .splitter(Box::new(NoHyphenation) as Box<dyn WordSplitter>)
        .word_separator(Box::new(AsciiSpace) as Box<dyn WordSeparator>);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<Box<dyn textwrap::WordSeparator>, Box<dyn textwrap::WordSplitter>>"
    );
}
