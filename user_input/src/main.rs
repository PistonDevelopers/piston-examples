#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use sdl2_game_window::GameWindowSDL2 as Window;
use glfw_game_window::GameWindowGLFW as Window;
use piston::input::keyboard;
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
            Input(input::Press(input::Keyboard(key))) => 
                app.key_press(&mut window, key),
            Input(input::Release(input::Keyboard(key))) => 
                println!("Released keyboard key '{}'", key),
            Input(input::Press(input::Mouse(button))) => 
                println!("Pressed mouse button '{}'", button),
            Input(input::Release(input::Mouse(button))) => 
                println!("Released mouse button '{}'", button),
            Input(input::Move(input::MouseCursor(x, y))) => 
                println!("Mouse moved '{} {}'", x, y),
            Input(input::Move(input::MouseScroll(dx, dy))) => 
                println!("Scrolled mouse '{}, {}'", dx, dy),
            Input(input::Move(input::MouseRelative(dx, dy))) => 
                println!("Relative mouse moved '{} {}'", dx, dy),
            Input(input::Text(text)) => println!("Typed '{}'", text),
            Render(_) => {},
            Update(_) => {},
        }
    }
}

