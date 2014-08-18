#![feature(globs)]

extern crate graphics;
extern crate piston;
// extern crate sdl2_game_window;
extern crate glfw_game_window;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::keyboard;

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
        args: &piston::KeyPressArgs
    ) {
        if args.key == keyboard::C {
            println!("Turned capture cursor on");
            self.capture_cursor = !self.capture_cursor;

            window.capture_cursor(self.capture_cursor);
        }

        println!("Pressed keyboard key '{}'", args.key);
    }

    fn key_release(
        &mut self,
        args: &piston::KeyReleaseArgs
    ) {
        println!("Released keyboard key '{}'", args.key);
    }

    fn mouse_move(
        &mut self,
        args: &piston::MouseMoveArgs
    ) {
        println!("Mouse moved '{} {}'", args.x, args.y);
    }

    fn mouse_relative_move(
        &mut self,
        args: &piston::MouseRelativeMoveArgs
    ) {
        println!("Relative mouse moved '{} {}'", args.dx, args.dy);
    }

    fn mouse_press(
        &mut self,
        args: &piston::MousePressArgs
    ) {
        println!("Pressed mouse button '{}'", args.button);
    }

    fn mouse_release(
        &mut self,
        args: &piston::MouseReleaseArgs
    ) {
        println!("Released mouse button '{}'", args.button);
    }

    fn mouse_scroll(
        &mut self,
        args: &piston::MouseScrollArgs
    ) {
        println!("Scrolled mouse '{}, {}'", args.x, args.y);
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
            piston::KeyPress(args) => app.key_press(&mut window, &args),
            piston::KeyRelease(args) => app.key_release(&args),
            piston::MousePress(args) => app.mouse_press(&args),
            piston::MouseRelease(args) => app.mouse_release(&args),
            piston::MouseMove(args) => app.mouse_move(&args),
            piston::MouseScroll(args) => app.mouse_scroll(&args),
            piston::MouseRelativeMove(args) => app.mouse_relative_move(&args),
            piston::Render(_) => {},
            piston::Update(_) => {},
        }
    }
}

