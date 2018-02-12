pub trait Application {
    fn new() -> Self;
    fn update(&mut self, dt: f64);
}
