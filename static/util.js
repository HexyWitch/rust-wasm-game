function copyCStr(module, ptr) {
    let orig_ptr = ptr;
    const collectCString = function* () {
        while (HEAPU8[ptr] !== 0) {
            if (HEAPU8[ptr] === undefined) { throw new Error("Tried to read undef mem") }
            yield HEAPU8[ptr]
            ptr += 1
        }
    }

    const buffer_as_u8 = new Uint8Array(collectCString())
    const utf8Decoder = new TextDecoder("UTF-8");
    const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    return buffer_as_utf8
}

function newString(module, str) {
    const utf8Encoder = new TextEncoder("UTF-8");
    let string_buffer = utf8Encoder.encode(str)
    let len = string_buffer.length
    let ptr = module.alloc_str(len + 1)

    for (i = 0; i < len; i++) {
        HEAPU8[ptr + i] = string_buffer[i]
    }
    HEAPU8[ptr + len] = 0;

    return ptr;
}