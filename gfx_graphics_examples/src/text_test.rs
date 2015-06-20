extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let window: PistonWindow = WindowSettings::new(
            "gfx_graphics: text_test",
            [300, 300]
        )
        .exit_on_esc(true)
        .into();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    println!("{:?}", assets);
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.borrow().clone();
    let mut glyphs = Glyphs::new(font, factory).unwrap();

    for e in window {
        e.draw_2d(|c, g| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::colored([0.0, 1.0, 0.0, 1.0], 32).draw(
                "Hello gfx_graphics!",
                &mut glyphs,
                &c.draw_state,
                transform, g
            );
        });
    }
}
