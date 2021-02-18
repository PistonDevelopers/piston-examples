extern crate piston_window;

use piston_window::*;

fn create_window(number: usize) -> PistonWindow {
    WindowSettings::new(format!("window {}", number + 1), [256, 256])
        .exit_on_esc(true).build().unwrap()
}

fn main() {
    let mut windows: Vec<_> = (0..3 as usize).into_iter().map(|n|
        create_window(n).position([100 + n as i32 * 300, 100])).collect();
    let colors = vec![[1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]];

    loop {
        let mut any_window_open = false;

        for (i, window) in windows.iter_mut().enumerate() {
            if let Some(e) = window.next() {
                any_window_open = true;
                window.draw_2d(&e, |_c, g, _device| {
                    clear(colors[i], g);
                });
            }
            if window.should_close() { window.hide() }
        }

        if !any_window_open { break }
    }
}
