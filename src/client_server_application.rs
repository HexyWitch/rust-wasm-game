use failure::Error;

use embla::input::Input;
use embla::window::Window;

use game_client::GameClient;
use game_server::GameServer;
use renderer::GameRenderer;

pub struct ClientServerApplication {
    renderer: GameRenderer,
    tick: usize,
    server: GameServer,
    client: GameClient,
    window: Window,
}

impl ClientServerApplication {
    pub fn new(window: Window) -> Result<Self, Error> {
        let server = GameServer::new()?;
        let client = GameClient::new()?;

        Ok(ClientServerApplication {
            renderer: GameRenderer::new(&window.renderer())?,
            tick: 0,
            server,
            client,
            window,
        })
    }

    pub fn update(&mut self, dt: f64, input: &Input) -> Result<(), Error> {
        self.server.update(dt)?;

        if self.tick % 3 == 0 {
            let packets = self.server.take_outgoing();
            for packet in packets {
                self.client.handle_incoming(&packet)?;
            }
        }

        self.client.update(dt, input)?;

        if self.tick % 3 == 0 {
            let packets = self.client.take_outgoing();
            for packet in packets {
                self.server.handle_incoming(&packet)?;
            }
        }

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render(&self.window.renderer()).unwrap();

        self.tick += 1;

        Ok(())
    }
}
