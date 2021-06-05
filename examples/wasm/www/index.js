import { draw_wrapped_text, WasmOptions } from "textwrap-wasm-demo";

fetch("build-info.json").then(response => response.json()).then(buildInfo => {
    if (buildInfo.date && buildInfo.commit) {
        document.getElementById("build-date").innerText = buildInfo.date;

        let link = document.createElement("a");
        link.href = `https://github.com/mgeisler/textwrap/commit/${buildInfo.commit}`;
        link.innerText = buildInfo.commit.slice(0, 7);
        document.getElementById("build-commit").replaceWith(link);
    }
})

function redraw(event) {
    let fontSize = document.getElementById("font-size").valueAsNumber;
    let fontFamily = document.getElementById("font-family").value;
    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext("2d");

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.font = `${fontSize}px ${fontFamily}`;

    let text = document.getElementById("text").value;
    let lineWidth = document.getElementById("line-width").valueAsNumber;
    let breakWords = document.getElementById("break-words").checked;
    let wordSeparator = document.getElementById("word-separator").value;
    let wordSplitter = document.getElementById("word-splitter").value;
    // TODO: The optimal-fit algorithm does not work well for
    // proportional fonts, so we always use FirstFit. See
    // https://github.com/mgeisler/textwrap/issues/326.
    let wrapAlgorithm = "FirstFit"; // document.getElementById("wrap-algorithm").value;
    let options = new WasmOptions(lineWidth, breakWords, wordSeparator, wordSplitter, wrapAlgorithm);
    draw_wrapped_text(ctx, options, text);
}

document.getElementById("line-width").addEventListener("input", (event) => {
    let lineWidthText = document.getElementById("line-width-text");
    lineWidthText.value = event.target.valueAsNumber;
});

document.getElementById("line-width-text").addEventListener("input", (event) => {
    let lineWidth = document.getElementById("line-width");
    lineWidth.value = event.target.valueAsNumber;
});


document.querySelectorAll("textarea, select, input").forEach((elem) => {
    elem.addEventListener("input", redraw);
});

window.addEventListener("resize", (event) => {
    const X_OFFSET = 8;  // To accommodate the size of the slider knob.

    let footer = document.getElementById("footer");
    let canvas = document.getElementById("canvas");
    let width = canvas.parentNode.clientWidth;

    canvas.width = width;
    canvas.height = footer.offsetTop - canvas.offsetTop;

    let lineWidth = document.getElementById("line-width");
    let lineWidthText = document.getElementById("line-width-text");
    lineWidth.max = width - 2 * X_OFFSET;
    lineWidthText.max = width - 2 * X_OFFSET;
    lineWidth.style.width = `${width}px`;

    redraw();
});

let lineWidth = document.getElementById("line-width");
let lineWidthText = document.getElementById("line-width-text");
lineWidthText.value = lineWidth.valueAsNumber;
window.dispatchEvent(new Event('resize'));
