#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use sdl2_game_window::GameWindowSDL2 as Window;
use glfw_game_window::GameWindowGLFW as Window;
use piston::input::keyboard;
use piston::input::mouse;
use piston::input;
use piston::{
    Input,
    Render,
    Update
};

pub struct App {
    capture_cursor: bool
}

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App { 
            capture_cursor: false 
        }
    }

    fn key_press<W: piston::GameWindow>(
        &mut self,
        window: &mut W,
        key: keyboard::Key
    ) {
        if key == keyboard::C {
            println!("Turned capture cursor on");
            self.capture_cursor = !self.capture_cursor;

            window.capture_cursor(self.capture_cursor);
        }

        println!("Pressed keyboard key '{}'", key);
    }

    fn key_release(&mut self, key: keyboard::Key) {
        println!("Released keyboard key '{}'", key);
    }

    fn mouse_move(&mut self, x: f64, y: f64) {
        println!("Mouse moved '{} {}'", x, y);
    }

    fn mouse_relative_move(&mut self, dx: f64, dy: f64) {
        println!("Relative mouse moved '{} {}'", dx, dy);
    }

    fn mouse_press(&mut self, button: mouse::Button) {
        println!("Pressed mouse button '{}'", button);
    }

    fn mouse_release(&mut self, button: mouse::Button) {
        println!("Released mouse button '{}'", button);
    }

    fn mouse_scroll(&mut self, dx: f64, dy: f64) {
        println!("Scrolled mouse '{}, {}'", dx, dy);
    }
}

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::GameWindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    println!("Press C to turn capture cursor on/off");

    let mut app = App::new();
    let game_iter_settings = piston::GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for e in piston::GameIterator::new(&mut window, &game_iter_settings) {
        match e {
            Input(input::KeyPress { key, .. }) => 
                app.key_press(&mut window, key),
            Input(input::KeyRelease { key, .. }) => 
                app.key_release(key),
            Input(input::MousePress { button, .. }) => 
                app.mouse_press(button),
            Input(input::MouseRelease { button, .. }) => 
                app.mouse_release(button),
            Input(input::MouseMove { draw_x, draw_y, .. }) => 
                app.mouse_move(draw_x, draw_y),
            Input(input::MouseScroll { dx, dy, .. }) => 
                app.mouse_scroll(dx, dy),
            Input(input::MouseRelativeMove { draw_dx, draw_dy, .. }) => 
                app.mouse_relative_move(draw_dx, draw_dy),
            Render(_) => {},
            Update(_) => {},
        }
    }
}

