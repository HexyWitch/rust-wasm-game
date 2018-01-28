use platform::websocket::WebSocket;

pub fn init() -> Box<FnMut()> {
    println!("Start the application!");

    let mut socket = WebSocket::connect("ws://localhost:3012").unwrap();
    let mut ping = true;
    let mut x = 1;
    Box::new(move || {
        if socket.open() {
            if ping {
                let msg = format!("{}", x);
                println!("Ping: {}", x);
                socket.send(&msg).unwrap();
                ping = false;
                x += 1;
            }

            while let Some(msg) = socket.next() {
                println!("Pong: {}", msg);
                ping = true;
            }
        }
    })
}
