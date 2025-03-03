"use strict";
(self["webpackChunktextwrap_wasm_demo_app"] = self["webpackChunktextwrap_wasm_demo_app"] || []).push([["index_js"],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony import */ var textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! textwrap-wasm-demo */ "../pkg/textwrap_wasm_demo.js");


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


/***/ }),

/***/ "../pkg/textwrap_wasm_demo.js":
/*!************************************!*\
  !*** ../pkg/textwrap_wasm_demo.js ***!
  \************************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   WasmOptions: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.WasmOptions),
/* harmony export */   WasmPenalties: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.WasmPenalties),
/* harmony export */   __wbg_actualBoundingBoxAscent_a8000a712133a37a: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_actualBoundingBoxAscent_a8000a712133a37a),
/* harmony export */   __wbg_actualBoundingBoxDescent_bf4c7c53b08b9c84: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_actualBoundingBoxDescent_bf4c7c53b08b9c84),
/* harmony export */   __wbg_beginPath_18ab569e70788cc1: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_beginPath_18ab569e70788cc1),
/* harmony export */   __wbg_error_7534b8e9a36f1ab4: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_7534b8e9a36f1ab4),
/* harmony export */   __wbg_fillText_f7c6f84859022688: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_fillText_f7c6f84859022688),
/* harmony export */   __wbg_lineTo_1321b7a30d82f376: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_lineTo_1321b7a30d82f376),
/* harmony export */   __wbg_measureText_87dbc58e2de1644f: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_measureText_87dbc58e2de1644f),
/* harmony export */   __wbg_moveTo_3069b186b2004933: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_moveTo_3069b186b2004933),
/* harmony export */   __wbg_new_8a6f238a6ece86ea: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_8a6f238a6ece86ea),
/* harmony export */   __wbg_restore_9ac3ed45c09936ff: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_restore_9ac3ed45c09936ff),
/* harmony export */   __wbg_save_2f42b396c1a97535: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_save_2f42b396c1a97535),
/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),
/* harmony export */   __wbg_setfont_b3126b9131bc56b4: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setfont_b3126b9131bc56b4),
/* harmony export */   __wbg_setstrokeStyle_7f8dbdddec47d488: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_setstrokeStyle_7f8dbdddec47d488),
/* harmony export */   __wbg_settextAlign_13ad1c3f136337a6: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_settextAlign_13ad1c3f136337a6),
/* harmony export */   __wbg_settextBaseline_33dcd187fb0bc648: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_settextBaseline_33dcd187fb0bc648),
/* harmony export */   __wbg_stack_0ed75d68575b0f3c: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_0ed75d68575b0f3c),
/* harmony export */   __wbg_stroke_0feb24d5e9f9c915: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stroke_0feb24d5e9f9c915),
/* harmony export */   __wbg_width_261453b21b342663: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_width_261453b21b342663),
/* harmony export */   __wbindgen_debug_string: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_debug_string),
/* harmony export */   __wbindgen_init_externref_table: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_init_externref_table),
/* harmony export */   __wbindgen_string_new: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_string_new),
/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw),
/* harmony export */   draw_wrapped_text: () => (/* reexport safe */ _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.draw_wrapped_text)
/* harmony export */ });
/* harmony import */ var _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./textwrap_wasm_demo_bg.wasm */ "../pkg/textwrap_wasm_demo_bg.wasm");
/* harmony import */ var _textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./textwrap_wasm_demo_bg.js */ "../pkg/textwrap_wasm_demo_bg.js");



(0,_textwrap_wasm_demo_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);
_textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();


/***/ }),

/***/ "../pkg/textwrap_wasm_demo_bg.js":
/*!***************************************!*\
  !*** ../pkg/textwrap_wasm_demo_bg.js ***!
  \***************************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   WasmOptions: () => (/* binding */ WasmOptions),
