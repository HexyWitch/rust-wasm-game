use platform::rendering_api::Renderer;
use platform::Application;

use renderer::GameRenderer;
use game::Game;

pub struct GameApplication<R: Renderer> {
    renderer: GameRenderer<R>,
    game: Game,
}

impl<R> Application for GameApplication<R>
where
    R: Renderer,
{
    fn new() -> Self {
        println!("Start the application!");
        GameApplication {
            renderer: GameRenderer::<R>::new((640.0, 480.0)).unwrap(),
            game: Game::new(),
        }
    }

    fn update(&mut self, dt: f64) {
        self.game.update(dt);

        self.game.render(&mut self.renderer);

        self.renderer.do_render().unwrap();
    }
}
