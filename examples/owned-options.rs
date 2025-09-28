//! Example showing how to deal with an owned options type.

/// Your owned options. You can put anything here which you need to
/// produce a `textwrap::Options<'a>` later. Here we specify a subset
/// of the fields in `Options`.
#[derive(Debug, Default)]
struct OwnedOptions {
    width: u16,             // Smaller integer type.
    initial_indent: String, // Owned string type.
}

impl OwnedOptions {
    fn new(width: u16) -> Self {
        Self {
            width,
            ..OwnedOptions::default()
        }
    }
}

/// All `textwrap` functions take an `impl Into<Options<'a>>`
/// argument, so with this implementation, we can transparently use
/// our own type in calls to `textwrap::wrap` and related functions.
impl<'a> From<&'a OwnedOptions> for textwrap::Options<'a> {
    fn from(owned_options: &'a OwnedOptions) -> textwrap::Options<'a> {
        textwrap::Options::new(owned_options.width.into()) // converted
            .initial_indent(&owned_options.initial_indent) // borrowed
            .break_words(true) // hard-coded
            .wrap_algorithm(textwrap::WrapAlgorithm::FirstFit)
    }
}

/// Update `options`.
///
/// We can only do this because we own the string field.
fn update_indent(n: usize, options: &mut OwnedOptions) {
    options.initial_indent = "-".repeat(n);
    options.initial_indent.push('>');
    options.initial_indent.push(' ');
}

fn main() {
    let mut owned_options = OwnedOptions::new(28);
    let text = "This text is wrapped using OwnedOptions, not the standard Options from Textwrap.";

    println!("Initial options: {owned_options:?}");
    println!("{}", textwrap::fill(text, &owned_options));
    println!();

    update_indent(5, &mut owned_options);
    println!("First update: {owned_options:?}");
    println!("{}", textwrap::fill(text, &owned_options));
    println!();

    update_indent(8, &mut owned_options);
    println!("Second update: {owned_options:?}");
    println!("{}", textwrap::fill(text, &owned_options));
}
