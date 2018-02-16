const input_buffer = {
    events: [],
    mouse_moved: false,
    event_size: 12,

    count: function () {
        return input_buffer.events.length;
    },
    size: function () {
        return input_buffer.event_size * input_buffer.events.length;
    },
    set_hooks: function () {
        window.addEventListener("mousemove", function (event) {
            if (!input_buffer.mouse_moved) {
                var window_rect = document.getElementById("window").getBoundingClientRect()
                input_buffer.events.push({
                    type: 0,
                    x: event.pageX - window_rect.left,
                    y: event.pageY - window_rect.top
                });
                input_buffer.mouse_moved = true;
            }
        });
        window.addEventListener("mousedown", function (event) {
            console.log(event);
            var window_rect = document.getElementById("window").getBoundingClientRect()
            input_buffer.events.push({
                type: 1,
                button: event.button,
                x: event.pageX - window_rect.left,
                y: event.pageY - window_rect.top
            });
        });
        window.addEventListener("mouseup", function (event) {
            var window_rect = document.getElementById("window").getBoundingClientRect()
            input_buffer.events.push({
                type: 2,
                button: event.button,
                x: event.pageX - window_rect.left,
                y: event.pageY - window_rect.top
            });
        });
        window.addEventListener("keydown", function (event) {
            input_buffer.events.push({
                type: 3,
                key: event.keyCode
            });
        });
        window.addEventListener("keyup", function (event) {
            input_buffer.events.push({
                type: 4,
                key: event.keyCode
            });
        });
    },
    clear: function () {
        input_buffer.events = []
        input_buffer.mouse_moved = false;
    },

    write: function (start_ptr) {
        var event_ptr = start_ptr;
        input_buffer.events.forEach(function (e) {
            HEAP8[event_ptr] = e.type; // enum type

            // Mouse down and up, write mouse button
            if (e.type == 1 || e.type == 2) {
                HEAP8[event_ptr + 1] = e.button;
            }
            // Mouse event, write position
            if (e.type == 0 || e.type == 1 || e.type == 2) {
                var pos_ptr = (event_ptr / 4) + 1; // mouse position
                HEAP32[pos_ptr] = e.x;
                HEAP32[pos_ptr + 1] = e.y;
            }
            // Key event, write key code
            if (e.type == 3 || e.type == 4) {
                var key_ptr = (event_ptr / 4) + 1;
                HEAP32[key_ptr] = e.key;
            }
            event_ptr += input_buffer.event_size;
        });
    }
}