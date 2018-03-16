import { GlContext } from './webgl';

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
        function runner() {
            cb.call();
            window.requestAnimationFrame(runner);
        }
        window.requestAnimationFrame(runner);
    }

    gl_context() {
        return new GlContext(this.canvas);
    }

    static log(msg) {
        console.log(msg);
    }
}