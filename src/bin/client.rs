extern crate game;
extern crate platform;

use game::ClientApplication;

pub fn main() {
    platform::run(|| {
        let mut application = ClientApplication::new().unwrap();
        move |dt, input| {
            application.update(dt, input)?;

            Ok(())
        }
    });
}
