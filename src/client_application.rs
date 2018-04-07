use bincode::{deserialize, serialize};
use failure::Error;

use embla;
use embla::input::Input;

use game_client::GameClient;
use net::Packet;
use renderer::GameRenderer;

pub struct ClientApplication {
    renderer: GameRenderer<embla::Renderer>,
    socket: embla::Websocket,
    client: GameClient,
}

impl ClientApplication {
    pub fn new() -> Result<Self, Error> {
        Ok(ClientApplication {
            renderer: GameRenderer::<embla::Renderer>::new()?,
            socket: embla::Websocket::connect("ws://127.0.0.1:2794")?,
            client: GameClient::new()?,
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        for m in self.socket.incoming()? {
            let packets: Vec<Packet> = deserialize(&m)?;
            self.client.handle_incoming_packets(&packets)?;
        }

        self.client.update(dt, input)?;

        if self.socket.open() {
            let client_outgoing: Vec<u8> = serialize(&self.client.take_outgoing_packets()?)?;
            self.socket.send(client_outgoing)?;
        }

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
