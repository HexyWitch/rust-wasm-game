window.websocket_connect = function (url, event_handler) {
    var socket = new WebSocket(url, ["rust-websocket"]);
    socket.binaryType = "arraybuffer"

    socket.onmessage = function (event) {
        event_handler.on_message(new Uint8Array(event.data));
    }
    socket.onopen = function (event) {
        event_handler.on_open();
    }
    socket.onerror = function (event) {
        event_handler.on_error();
    }
    socket.onclose = function (event) {
        event_handler.on_close();
        event_handler.free();
    }

    return socket
}