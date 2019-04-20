use specs::{Join, ReadStorage, System, WriteStorage};

use components::{Transform, Velocity};
use game_server::TIMESTEP;

pub struct MovementSystem {}

impl MovementSystem {
    pub fn new() -> MovementSystem {
        MovementSystem {}
    }
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Transform>, ReadStorage<'a, Velocity>);
    fn run(&mut self, (mut transform, vel): Self::SystemData) {
        for (transform, velocity) in (&mut transform, &vel).join() {
            transform.position += velocity.0 * TIMESTEP as f32;
        }
    }
}
