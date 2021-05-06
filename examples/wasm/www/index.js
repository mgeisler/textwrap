import { draw_wrapped_text } from "textwrap-wasm-demo";

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
    let fontFamily = document.getElementById("font-family").value;
    let canvas = document.getElementById("canvas");
    let ctx = canvas.getContext("2d");

    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.font = `20px ${fontFamily}`;

    let text = document.getElementById("text").value;
    let lineWidth = document.getElementById("line-width");
    draw_wrapped_text(ctx, text, lineWidth.valueAsNumber);
}

document.getElementById("text").addEventListener("input", redraw);
document.getElementById("font-family").addEventListener("input", redraw);

document.getElementById("line-width").addEventListener("input", (event) => {
    let lineWidthText = document.getElementById("line-width-text");
    lineWidthText.value = event.target.valueAsNumber;
    redraw();
});

document.getElementById("line-width-text").addEventListener("input", (event) => {
    let lineWidth = document.getElementById("line-width");
    lineWidth.value = event.target.valueAsNumber;
    redraw();
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
