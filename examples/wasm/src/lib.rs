use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use textwrap::word_splitters::split_words;
use textwrap::wrap_algorithms::{wrap_first_fit, wrap_optimal_fit, Penalties};
use textwrap::{WordSeparator, WordSplitter};

#[wasm_bindgen]
extern "C" {
    // https://github.com/rustwasm/wasm-bindgen/issues/2069#issuecomment-774038243
    type ExtendedTextMetrics;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxLeft)]
    fn actual_bounding_box_left(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxRight)]
    fn actual_bounding_box_right(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxAscent)]
    fn actual_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter, js_name = actualBoundingBoxDescent)]
    fn actual_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

    // TODO: Enable when Firefox and Edge support these methods, see
    // https://developer.mozilla.org/en-US/docs/Web/API/TextMetrics
    //
    // #[wasm_bindgen(method, getter, js_name = fontBoundingBoxAscent)]
    // fn font_bounding_box_ascent(this: &ExtendedTextMetrics) -> f64;
    //
    // #[wasm_bindgen(method, getter, js_name = fontBoundingBoxDescent)]
    // fn font_bounding_box_descent(this: &ExtendedTextMetrics) -> f64;

    #[wasm_bindgen(method, getter)]
    fn width(this: &ExtendedTextMetrics) -> f64;
}

fn canvas_width(ctx: &web_sys::CanvasRenderingContext2d, text: &str) -> f64 {
    ctx.measure_text(text).unwrap().width()
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CanvasWord<'a> {
    word: &'a str,
    width: f64,
    whitespace: &'a str,
    whitespace_width: f64,
    penalty: &'a str,
    penalty_width: f64,
}

impl<'a> CanvasWord<'a> {
    fn from(ctx: &'_ web_sys::CanvasRenderingContext2d, word: textwrap::core::Word<'a>) -> Self {
        CanvasWord {
            word: word.word,
            width: canvas_width(ctx, word.word),
            whitespace: word.whitespace,
            whitespace_width: canvas_width(ctx, word.whitespace),
            penalty: word.penalty,
            penalty_width: canvas_width(ctx, word.penalty),
        }
    }

    fn break_apart(
        self,
        ctx: &'_ web_sys::CanvasRenderingContext2d,
        max_width: f64,
    ) -> Vec<CanvasWord<'a>> {
        if self.width <= max_width {
            return vec![self];
        }

        let mut start = 0;
        let mut words = Vec::new();
        for (idx, grapheme) in self.word.grapheme_indices(true) {
            let with_grapheme = &self.word[start..idx + grapheme.len()];
            let without_grapheme = &self.word[start..idx];
            if idx > 0 && canvas_width(ctx, with_grapheme) > max_width {
                // The part without the grapheme fits on the line. We
                // give it a width of max_width instead of its natural
                // width to ensure that it takes up the full line.
                //
                // Otherwise, we can end up with a situation where a
                // text fits in _fewer_ lines when the line width is
                // _smaller_. This happens with proportional fonts,
                // such as the sans-serif or serif fonts. An example
                // text which illustrates the problem is:
                //
                //   i XYZ
                //
                // Line width: 42px. Normal break, XYZ doesn't fit on
                // first line:
                //
                //   i
                //   XYZ
                //
                // Line width: 41px. XYZ takes up 41.1px, so it is
                // broken apart. The first part now fits on the first
                // line:
                //
                //   i XY
                //   Z
                //
                // Line width: 39px. There is no longer room for XY on
                // the first line:
                //
                //   i
                //   XY
                //   Z
                //
                // Line width: 28px. XY takes up 28.9px, so it is
                // broken apart. YZ takes up 26.7px, so everything
                // suddenly fits on two lines again:
                //
                //   i X
                //   YZ
                //
                // We can be a more "natural" or "monotone" behavior
                // by making the parts take up at least the full line
                // width.
                let natural_width = canvas_width(ctx, without_grapheme);
                words.push(CanvasWord {
                    word: without_grapheme,
                    width: max_width.max(natural_width),
                    whitespace: "",
                    whitespace_width: 0.0,
                    penalty: "",
                    penalty_width: 0.0,
                });
                start = idx;
            }
        }

