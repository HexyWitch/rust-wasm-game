const sockets = [];

function websocket_create(url_ptr, url_len) {
    let url_str = getString(url_ptr, url_len);
    var sock = new WebSocket(url_str, "rust-websocket");
    sock.binaryType = "arraybuffer"
    var len = sockets.push(sock);
    return len - 1;
}

function websocket_send(socket_id, data_ptr, len) {
    let data_view = new Uint8Array(Module.instance.exports.memory.buffer, data_ptr, len);
    let sock = sockets[socket_id];
    sock.send(data_view);
}

function websocket_onopen(socket_id, fn_ptr, arg) {
    var socket = sockets[socket_id];
    var f = Module.instance.exports.__web_table.get(fn_ptr)
    socket.onopen = function () {
        f(arg);
    }
}

function websocket_close(socket_id, code, reason_ptr) {
    var socket = sockets[socket_id];
    var reason = getString(reason_ptr, reason_len);
    sockets[socket_id] = null;
    socket.close(code, reason);
}

function websocket_onmessage(socket_id, fn_ptr, arg) {
    var socket = sockets[socket_id];
    var f = Module.instance.exports.__web_table.get(fn_ptr)
    socket.onmessage = function (message) {
        let data_view = new Uint8Array(message.data);
        var ptr = pushData(data_view);
        f(ptr, data_view.length, arg);
    }
}