/* harmony export */   WasmPenalties: () => (/* binding */ WasmPenalties),
/* harmony export */   __wbg_actualBoundingBoxAscent_a8000a712133a37a: () => (/* binding */ __wbg_actualBoundingBoxAscent_a8000a712133a37a),
/* harmony export */   __wbg_actualBoundingBoxDescent_bf4c7c53b08b9c84: () => (/* binding */ __wbg_actualBoundingBoxDescent_bf4c7c53b08b9c84),
/* harmony export */   __wbg_beginPath_18ab569e70788cc1: () => (/* binding */ __wbg_beginPath_18ab569e70788cc1),
/* harmony export */   __wbg_error_7534b8e9a36f1ab4: () => (/* binding */ __wbg_error_7534b8e9a36f1ab4),
/* harmony export */   __wbg_fillText_f7c6f84859022688: () => (/* binding */ __wbg_fillText_f7c6f84859022688),
/* harmony export */   __wbg_lineTo_1321b7a30d82f376: () => (/* binding */ __wbg_lineTo_1321b7a30d82f376),
/* harmony export */   __wbg_measureText_87dbc58e2de1644f: () => (/* binding */ __wbg_measureText_87dbc58e2de1644f),
/* harmony export */   __wbg_moveTo_3069b186b2004933: () => (/* binding */ __wbg_moveTo_3069b186b2004933),
/* harmony export */   __wbg_new_8a6f238a6ece86ea: () => (/* binding */ __wbg_new_8a6f238a6ece86ea),
/* harmony export */   __wbg_restore_9ac3ed45c09936ff: () => (/* binding */ __wbg_restore_9ac3ed45c09936ff),
/* harmony export */   __wbg_save_2f42b396c1a97535: () => (/* binding */ __wbg_save_2f42b396c1a97535),
/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),
/* harmony export */   __wbg_setfont_b3126b9131bc56b4: () => (/* binding */ __wbg_setfont_b3126b9131bc56b4),
/* harmony export */   __wbg_setstrokeStyle_7f8dbdddec47d488: () => (/* binding */ __wbg_setstrokeStyle_7f8dbdddec47d488),
/* harmony export */   __wbg_settextAlign_13ad1c3f136337a6: () => (/* binding */ __wbg_settextAlign_13ad1c3f136337a6),
/* harmony export */   __wbg_settextBaseline_33dcd187fb0bc648: () => (/* binding */ __wbg_settextBaseline_33dcd187fb0bc648),
/* harmony export */   __wbg_stack_0ed75d68575b0f3c: () => (/* binding */ __wbg_stack_0ed75d68575b0f3c),
/* harmony export */   __wbg_stroke_0feb24d5e9f9c915: () => (/* binding */ __wbg_stroke_0feb24d5e9f9c915),
/* harmony export */   __wbg_width_261453b21b342663: () => (/* binding */ __wbg_width_261453b21b342663),
/* harmony export */   __wbindgen_debug_string: () => (/* binding */ __wbindgen_debug_string),
/* harmony export */   __wbindgen_init_externref_table: () => (/* binding */ __wbindgen_init_externref_table),
/* harmony export */   __wbindgen_string_new: () => (/* binding */ __wbindgen_string_new),
/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw),
/* harmony export */   draw_wrapped_text: () => (/* binding */ draw_wrapped_text)
/* harmony export */ });
let wasm;
function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_3.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}
/**
 * @param {CanvasRenderingContext2D} ctx
 * @param {WasmOptions} options
 * @param {string} text
 */
function draw_wrapped_text(ctx, options, text) {
    _assertClass(options, WasmOptions);
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.draw_wrapped_text(ctx, options.__wbg_ptr, ptr0, len0);
    if (ret[1]) {
        throw takeFromExternrefTable0(ret[0]);
    }
}

const __wbindgen_enum_WasmWordSeparator = ["AsciiSpace", "UnicodeBreakProperties"];

const __wbindgen_enum_WasmWordSplitter = ["NoHyphenation", "HyphenSplitter"];

const __wbindgen_enum_WasmWrapAlgorithm = ["FirstFit", "OptimalFit"];

const WasmOptionsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmoptions_free(ptr >>> 0, 1));

