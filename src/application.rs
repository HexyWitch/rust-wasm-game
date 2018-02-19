use platform::{Application, PlatformApi};
use platform::input::InputEvent;

use failure::Error;

use renderer::GameRenderer;
use game::Game;

pub struct GameApplication<A: PlatformApi> {
    renderer: GameRenderer<A::Renderer>,
    game: Game,
}

impl<A> Application for GameApplication<A>
where
    A: PlatformApi,
{
    fn new() -> Result<Self, Error> {
        Ok(GameApplication {
            renderer: GameRenderer::<A::Renderer>::new((640.0, 480.0))?,
            game: Game::new()?,
        })
    }

    fn update(&mut self, dt: f64, input_events: &[InputEvent]) -> Result<(), Error> {
        self.game.update(dt, input_events);

        self.game.render(&mut self.renderer)?;

        self.renderer.do_render().unwrap();

        Ok(())
    }
}
