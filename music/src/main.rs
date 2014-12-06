//! Playing a music file with SDL2 mixer.

extern crate shader_version;
extern crate event;
extern crate sdl2_window;
extern crate sdl2_mixer;
extern crate sdl2;

use std::cell::RefCell;
use sdl2_window::Sdl2Window;
use sdl2_mixer as mix;
use event::{ Events, WindowSettings };

fn init_audio() {
    sdl2::init(sdl2::INIT_AUDIO | sdl2::INIT_TIMER);
    // Load dynamic libraries.
    mix::init(
          mix::INIT_MP3 
        | mix::INIT_FLAC 
        | mix::INIT_MOD 
        | mix::INIT_FLUIDSYNTH
        | mix::INIT_MODPLUG
        | mix::INIT_OGG
    );
    mix::open_audio(
        mix::DEFAULT_FREQUENCY,
        mix::DEFAULT_FORMAT,
        mix::DEFAULT_CHANNELS,
        1024
    ).unwrap();
    mix::allocate_channels(mix::DEFAULT_CHANNELS); 
}

fn main() {
    let window = Sdl2Window::new(
        shader_version::opengl::OpenGL::OpenGL_3_2,
        WindowSettings {
            title: "Music".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        }
    );

    init_audio();
 
    // Load music file. 
    let file = Path::new("./bin/assets/piano.wav");
    let music = mix::Music::from_file(&file).unwrap();
   
    // Loop four times. 
    music.play(4).unwrap();

    let window = RefCell::new(window);
    for _e in Events::new(&window) {}
}

