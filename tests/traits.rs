use textwrap::{NoHyphenation, Options};

/// Cleaned up type name.
fn type_name<T: ?Sized>(_val: &T) -> String {
    std::any::type_name::<T>()
        .replace("alloc::boxed::Box", "Box")
        .replace("textwrap::splitting", "textwrap")
}

#[test]
fn static_hyphensplitter() {
    // Inferring the full type.
    let options = Options::new(10);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<textwrap::HyphenSplitter>"
    );

    // Explicitly making both parameters inferred.
    let options: Options<'_, _> = Options::new(10);
    assert_eq!(
        type_name(&options),
        "textwrap::Options<textwrap::HyphenSplitter>"
    );
}

#[test]
fn box_static_nohyphenation() {
    // Inferred static type.
    let options = Options::new(10).splitter(Box::new(NoHyphenation));
    assert_eq!(
        type_name(&options),
        "textwrap::Options<Box<textwrap::NoHyphenation>>"
    );
}

#[test]
fn box_dyn_wordsplitter() {
    // Inferred dynamic type due to default type parameter.
    let options: Options = Options::new(10).splitter(Box::new(NoHyphenation));
    assert_eq!(
        type_name(&options),
        "textwrap::Options<Box<dyn textwrap::WordSplitter>>"
    );
}
