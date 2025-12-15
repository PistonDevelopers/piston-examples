use piston_window::*;
use wgpu_graphics::{Texture, TextureSettings};

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("piston: image", [300, 300])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let rust_logo = assets.join("rust.png");
    let rust_logo: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            &rust_logo,
            &TextureSettings::new()
        ).unwrap();
    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            use graphics::*;

            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
        });
    }
}
