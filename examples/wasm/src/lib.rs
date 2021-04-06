use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use textwrap::core;

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
    fn from(ctx: &'_ web_sys::CanvasRenderingContext2d, word: core::Word<'a>) -> Self {
        CanvasWord {
            word: word.word,
            width: canvas_width(ctx, word.word),
            whitespace: word.whitespace,
            whitespace_width: canvas_width(ctx, word.whitespace),
            penalty: word.penalty,
            penalty_width: canvas_width(ctx, word.penalty),
        }
    }
}

const PRECISION: usize = 10;

impl core::Fragment for CanvasWord<'_> {
    #[inline]
    fn width(&self) -> usize {
        (self.width * PRECISION as f64) as usize
    }

    #[inline]
    fn whitespace_width(&self) -> usize {
        (self.whitespace_width * PRECISION as f64) as usize
    }

    #[inline]
    fn penalty_width(&self) -> usize {
        (self.penalty_width * PRECISION as f64) as usize
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
pub fn draw_wrapped_text(
    ctx: &web_sys::CanvasRenderingContext2d,
    text: &str,
    width: usize,
) -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let metrics: web_sys::TextMetrics = ctx.measure_text("│").unwrap();
    let metrics: ExtendedTextMetrics = metrics.unchecked_into();
    // TODO: use metrics.font_bounding_box_ascent() +
    // metrics.font_bounding_box_descent() and measure "" instead of a
    // tall character when supported by Firefox.
    let line_height = metrics.actual_bounding_box_ascent() + metrics.actual_bounding_box_descent();
    let baseline_distance = 1.5 * line_height;
    let options = textwrap::Options::from(width);

    let mut lineno = 0;
    for line in text.split('\n') {
        let words = core::find_words(line);
        let split_words = core::split_words(words, &options);

        let canvas_words = split_words
            .map(|word| CanvasWord::from(ctx, word))
            .collect::<Vec<_>>();

        let line_lengths = |_| width * PRECISION;
        let wrapped_words = core::wrap_first_fit(&canvas_words, line_lengths);

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
                1.5 * X_OFFSET + width as f64,
                y,
            )?;
            ctx.restore();
        }
    }

    draw_path(
        ctx,
        "blue",
        (
            X_OFFSET + width as f64,
            metrics.actual_bounding_box_ascent(),
        ),
        &[(0.0, baseline_distance * lineno as f64)],
    );

    Ok(())
}
