use std::rc::Rc;
use std::f32;

use failure::Error;

use core::assets::image_from_png;
use platform::input::Input;

use texture_image::TextureImage;
use render_interface::RenderInterface;
use ship::Ship;
use net::Packet;

pub struct Assets {
    pub ship: TextureImage,
}

enum GameState {
    Connecting,
    Initializing,
    Running {
        player: Ship   
    },
}

pub struct GameClient {
    assets: Assets,
    state: GameState,
}

impl GameClient {
    pub fn new() -> Result<GameClient, Error> {
        let assets = Assets {
            ship: TextureImage::new(Rc::new(image_from_png(include_bytes!(
                "../assets/ship.png"
            ))?)),
        };

        Ok(GameClient {
            assets: assets,
            state: GameState::Connecting
        })
    }

    pub fn update(&mut self, dt: f32, input: &Input) -> Result<(), Error> {
        match self.state {
            GameState::Running{ref mut player} => {
                player.update(dt, input)?;
            },
            _ => {}
        }
        Ok(())
    }

    pub fn render(&self, renderer: &mut RenderInterface) -> Result<(), Error> {
        match self.state {
            GameState::Running{ref player} => {
                player.render(&self.assets, renderer)?;
            },
            _ => {}
        }
        Ok(())
    }

    pub fn handle_incoming_packets(&mut self, packets: &[Packet]) -> Result<(), Error> {
        for p in packets {
            match self.state {
                GameState::Connecting => match *p {
                    Packet::ClientConnected => {
                        self.state = GameState::Initializing;
                    }
                    _ => {
                        return Err(format_err!(
                            "received unexpected packet during Connecting state"
                        ));
                    }
                },
                GameState::Initializing => match *p {
                    Packet::ClientInit { player_position } => {
                        let mut player = Ship::new();
                        player.set_position(player_position);
                        self.state = GameState::Running{player};
                    }
                    _ => {
                        return Err(format_err!(
                            "received unexpected packet during Initializing state"
                        ));
                    }
                },
                GameState::Running{..} => {}
            }
        }
        Ok(())
    }

    pub fn take_outgoing_packets(&mut self) -> Result<Vec<Packet>, Error> {
        Ok(Vec::new())
    }
}
