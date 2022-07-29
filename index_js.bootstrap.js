"use strict";
(self["webpackChunktextwrap_wasm_demo_app"] = self["webpackChunktextwrap_wasm_demo_app"] || []).push([["index_js"],{

/***/ "../pkg/textwrap_wasm_demo_bg.js":
/*!***************************************!*\
  !*** ../pkg/textwrap_wasm_demo_bg.js ***!
  \***************************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "WasmOptions": () => (/* binding */ WasmOptions),
/* harmony export */   "WasmPenalties": () => (/* binding */ WasmPenalties),
/* harmony export */   "__wbg_actualBoundingBoxAscent_da96fa51615769e1": () => (/* binding */ __wbg_actualBoundingBoxAscent_da96fa51615769e1),
/* harmony export */   "__wbg_actualBoundingBoxDescent_e900cdc3ea67db47": () => (/* binding */ __wbg_actualBoundingBoxDescent_e900cdc3ea67db47),
/* harmony export */   "__wbg_beginPath_e040b5521d41f537": () => (/* binding */ __wbg_beginPath_e040b5521d41f537),
/* harmony export */   "__wbg_error_09919627ac0992f5": () => (/* binding */ __wbg_error_09919627ac0992f5),
/* harmony export */   "__wbg_fillText_a9da23f2c00b2b51": () => (/* binding */ __wbg_fillText_a9da23f2c00b2b51),
/* harmony export */   "__wbg_lineTo_e0f6cb3b8836cedb": () => (/* binding */ __wbg_lineTo_e0f6cb3b8836cedb),
/* harmony export */   "__wbg_measureText_7137f00ee7bb9969": () => (/* binding */ __wbg_measureText_7137f00ee7bb9969),
/* harmony export */   "__wbg_moveTo_8d00712d6e75a749": () => (/* binding */ __wbg_moveTo_8d00712d6e75a749),
/* harmony export */   "__wbg_new_693216e109162396": () => (/* binding */ __wbg_new_693216e109162396),
/* harmony export */   "__wbg_restore_56c80343ddc70aeb": () => (/* binding */ __wbg_restore_56c80343ddc70aeb),
/* harmony export */   "__wbg_save_faa4566184f134f6": () => (/* binding */ __wbg_save_faa4566184f134f6),
/* harmony export */   "__wbg_setfont_7152cc4657609a93": () => (/* binding */ __wbg_setfont_7152cc4657609a93),
/* harmony export */   "__wbg_setstrokeStyle_32540003cbfe210b": () => (/* binding */ __wbg_setstrokeStyle_32540003cbfe210b),
/* harmony export */   "__wbg_settextAlign_b915113cbbf1e047": () => (/* binding */ __wbg_settextAlign_b915113cbbf1e047),
/* harmony export */   "__wbg_settextBaseline_38fba0bc777dfc84": () => (/* binding */ __wbg_settextBaseline_38fba0bc777dfc84),
/* harmony export */   "__wbg_stack_0ddaca5d1abfb52f": () => (/* binding */ __wbg_stack_0ddaca5d1abfb52f),
/* harmony export */   "__wbg_stroke_63664360a52ce7d1": () => (/* binding */ __wbg_stroke_63664360a52ce7d1),
/* harmony export */   "__wbg_width_1d637c56a808b6a2": () => (/* binding */ __wbg_width_1d637c56a808b6a2),
/* harmony export */   "__wbindgen_debug_string": () => (/* binding */ __wbindgen_debug_string),
/* harmony export */   "__wbindgen_object_drop_ref": () => (/* binding */ __wbindgen_object_drop_ref),
/* harmony export */   "__wbindgen_string_get": () => (/* binding */ __wbindgen_string_get),
/* harmony export */   "__wbindgen_string_new": () => (/* binding */ __wbindgen_string_new),
/* harmony export */   "__wbindgen_throw": () => (/* binding */ __wbindgen_throw),
/* harmony export */   "draw_wrapped_text": () => (/* binding */ draw_wrapped_text)
/* harmony export */ });
/* harmony import */ var _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./textwrap_wasm_demo_bg.wasm */ "../pkg/textwrap_wasm_demo_bg.wasm");
/* module decorator */ module = __webpack_require__.hmd(module);


