extern crate piston_window;
extern crate find_folder;

use piston_window::draw_state::BlendPreset;
use piston_window::*;

fn main() {
    println!("Press A to change blending");
    println!("Press S to change clip inside/out");

    let window: PistonWindow = WindowSettings::new(
            "piston: draw_state",
            [600, 600]
        )
        .exit_on_esc(true)
        .samples(4)
        .into();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut blend = BlendPreset::Alpha;
    let mut clip_inside = true;
    let rust_logo = Texture::from_path(&mut *window.factory.borrow_mut(),
                                       assets.join("rust.png"),
                                       Flip::None,
                                       &TextureSettings::new()).unwrap();
    for e in window {
        e.draw_2d(|c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            g.clear_stencil(0);
            Rectangle::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 100.0, 100.0], &c.draw_state, c.transform, g);

            let draw_state = c.draw_state.blend(blend);
            Rectangle::new([0.5, 1.0, 0.0, 0.3])
                .draw([50.0, 50.0, 100.0, 100.0], &draw_state, c.transform, g);

            let transform = c.transform.trans(100.0, 100.0);
            // Compute clip rectangle from upper left corner.
            let (clip_x, clip_y, clip_w, clip_h) = (100, 100, 100, 100);
            let (clip_x, clip_y, clip_w, clip_h) =
                (clip_x, c.viewport.unwrap().draw_size[1] as u16 - clip_y - clip_h, clip_w, clip_h);
            let clipped = c.draw_state.scissor(clip_x, clip_y, clip_w, clip_h);
            Image::new().draw(&rust_logo, &clipped, transform, g);

            let transform = c.transform.trans(200.0, 200.0);
            Ellipse::new([1.0, 0.0, 0.0, 1.0])
                .draw([0.0, 0.0, 50.0, 50.0], clip_draw_state(), transform, g);
            Image::new().draw(&rust_logo,
                if clip_inside { inside_draw_state() }
                else { outside_draw_state() },
                transform, g);
        });

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            blend = match blend {
                BlendPreset::Alpha => BlendPreset::Add,
                BlendPreset::Add => BlendPreset::Multiply,
                BlendPreset::Multiply => BlendPreset::Invert,
                BlendPreset::Invert => BlendPreset::Alpha,
            };
            println!("Changed blending to {:?}", blend);
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            clip_inside = !clip_inside;
            if clip_inside {
                println!("Changed to clip inside");
            } else {
                println!("Changed to clip outside");
            }
        }
    }
}
