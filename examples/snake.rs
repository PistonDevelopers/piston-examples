extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use crate::piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, ButtonArgs, ButtonEvent, Button, ButtonState, Key};
use piston::window::WindowSettings;
use rand::Rng;


enum Direction{
    Up,Down,Left,Right
}

pub struct Segment{
    x: i32,
    y: i32
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    segments: Vec<Segment>,
    direction: Direction,
    applex: i32,
    appley: i32,
    score: u32,
    gameover: bool,

}

impl App{
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let mut square_segments: Vec<[f64; 4]> = Vec::new();
        for i in &self.segments{
            let x = i.x as f64;
            let y = i.y as f64;
            square_segments.push(rectangle::square(x, y, 10.0));
        }

        let apple = rectangle::square(self.applex as f64, self.appley as f64, 10.0);

        self.gl.draw(args.viewport(), |c, gl|{
            clear(WHITE, gl);
            let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
            for i in square_segments {
                rectangle(BLUE, i, transform, gl);
            }
            rectangle(GREEN, apple, transform, gl);

        });
    }
    fn update(&mut self, _args: &UpdateArgs, windowx: &u32, windowy: &u32){
        if self.gameover {
            return;
        }
        if matches!(self.direction, Direction::Up) {
            self.segments.insert(0, Segment{x: self.segments[0].x, y: self.segments[0].y - 10});
        }
        if matches!(self.direction, Direction::Down) {
            self.segments.insert(0, Segment{x: self.segments[0].x, y: self.segments[0].y + 10});
        }
        if matches!(self.direction, Direction::Left) {
            self.segments.insert(0, Segment{x: self.segments[0].x - 10, y: self.segments[0].y});
        }
        if matches!(self.direction, Direction::Right) {
            self.segments.insert(0, Segment{x: self.segments[0].x + 10, y: self.segments[0].y});
        }
        if self.check_if_collision(&windowx, &windowy) {
            self.gameover = true;
            return;
        }
        if self.segments[0].x == self.applex && self.segments[0].y == self.appley {
            self.gen_apple_coords(&windowx, &windowy);
            self.score += 1;
        } else {
            self.segments.pop();
        }
    }
    fn check_if_collision(&mut self, windowx: &u32, windowy: &u32) -> bool{
        if (self.segments[0].x < 0 || self.segments[0].y < 0) || (self.segments[0].x > *windowx as i32 || self.segments[0].y > *windowy as i32) {
            return true;
        }
        for i in 1..self.segments.len() {
            if self.segments[0].x == self.segments[i].x && self.segments[0].y == self.segments[i].y {
                return true;
            }
        }
        return false;
    }
    fn change_directions(&mut self, args: &ButtonArgs){
        if args.state == ButtonState::Press {
            if args.button == Button::Keyboard(Key::Up) && check_directions(&self.direction, Direction::Up) {
                self.direction = Direction::Up;
            }
            if args.button == Button::Keyboard(Key::Down) && check_directions(&self.direction, Direction::Down) {
                self.direction = Direction::Down;
            }
            if args.button == Button::Keyboard(Key::Left) && check_directions(&self.direction, Direction::Left) {
                self.direction = Direction::Left;
            }
            if args.button == Button::Keyboard(Key::Right) && check_directions(&self.direction, Direction::Right) {
                self.direction = Direction::Right;
            }

            
        }
    }
    fn gen_apple_coords(&mut self, windowx: &u32, windowy: &u32){
        self.applex = round_to_nearest_10(rand::thread_rng().gen_range(0..*windowx as i32));
        self.appley = round_to_nearest_10(rand::thread_rng().gen_range(0..*windowy as i32));
        for i in &self.segments{
            if i.x == self.applex && i.y == self.appley {
                self.gen_apple_coords(windowx, windowy);
                break;
            }
        }
    }
    
    
    
}
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let windowx: u32 = 300;
    let windowy: u32 = 300;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("Snake", [windowx, windowy])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    
    let mut app = App {
        gl: GlGraphics::new(opengl),
        segments: vec![Segment{x:50, y:30}, Segment{x:40, y:30}, Segment{x:30, y:30}],
        direction: Direction::Right,
        applex: 200,
        appley: 30,
        score: 0,
        gameover: false,
    };
    
    let event_settings = EventSettings::new().ups(15);
    let mut events = Events::new(event_settings);
    let mut already_pressed = true;
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            let newwindowy = windowy + 50;
            already_pressed = false;
            app.update(&args, &windowx, &newwindowy);
        }
        if app.gameover {
            println!("Game over! Your score is: {}", app.score);
            return;
        }
        if let Some(args) = e.button_args() {
            if !(already_pressed) {
                already_pressed = true;
                app.change_directions(&args);
            }
            
        }
    }
}

fn round_to_nearest_10(n: i32) -> i32{
    let a = (n/10) * 10 as i32;
    let b = a + 10;
    if n - a > b - n {
        return b;
    }
    return a;
}
fn check_directions(dir1: &Direction, dir2: Direction) -> bool{
        if (matches!(dir1, Direction::Down) && matches!(dir2, Direction::Up)) || (matches!(dir1 ,Direction::Up) && matches!(dir2 ,Direction::Down)) || (matches!(dir1, Direction::Left) && matches!(dir2, Direction::Right)) || (matches!(dir1, Direction::Right) && matches!(dir2, Direction::Left)) {
            return false;
        }
        return true;
}
