extern crate piston_window;
extern crate jump_n_shoot;

use piston_window::*;
use jump_n_shoot::App;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Jump & Shoot", [500, 250])
        .build()
        .unwrap();

    let mut app = App::new();

    while let Some(e) = window.next() {
        e.update(|args| app.update(args.dt));
        e.press(|button| {
            if let Button::Keyboard(key) = button {
                app.key(key)
            }
        });
        window.draw_2d(&e, |c, g| app.view().render(c, g));
    }
}
