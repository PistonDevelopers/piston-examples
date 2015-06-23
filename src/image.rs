extern crate piston_window;
extern crate find_folder;

use piston_window::*;

fn main() {
    let opengl = OpenGL::_3_2;
    let window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .opengl(opengl)
        .into();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust.png");
    let rust_logo = Texture::from_path(
            &mut *window.factory.borrow_mut(),
            &rust_logo,
            Flip::None,
            &TextureSettings::new()
        ).unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }
}
