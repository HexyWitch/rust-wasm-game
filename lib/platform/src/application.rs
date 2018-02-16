use input::InputEvent;

pub trait Application {
    fn new() -> Self;
    fn update(&mut self, dt: f64, input_events: &[InputEvent]);
}
