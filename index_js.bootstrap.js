"use strict";
(self["webpackChunktextwrap_wasm_demo_app"] = self["webpackChunktextwrap_wasm_demo_app"] || []).push([["index_js"],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony import */ var textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! textwrap-wasm-demo */ "./node_modules/textwrap-wasm-demo/textwrap_wasm_demo_bg.js");


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
    let wrapAlgorithm = document.getElementById("wrap-algorithm").value;
    let penalties = new textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__.WasmPenalties(document.getElementById("nline-penalty").valueAsNumber,
                                      document.getElementById("overflow-penalty").valueAsNumber,
                                      document.getElementById("short-line-fraction").valueAsNumber,
                                      document.getElementById("short-last-line-penalty").valueAsNumber,
                                      document.getElementById("hyphen-penalty").valueAsNumber);
    let options = new textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__.WasmOptions(lineWidth, breakWords, wordSeparator, wordSplitter, wrapAlgorithm, penalties);
    (0,textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__.draw_wrapped_text)(ctx, options, text, penalties);
}

document.getElementById("wrap-algorithm").addEventListener("input", (event) => {
    let disablePenaltiesParams = (event.target.value == "FirstFit");
    let rangeInputIds = ["nline-penalty",
               "overflow-penalty",
               "short-line-fraction",
               "short-last-line-penalty",
               "hyphen-penalty"];
    rangeInputIds.forEach((rangeInputId) => {
        let rangeInput = document.getElementById(rangeInputId);
        let textInput = document.getElementById(`${rangeInputId}-text`);
        rangeInput.disabled = disablePenaltiesParams;
        textInput.disabled = disablePenaltiesParams;
    });
});


document.querySelectorAll("input[type=range]").forEach((rangeInput) => {
    let textInput = document.getElementById(`${rangeInput.id}-text`);
    textInput.min = rangeInput.min;
    textInput.max = rangeInput.max;
    textInput.value = rangeInput.value;

    rangeInput.addEventListener("input", (event) => {
        textInput.value = rangeInput.valueAsNumber;
    });
    textInput.addEventListener("input", (event) => {
        rangeInput.value = textInput.valueAsNumber;
    });
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


/***/ })

}]);
//# sourceMappingURL=index_js.bootstrap.js.map