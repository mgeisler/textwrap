use hyphenation::{Language, Load, Standard};
use rand::Rng as _;
use textwrap::word_separators::WordSeparator as _;

#[derive(Debug)]
struct StyledWord<'a> {
    word: &'a str,
    whitespace: &'a str,
    hyphen: bool,
    style: Option<text_style::Style>,
}

impl StyledWord<'_> {
    fn render(&self, is_end: bool) {
        use text_style::termion::Termion as _;

        print!(
            "{}",
            text_style::StyledStr::new(self.word, self.style).termion()
        );

        if is_end {
            if self.hyphen {
                print!("{}", text_style::StyledStr::new("-", self.style).termion());
            }
        } else {
            print!("{}", self.whitespace);
        }
    }
}

impl AsRef<str> for StyledWord<'_> {
    fn as_ref(&self) -> &str {
        &self.word
    }
}

impl<'a> From<text_style::StyledStr<'a>> for StyledWord<'a> {
    fn from(word: text_style::StyledStr<'a>) -> Self {
        let trimmed = word.s.trim_end_matches(' ');
        Self {
            word: trimmed,
            whitespace: &word.s[trimmed.len()..],
            hyphen: false,
            style: word.style,
        }
    }
}

impl textwrap::core::Fragment for StyledWord<'_> {
    fn width(&self) -> usize {
        self.word.len()
    }

    fn whitespace_width(&self) -> usize {
        self.whitespace.len()
    }

    fn penalty_width(&self) -> usize {
        if self.hyphen {
            1
        } else {
            0
        }
    }
}

impl textwrap::word_splitters::Splittable for StyledWord<'_> {
    type Output = Self;

    fn split(&self, range: std::ops::Range<usize>, keep_ending: bool) -> Self::Output {
        let word = &self.word[range];
        Self {
            word,
            whitespace: if keep_ending { self.whitespace } else { "" },
            hyphen: if keep_ending {
                self.hyphen
            } else {
                !word.ends_with('-')
            },
            style: self.style,
        }
    }
}

fn generate_style(rng: &mut impl rand::Rng) -> text_style::Style {
    let mut style = text_style::Style::default();

    style.set_bold(rng.gen_bool(0.1));
    style.set_italic(rng.gen_bool(0.1));
    style.set_underline(rng.gen_bool(0.1));
    style.strikethrough(rng.gen_bool(0.01));

    style.fg = match rng.gen_range(0..100) {
        0..=10 => Some(text_style::AnsiColor::Red),
        11..=20 => Some(text_style::AnsiColor::Green),
        21..=30 => Some(text_style::AnsiColor::Blue),
        _ => None,
    }
    .map(|color| text_style::Color::Ansi {
        color,
        mode: text_style::AnsiMode::Light,
    });

    style
}

fn main() {
    let dictionary = Standard::from_embedded(Language::EnglishUS).unwrap();
    let mut rng = rand::thread_rng();

    let text = lipsum::lipsum(rng.gen_range(100..500));

    let styled = text
        .split_inclusive(' ')
        .map(|s| text_style::StyledStr::styled(s, generate_style(&mut rng)));
    let words: Vec<_> = styled
        .flat_map(|s| {
            textwrap::word_separators::AsciiSpace
                .find_word_ranges(&s.s)
                .map(move |range| text_style::StyledStr::new(&s.s[range], s.style))
        })
        .map(StyledWord::from)
        .flat_map(|w| textwrap::word_splitters::Fragments::new(w, &dictionary))
        .collect();

    let lines = textwrap::wrap_algorithms::wrap_first_fit(&words, &[50]);
    for line in lines {
        for (idx, fragment) in line.into_iter().enumerate() {
            fragment.render(idx + 1 == line.len());
        }
        println!();
    }
}
