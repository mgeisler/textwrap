//! Word splitting functionality.
//!
//! To wrap text into lines, long words sometimes need to be split
//! across lines. The [`WordSplitter`] trait defines this
//! functionality. [`HyphenSplitter`] is the default implementation of
//! this treat: it will simply split words on existing hyphens.

/// The `WordSplitter` trait describes where words can be split.
///
/// If the textwrap crate has been compiled with the `hyphenation`
/// Cargo feature enabled, you will find an implementation of
/// `WordSplitter` by the `hyphenation::Standard` struct. Use this
/// struct for language-aware hyphenation:
///
/// ```
/// #[cfg(feature = "hyphenation")]
/// {
///     use hyphenation::{Language, Load, Standard};
///     use textwrap::{wrap, Options};
///
///     let text = "Oxidation is the loss of electrons.";
///     let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
///     let options = Options::new(8).splitter(dictionary);
///     assert_eq!(wrap(text, &options), vec!["Oxida-",
///                                           "tion is",
///                                           "the loss",
///                                           "of elec-",
///                                           "trons."]);
/// }
/// ```
///
/// Please see the documentation for the [hyphenation] crate for more
/// details.
///
/// [hyphenation]: https://docs.rs/hyphenation/
pub trait WordSplitter: std::fmt::Debug {
    /// Return all possible indices where `word` can be split.
    ///
    /// The indices returned must be in range `0..word.len()`. They
    /// should point to the index _after_ the split point, i.e., after
    /// `-` if splitting on hyphens. This way, `word.split_at(idx)`
    /// will break the word into two well-formed pieces.
    ///
    /// # Examples
    ///
    /// ```
    /// use textwrap::{HyphenSplitter, NoHyphenation, WordSplitter};
    /// assert_eq!(NoHyphenation.split_points("cannot-be-split"), vec![]);
    /// assert_eq!(HyphenSplitter.split_points("can-be-split"), vec![4, 7]);
    /// ```
    fn split_points(&self, word: &str) -> Vec<usize>;
}

impl<S: WordSplitter + ?Sized> WordSplitter for Box<S> {
    fn split_points(&self, word: &str) -> Vec<usize> {
        use std::ops::Deref;
        self.deref().split_points(word)
    }
}

impl<T: ?Sized + WordSplitter> WordSplitter for &T {
    fn split_points(&self, word: &str) -> Vec<usize> {
        (*self).split_points(word)
    }
}

/// Use this as a [`Options.splitter`] to avoid any kind of
/// hyphenation:
///
/// ```
/// use textwrap::{wrap, NoHyphenation, Options};
///
/// let options = Options::new(8).splitter(NoHyphenation);
/// assert_eq!(wrap("foo bar-baz", &options),
///            vec!["foo", "bar-baz"]);
/// ```
///
/// [`Options.splitter`]: super::Options::splitter
#[derive(Clone, Copy, Debug)]
pub struct NoHyphenation;

/// `NoHyphenation` implements `WordSplitter` by not splitting the
/// word at all.
impl WordSplitter for NoHyphenation {
    fn split_points(&self, _: &str) -> Vec<usize> {
        Vec::new()
    }
}

/// Simple and default way to split words: splitting on existing
/// hyphens only.
///
/// You probably don't need to use this type since it's already used
/// by default by [`Options::new`](super::Options::new).
#[derive(Clone, Copy, Debug)]
pub struct HyphenSplitter;

/// `HyphenSplitter` is the default `WordSplitter` used by
/// [`Options::new`](super::Options::new). It will split words on any
/// existing hyphens in the word.
///
/// It will only use hyphens that are surrounded by alphanumeric
/// characters, which prevents a word like `"--foo-bar"` from being
/// split into `"--"` and `"foo-bar"`.
impl WordSplitter for HyphenSplitter {
    fn split_points(&self, word: &str) -> Vec<usize> {
        let mut splits = Vec::new();

        for (idx, _) in word.match_indices('-') {
            // We only use hyphens that are surrounded by alphanumeric
            // characters. This is to avoid splitting on repeated hyphens,
            // such as those found in --foo-bar.
            let prev = word[..idx].chars().next_back();
            let next = word[idx + 1..].chars().next();

            if prev.filter(|ch| ch.is_alphanumeric()).is_some()
                && next.filter(|ch| ch.is_alphanumeric()).is_some()
            {
                splits.push(idx + 1); // +1 due to width of '-'.
            }
        }

        splits
    }
}

/// A hyphenation dictionary can be used to do language-specific
/// hyphenation using patterns from the [hyphenation] crate.
///
/// **Note:** Only available when the `hyphenation` Cargo feature is
/// enabled.
///
/// [hyphenation]: https://docs.rs/hyphenation/
#[cfg(feature = "hyphenation")]
impl WordSplitter for hyphenation::Standard {
    fn split_points(&self, word: &str) -> Vec<usize> {
        use hyphenation::Hyphenator;
        self.hyphenate(word).breaks
    }
}
