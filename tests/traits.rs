use textwrap::wrap_algorithms::{FirstFit, WrapAlgorithm};
use textwrap::Options;
use textwrap::{AsciiSpace, WordSeparator};
use textwrap::{NoHyphenation, WordSplitter};

/// Cleaned up type name.
fn type_name<T: ?Sized>(_val: &T) -> String {
    std::any::type_name::<T>()
        .replace("alloc::boxed::Box", "Box")
        .replace("textwrap::word_separator", "textwrap")
        .replace("textwrap::splitting", "textwrap")
}

#[test]
#[cfg(not(feature = "smawk"))]
#[cfg(not(feature = "unicode-linebreak"))]
fn static_hyphensplitter() {
    // Inferring the full type.
    let options = Options::new(10);
    assert_eq!(
        type_name(&options),
        format!(
            "textwrap::Options<{}, {}, {}>",
            "textwrap::wrap_algorithms::FirstFit",
            "textwrap::AsciiSpace",
            "textwrap::HyphenSplitter"
        )
    );

    // Inferring part of the type.
    let options: Options<_, _, textwrap::HyphenSplitter> = Options::new(10);
    assert_eq!(
        type_name(&options),
        format!(
            "textwrap::Options<{}, {}, {}>",
            "textwrap::wrap_algorithms::FirstFit",
            "textwrap::AsciiSpace",
            "textwrap::HyphenSplitter"
        )
    );

    // Explicitly making all parameters inferred.
    let options: Options<_, _, _> = Options::new(10);
    assert_eq!(
        type_name(&options),
        format!(
            "textwrap::Options<{}, {}, {}>",
            "textwrap::wrap_algorithms::FirstFit",
            "textwrap::AsciiSpace",
            "textwrap::HyphenSplitter"
        )
    );
}

#[test]
fn box_static_nohyphenation() {
    // Inferred static type.
    let options = Options::new(10)
        .wrap_algorithm(Box::new(FirstFit))
        .splitter(Box::new(NoHyphenation))
        .word_separator(Box::new(AsciiSpace));
    assert_eq!(
        type_name(&options),
        format!(
            "textwrap::Options<{}, {}, {}>",
            "Box<textwrap::wrap_algorithms::FirstFit>",
            "Box<textwrap::AsciiSpace>",
            "Box<textwrap::NoHyphenation>"
        )
    );
}

#[test]
fn box_dyn_wordsplitter() {
    // Inferred dynamic type due to default type parameter.
    let options = Options::new(10)
        .wrap_algorithm(Box::new(FirstFit) as Box<dyn WrapAlgorithm>)
        .splitter(Box::new(NoHyphenation) as Box<dyn WordSplitter>)
        .word_separator(Box::new(AsciiSpace) as Box<dyn WordSeparator>);
    assert_eq!(
        type_name(&options),
        format!(
            "textwrap::Options<{}, {}, {}>",
            "Box<dyn textwrap::wrap_algorithms::WrapAlgorithm>",
            "Box<dyn textwrap::WordSeparator>",
            "Box<dyn textwrap::WordSplitter>"
        )
    );
}
