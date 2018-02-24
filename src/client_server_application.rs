use failure::Error;

use platform::{Application, PlatformApi};
use platform::input::Input;

use renderer::GameRenderer;
use game_client::GameClient;
use game_server::GameServer;
use net::{Packet, ClientId};

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
        let mut server = GameServer::new()?;
        let mut client = GameClient::new()?;
        let client_id = 0;

        // Local server, client is already connected
        let mut packets = Vec::new();
        packets.push(Packet::ClientConnected);
        client.handle_incoming_packets(&packets)?;

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
        let server_outgoing = self.server.take_outgoing_packets(&self.client_id)?;

        self.client.handle_incoming_packets(&server_outgoing)?;

        self.client.update(dt, input)?;

        let client_outgoing = self.client.take_outgoing_packets()?;

        self.server.handle_incoming_packets(&client_outgoing)?;

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
