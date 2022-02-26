import { Interpreter } from './snippets/dioxus-web-d9f09a47683c47a5/src/interpreter.js';

let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_18(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcd68a8b02578a45e(arg0, arg1, addHeapObject(arg2));
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_21(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h7204ec9f00c97312(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hea38e3f52c0e8f95(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_27(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h71abd0148beab53a(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

const u32CvtShim = new Uint32Array(2);

const uint64CvtShim = new BigUint64Array(u32CvtShim.buffer);

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }

function isLikeNone(x) {
    return x === undefined || x === null;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('index-850672ebdf297569_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_mark_abc7631bdced64f0 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
        performance.mark(v0);
    };
    imports.wbg.__wbg_measure_c528ff64085b7146 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    var v1 = getCachedStringFromWasm0(arg2, arg3);
if (arg2 !== 0) { wasm.__wbindgen_free(arg2, arg3); }
performance.measure(v0, v1);
}, arguments) };
imports.wbg.__wbg_log_02e20a3c32305fb7 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
console.log(v0);
};
imports.wbg.__wbg_log_5c7513aa8c164502 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
var v1 = getCachedStringFromWasm0(arg2, arg3);
var v2 = getCachedStringFromWasm0(arg4, arg5);
var v3 = getCachedStringFromWasm0(arg6, arg7);
console.log(v0, v1, v2, v3);
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_new_48900979f8f40f34 = function(arg0) {
    var ret = new Interpreter(takeObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_setnode_8377ee4c1ddf6dfd = function(arg0, arg1, arg2) {
    getObject(arg0).set_node(arg1 >>> 0, takeObject(arg2));
};
imports.wbg.__wbg_PushRoot_522f0ca4895fd456 = function(arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).PushRoot(n0);
};
imports.wbg.__wbg_AppendChildren_635b7d595ea71402 = function(arg0, arg1) {
    getObject(arg0).AppendChildren(arg1 >>> 0);
};
imports.wbg.__wbg_ReplaceWith_c3276a33dadd8922 = function(arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).ReplaceWith(n0, arg3 >>> 0);
};
imports.wbg.__wbg_InsertAfter_da4a45abf382e33d = function(arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).InsertAfter(n0, arg3 >>> 0);
};
imports.wbg.__wbg_InsertBefore_a34036be3e2bc4cc = function(arg0, arg1, arg2, arg3) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).InsertBefore(n0, arg3 >>> 0);
};
imports.wbg.__wbg_Remove_9c7e3aa34ef0a395 = function(arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).Remove(n0);
};
imports.wbg.__wbg_CreateTextNode_bdb1fb6b6ebe2710 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    getObject(arg0).CreateTextNode(v0, n1);
};
imports.wbg.__wbg_CreateElement_1eadb75f94d0cd88 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    getObject(arg0).CreateElement(v0, n1);
};
imports.wbg.__wbg_CreateElementNs_0a8b5df300b16678 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).CreateElementNs(v0, n1, v2);
};
imports.wbg.__wbg_CreatePlaceholder_6eba6b6fd8cf0f62 = function(arg0, arg1, arg2) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    getObject(arg0).CreatePlaceholder(n0);
};
imports.wbg.__wbg_NewEventListener_3968909e0cd1e625 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    u32CvtShim[0] = arg3;
    u32CvtShim[1] = arg4;
    const n1 = uint64CvtShim[0];
    getObject(arg0).NewEventListener(v0, n1, getObject(arg5));
};
imports.wbg.__wbg_RemoveEventListener_a9d4a9aa8f93852a = function(arg0, arg1, arg2, arg3, arg4) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).RemoveEventListener(n0, v1);
};
imports.wbg.__wbg_SetText_9237827d957fd2f5 = function(arg0, arg1, arg2, arg3, arg4) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).SetText(n0, v1);
};
imports.wbg.__wbg_SetAttribute_35a0203984ff28a7 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    var v3 = getCachedStringFromWasm0(arg7, arg8);
    getObject(arg0).SetAttribute(n0, v1, v2, v3);
};
imports.wbg.__wbg_RemoveAttribute_d591f0c53f20381f = function(arg0, arg1, arg2, arg3, arg4) {
    u32CvtShim[0] = arg1;
    u32CvtShim[1] = arg2;
    const n0 = uint64CvtShim[0];
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).RemoveAttribute(n0, v1);
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};
imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
console.error(v0);
};
imports.wbg.__wbg_new_693216e109162396 = function() {
    var ret = new Error();
    return addHeapObject(ret);
};
imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setTimeout_290c28f3580809b6 = function() { return handleError(function (arg0, arg1) {
    var ret = setTimeout(getObject(arg0), arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_clearTimeout_d8b36ad8fa330187 = typeof clearTimeout == 'function' ? clearTimeout : notDefined('clearTimeout');
imports.wbg.__wbg_instanceof_Window_434ce1849eb4e0fc = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};
imports.wbg.__wbg_document_5edd43643d1060d9 = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_requestAnimationFrame_0c71cd3c6779a371 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_requestIdleCallback_83096961d4f988d3 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestIdleCallback(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_getElementById_b30e88aff96f66a1 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlSelectElement_e3dbe2a40aa03cfe = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLSelectElement;
    return ret;
};
imports.wbg.__wbg_value_d4cea9e999ffb147 = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_KeyboardEvent_39e350edcd4936d4 = function(arg0) {
    var ret = getObject(arg0) instanceof KeyboardEvent;
    return ret;
};
imports.wbg.__wbg_charCode_e15a2aba71bbaa8c = function(arg0) {
    var ret = getObject(arg0).charCode;
    return ret;
};
imports.wbg.__wbg_keyCode_8a05b1390fced3c8 = function(arg0) {
    var ret = getObject(arg0).keyCode;
    return ret;
};
imports.wbg.__wbg_altKey_773e7f8151c49bb1 = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_8c7ff99be598479e = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_894b631364d8db13 = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_metaKey_99a7d3732e1b7856 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_location_802154aca73cf67f = function(arg0) {
    var ret = getObject(arg0).location;
    return ret;
};
imports.wbg.__wbg_repeat_248c3b6d2b3e0a33 = function(arg0) {
    var ret = getObject(arg0).repeat;
    return ret;
};
imports.wbg.__wbg_key_7f10b1291a923361 = function(arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_Comment_c8fb3a7fd7b3392c = function(arg0) {
    var ret = getObject(arg0) instanceof Comment;
    return ret;
};
imports.wbg.__wbg_instanceof_Node_235c78aca8f70c08 = function(arg0) {
    var ret = getObject(arg0) instanceof Node;
    return ret;
};
imports.wbg.__wbg_parentElement_96e1e07348340043 = function(arg0) {
    var ret = getObject(arg0).parentElement;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_childNodes_2cc9324ea7605e96 = function(arg0) {
    var ret = getObject(arg0).childNodes;
    return addHeapObject(ret);
};
imports.wbg.__wbg_nodeValue_486a51e2b63cc20b = function(arg0, arg1) {
    var ret = getObject(arg1).nodeValue;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_textContent_dbe4d2d612abcd96 = function(arg0, arg1) {
    var ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_settextContent_07dfb193b5deabbc = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_instanceof_Text_2b91a768db957a84 = function(arg0) {
    var ret = getObject(arg0) instanceof Text;
    return ret;
};
imports.wbg.__wbg_pageX_41351f8d39f32f3b = function(arg0) {
    var ret = getObject(arg0).pageX;
    return ret;
};
imports.wbg.__wbg_pageY_c0e51cfead96c94d = function(arg0) {
    var ret = getObject(arg0).pageY;
    return ret;
};
imports.wbg.__wbg_which_a82f3f5ff4c2de7b = function(arg0) {
    var ret = getObject(arg0).which;
    return ret;
};
imports.wbg.__wbg_instanceof_IdleDeadline_4fad2202696b050a = function(arg0) {
    var ret = getObject(arg0) instanceof IdleDeadline;
    return ret;
};
imports.wbg.__wbg_timeRemaining_a95c5045575c3f7f = function(arg0) {
    var ret = getObject(arg0).timeRemaining();
    return ret;
};
imports.wbg.__wbg_instanceof_CompositionEvent_98c7ac3087e63202 = function(arg0) {
    var ret = getObject(arg0) instanceof CompositionEvent;
    return ret;
};
imports.wbg.__wbg_data_9562112603a9aa89 = function(arg0, arg1) {
    var ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_PointerEvent_7a157d9f2eb5fc33 = function(arg0) {
    var ret = getObject(arg0) instanceof PointerEvent;
    return ret;
};
imports.wbg.__wbg_pointerId_60c6c29cc58f32a9 = function(arg0) {
    var ret = getObject(arg0).pointerId;
    return ret;
};
imports.wbg.__wbg_width_2cc86e9ec447ca00 = function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
};
imports.wbg.__wbg_height_1fe88e19b9767a03 = function(arg0) {
    var ret = getObject(arg0).height;
    return ret;
};
imports.wbg.__wbg_pressure_37cef495bc1ea47f = function(arg0) {
    var ret = getObject(arg0).pressure;
    return ret;
};
imports.wbg.__wbg_tangentialPressure_33f4ea2cd1cda77f = function(arg0) {
    var ret = getObject(arg0).tangentialPressure;
    return ret;
};
imports.wbg.__wbg_tiltX_3f463ec69e4f7b1f = function(arg0) {
    var ret = getObject(arg0).tiltX;
    return ret;
};
imports.wbg.__wbg_tiltY_c048ee33d0c56e13 = function(arg0) {
    var ret = getObject(arg0).tiltY;
    return ret;
};
imports.wbg.__wbg_twist_fdd6e53521038b9d = function(arg0) {
    var ret = getObject(arg0).twist;
    return ret;
};
imports.wbg.__wbg_pointerType_d791634374f3f4d4 = function(arg0, arg1) {
    var ret = getObject(arg1).pointerType;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_isPrimary_a34bca42dea9ee74 = function(arg0) {
    var ret = getObject(arg0).isPrimary;
    return ret;
};
imports.wbg.__wbg_instanceof_TransitionEvent_3a2d8321fec2d7c8 = function(arg0) {
    var ret = getObject(arg0) instanceof TransitionEvent;
    return ret;
};
imports.wbg.__wbg_propertyName_d092e58e9cc512c9 = function(arg0, arg1) {
    var ret = getObject(arg1).propertyName;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_c9532b60001b7ab9 = function(arg0) {
    var ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_fbd578e5f7b13974 = function(arg0, arg1) {
    var ret = getObject(arg1).pseudoElement;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_WheelEvent_4b1a05cbd5815664 = function(arg0) {
    var ret = getObject(arg0) instanceof WheelEvent;
    return ret;
};
imports.wbg.__wbg_deltaX_df228181f4d1a561 = function(arg0) {
    var ret = getObject(arg0).deltaX;
    return ret;
};
imports.wbg.__wbg_deltaY_afa6edde136e1500 = function(arg0) {
    var ret = getObject(arg0).deltaY;
    return ret;
};
imports.wbg.__wbg_deltaZ_30dcbd02ca271c39 = function(arg0) {
    var ret = getObject(arg0).deltaZ;
    return ret;
};
imports.wbg.__wbg_deltaMode_ed9d7974a0c11323 = function(arg0) {
    var ret = getObject(arg0).deltaMode;
    return ret;
};
imports.wbg.__wbg_instanceof_MouseEvent_e20234cd6f6ebeb5 = function(arg0) {
    var ret = getObject(arg0) instanceof MouseEvent;
    return ret;
};
imports.wbg.__wbg_screenX_04a681cd21c0ca9b = function(arg0) {
    var ret = getObject(arg0).screenX;
    return ret;
};
imports.wbg.__wbg_screenY_a78aa0201d03a4e8 = function(arg0) {
    var ret = getObject(arg0).screenY;
    return ret;
};
imports.wbg.__wbg_clientX_849ccdf456d662ac = function(arg0) {
    var ret = getObject(arg0).clientX;
    return ret;
};
imports.wbg.__wbg_clientY_1aaff30fe0cd0876 = function(arg0) {
    var ret = getObject(arg0).clientY;
    return ret;
};
imports.wbg.__wbg_ctrlKey_4e536bedb069129f = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_cc93bd2f12bfcc9c = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_d24e3f7e465410ec = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_0b396e35a4941247 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_button_a18f33eb55774d89 = function(arg0) {
    var ret = getObject(arg0).button;
    return ret;
};
imports.wbg.__wbg_buttons_974d3032e355335f = function(arg0) {
    var ret = getObject(arg0).buttons;
    return ret;
};
imports.wbg.__wbg_instanceof_AnimationEvent_8cf515b4164539c2 = function(arg0) {
    var ret = getObject(arg0) instanceof AnimationEvent;
    return ret;
};
imports.wbg.__wbg_animationName_adbb951bd855fd25 = function(arg0, arg1) {
    var ret = getObject(arg1).animationName;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_2bfabf1ffd1449f6 = function(arg0) {
    var ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_f440b49d6733a4f1 = function(arg0, arg1) {
    var ret = getObject(arg1).pseudoElement;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_HtmlTextAreaElement_16f2c6ca1b8ccd65 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLTextAreaElement;
    return ret;
};
imports.wbg.__wbg_value_d3a30bc2c7caf357 = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_get_a307c30b5f5df814 = function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_TouchEvent_6f8e0e90b91dcab3 = function(arg0) {
    var ret = getObject(arg0) instanceof TouchEvent;
    return ret;
};
imports.wbg.__wbg_altKey_a69529f01610f7e8 = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_d76f860a7314b6c9 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_6fb659a0acdb08d2 = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_644bf95e5ac01f02 = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_instanceof_Element_c9423704dd5d9b1d = function(arg0) {
    var ret = getObject(arg0) instanceof Element;
    return ret;
};
imports.wbg.__wbg_tagName_46df689351536098 = function(arg0, arg1) {
    var ret = getObject(arg1).tagName;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_getAttribute_25ddac571fec98e1 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_setAttribute_1776fcc9b98d464e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlElement_d3e8f1c1d6788b24 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLElement;
    return ret;
};
imports.wbg.__wbg_data_88cb4195a3450465 = function(arg0, arg1) {
    var ret = getObject(arg1).data;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_HtmlInputElement_8969541a2a0bded0 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};
imports.wbg.__wbg_checked_5b6eab0ab31f5d34 = function(arg0) {
    var ret = getObject(arg0).checked;
    return ret;
};
imports.wbg.__wbg_type_f7dc0f33611f497c = function(arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_value_fc1c354d1a0e9714 = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_Event_39e54e1fe6593f4c = function(arg0) {
    var ret = getObject(arg0) instanceof Event;
    return ret;
};
imports.wbg.__wbg_type_e32f387f5584c765 = function(arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_target_e560052e31e4567c = function(arg0) {
    var ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_preventDefault_fa00541ff125b78c = function(arg0) {
    getObject(arg0).preventDefault();
};
imports.wbg.__wbg_newnoargs_f579424187aa1717 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_89558c3e96703ca1 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Object_d6dae7f4da812832 = function(arg0) {
    var ret = getObject(arg0) instanceof Object;
    return ret;
};
imports.wbg.__wbg_hasOwnProperty_5f7af26bc3dc1b0f = function(arg0, arg1) {
    var ret = getObject(arg0).hasOwnProperty(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_resolve_4f8f547f26b30b27 = function(arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_a6860c82b90816ca = function(arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_globalThis_d61b1f48a57191ae = function() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_self_e23d74ae45fb17d1 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_b4be7f48b24ac56e = function() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_e7669da72fd7f239 = function() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbindgen_is_function = function(arg0) {
    var ret = typeof(getObject(arg0)) === 'function';
    return ret;
};
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_closure_wrapper5921 = function(arg0, arg1, arg2) {
    var ret = makeClosure(arg0, arg1, 348, __wbg_adapter_18);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper5923 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 346, __wbg_adapter_21);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper6241 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 438, __wbg_adapter_24);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper6852 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 476, __wbg_adapter_27);
    return addHeapObject(ret);
};

if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
    input = fetch(input);
}



const { instance, module } = await load(await input, imports);

wasm = instance.exports;
init.__wbindgen_wasm_module = module;
wasm.__wbindgen_start();
return wasm;
}

export default init;

