extern crate rand;
extern crate piston_window;
extern crate image as im;

use im::GenericImage;
use piston_window::*;

fn main() {
    let texture_count = 1024;
    let frames = 200;
    let size = 32.0;

    let window: PistonWindow = WindowSettings::new("piston", [1024; 2]).build().unwrap();

    let textures = {
        let mut factory = window.factory.borrow_mut();
        let factory = &mut *factory;
        (0..texture_count).map(|_| {
            let mut img = im::ImageBuffer::new(2, 2);
            for x in 0..2 {
                for y in 0..2 {
                    img.put_pixel(x, y,
                        im::Rgba([rand::random(), rand::random(), rand::random(), 255]));
                }
            }
            Texture::from_image(
                factory,
                &img,
                &TextureSettings::new()
            ).unwrap()
        }).collect::<Vec<Texture<_>>>()
    };

    let mut positions = (0..texture_count)
        .map(|_| (rand::random(), rand::random()))
        .collect::<Vec<(f64, f64)>>();

    let mut counter = 0;
    for e in window.bench_mode(true) {
        if let Some(_) = e.render_args() {
            counter += 1;
            if counter > frames { break; }
        }
        e.draw_2d(|c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            for p in &mut positions {
                let (x, y) = *p;
                *p = (x + (rand::random::<f64>() - 0.5) * 0.01,
                      y + (rand::random::<f64>() - 0.5) * 0.01);
            }
            for i in 0..texture_count {
                let p = positions[i];
                image(&textures[i], c.transform
                    .trans(p.0 * 1024.0, p.1 * 1024.0).zoom(size), g);
            }
        });
    }
}