const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(_textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
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
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(_textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.memory.buffer);
    }
    return cachegetInt32Memory0;
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
    if (builtInMatches.length > 1) {
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
    return instance.ptr;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* @param {CanvasRenderingContext2D} ctx
* @param {WasmOptions} options
* @param {string} text
*/
function draw_wrapped_text(ctx, options, text) {
    try {
        const retptr = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(-16);
        _assertClass(options, WasmOptions);
        const ptr0 = passStringToWasm0(text, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.draw_wrapped_text(retptr, addBorrowedObject(ctx), options.ptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        if (r1) {
            throw takeObject(r0);
        }
    } finally {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_add_to_stack_pointer(16);
        heap[stack_pointer++] = undefined;
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_exn_store(addHeapObject(e));
    }
}
/**
*/
class WasmOptions {

    static __wrap(ptr) {
        const obj = Object.create(WasmOptions.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_wasmoptions_free(ptr);
    }
    /**
    */
    get width() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_width(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set width(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_width(this.ptr, arg0);
    }
    /**
    */
    get break_words() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_break_words(this.ptr);
        return ret !== 0;
    }
    /**
    * @param {boolean} arg0
    */
    set break_words(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_break_words(this.ptr, arg0);
    }
    /**
    */
    get word_separator() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_word_separator(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set word_separator(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_word_separator(this.ptr, addHeapObject(arg0));
    }
    /**
    */
    get word_splitter() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_word_splitter(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set word_splitter(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_word_splitter(this.ptr, addHeapObject(arg0));
    }
    /**
    */
    get wrap_algorithm() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_wrap_algorithm(this.ptr);
        return takeObject(ret);
    }
    /**
    * @param {any} arg0
    */
    set wrap_algorithm(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_wrap_algorithm(this.ptr, addHeapObject(arg0));
    }
    /**
    */
    get penalties() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmoptions_penalties(this.ptr);
        return WasmPenalties.__wrap(ret);
    }
    /**
    * @param {WasmPenalties} arg0
    */
    set penalties(arg0) {
        _assertClass(arg0, WasmPenalties);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmoptions_penalties(this.ptr, ptr0);
    }
    /**
    * @param {number} width
    * @param {boolean} break_words
    * @param {any} word_separator
    * @param {any} word_splitter
    * @param {any} wrap_algorithm
    * @param {WasmPenalties} penalties
    */
    constructor(width, break_words, word_separator, word_splitter, wrap_algorithm, penalties) {
        _assertClass(penalties, WasmPenalties);
        var ptr0 = penalties.ptr;
        penalties.ptr = 0;
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.wasmoptions_new(width, break_words, addHeapObject(word_separator), addHeapObject(word_splitter), addHeapObject(wrap_algorithm), ptr0);
        return WasmOptions.__wrap(ret);
    }
}
/**
*/
class WasmPenalties {

    static __wrap(ptr) {
        const obj = Object.create(WasmPenalties.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_wasmpenalties_free(ptr);
    }
    /**
    */
    get nline_penalty() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmpenalties_nline_penalty(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set nline_penalty(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmpenalties_nline_penalty(this.ptr, arg0);
    }
    /**
    */
    get overflow_penalty() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmpenalties_overflow_penalty(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set overflow_penalty(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmpenalties_overflow_penalty(this.ptr, arg0);
    }
    /**
    */
    get short_last_line_fraction() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmpenalties_short_last_line_fraction(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set short_last_line_fraction(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmpenalties_short_last_line_fraction(this.ptr, arg0);
    }
    /**
    */
    get short_last_line_penalty() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmpenalties_short_last_line_penalty(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set short_last_line_penalty(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmpenalties_short_last_line_penalty(this.ptr, arg0);
    }
    /**
    */
    get hyphen_penalty() {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_get_wasmpenalties_hyphen_penalty(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set hyphen_penalty(arg0) {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasmpenalties_hyphen_penalty(this.ptr, arg0);
    }
    /**
    * @param {number} nline_penalty
    * @param {number} overflow_penalty
    * @param {number} short_last_line_fraction
    * @param {number} short_last_line_penalty
    * @param {number} hyphen_penalty
    */
    constructor(nline_penalty, overflow_penalty, short_last_line_fraction, short_last_line_penalty, hyphen_penalty) {
        const ret = _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.wasmpenalties_new(nline_penalty, overflow_penalty, short_last_line_fraction, short_last_line_penalty, hyphen_penalty);
        return WasmPenalties.__wrap(ret);
    }
}

function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

function __wbindgen_string_get(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbg_actualBoundingBoxAscent_da96fa51615769e1(arg0) {
    const ret = getObject(arg0).actualBoundingBoxAscent;
    return ret;
};

function __wbg_actualBoundingBoxDescent_e900cdc3ea67db47(arg0) {
    const ret = getObject(arg0).actualBoundingBoxDescent;
    return ret;
};

function __wbg_new_693216e109162396() {
    const ret = new Error();
    return addHeapObject(ret);
};

function __wbg_stack_0ddaca5d1abfb52f(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr0 = passStringToWasm0(ret, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbg_error_09919627ac0992f5(arg0, arg1) {
    try {
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_free(arg0, arg1);
    }
};

function __wbg_setstrokeStyle_32540003cbfe210b(arg0, arg1) {
    getObject(arg0).strokeStyle = getObject(arg1);
};

function __wbg_setfont_7152cc4657609a93(arg0, arg1, arg2) {
    getObject(arg0).font = getStringFromWasm0(arg1, arg2);
};

function __wbg_settextAlign_b915113cbbf1e047(arg0, arg1, arg2) {
    getObject(arg0).textAlign = getStringFromWasm0(arg1, arg2);
};

function __wbg_settextBaseline_38fba0bc777dfc84(arg0, arg1, arg2) {
    getObject(arg0).textBaseline = getStringFromWasm0(arg1, arg2);
};

function __wbg_beginPath_e040b5521d41f537(arg0) {
    getObject(arg0).beginPath();
};

function __wbg_stroke_63664360a52ce7d1(arg0) {
    getObject(arg0).stroke();
};

function __wbg_lineTo_e0f6cb3b8836cedb(arg0, arg1, arg2) {
    getObject(arg0).lineTo(arg1, arg2);
};

function __wbg_moveTo_8d00712d6e75a749(arg0, arg1, arg2) {
    getObject(arg0).moveTo(arg1, arg2);
};

function __wbg_restore_56c80343ddc70aeb(arg0) {
    getObject(arg0).restore();
};

function __wbg_save_faa4566184f134f6(arg0) {
    getObject(arg0).save();
};

function __wbg_fillText_a9da23f2c00b2b51() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
}, arguments) };

function __wbg_measureText_7137f00ee7bb9969() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).measureText(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

function __wbg_width_1d637c56a808b6a2(arg0) {
    const ret = getObject(arg0).width;
    return ret;
};

function __wbindgen_debug_string(arg0, arg1) {
    const ret = debugString(getObject(arg1));
    const ptr0 = passStringToWasm0(ret, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_malloc, _textwrap_wasm_demo_bg_wasm__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};



/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony import */ var textwrap_wasm_demo__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! textwrap-wasm-demo */ "../pkg/textwrap_wasm_demo_bg.js");


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