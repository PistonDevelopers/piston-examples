extern crate piston_window;
extern crate image as im;
extern crate vecmath;

use im::GenericImage;
use piston_window::*;
use vecmath::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (300, 300);
    let window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut draw = false;
    let mut texture = Texture::from_image(
            &mut *window.factory.borrow_mut(),
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut last_pos = None;

    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            image(&texture, c.transform, g);
        });
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
                last_pos = e.mouse_cursor_args()
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

                match last_pos {
                    Some(p) => {
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
                                texture.update(&mut*e.factory.borrow_mut(), &canvas).unwrap();
                            };
                        };
                    },
                    None => {}
                };

                last_pos = Some(pos)
            };

        }
    }
}

