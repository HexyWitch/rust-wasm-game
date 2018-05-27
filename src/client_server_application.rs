use failure::Error;

use embla;
use embla::input::Input;

use game_client::GameClient;
use renderer::GameRenderer;

pub struct ClientServerApplication {
    renderer: GameRenderer<embla::Renderer>,
    client: GameClient,
}

impl ClientServerApplication {
    pub fn new() -> Result<Self, Error> {
        let client = GameClient::new()?;

        Ok(ClientServerApplication {
            renderer: GameRenderer::<embla::Renderer>::new()?,
            client: client,
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        self.client.update(dt, input)?;

        self.client.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