class WasmOptions {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmOptionsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmoptions_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get width() {
        const ret = wasm.__wbg_get_wasmoptions_width(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} arg0
     */
    set width(arg0) {
        wasm.__wbg_set_wasmoptions_width(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {boolean}
     */
    get break_words() {
        const ret = wasm.__wbg_get_wasmoptions_break_words(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {boolean} arg0
     */
    set break_words(arg0) {
        wasm.__wbg_set_wasmoptions_break_words(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {WasmWordSeparator}
     */
    get word_separator() {
        const ret = wasm.__wbg_get_wasmoptions_word_separator(this.__wbg_ptr);
        return __wbindgen_enum_WasmWordSeparator[ret];
    }
    /**
     * @param {WasmWordSeparator} arg0
     */
    set word_separator(arg0) {
        wasm.__wbg_set_wasmoptions_word_separator(this.__wbg_ptr, (__wbindgen_enum_WasmWordSeparator.indexOf(arg0) + 1 || 3) - 1);
    }
    /**
     * @returns {WasmWordSplitter}
     */
    get word_splitter() {
        const ret = wasm.__wbg_get_wasmoptions_word_splitter(this.__wbg_ptr);
        return __wbindgen_enum_WasmWordSplitter[ret];
    }
    /**
     * @param {WasmWordSplitter} arg0
     */
    set word_splitter(arg0) {
        wasm.__wbg_set_wasmoptions_word_splitter(this.__wbg_ptr, (__wbindgen_enum_WasmWordSplitter.indexOf(arg0) + 1 || 3) - 1);
    }
    /**
     * @returns {WasmWrapAlgorithm}
     */
    get wrap_algorithm() {
        const ret = wasm.__wbg_get_wasmoptions_wrap_algorithm(this.__wbg_ptr);
        return __wbindgen_enum_WasmWrapAlgorithm[ret];
    }
    /**
     * @param {WasmWrapAlgorithm} arg0
     */
    set wrap_algorithm(arg0) {
        wasm.__wbg_set_wasmoptions_wrap_algorithm(this.__wbg_ptr, (__wbindgen_enum_WasmWrapAlgorithm.indexOf(arg0) + 1 || 3) - 1);
    }
    /**
     * @returns {WasmPenalties}
     */
    get penalties() {
        const ret = wasm.__wbg_get_wasmoptions_penalties(this.__wbg_ptr);
        return WasmPenalties.__wrap(ret);
    }
    /**
     * @param {WasmPenalties} arg0
     */
    set penalties(arg0) {
        _assertClass(arg0, WasmPenalties);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_wasmoptions_penalties(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {number} width
     * @param {boolean} break_words
     * @param {WasmWordSeparator} word_separator
     * @param {WasmWordSplitter} word_splitter
     * @param {WasmWrapAlgorithm} wrap_algorithm
     * @param {WasmPenalties} penalties
     */
    constructor(width, break_words, word_separator, word_splitter, wrap_algorithm, penalties) {
        _assertClass(penalties, WasmPenalties);
        var ptr0 = penalties.__destroy_into_raw();
        const ret = wasm.wasmoptions_new(width, break_words, (__wbindgen_enum_WasmWordSeparator.indexOf(word_separator) + 1 || 3) - 1, (__wbindgen_enum_WasmWordSplitter.indexOf(word_splitter) + 1 || 3) - 1, (__wbindgen_enum_WasmWrapAlgorithm.indexOf(wrap_algorithm) + 1 || 3) - 1, ptr0);
        this.__wbg_ptr = ret >>> 0;
        WasmOptionsFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

const WasmPenaltiesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmpenalties_free(ptr >>> 0, 1));

class WasmPenalties {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmPenalties.prototype);
        obj.__wbg_ptr = ptr;
        WasmPenaltiesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmPenaltiesFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmpenalties_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get nline_penalty() {
        const ret = wasm.__wbg_get_wasmpenalties_nline_penalty(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set nline_penalty(arg0) {
        wasm.__wbg_set_wasmpenalties_nline_penalty(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get overflow_penalty() {
        const ret = wasm.__wbg_get_wasmpenalties_overflow_penalty(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set overflow_penalty(arg0) {
        wasm.__wbg_set_wasmpenalties_overflow_penalty(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get short_last_line_fraction() {
        const ret = wasm.__wbg_get_wasmpenalties_short_last_line_fraction(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set short_last_line_fraction(arg0) {
        wasm.__wbg_set_wasmpenalties_short_last_line_fraction(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get short_last_line_penalty() {
        const ret = wasm.__wbg_get_wasmpenalties_short_last_line_penalty(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set short_last_line_penalty(arg0) {
        wasm.__wbg_set_wasmpenalties_short_last_line_penalty(this.__wbg_ptr, arg0);
    }
    /**
     * @returns {number}
     */
    get hyphen_penalty() {
        const ret = wasm.__wbg_get_wasmpenalties_hyphen_penalty(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set hyphen_penalty(arg0) {
        wasm.__wbg_set_wasmpenalties_hyphen_penalty(this.__wbg_ptr, arg0);
    }
    /**
     * @param {number} nline_penalty
     * @param {number} overflow_penalty
     * @param {number} short_last_line_fraction
     * @param {number} short_last_line_penalty
     * @param {number} hyphen_penalty
     */
    constructor(nline_penalty, overflow_penalty, short_last_line_fraction, short_last_line_penalty, hyphen_penalty) {
        const ret = wasm.wasmpenalties_new(nline_penalty, overflow_penalty, short_last_line_fraction, short_last_line_penalty, hyphen_penalty);
        this.__wbg_ptr = ret >>> 0;
        WasmPenaltiesFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
}

function __wbg_actualBoundingBoxAscent_a8000a712133a37a(arg0) {
    const ret = arg0.actualBoundingBoxAscent;
    return ret;
};

function __wbg_actualBoundingBoxDescent_bf4c7c53b08b9c84(arg0) {
    const ret = arg0.actualBoundingBoxDescent;
    return ret;
};

function __wbg_beginPath_18ab569e70788cc1(arg0) {
    arg0.beginPath();
};

function __wbg_error_7534b8e9a36f1ab4(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

function __wbg_fillText_f7c6f84859022688() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
}, arguments) };

function __wbg_lineTo_1321b7a30d82f376(arg0, arg1, arg2) {
    arg0.lineTo(arg1, arg2);
};

function __wbg_measureText_87dbc58e2de1644f() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.measureText(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments) };

function __wbg_moveTo_3069b186b2004933(arg0, arg1, arg2) {
    arg0.moveTo(arg1, arg2);
};

function __wbg_new_8a6f238a6ece86ea() {
    const ret = new Error();
    return ret;
};

function __wbg_restore_9ac3ed45c09936ff(arg0) {
    arg0.restore();
};

function __wbg_save_2f42b396c1a97535(arg0) {
    arg0.save();
};

function __wbg_setfont_b3126b9131bc56b4(arg0, arg1, arg2) {
    arg0.font = getStringFromWasm0(arg1, arg2);
};

function __wbg_setstrokeStyle_7f8dbdddec47d488(arg0, arg1, arg2) {
    arg0.strokeStyle = getStringFromWasm0(arg1, arg2);
};

function __wbg_settextAlign_13ad1c3f136337a6(arg0, arg1, arg2) {
    arg0.textAlign = getStringFromWasm0(arg1, arg2);
};

function __wbg_settextBaseline_33dcd187fb0bc648(arg0, arg1, arg2) {
    arg0.textBaseline = getStringFromWasm0(arg1, arg2);
};

function __wbg_stack_0ed75d68575b0f3c(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

function __wbg_stroke_0feb24d5e9f9c915(arg0) {
    arg0.stroke();
};

function __wbg_width_261453b21b342663(arg0) {
    const ret = arg0.width;
    return ret;
};

function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_3;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};



/***/ }),

/***/ "../pkg/textwrap_wasm_demo_bg.wasm":
/*!*****************************************!*\
  !*** ../pkg/textwrap_wasm_demo_bg.wasm ***!
  \*****************************************/
/***/ ((module, exports, __webpack_require__) => {

"use strict";
// Instantiate WebAssembly module
var wasmExports = __webpack_require__.w[module.id];
__webpack_require__.r(exports);
// export exports from WebAssembly module
for(var name in wasmExports) if(name) exports[name] = wasmExports[name];
// exec imports from WebAssembly module (for esm order)
/* harmony import */ var m0 = __webpack_require__(/*! ./textwrap_wasm_demo_bg.js */ "../pkg/textwrap_wasm_demo_bg.js");


// exec wasm module
wasmExports[""]()

/***/ })

}]);
//# sourceMappingURL=index_js.bootstrap.js.map