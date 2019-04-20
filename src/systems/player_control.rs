use embla::math::Vec2;

use specs::{Join, ReadStorage, System, WriteStorage};

use components::{Player, Transform, Velocity};
use game_server::TIMESTEP;

pub struct PlayerControlSystem {}

impl PlayerControlSystem {
    pub fn new() -> PlayerControlSystem {
        PlayerControlSystem {}
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Player>,
    );
    fn run(&mut self, (mut transform, mut vel, player): Self::SystemData) {
        for (transform, velocity, player) in (&mut transform, &mut vel, &player).join() {
            if player.left {
                transform.rotation += 5.0 * TIMESTEP as f32;
            }
            if player.right {
                transform.rotation -= 5.0 * TIMESTEP as f32;
            }
            if player.up {
                velocity.0 = Vec2::with_angle(transform.rotation) * 300.0;
            } else {
                velocity.0 = Vec2::zero();
            }
        }
    }
}
