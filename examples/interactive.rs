// The example only works on Linux since Termion does not yet support
// Windows: https://gitlab.redox-os.org/redox-os/termion/-/issues/103
// The precise library doesn't matter much, so feel free to send a PR
// if there is a library with good Windows support.

fn main() -> Result<(), std::io::Error> {
    #[cfg(not(unix))]
    panic!("Sorry, this example currently only works on Unix!");

    #[cfg(unix)]
    unix_only::main()
}

#[cfg(unix)]
mod unix_only {
    use std::io::{self, Write};
    use termion::event::Key;
    use termion::input::TermRead;
    use termion::raw::{IntoRawMode, RawTerminal};
    use termion::screen::IntoAlternateScreen;
    use termion::{color, cursor, style};
    use textwrap::{wrap, Options, WordSeparator, WordSplitter, WrapAlgorithm};

    #[cfg(feature = "hyphenation")]
    use hyphenation::{Language, Load, Standard};

    fn draw_margins(
        row: u16,
        col: u16,
        line_width: u16,
        left: char,
        right: char,
        stdout: &mut RawTerminal<io::Stdout>,
    ) -> Result<(), io::Error> {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(col - 1, row),
            color::Fg(color::Red),
            left,
            color::Fg(color::Reset),
        )?;
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(col + line_width, row),
            color::Fg(color::Red),
            right,
            color::Fg(color::Reset),
        )?;

        Ok(())
    }

    fn draw_text<'a>(
        text: &str,
        options: &Options<'a>,
        word_splitter_label: &str,
        stdout: &mut RawTerminal<io::Stdout>,
    ) -> Result<(), io::Error> {
        let mut left_row: u16 = 1;
        let left_col: u16 = 3;

        write!(stdout, "{}", termion::clear::All)?;
        write!(
            stdout,
            "{}{}Options:{}",
            cursor::Goto(left_col, left_row),
            style::Bold,
            style::Reset,
        )?;
        left_row += 1;

        write!(
            stdout,
            "{}- width: {}{}{} (use ← and → to change)",
            cursor::Goto(left_col, left_row),
            style::Bold,
            options.width,
            style::Reset,
        )?;
        left_row += 1;

        write!(
            stdout,
            "{}- break_words: {}{:?}{} (toggle with Ctrl-b)",
            cursor::Goto(left_col, left_row),
            style::Bold,
            options.break_words,
            style::Reset,
        )?;
        left_row += 1;

        write!(
            stdout,
            "{}- splitter: {}{}{} (cycle with Ctrl-s)",
            cursor::Goto(left_col, left_row),
            style::Bold,
            word_splitter_label,
            style::Reset,
        )?;
        left_row += 1;

        #[cfg(feature = "smawk")]
        {
            // The OptimalFit struct formats itself with a ton of
            // parameters. This removes the parameters, leaving only
            // the struct name behind.
            let wrap_algorithm_label = format!("{:?}", options.wrap_algorithm)
                .split('(')
                .next()
                .unwrap()
                .to_string();
            write!(
                stdout,
                "{}- algorithm: {}{}{} (toggle with Ctrl-o)",
                cursor::Goto(left_col, left_row),
                style::Bold,
                wrap_algorithm_label,
                style::Reset,
            )?;
            left_row += 1;
        }

        let now = std::time::Instant::now();
        let mut lines = wrap(text, options);
        let elapsed = now.elapsed();

        let right_col: u16 = 55;
        let mut right_row: u16 = 1;
        write!(
            stdout,
            "{}{}Performance:{}",
            cursor::Goto(right_col, right_row),
            style::Bold,
            style::Reset,
        )?;
        right_row += 1;

        write!(
            stdout,
            "{}- build: {}{}{}",
            cursor::Goto(right_col, right_row),
            style::Bold,
            if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
            style::Reset,
        )?;
        right_row += 1;

        write!(
            stdout,
            "{}- words: {}{}{}",
            cursor::Goto(right_col, right_row),
            style::Bold,
            text.split_whitespace().count(),
            style::Reset,
        )?;
        right_row += 1;

        write!(
            stdout,
            "{}- characters: {}{}{}",
            cursor::Goto(right_col, right_row),
            style::Bold,
            text.chars().count(),
            style::Reset,
        )?;
        right_row += 1;

        write!(
            stdout,
            "{}- latency: {}{} usec{}",
            cursor::Goto(right_col, right_row),
            style::Bold,
            elapsed.as_micros(),
            style::Reset,
        )?;

        // Empty line.
        left_row += 1;

        if let Some(line) = lines.last_mut() {
            let trailing_whitespace = &text[text.trim_end_matches(' ').len()..];
            if !trailing_whitespace.is_empty() {
                // Trailing whitespace is discarded by
                // `textwrap::wrap`. We reinsert it here. If multiple
                // spaces are added, this can overflow the margins
                // which look a bit odd. Handling this would require
                // some more tinkering...
                *line = format!("{}{}", line, trailing_whitespace).into();
            } else if line.ends_with('\n') {
                // If `text` ends with a newline, the final wrapped line
                // contains this newline. This will in turn leave the
                // cursor hanging in the middle of the line. Pushing an
                // extra empty line fixes this.
                lines.push("".into());
            }
        } else {
            // No lines -> we add an empty line so we have a place
            // where we can display the cursor.
            lines.push("".into());
        }

        // Draw margins above and below the wrapped text. We draw the
        // margin before the text so that 1) the text can overwrite
        // the margin if `break_words` is `false` and `width` is very
        // small and 2) so the cursor remains at the end of the last
        // line of text.
        draw_margins(left_row, left_col, options.width as u16, '┌', '┐', stdout)?;
        left_row += 1;
        let final_row = left_row + lines.len() as u16;
        draw_margins(final_row, left_col, options.width as u16, '└', '┘', stdout)?;

        let (_, rows) = termion::terminal_size()?;
        write!(stdout, "{}", cursor::Show)?;
        for line in lines {
            if left_row > rows {
                // The text does not fits on the terminal -- we hide
                // the cursor since it's supposed to be "below" the
                // bottom of the terminal.
                write!(stdout, "{}", cursor::Hide)?;
                break;
            }
            draw_margins(left_row, left_col, options.width as u16, '│', '│', stdout)?;
            write!(stdout, "{}{}", cursor::Goto(left_col, left_row), line)?;
            left_row += 1;
        }

        stdout.flush()
    }

    pub fn main() -> Result<(), io::Error> {
        let mut wrap_algorithms = Vec::new();
        #[cfg(feature = "smawk")]
        wrap_algorithms.push(WrapAlgorithm::OptimalFit(
            textwrap::wrap_algorithms::Penalties::new(),
        ));
        wrap_algorithms.push(WrapAlgorithm::FirstFit);

        let mut word_splitters: Vec<WordSplitter> =
            vec![WordSplitter::HyphenSplitter, WordSplitter::NoHyphenation];
        let mut word_splitter_labels: Vec<String> =
            word_splitters.iter().map(|s| format!("{:?}", s)).collect();

        // If you like, you can download more dictionaries from
        // https://github.com/tapeinosyne/hyphenation/tree/master/dictionaries
        // Place the dictionaries in the examples/ directory. Here we
        // just load the embedded en-us dictionary.
        #[cfg(feature = "hyphenation")]
        for lang in &[Language::EnglishUS] {
            let dictionary = Standard::from_embedded(*lang).or_else(|_| {
                let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("examples")
                    .join(format!("{}.standard.bincode", lang.code()));
                Standard::from_path(*lang, &path)
            });

            if let Ok(dict) = dictionary {
                word_splitters.insert(0, WordSplitter::Hyphenation(dict));
                word_splitter_labels.insert(0, format!("{} hyphenation", lang.code()));
            }
        }

        let mut options = Options::new(35)
            .break_words(false)
            .wrap_algorithm(wrap_algorithms.remove(0))
            .word_splitter(word_splitters.remove(0))
            .word_separator(WordSeparator::AsciiSpace);
        let mut word_splitter_label = word_splitter_labels.remove(0);

        let args = std::env::args().collect::<Vec<_>>();
        let mut text = if args.len() > 1 {
            args[1..].join(" ")
        } else {
            String::from(
                "Welcome to the interactive demo! The following is from The \
            Emperor’s New Clothes by Hans Christian Andersen. You can edit the \
            text!\n\n\
            Many years ago there was an Emperor, who was so excessively fond \
            of new clothes that he spent all his money on them. He cared \
            nothing about his soldiers, nor for the theatre, nor for driving \
            in the woods except for the sake of showing off his new clothes. \
            He had a costume for every hour in the day, and instead of saying, \
            as one does about any other king or emperor, ‘He is in his council \
            chamber,’ here one always said, ‘The Emperor is in his \
            dressing-room.’",
            )
        };

        let stdin = io::stdin();
        let mut screen = io::stdout().into_raw_mode()?.into_alternate_screen()?;
        write!(screen, "{}", cursor::BlinkingUnderline)?;
        draw_text(&text, &options, &word_splitter_label, &mut screen)?;

        for c in stdin.keys() {
            match c? {
                Key::Esc | Key::Ctrl('c') => break,
                Key::Left => options.width = options.width.saturating_sub(1),
                Key::Right => options.width = options.width.saturating_add(1),
                Key::Ctrl('b') => options.break_words = !options.break_words,
                #[cfg(feature = "smawk")]
                Key::Ctrl('o') => {
                    std::mem::swap(&mut options.wrap_algorithm, &mut wrap_algorithms[0]);
                    wrap_algorithms.rotate_left(1);
                }
                Key::Ctrl('s') => {
                    // We always keep the next splitter at position 0.
                    std::mem::swap(&mut options.word_splitter, &mut word_splitters[0]);
                    word_splitters.rotate_left(1);
                    std::mem::swap(&mut word_splitter_label, &mut word_splitter_labels[0]);
                    word_splitter_labels.rotate_left(1);
                }
                Key::Char(c) => text.push(c),
                Key::Backspace => {
                    text.pop();
                }
                // Also known as Ctrl-Backspace
                Key::Ctrl('h') => text.truncate(text.rfind(' ').unwrap_or(0)),
                _ => {}
            }

            draw_text(&text, &options, &word_splitter_label, &mut screen)?;
        }

        // TODO: change to cursor::DefaultStyle if
        // https://github.com/redox-os/termion/pull/157 is merged.
        screen.write_all(b"\x1b[0 q")?;
        screen.flush()
    }
}
