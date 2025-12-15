
use piston_window::*;
use graphics::{Context, Graphics};
use rand::Rng;

enum Direction{
    Up,
    Down,
    Left,
    Right,
}

pub struct App {
    segments: Vec<[i32; 2]>,
    direction: Direction,
    apple: [i32; 2],
    score: u32,
    gameover: bool,
}

impl App{
    fn render<G: Graphics>(&mut self, c: &Context, gl: &mut G) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

        let mut square_segments: Vec<[f64; 4]> = Vec::new();
        for i in &self.segments{
            let x = i[0] as f64;
            let y = i[1] as f64;
            square_segments.push(rectangle::square(x, y, 10.0));
        }

        let apple = rectangle::square(self.apple[0] as f64, self.apple[1] as f64, 10.0);

        clear(WHITE, gl);
        let transform = c.transform.trans(0.0,0.0).rot_deg(0.0);
        for i in square_segments {
            rectangle(BLUE, i, transform, gl);
        }
        rectangle(GREEN, apple, transform, gl);
    }
    fn update(&mut self, _args: &UpdateArgs, size: [i32; 2]){
        if self.gameover {
            return;
        }

        if self.segments[0] == self.apple {
            self.gen_apple_coords(size);
            self.score += 1;
        } else {
            self.segments.pop();
        }

        if matches!(self.direction, Direction::Up) {
            self.segments.insert(0, [
                self.segments[0][0],
                self.segments[0][1] - 10
            ]);
        }
        if matches!(self.direction, Direction::Down) {
            self.segments.insert(0, [
                self.segments[0][0],
                self.segments[0][1] + 10
            ]);
        }
        if matches!(self.direction, Direction::Left) {
            self.segments.insert(0, [
                self.segments[0][0] - 10,
                self.segments[0][1]
            ]);
        }
        if matches!(self.direction, Direction::Right) {
            self.segments.insert(0, [self.segments[0][0] + 10, self.segments[0][1]]);
        }
        if self.check_if_collision(size) {
            self.gameover = true;
            return;
        }
    }
    fn check_if_collision(&mut self, size: [i32; 2]) -> bool {
        if (self.segments[0][0] < 0 ||
            self.segments[0][1] < 0) ||
           (self.segments[0][0] >= size[0] ||
            self.segments[0][1] > size[1])
        {
            return true;
        }
        for i in 1..self.segments.len() {
            if self.segments[i] == self.segments[0] {
                return true;
            }
        }
        return false;
    }
    fn change_directions(&mut self, args: &ButtonArgs) {
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
    fn gen_apple_coords(&mut self, size: [i32; 2]){
        let w: i32 = size[0] / 10;
        let h: i32 = size[1] / 10;
        let mut rng = rand::rng();
        self.apple = [10_i32 * rng.random_range(0..w),
                      10_i32 * rng.random_range(0..h)];
        for pos in &self.segments{
            if *pos == self.apple {
                self.gen_apple_coords(size);
                break;
            }
        }
    }
}
fn main() {
    let size: [i32; 2] = [300, 300];

    let mut window: PistonWindow = WindowSettings::new("Snake", [size[0] as u32, size[1] as u32])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.

    let mut app = App {
        segments: vec![[50, 30], [40, 30], [30, 30]],
        direction: Direction::Right,
        apple: [200, 30],
        score: 0,
        gameover: false,
    };

    let event_settings = EventSettings::new().ups(15);
    let mut events = Events::new(event_settings);
    let mut already_pressed = true;
    while let Some(e) = events.next(&mut window) {
        window.draw_2d(&e, |c, g, _| {
            app.render(&c, g);
        });

        if let Some(args) = e.update_args() {
            already_pressed = false;
            app.update(&args, size);
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

fn check_directions(dir1: &Direction, dir2: Direction) -> bool{
    if (matches!(dir1, Direction::Down) &&
        matches!(dir2, Direction::Up)) ||
       (matches!(dir1 ,Direction::Up) &&
        matches!(dir2 ,Direction::Down)) ||
       (matches!(dir1, Direction::Left) &&
        matches!(dir2, Direction::Right)) ||
       (matches!(dir1, Direction::Right) &&
        matches!(dir2, Direction::Left))
    {
        return false;
    } else {
        return true;
    }
}
