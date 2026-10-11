//! Functions for dry-run wrapping text.

use crate::core::{break_words, display_width, Fragment, Word};
use crate::word_splitters::split_words;
use crate::Options;

/// Try wrapping a line of text at a given width.
///
/// The result is a vector of display widths of each line.
///
/// Usage is identical to [`wrap()`](crate::wrap()).
pub fn try_wrap<'a, Opt>(text: &str, width_or_options: Opt) -> Vec<usize>
where
    Opt: Into<Options<'a>>,
{
    let options: Options = width_or_options.into();
    let line_ending_str = options.line_ending.as_str();

    let mut counts = Vec::new();
    for line in text.split(line_ending_str) {
        try_wrap_single_line(line, &options, &mut counts);
    }

    counts
}

pub(crate) fn try_wrap_single_line(line: &str, options: &Options<'_>, counts: &mut Vec<usize>) {
    let indent = if counts.is_empty() {
        options.initial_indent
    } else {
        options.subsequent_indent
    };
    if line.len() < options.width && indent.is_empty() {
        counts.push(display_width(line.trim_end_matches(' ')));
    } else {
        try_wrap_single_line_slow_path(line, options, counts);
    }
}

pub(crate) fn try_wrap_single_line_slow_path(
    line: &str,
    options: &Options<'_>,
    counts: &mut Vec<usize>,
) {
    let initial_indent_dw = display_width(options.initial_indent);
    let subsequent_indent_dw = display_width(options.subsequent_indent);
    let initial_width = options.width.saturating_sub(initial_indent_dw);
    let subsequent_width = options.width.saturating_sub(subsequent_indent_dw);
    let line_widths = [initial_width, subsequent_width];

    let words = options.word_separator.find_words(line);
    let split_words = split_words(words, &options.word_splitter);
    let broken_words = if options.break_words {
        let mut broken_words = break_words(split_words, line_widths[1]);
        if !options.initial_indent.is_empty() {
            // Without this, the first word will always go into the
            // first line. However, since we break words based on the
            // _second_ line width, it can be wrong to unconditionally
            // put the first word onto the first line. An empty
            // zero-width word fixed this.
            broken_words.insert(0, Word::from(""));
        }
        broken_words
    } else {
        split_words.collect::<Vec<_>>()
    };

    let wrapped_words = options.wrap_algorithm.wrap(&broken_words, &line_widths);

    for words in wrapped_words {
        let last_word = match words.last() {
            None => {
                counts.push(0);
                continue;
            }
            Some(word) => word,
        };

        let mut width = words
            .iter()
            .map(|word| (word.width() as usize) + (word.whitespace_width() as usize))
            .sum::<usize>()
            - (last_word.whitespace_width() as usize);
        if !last_word.penalty.is_empty() {
            width += last_word.penalty_width() as usize;
        }

        let cnt = if counts.is_empty() && !options.initial_indent.is_empty() {
            initial_indent_dw
        } else if !counts.is_empty() && !options.subsequent_indent.is_empty() {
            subsequent_indent_dw
        } else {
            0
        };
        let cnt = cnt + width;
        counts.push(cnt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{WordSeparator, WordSplitter, WrapAlgorithm};

    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};

    /// Create a vec of `display_width`s.
    macro_rules! dw_vec {
    ( $( $x:expr ),* $(,)? ) => {
        vec![$( crate::core::display_width($x) ),*]
    };
}

    #[test]
    fn no_wrap() {
        assert_eq!(try_wrap("foo", 10), dw_vec!["foo"]);
    }

    #[test]
    fn wrap_simple() {
        assert_eq!(try_wrap("foo bar baz", 5), dw_vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn to_be_or_not() {
        assert_eq!(
            try_wrap(
                "To be, or not to be, that is the question.",
                Options::new(10).wrap_algorithm(WrapAlgorithm::FirstFit)
            ),
            dw_vec!["To be, or", "not to be,", "that is", "the", "question."]
        );
    }

    #[test]
    fn multiple_words_on_first_line() {
        assert_eq!(try_wrap("foo bar baz", 10), dw_vec!["foo bar", "baz"]);
    }

    #[test]
    fn long_word() {
        assert_eq!(try_wrap("foo", 0), dw_vec!["f", "o", "o"]);
    }

    #[test]
    fn long_words() {
        assert_eq!(
            try_wrap("foo bar", 0),
            dw_vec!["f", "o", "o", "b", "a", "r"]
        );
    }

    #[test]
    fn max_width() {
        assert_eq!(try_wrap("foo bar", usize::MAX), dw_vec!["foo bar"]);

        let text = "Hello there! This is some English text. \
                    It should not be wrapped given the extents below.";
        assert_eq!(try_wrap(text, usize::MAX), dw_vec![text]);
    }

    #[test]
    fn leading_whitespace() {
        assert_eq!(try_wrap("  foo bar", 6), dw_vec!["  foo", "bar"]);
    }

    #[test]
    fn leading_whitespace_empty_first_line() {
        // If there is no space for the first word, the first line
        // will be empty. This is because the string is split into
        // words like [" ", "foobar ", "baz"], which puts "foobar " on
        // the second line. We never output trailing whitespace
        assert_eq!(try_wrap(" foobar baz", 6), dw_vec!["", "foobar", "baz"]);
    }

    #[test]
    fn trailing_whitespace() {
        // Whitespace is only significant inside a line. After a line
        // gets too long and is broken, the first word starts in
        // column zero and is not indented.
        assert_eq!(
            try_wrap("foo     bar     baz  ", 5),
            dw_vec!["foo", "bar", "baz"]
        );
    }

    #[test]
    fn issue_99() {
        // We did not reset the in_whitespace flag correctly and did
        // not handle single-character words after a line break.
        assert_eq!(
            try_wrap("aaabbbccc x yyyzzzwww", 9),
            dw_vec!["aaabbbccc", "x", "yyyzzzwww"]
        );
    }

    #[test]
    fn issue_129() {
        // The dash is an em-dash which takes up four bytes. We used
        // to panic since we tried to index into the character.
        let options = Options::new(1).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(try_wrap("x – x", options), dw_vec!["x", "–", "x"]);
    }

    #[test]
    fn wide_character_handling() {
        assert_eq!(try_wrap("Hello, World!", 15), dw_vec!["Hello, World!"]);
        assert_eq!(
            try_wrap(
                "Ｈｅｌｌｏ, Ｗｏｒｌｄ!",
                Options::new(15).word_separator(WordSeparator::AsciiSpace)
            ),
            dw_vec!["Ｈｅｌｌｏ,", "Ｗｏｒｌｄ!"]
        );

        // Wide characters are allowed to break if the
        // unicode-linebreak feature is enabled.
        #[cfg(feature = "unicode-linebreak")]
        assert_eq!(
            try_wrap(
                "Ｈｅｌｌｏ, Ｗｏｒｌｄ!",
                Options::new(15).word_separator(WordSeparator::UnicodeBreakProperties),
            ),
            dw_vec!["Ｈｅｌｌｏ, Ｗ", "ｏｒｌｄ!"]
        );
    }

    #[test]
    fn indent_empty_line() {
        // Previously, indentation was not applied to empty lines.
        // However, this is somewhat inconsistent and undesirable if
        // the indentation is something like a border ("| ") which you
        // want to apply to all lines, empty or not.
        let options = Options::new(10).initial_indent("!!!");
        assert_eq!(try_wrap("", &options), dw_vec!["!!!"]);
    }

    #[test]
    fn indent_single_line() {
        let options = Options::new(10).initial_indent(">>>"); // No trailing space
        assert_eq!(try_wrap("foo", &options), dw_vec![">>>foo"]);
    }

    #[test]
    fn indent_first_emoji() {
        let options = Options::new(10).initial_indent("👉👉");
        assert_eq!(
            try_wrap("x x x x x x x x x x x x x", &options),
            dw_vec!["👉👉x x x", "x x x x x", "x x x x x"]
        );
    }

    #[test]
    fn indent_multiple_lines() {
        let options = Options::new(6).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(
            try_wrap("foo bar baz", &options),
            dw_vec!["* foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn only_initial_indent_multiple_lines() {
        let options = Options::new(10).initial_indent("  ");
        assert_eq!(
            try_wrap("foo\nbar\nbaz", &options),
            dw_vec!["  foo", "bar", "baz"]
        );
    }

    #[test]
    fn only_subsequent_indent_multiple_lines() {
        let options = Options::new(10).subsequent_indent("  ");
        assert_eq!(
            try_wrap("foo\nbar\nbaz", &options),
            dw_vec!["foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn indent_break_words() {
        let options = Options::new(5).initial_indent("* ").subsequent_indent("  ");
        assert_eq!(
            try_wrap("foobarbaz", &options),
            dw_vec!["* foo", "  bar", "  baz"]
        );
    }

    #[test]
    fn initial_indent_break_words() {
        // This is a corner-case showing how the long word is broken
        // according to the width of the subsequent lines. The first
        // fragment of the word no longer fits on the first line,
        // which ends up being pure indentation.
        let options = Options::new(5).initial_indent("-->");
        assert_eq!(
            try_wrap("foobarbaz", &options),
            dw_vec!["-->", "fooba", "rbaz"]
        );
    }

    #[test]
    fn hyphens() {
        assert_eq!(try_wrap("foo-bar", 5), dw_vec!["foo-", "bar"]);
    }

    #[test]
    fn trailing_hyphen() {
        let options = Options::new(5).break_words(false);
        assert_eq!(try_wrap("foobar-", &options), dw_vec!["foobar-"]);
    }

    #[test]
    fn multiple_hyphens() {
        assert_eq!(try_wrap("foo-bar-baz", 5), dw_vec!["foo-", "bar-", "baz"]);
    }

    #[test]
    fn hyphens_flag() {
        let options = Options::new(5).break_words(false);
        assert_eq!(
            try_wrap("The --foo-bar flag.", &options),
            dw_vec!["The", "--foo-", "bar", "flag."]
        );
    }

    #[test]
    fn repeated_hyphens() {
        let options = Options::new(4).break_words(false);
        assert_eq!(try_wrap("foo--bar", &options), dw_vec!["foo--bar"]);
    }

    #[test]
    fn hyphens_alphanumeric() {
        assert_eq!(try_wrap("Na2-CH4", 5), dw_vec!["Na2-", "CH4"]);
    }

    #[test]
    fn hyphens_non_alphanumeric() {
        let options = Options::new(5).break_words(false);
        assert_eq!(try_wrap("foo(-)bar", &options), dw_vec!["foo(-)bar"]);
    }

    #[test]
    fn multiple_splits() {
        assert_eq!(try_wrap("foo-bar-baz", 9), dw_vec!["foo-bar-", "baz"]);
    }

    #[test]
    fn forced_split() {
        let options = Options::new(5).break_words(false);
        assert_eq!(try_wrap("foobar-baz", &options), dw_vec!["foobar-", "baz"]);
    }

    #[test]
    fn multiple_unbroken_words_issue_193() {
        let options = Options::new(3).break_words(false);
        assert_eq!(
            try_wrap("small large tiny", &options),
            dw_vec!["small", "large", "tiny"]
        );
        assert_eq!(
            try_wrap("small  large   tiny", &options),
            dw_vec!["small", "large", "tiny"]
        );
    }

    #[test]
    fn very_narrow_lines_issue_193() {
        let options = Options::new(1).break_words(false);
        assert_eq!(try_wrap("fooo x y", &options), dw_vec!["fooo", "x", "y"]);
        assert_eq!(
            try_wrap("fooo   x     y", &options),
            dw_vec!["fooo", "x", "y"]
        );
    }

    #[test]
    fn simple_hyphens() {
        let options = Options::new(8).word_splitter(WordSplitter::HyphenSplitter);
        assert_eq!(
            try_wrap("foo bar-baz", &options),
            dw_vec!["foo bar-", "baz"]
        );
    }

    #[test]
    fn no_hyphenation() {
        let options = Options::new(8).word_splitter(WordSplitter::NoHyphenation);
        assert_eq!(try_wrap("foo bar-baz", &options), dw_vec!["foo", "bar-baz"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_double_hyphenation() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            try_wrap("Internationalization", &options),
            dw_vec!["Internatio", "nalization"]
        );

        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            try_wrap("Internationalization", &options),
            dw_vec!["Interna-", "tionaliza-", "tion"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_issue_158() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10);
        assert_eq!(
            try_wrap("participation is the key to success", &options),
            dw_vec!["participat", "ion is", "the key to", "success"]
        );

        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            try_wrap("participation is the key to success", &options),
            dw_vec!["partici-", "pation is", "the key to", "success"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn split_len_hyphenation() {
        // Test that hyphenation takes the width of the whitespace
        // into account.
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(15).word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            try_wrap("garbage   collection", &options),
            dw_vec!["garbage   col-", "lection"]
        );
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn borrowed_lines() {
        // Lines that end with an extra hyphen are owned, the final
        // line is borrowed.
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(10).word_splitter(WordSplitter::Hyphenation(dictionary));
        let lines = try_wrap("Internationalization", &options);
        assert_eq!(lines, dw_vec!["Interna-", "tionaliza-", "tion"]);
    }

    #[test]
    #[cfg(feature = "hyphenation")]
    fn auto_hyphenation_with_hyphen() {
        let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
        let options = Options::new(8).break_words(false);
        assert_eq!(
            try_wrap("over-caffinated", &options),
            dw_vec!["over-", "caffinated"]
        );

        let options = options.word_splitter(WordSplitter::Hyphenation(dictionary));
        assert_eq!(
            try_wrap("over-caffinated", &options),
            dw_vec!["over-", "caffi-", "nated"]
        );
    }

    #[test]
    fn break_words() {
        assert_eq!(try_wrap("foobarbaz", 3), dw_vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn break_words_wide_characters() {
        // Even the poor man's version of `ch_width` counts these
        // characters as wide.
        let options = Options::new(5).word_separator(WordSeparator::AsciiSpace);
        assert_eq!(
            try_wrap("Ｈｅｌｌｏ", options),
            dw_vec!["Ｈｅ", "ｌｌ", "ｏ"]
        );
    }

    #[test]
    fn break_words_zero_width() {
        assert_eq!(try_wrap("foobar", 0), dw_vec!["f", "o", "o", "b", "a", "r"]);
    }

    #[test]
    fn break_long_first_word() {
        assert_eq!(try_wrap("testx y", 4), dw_vec!["test", "x y"]);
    }

    #[test]
    fn wrap_preserves_line_breaks_trims_whitespace() {
        assert_eq!(try_wrap("  ", 80), dw_vec![""]);
        assert_eq!(try_wrap("  \n  ", 80), dw_vec!["", ""]);
        assert_eq!(try_wrap("  \n \n  \n ", 80), dw_vec!["", "", "", ""]);
    }

    #[test]
    fn wrap_colored_text() {
        // The words are much longer than 6 bytes, but they remain
        // intact after filling the text.
        let green_hello = "\u{1b}[0m\u{1b}[32mHello\u{1b}[0m";
        let blue_world = "\u{1b}[0m\u{1b}[34mWorld!\u{1b}[0m";
        assert_eq!(
            try_wrap(&format!("{} {}", green_hello, blue_world), 6),
            dw_vec![green_hello, blue_world],
        );
    }
}
