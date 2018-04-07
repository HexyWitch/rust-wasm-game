extern crate embla;
extern crate game;

use game::ClientApplication;

pub fn main() {
    embla::run(|| {
        let mut application = ClientApplication::new().unwrap();
        move |dt, input| {
            application.update(dt, input)?;

            Ok(())
        }
    });
}
