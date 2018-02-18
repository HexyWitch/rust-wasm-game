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

var MouseMoveFormat = new StructFormat({
    type_id: {
        type: "u8",
        offset: 0
    },
    x: {
        type: "i32",
        offset: 4
    },
    y: {
        type: "i32",
        offset: 8
    }
});

var MouseButtonFormat = new StructFormat({
    type_id: {
        type: "u8",
        offset: 0
    },
    button: {
        type: "i8",
        offset: 1
    },
    x: {
        type: "i32",
        offset: 4
    },
    y: {
        type: "i32",
        offset: 8
    }
});

var KeyFormat = new StructFormat({
    type_id: {
        type: "u8",
        offset: 0,
    },
    key: {
        type: "i32",
        offset: 4,
    }
});

const input_buffer = {
    events: [],
    mouse_moved: false,
    event_size: 12,

    count: function () {
        return input_buffer.events.length;
    },
    size: function () {
        var sum = 0;
        input_buffer.events.forEach(function (v) {
            sum += v.size();
        });
        return sum;
    },
    set_hooks: function () {
        window.addEventListener("mousemove", function (event) {
            if (!input_buffer.mouse_moved) {
                var window_rect = document.getElementById("window").getBoundingClientRect()
                input_buffer.events.push(MouseMoveFormat.value({
                    type_id: 0,
                    x: event.pageX - window_rect.left,
                    y: event.pageY - window_rect.top
                }));
                input_buffer.mouse_moved = true;
            }
        });
        window.addEventListener("mousedown", function (event) {
            var window_rect = document.getElementById("window").getBoundingClientRect()
            input_buffer.events.push(MouseButtonFormat.value({
                type_id: 1,
                button: event.button,
                x: event.pageX - window_rect.left,
                y: event.pageY - window_rect.top
            }));
        });
        window.addEventListener("mouseup", function (event) {
            var window_rect = document.getElementById("window").getBoundingClientRect()
            input_buffer.events.push(MouseButtonFormat.value({
                type_id: 2,
                button: event.button,
                x: event.pageX - window_rect.left,
                y: event.pageY - window_rect.top
            }));
        });
        window.addEventListener("keydown", function (event) {
            input_buffer.events.push(KeyFormat.value({
                type_id: 3,
                key: event.keyCode
            }));
        });
        window.addEventListener("keyup", function (event) {
            input_buffer.events.push(KeyFormat.value({
                base_format: KeyFormat,
                type_id: 4,
                key: event.keyCode
            }));
        });
    },
    clear: function () {
        input_buffer.events = []
        input_buffer.mouse_moved = false;
    },

    write: function (start_ptr) {
        var event_ptr = start_ptr;
        input_buffer.events.forEach(function (e) {
            e.write(event_ptr);
            event_ptr += e.size();
        });
    }
}