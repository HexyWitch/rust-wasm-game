export class Window {
    constructor(canvas_id, input_handler) {
        this.canvas = document.getElementById("window");

        this.canvas.addEventListener("mousemove", function (event) {
            input_handler.mouse_move(event.offsetX, event.offsetY);
        });
        this.canvas.addEventListener("mousedown", function (event) {
            input_handler.mouse_down(event.button, event.offsetX, event.offsetY);
        });
        this.canvas.addEventListener("mouseup", function (event) {
            input_handler.mouse_up(event.button, event.offsetX, event.offsetY);
        });
        window.addEventListener("keydown", function (event) {
            input_handler.key_down(event.keyCode);
        });
        window.addEventListener("keyup", function (event) {
            input_handler.key_up(event.keyCode);
        });
    }

    set_main_loop(cb) {
        var input_events = [];

        function runner() {
            cb.call(input_events);
            input_events = [];
            window.requestAnimationFrame(runner);
        }
        window.requestAnimationFrame(runner);
    }

    static log(msg) {
        console.log(msg);
    }
}