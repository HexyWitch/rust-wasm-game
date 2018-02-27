// Wasm globals
const Module = {};

let HEAP8 = null;
let HEAP16 = null;
let HEAP32 = null;
let HEAPU8 = null;
let HEAPU16 = null;
let HEAPU32 = null;
let HEAPF32 = null;
let HEAPF64 = null;

// Helpers
function getString(ptr, len) {
    const buffer_as_u8 = new Uint8Array(Module.instance.exports.memory.buffer, ptr, len);
    const utf8Decoder = new TextDecoder("UTF-8");
    return utf8Decoder.decode(buffer_as_u8);
}

function pushData(data) {
    let len = data.length;
    let ptr = Module.alloc(len);
    for (i = 0; i < len; i++) {
        HEAPU8[ptr + i] = data[i];
    }
    return ptr
}

// Javascript object stack
// Used to reference javascript objects over the wasm boundary
var free_refs = [];
var js_refs = [];

function createObject(item) {
    var key = free_refs.pop();
    if (key == undefined) {
        key = js_refs.push(item) - 1;
    } else {
        js_refs[key] = item;
    }
    return key
}

function dropObject(key) {
    js_refs[key] = null;
    free_refs.push(key);
}

function getObject(key) {
    return js_refs[key];
}

function takeObject(key) {
    var handle = getObject(key);
    dropObject(key);
    return handle;
}

// Takes a struct data format descriptor in the form of:
// properties: {
//     type_id: {
//         type: "u8",
//         offset: 0
//     },
//     x: {
//         type: "i32",
//         offset: 4
//     }
// }
// Where type is specified in raw_types and offset is in bytes regardless of the type
// Used to construct raw types from javascript
class StructFormat {
    constructor(format) {
        this.format = format

        this.size = 0;
        for (var k in format) {
            var v = format[k];
            var size;
            switch (v.type) {
                case "i8": size = 1; break
                case "i16": size = 2; break
                case "i32": size = 4; break
                case "u8": size = 1; break
                case "u16": size = 2; break
                case "u32": size = 4; break
                case "f32": size = 4; break
                case "f64": size = 8; break
            }
            this.size = Math.max(this.size, v.offset + size);
        }
    }

    value(data) {
        var size = this.size;
        var writer = this;
        return {
            size: function () {
                return size
            },
            write: function (ptr) {
                writer.write(ptr, data)
            }
        }
    }

    write(ptr, data) {
        for (var k in this.format) {
            var v = this.format[k];
            let property_ptr = (ptr + v.offset);
            switch (v.type) {
                case "i8": HEAP8[property_ptr] = data[k]; break
                case "i16": HEAP16[property_ptr / 2] = data[k]; break
                case "i32": HEAP32[property_ptr / 4] = data[k]; break
                case "u8": HEAPU8[property_ptr] = data[k]; break
                case "u16": HEAPU16[property_ptr / 2] = data[k]; break
                case "u32": HEAPU32[property_ptr / 4] = data[k]; break
                case "f32": HEAPF32[property_ptr / 4] = data[k]; break
                case "f64": HEAPF64[property_ptr / 8] = data[k]; break
            }
        }
    }
}