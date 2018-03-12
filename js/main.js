export function console_log(str) {
    console.log(str);
}

export function set_main_loop(f) {
    function runner() {
        f();
        window.requestAnimationFrame(runner);
    }
    window.requestAnimationFrame(runner);
}