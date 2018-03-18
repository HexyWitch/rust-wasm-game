use failure::Error;
use bincode::{deserialize, serialize};

use platform::{Application, PlatformApi};
use platform::input::Input;
use platform::websocket::WebSocket;

use renderer::GameRenderer;
use game_client::GameClient;
use net::Packet;

pub struct ClientApplication<A: PlatformApi> {
    renderer: GameRenderer<A::Renderer>,
    socket: A::Socket,
    client: GameClient,
}

impl<A> Application for ClientApplication<A>
where
    A: PlatformApi,
{
    fn new() -> Result<Self, Error> {
        Ok(ClientApplication {
            renderer: GameRenderer::<A::Renderer>::new()?,
            socket: A::Socket::connect("ws://127.0.0.1:2794")?,
            client: GameClient::new()?,
        })
    }

    fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
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
