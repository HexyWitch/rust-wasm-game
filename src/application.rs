use platform::{Application, PlatformApi};
use platform::input::InputEvent;

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
    fn new() -> Self {
        GameApplication {
            renderer: GameRenderer::<A::Renderer>::new((640.0, 480.0)).unwrap(),
            game: Game::new(),
        }
    }

    fn update(&mut self, dt: f64, input_events: &[InputEvent]) {
        self.game.update(dt, input_events);

        self.game.render(&mut self.renderer);

        self.renderer.do_render().unwrap();
    }
}
