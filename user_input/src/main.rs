#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    Game,
    GameIteratorSettings,
    GameWindowSettings,
    KeyPressArgs,
    KeyReleaseArgs,
    MousePressArgs,
    MouseReleaseArgs,
    MouseScrollArgs,
};

pub struct App;

impl App {
    /// Creates a new application.
    pub fn new() -> App {
        App
    }
}

impl Game for App {
    fn key_press(
        &mut self,
        args: &KeyPressArgs
    ) {
        println!("Pressed keyboard key '{}'", args.key);
    }

    fn key_release(
        &mut self,
        args: &KeyReleaseArgs
    ) {
        println!("Released keyboard key '{}'", args.key);
    }

    fn mouse_press(
        &mut self,
        args: &MousePressArgs
    ) {
        println!("Pressed mouse button '{}'", args.button);
    }

    fn mouse_release(
        &mut self,
        args: &MouseReleaseArgs
    ) {
        println!("Released mouse button '{}'", args.button);
    }

    fn mouse_scroll(
        &mut self,
        args: &MouseScrollArgs
    ) {
        println!("Scrolled mouse '{}, {}'", args.x, args.y);
    }
}

fn main() {
    let mut window = Window::new(
        GameWindowSettings {
            title: "Keycode".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );

    let mut app = App::new();
    let game_iter_settings = GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    app.run(&mut window, &game_iter_settings);
}

