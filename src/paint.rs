extern crate piston_window;
extern crate image as im;

use im::GenericImage;
use piston_window::*;

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
    for e in window {
        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            image(&texture, c.transform, g);
        });
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false
            }
        };
        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as u32, pos[1] as u32);
                if x < width && y < height {
                    canvas.put_pixel(x, y, im::Rgba([0, 0, 0, 255]));
                    texture.update(&mut*e.factory.borrow_mut(), &canvas).unwrap();
                }
            };
        }
    }
}

