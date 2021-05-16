/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	var __webpack_modules__ = ({

/***/ "./bootstrap.js":
/*!**********************!*\
  !*** ./bootstrap.js ***!
  \**********************/
/***/ ((__unused_webpack_module, __unused_webpack_exports, __webpack_require__) => {

eval("// A dependency graph that contains any wasm must all be imported\n// asynchronously. This `bootstrap.js` file does the single async import, so\n// that no one else needs to worry about it again.\n__webpack_require__.e(/*! import() */ \"index_js\").then(__webpack_require__.bind(__webpack_require__, /*! ./index.js */ \"./index.js\"))\n  .catch(e => console.error(\"Error importing `index.js`:\", e));\n\n\n//# sourceURL=webpack://textwrap-wasm-demo-app/./bootstrap.js?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			loaded: false,
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Flag the module as loaded
/******/ 		module.loaded = true;
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = __webpack_modules__;
/******/ 	
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = __webpack_module_cache__;
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/ensure chunk */
/******/ 	(() => {
/******/ 		__webpack_require__.f = {};
/******/ 		// This file contains only the entry chunk.
/******/ 		// The chunk loading function for additional chunks
/******/ 		__webpack_require__.e = (chunkId) => {
/******/ 			return Promise.all(Object.keys(__webpack_require__.f).reduce((promises, key) => {
/******/ 				__webpack_require__.f[key](chunkId, promises);
/******/ 				return promises;
/******/ 			}, []));
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/get javascript chunk filename */
/******/ 	(() => {
/******/ 		// This function allow to reference async chunks
/******/ 		__webpack_require__.u = (chunkId) => {
/******/ 			// return url for filenames based on template
/******/ 			return "" + chunkId + ".bootstrap.js";
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/harmony module decorator */
/******/ 	(() => {
/******/ 		__webpack_require__.hmd = (module) => {
/******/ 			module = Object.create(module);
/******/ 			if (!module.children) module.children = [];
/******/ 			Object.defineProperty(module, 'exports', {
/******/ 				enumerable: true,
/******/ 				set: () => {
/******/ 					throw new Error('ES Modules may not assign module.exports or exports.*, Use ESM export syntax, instead: ' + module.id);
/******/ 				}
/******/ 			});
/******/ 			return module;
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/load script */
/******/ 	(() => {
/******/ 		var inProgress = {};
/******/ 		var dataWebpackPrefix = "textwrap-wasm-demo-app:";
/******/ 		// loadScript function to load a script via script tag
/******/ 		__webpack_require__.l = (url, done, key, chunkId) => {
/******/ 			if(inProgress[url]) { inProgress[url].push(done); return; }
/******/ 			var script, needAttach;
/******/ 			if(key !== undefined) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				for(var i = 0; i < scripts.length; i++) {
/******/ 					var s = scripts[i];
/******/ 					if(s.getAttribute("src") == url || s.getAttribute("data-webpack") == dataWebpackPrefix + key) { script = s; break; }
/******/ 				}
/******/ 			}
/******/ 			if(!script) {
/******/ 				needAttach = true;
/******/ 				script = document.createElement('script');
/******/ 		
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.setAttribute("data-webpack", dataWebpackPrefix + key);
/******/ 				script.src = url;
/******/ 			}
/******/ 			inProgress[url] = [done];
/******/ 			var onScriptComplete = (prev, event) => {
/******/ 				// avoid mem leaks in IE.
/******/ 				script.onerror = script.onload = null;
/******/ 				clearTimeout(timeout);
/******/ 				var doneFns = inProgress[url];
/******/ 				delete inProgress[url];
/******/ 				script.parentNode && script.parentNode.removeChild(script);
/******/ 				doneFns && doneFns.forEach((fn) => (fn(event)));
/******/ 				if(prev) return prev(event);
/******/ 			}
/******/ 			;
/******/ 			var timeout = setTimeout(onScriptComplete.bind(null, undefined, { type: 'timeout', target: script }), 120000);
/******/ 			script.onerror = onScriptComplete.bind(null, script.onerror);
/******/ 			script.onload = onScriptComplete.bind(null, script.onload);
/******/ 			needAttach && document.head.appendChild(script);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript)
/******/ 				scriptUrl = document.currentScript.src
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) scriptUrl = scripts[scripts.length - 1].src
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/jsonp chunk loading */
/******/ 	(() => {
/******/ 		// no baseURI
/******/ 		
/******/ 		// object to store loaded and loading chunks
/******/ 		// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 		// [resolve, reject, Promise] = chunk loading, 0 = chunk loaded
/******/ 		var installedChunks = {
/******/ 			"main": 0
/******/ 		};
/******/ 		
/******/ 		__webpack_require__.f.j = (chunkId, promises) => {
/******/ 				// JSONP chunk loading for javascript
/******/ 				var installedChunkData = __webpack_require__.o(installedChunks, chunkId) ? installedChunks[chunkId] : undefined;
/******/ 				if(installedChunkData !== 0) { // 0 means "already installed".
/******/ 		
/******/ 					// a Promise means "currently loading".
/******/ 					if(installedChunkData) {
/******/ 						promises.push(installedChunkData[2]);
/******/ 					} else {
/******/ 						if(true) { // all chunks have JS
/******/ 							// setup Promise in chunk cache
/******/ 							var promise = new Promise((resolve, reject) => (installedChunkData = installedChunks[chunkId] = [resolve, reject]));
/******/ 							promises.push(installedChunkData[2] = promise);
/******/ 		
/******/ 							// start chunk loading
/******/ 							var url = __webpack_require__.p + __webpack_require__.u(chunkId);
/******/ 							// create error before stack unwound to get useful stacktrace later
/******/ 							var error = new Error();
/******/ 							var loadingEnded = (event) => {
/******/ 								if(__webpack_require__.o(installedChunks, chunkId)) {
/******/ 									installedChunkData = installedChunks[chunkId];
/******/ 									if(installedChunkData !== 0) installedChunks[chunkId] = undefined;
/******/ 									if(installedChunkData) {
/******/ 										var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 										var realSrc = event && event.target && event.target.src;
/******/ 										error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 										error.name = 'ChunkLoadError';
/******/ 										error.type = errorType;
/******/ 										error.request = realSrc;
/******/ 										installedChunkData[1](error);
/******/ 									}
/******/ 								}
/******/ 							};
/******/ 							__webpack_require__.l(url, loadingEnded, "chunk-" + chunkId, chunkId);
/******/ 						} else installedChunks[chunkId] = 0;
/******/ 					}
/******/ 				}
/******/ 		};
/******/ 		
/******/ 		// no prefetching
/******/ 		
/******/ 		// no preloaded
/******/ 		
/******/ 		// no HMR
/******/ 		
/******/ 		// no HMR manifest
/******/ 		
/******/ 		// no on chunks loaded
/******/ 		
/******/ 		// install a JSONP callback for chunk loading
/******/ 		var webpackJsonpCallback = (parentChunkLoadingFunction, data) => {
/******/ 			var [chunkIds, moreModules, runtime] = data;
/******/ 			// add "moreModules" to the modules object,
/******/ 			// then flag all "chunkIds" as loaded and fire callback
/******/ 			var moduleId, chunkId, i = 0;
/******/ 			for(moduleId in moreModules) {
/******/ 				if(__webpack_require__.o(moreModules, moduleId)) {
/******/ 					__webpack_require__.m[moduleId] = moreModules[moduleId];
/******/ 				}
/******/ 			}
/******/ 			if(runtime) var result = runtime(__webpack_require__);
/******/ 			if(parentChunkLoadingFunction) parentChunkLoadingFunction(data);
/******/ 			for(;i < chunkIds.length; i++) {
/******/ 				chunkId = chunkIds[i];
/******/ 				if(__webpack_require__.o(installedChunks, chunkId) && installedChunks[chunkId]) {
/******/ 					installedChunks[chunkId][0]();
/******/ 				}
/******/ 				installedChunks[chunkIds[i]] = 0;
/******/ 			}
/******/ 		
/******/ 		}
/******/ 		
/******/ 		var chunkLoadingGlobal = self["webpackChunktextwrap_wasm_demo_app"] = self["webpackChunktextwrap_wasm_demo_app"] || [];
/******/ 		chunkLoadingGlobal.forEach(webpackJsonpCallback.bind(null, 0));
/******/ 		chunkLoadingGlobal.push = webpackJsonpCallback.bind(null, chunkLoadingGlobal.push.bind(chunkLoadingGlobal));
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm chunk loading */
/******/ 	(() => {
/******/ 		// object to store loaded and loading wasm modules
/******/ 		var installedWasmModules = {};
/******/ 		
/******/ 		function promiseResolve() { return Promise.resolve(); }
/******/ 		
/******/ 		var wasmImportedFuncCache0;
/******/ 		var wasmImportedFuncCache1;
/******/ 		var wasmImportedFuncCache2;
/******/ 		var wasmImportedFuncCache3;
/******/ 		var wasmImportedFuncCache4;
/******/ 		var wasmImportedFuncCache5;
/******/ 		var wasmImportedFuncCache6;
/******/ 		var wasmImportedFuncCache7;
/******/ 		var wasmImportedFuncCache8;
/******/ 		var wasmImportedFuncCache9;
/******/ 		var wasmImportedFuncCache10;
/******/ 		var wasmImportedFuncCache11;
/******/ 		var wasmImportedFuncCache12;
/******/ 		var wasmImportedFuncCache13;
/******/ 		var wasmImportedFuncCache14;
/******/ 		var wasmImportedFuncCache15;
/******/ 		var wasmImportedFuncCache16;
/******/ 		var wasmImportedFuncCache17;
/******/ 		var wasmImportedFuncCache18;
/******/ 		var wasmImportedFuncCache19;
/******/ 		var wasmImportedFuncCache20;
/******/ 		var wasmImportedFuncCache21;
/******/ 		var wasmImportedFuncCache22;
/******/ 		var wasmImportObjects = {
/******/ 			"../pkg/textwrap_wasm_demo_bg.wasm": function() {
/******/ 				return {
/******/ 					"./textwrap_wasm_demo_bg.js": {
/******/ 						"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 							if(wasmImportedFuncCache0 === undefined) wasmImportedFuncCache0 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache0["__wbindgen_object_drop_ref"](p0i32);
/******/ 						},
/******/ 						"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache1 === undefined) wasmImportedFuncCache1 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache1["__wbindgen_string_new"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_actualBoundingBoxAscent_da96fa51615769e1": function(p0i32) {
/******/ 							if(wasmImportedFuncCache2 === undefined) wasmImportedFuncCache2 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache2["__wbg_actualBoundingBoxAscent_da96fa51615769e1"](p0i32);
/******/ 						},
/******/ 						"__wbg_actualBoundingBoxDescent_e900cdc3ea67db47": function(p0i32) {
/******/ 							if(wasmImportedFuncCache3 === undefined) wasmImportedFuncCache3 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache3["__wbg_actualBoundingBoxDescent_e900cdc3ea67db47"](p0i32);
/******/ 						},
/******/ 						"__wbg_new_59cb74e423758ede": function() {
/******/ 							if(wasmImportedFuncCache4 === undefined) wasmImportedFuncCache4 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache4["__wbg_new_59cb74e423758ede"]();
/******/ 						},
/******/ 						"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache5 === undefined) wasmImportedFuncCache5 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache5["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache6 === undefined) wasmImportedFuncCache6 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache6["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setstrokeStyle_72f1ad8117744d41": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache7 === undefined) wasmImportedFuncCache7 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache7["__wbg_setstrokeStyle_72f1ad8117744d41"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbg_setfont_781d8a4777f9b05d": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache8 === undefined) wasmImportedFuncCache8 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache8["__wbg_setfont_781d8a4777f9b05d"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_settextAlign_92c91d5fffbd336b": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache9 === undefined) wasmImportedFuncCache9 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache9["__wbg_settextAlign_92c91d5fffbd336b"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_settextBaseline_62a9471d2b93428a": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache10 === undefined) wasmImportedFuncCache10 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache10["__wbg_settextBaseline_62a9471d2b93428a"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_beginPath_0dcd4a1d09e0223c": function(p0i32) {
/******/ 							if(wasmImportedFuncCache11 === undefined) wasmImportedFuncCache11 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache11["__wbg_beginPath_0dcd4a1d09e0223c"](p0i32);
/******/ 						},
/******/ 						"__wbg_stroke_bed807f727b58a90": function(p0i32) {
/******/ 							if(wasmImportedFuncCache12 === undefined) wasmImportedFuncCache12 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache12["__wbg_stroke_bed807f727b58a90"](p0i32);
/******/ 						},
/******/ 						"__wbg_lineTo_3acee3da29728cb9": function(p0i32,p1f64,p2f64) {
/******/ 							if(wasmImportedFuncCache13 === undefined) wasmImportedFuncCache13 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache13["__wbg_lineTo_3acee3da29728cb9"](p0i32,p1f64,p2f64);
/******/ 						},
/******/ 						"__wbg_moveTo_bd43ecdbb947f343": function(p0i32,p1f64,p2f64) {
/******/ 							if(wasmImportedFuncCache14 === undefined) wasmImportedFuncCache14 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache14["__wbg_moveTo_bd43ecdbb947f343"](p0i32,p1f64,p2f64);
/******/ 						},
/******/ 						"__wbg_restore_c480175de20d25ec": function(p0i32) {
/******/ 							if(wasmImportedFuncCache15 === undefined) wasmImportedFuncCache15 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache15["__wbg_restore_c480175de20d25ec"](p0i32);
/******/ 						},
/******/ 						"__wbg_save_d60be08cdef5b02f": function(p0i32) {
/******/ 							if(wasmImportedFuncCache16 === undefined) wasmImportedFuncCache16 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache16["__wbg_save_d60be08cdef5b02f"](p0i32);
/******/ 						},
/******/ 						"__wbg_fillText_1a4eaffef23bd8b7": function(p0i32,p1i32,p2i32,p3f64,p4f64) {
/******/ 							if(wasmImportedFuncCache17 === undefined) wasmImportedFuncCache17 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache17["__wbg_fillText_1a4eaffef23bd8b7"](p0i32,p1i32,p2i32,p3f64,p4f64);
/******/ 						},
/******/ 						"__wbg_measureText_8c14bb26761d3435": function(p0i32,p1i32,p2i32) {
/******/ 							if(wasmImportedFuncCache18 === undefined) wasmImportedFuncCache18 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache18["__wbg_measureText_8c14bb26761d3435"](p0i32,p1i32,p2i32);
/******/ 						},
/******/ 						"__wbg_width_9c3c9e775c405aaa": function(p0i32) {
/******/ 							if(wasmImportedFuncCache19 === undefined) wasmImportedFuncCache19 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache19["__wbg_width_9c3c9e775c405aaa"](p0i32);
/******/ 						},
/******/ 						"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache20 === undefined) wasmImportedFuncCache20 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache20["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 							if(wasmImportedFuncCache21 === undefined) wasmImportedFuncCache21 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache21["__wbindgen_throw"](p0i32,p1i32);
/******/ 						},
/******/ 						"__wbindgen_rethrow": function(p0i32) {
/******/ 							if(wasmImportedFuncCache22 === undefined) wasmImportedFuncCache22 = __webpack_require__.c["../pkg/textwrap_wasm_demo_bg.js"].exports;
/******/ 							return wasmImportedFuncCache22["__wbindgen_rethrow"](p0i32);
/******/ 						}
/******/ 					}
/******/ 				};
/******/ 			},
/******/ 		};
/******/ 		
/******/ 		var wasmModuleMap = {
/******/ 			"index_js": [
/******/ 				"../pkg/textwrap_wasm_demo_bg.wasm"
/******/ 			]
/******/ 		};
/******/ 		
/******/ 		// object with all WebAssembly.instance exports
/******/ 		__webpack_require__.w = {};
/******/ 		
/******/ 		// Fetch + compile chunk loading for webassembly
/******/ 		__webpack_require__.f.wasm = function(chunkId, promises) {
/******/ 		
/******/ 			var wasmModules = wasmModuleMap[chunkId] || [];
/******/ 		
/******/ 			wasmModules.forEach(function(wasmModuleId, idx) {
/******/ 				var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/ 		
/******/ 				// a Promise means "currently loading" or "already loaded".
/******/ 				if(installedWasmModuleData)
/******/ 					promises.push(installedWasmModuleData);
/******/ 				else {
/******/ 					var importObject = wasmImportObjects[wasmModuleId]();
/******/ 					var req = fetch(__webpack_require__.p + "" + {"index_js":{"../pkg/textwrap_wasm_demo_bg.wasm":"1cd1f89f4a1a3d292977"}}[chunkId][wasmModuleId] + ".module.wasm");
/******/ 					var promise;
/******/ 					if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 						promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 							return WebAssembly.instantiate(items[0], items[1]);
/******/ 						});
/******/ 					} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 						promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 					} else {
/******/ 						var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 						promise = bytesPromise.then(function(bytes) {
/******/ 							return WebAssembly.instantiate(bytes, importObject);
/******/ 						});
/******/ 					}
/******/ 					promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 						return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 					}));
/******/ 				}
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// module cache are used so entry inlining is disabled
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	var __webpack_exports__ = __webpack_require__("./bootstrap.js");
/******/ 	
/******/ })()
;