        words.push(CanvasWord {
            word: &self.word[start..],
            width: canvas_width(ctx, &self.word[start..]),
            whitespace: self.whitespace,
            whitespace_width: self.whitespace_width,
            penalty: self.penalty,
            penalty_width: self.penalty_width,
        });

        words
    }
}

impl textwrap::core::Fragment for CanvasWord<'_> {
    #[inline]
    fn width(&self) -> f64 {
        self.width
    }

    #[inline]
    fn whitespace_width(&self) -> f64 {
        self.whitespace_width
    }

    #[inline]
    fn penalty_width(&self) -> f64 {
        self.penalty_width
    }
}

fn draw_path(
    ctx: &web_sys::CanvasRenderingContext2d,
    style: &str,
    (mut x, mut y): (f64, f64),
    steps: &[(f64, f64)],
) {
    ctx.save();
    ctx.set_stroke_style(&style.into());
    ctx.begin_path();
    ctx.move_to(x, y);
    for (delta_x, delta_y) in steps {
        x += delta_x;
        y += delta_y;
        ctx.line_to(x, y);
    }
    ctx.stroke();
    ctx.restore();
}

// We offset all text by the width of the round slider. This ensures
// no clipping due to anti-aliasing.
const X_OFFSET: f64 = 8.0;

