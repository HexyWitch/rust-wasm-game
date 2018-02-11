const sockets = [];

function websocket_create(url_ptr) {
    let url_str = copyCStr(url_ptr);
    var sock = new WebSocket(url_str);
    var len = sockets.push(sock);
    return len - 1;
}

function websocket_send(socket_id, data_ptr) {
    let data_str = copyCStr(data_ptr);
    let sock = sockets[socket_id];
    sock.send(data_str);
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
    var reason = copyCStr(reason_ptr);
    sockets[socket_id] = null;
    socket.close(code, reason);
}

function websocket_onmessage(socket_id, fn_ptr, arg) {
    var socket = sockets[socket_id];
    var f = Module.instance.exports.__web_table.get(fn_ptr)
    socket.onmessage = function (message) {
        var message_ptr = newString(message.data);
        f(message_ptr, arg);
        Module.dealloc_str(message_ptr);
    }
}