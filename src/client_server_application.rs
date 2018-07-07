use failure::Error;

use embla::input::Input;
use embla::window::Window;

use game_client::GameClient;
use renderer::GameRenderer;

pub struct ClientServerApplication {
    renderer: GameRenderer,
    client: GameClient,
    window: Window,
}

impl ClientServerApplication {
    pub fn new(window: Window) -> Result<Self, Error> {
        let client = GameClient::new()?;

        Ok(ClientServerApplication {
            renderer: GameRenderer::new(&window.renderer())?,
            client: client,
            window,
        })
    }

    pub fn update(&mut self, dt: f64, input: &Input) -> Result<(), Error> {
        self.client.update(dt, input)?;

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render(&self.window.renderer()).unwrap();

        Ok(())
    }
}
