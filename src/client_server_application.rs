use failure::Error;
use bincode::{deserialize, serialize};

use platform::{Application, PlatformApi};
use platform::input::Input;

use renderer::GameRenderer;
use game_client::GameClient;
use game_server::GameServer;
use net::{ClientId, Packet};

pub struct ClientServerApplication<A: PlatformApi> {
    renderer: GameRenderer<A::Renderer>,
    server: GameServer,
    client_id: ClientId,
    client: GameClient,
}

impl<A> Application for ClientServerApplication<A>
where
    A: PlatformApi,
{
    fn new() -> Result<Self, Error> {
        let client_id = 0;
        let client = GameClient::new()?;

        let mut server = GameServer::new()?;
        server.add_player(client_id)?;

        Ok(ClientServerApplication {
            renderer: GameRenderer::<A::Renderer>::new((640.0, 480.0))?,
            server: server,
            client_id: client_id,
            client: client,
        })
    }

    fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        self.server.update(dt)?;
        let server_outgoing: Vec<u8> =
            serialize(&self.server.take_outgoing_packets(&self.client_id)?)?;

        let client_incoming: Vec<Packet> = deserialize(&server_outgoing)?;
        self.client.handle_incoming_packets(&client_incoming)?;

        self.client.update(dt, input)?;

        let client_outgoing: Vec<u8> = serialize(&self.client.take_outgoing_packets()?)?;

        let server_incoming: Vec<Packet> = deserialize(&client_outgoing)?;
        self.server
            .handle_incoming_packets(&self.client_id, &server_incoming)?;

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