fn draw_word(
    ctx: &web_sys::CanvasRenderingContext2d,
    x: f64,
    y: f64,
    word: &CanvasWord,
    last_word: bool,
) -> Result<(), JsValue> {
    ctx.fill_text(word.word, x, y)?;

    draw_path(
        ctx,
        "orange",
        (x, y - 10.0),
        &[(0.0, 10.0), (word.width, 0.0)],
    );

    ctx.save();
    ctx.set_font("10px sans-serif");
    ctx.set_text_align("center");
    ctx.set_text_baseline("top");
    ctx.fill_text(
        &format!("{:.1}px", word.width),
        x + word.width / 2.0,
        y + 3.0,
    )?;
    ctx.restore();

    let x = x + word.width;
    if last_word {
        ctx.fill_text(word.penalty, x, y)?;
        draw_path(ctx, "red", (x, y), &[(word.penalty_width, 0.0)]);
    } else {
        ctx.fill_text(word.whitespace, x, y)?;
        draw_path(ctx, "lightblue", (x, y), &[(word.whitespace_width, 0.0)]);
    }

    Ok(())
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WasmWordSeparator {
    AsciiSpace = "AsciiSpace",
    UnicodeBreakProperties = "UnicodeBreakProperties",
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WasmWordSplitter {
    NoHyphenation = "NoHyphenation",
    HyphenSplitter = "HyphenSplitter",
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum WasmWrapAlgorithm {
    FirstFit = "FirstFit",
    OptimalFit = "OptimalFit",
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Default)]
pub struct WasmPenalties {
    pub nline_penalty: usize,
    pub overflow_penalty: usize,
    pub short_last_line_fraction: usize,
    pub short_last_line_penalty: usize,
    pub hyphen_penalty: usize,
}

#[wasm_bindgen]
impl WasmPenalties {
    #[wasm_bindgen(constructor)]
    pub fn new(
        nline_penalty: usize,
        overflow_penalty: usize,
        short_last_line_fraction: usize,
        short_last_line_penalty: usize,
        hyphen_penalty: usize,
    ) -> WasmPenalties {
        WasmPenalties {
            nline_penalty,
            overflow_penalty,
            short_last_line_fraction,
            short_last_line_penalty,
            hyphen_penalty,
        }
    }
}

impl From<WasmPenalties> for Penalties {
    fn from(val: WasmPenalties) -> Self {
        Penalties {
            nline_penalty: val.nline_penalty,
            overflow_penalty: val.overflow_penalty,
            short_last_line_fraction: val.short_last_line_fraction,
            short_last_line_penalty: val.short_last_line_penalty,
            hyphen_penalty: val.hyphen_penalty,
        }
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct WasmOptions {
    pub width: f64,
    pub break_words: bool,
    pub word_separator: WasmWordSeparator,
    pub word_splitter: WasmWordSplitter,
    pub wrap_algorithm: WasmWrapAlgorithm,
    pub penalties: WasmPenalties,
}

#[wasm_bindgen]
impl WasmOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(
        width: f64,
        break_words: bool,
        word_separator: WasmWordSeparator,
        word_splitter: WasmWordSplitter,
        wrap_algorithm: WasmWrapAlgorithm,
        penalties: WasmPenalties,
    ) -> WasmOptions {
        WasmOptions {
            width,
            break_words,
            word_separator,
            word_splitter,
            wrap_algorithm,
            penalties,
        }
    }
}

#[wasm_bindgen]
pub fn draw_wrapped_text(
    ctx: &web_sys::CanvasRenderingContext2d,
    options: &WasmOptions,
    text: &str,
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let metrics: web_sys::TextMetrics = ctx.measure_text("│").unwrap();
    let metrics: ExtendedTextMetrics = metrics.unchecked_into();
    // TODO: use metrics.font_bounding_box_ascent() +
    // metrics.font_bounding_box_descent() and measure "" instead of a
    // tall character when supported by Firefox.
    let line_height = metrics.actual_bounding_box_ascent() + metrics.actual_bounding_box_descent();
    let baseline_distance = 1.5 * line_height;

    let word_separator = match options.word_separator {
        WasmWordSeparator::AsciiSpace => WordSeparator::AsciiSpace,
        WasmWordSeparator::UnicodeBreakProperties => WordSeparator::UnicodeBreakProperties,
        _ => Err("WasmOptions has an invalid word_separator field")?,
    };

    let word_splitter = match options.word_splitter {
        WasmWordSplitter::NoHyphenation => WordSplitter::NoHyphenation,
        WasmWordSplitter::HyphenSplitter => WordSplitter::HyphenSplitter,
        _ => Err("WasmOptions has an invalid word_splitter field")?,
    };

    let mut lineno = 0;
    for line in text.split('\n') {
        let words = word_separator.find_words(line);
        let split_words = split_words(words, &word_splitter);

        let canvas_words = split_words
            .flat_map(|word| {
                let canvas_word = CanvasWord::from(ctx, word);
                if options.break_words {
                    canvas_word.break_apart(ctx, options.width)
                } else {
                    vec![canvas_word]
                }
            })
            .collect::<Vec<_>>();

        let line_lengths = [options.width];
        let wrapped_words = match options.wrap_algorithm {
            WasmWrapAlgorithm::FirstFit => wrap_first_fit(&canvas_words, &line_lengths),
            WasmWrapAlgorithm::OptimalFit => {
                let penalties = options.penalties.into();
                wrap_optimal_fit(&canvas_words, &line_lengths, &penalties).unwrap()
            }
            _ => Err("WasmOptions has an invalid wrap_algorithm field")?,
        };

        for words_in_line in wrapped_words {
            lineno += 1;
            let mut x = X_OFFSET;
            let y = baseline_distance * lineno as f64;

            for (i, word) in words_in_line.iter().enumerate() {
                let last_word = i == words_in_line.len() - 1;
                draw_word(ctx, x, y, word, last_word)?;
                x += word.width;
                x += if last_word {
                    word.penalty_width
                } else {
                    word.whitespace_width
                };
            }

            ctx.save();
            ctx.set_font("10px sans-serif");
            ctx.fill_text(
                &format!("{:.1}px", x - X_OFFSET),
                1.5 * X_OFFSET + options.width,
                y,
            )?;
            ctx.restore();
        }
    }

    draw_path(
        ctx,
        "blue",
        (
            X_OFFSET + options.width,
            metrics.actual_bounding_box_ascent(),
        ),
        &[(0.0, baseline_distance * lineno as f64)],
    );

    Ok(())
}
