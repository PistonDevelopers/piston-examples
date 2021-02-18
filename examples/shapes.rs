extern crate piston_window;

use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("shapes", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    window.set_lazy(true);

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            for i in 0..5 {
                let c = c.trans(0.0, i as f64 * 100.0);
                let black = [0.0, 0.0, 0.0, 1.0];
                let red = [1.0, 0.0, 0.0, 1.0];
                let rect = math::margin_rectangle([20.0, 20.0, 60.0, 60.0], i as f64 * 5.0);
                rectangle(red, rect, c.transform, g);
                Rectangle::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
                let green = [0.0, 1.0, 0.0, 1.0];
                let h = 60.0 * (1.0 - i as f64 / 5.0);
                let rect = [120.0, 50.0 - h / 2.0, 60.0, h];
                ellipse(green, rect, c.transform, g);
                Ellipse::new_border(black, 2.0).draw(rect, &c.draw_state, c.transform, g);
                let blue = [0.0, 0.0, 1.0, 1.0];
                circle_arc(blue, 10.0, 0.0, f64::_360() - i as f64 * 1.2, [230.0, 30.0, 40.0, 40.0],
                           c.transform, g);
                let orange = [1.0, 0.5, 0.0, 1.0];
                line(orange, 5.0, [320.0 + i as f64 * 15.0, 20.0, 380.0 - i as f64 * 15.0, 80.0],
                     c.transform, g);
                let magenta = [1.0, 0.0, 0.5, 1.0];
                polygon(magenta, &[
                        [420.0, 20.0],
                        [480.0, 20.0],
                        [480.0 - i as f64 * 15.0, 80.0]
                    ], c.transform, g);
            }
        });
    }
}
