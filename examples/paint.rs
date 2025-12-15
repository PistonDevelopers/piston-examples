use piston_window::*;
use vecmath::*;
use image as im;
use wgpu_graphics::{Format, Texture, TextureSettings, UpdateTexture};

fn main() {
    let (width, height) = (300, 300);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture_context = window.create_texture_context();
    let mut texture: G2dTexture = Texture::from_image(
            &mut texture_context,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            let (w, h) = canvas.dimensions();
            texture.update(
                &mut texture_context,
                Format::Rgba8,
                &canvas,
                [0, 0],
                [w, h],
            ).unwrap();
            window.draw_2d(&e, |c, g, _device| {
                use graphics::*;

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                last_pos = None
            }
        };
        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = last_pos {
                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                    let distance = vec2_len(vec2_sub(p, pos)) as u32;

                    for i in 0..distance {
                        let diff_x = x - last_x;
                        let diff_y = y - last_y;
                        let delta = i as f32 / distance as f32;
                        let new_x = (last_x + (diff_x * delta)) as u32;
                        let new_y = (last_y + (diff_y * delta)) as u32;
                        if new_x < width && new_y < height {
                            canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
                        };
                    };
                };

                last_pos = Some(pos)
            };

        }
    }
}
