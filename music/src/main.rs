//! Playing a music file with SDL2 mixer.

extern crate shader_version;
extern crate event;
extern crate sdl2_game_window;
extern crate sdl2_mixer;
extern crate sdl2;

use sdl2_game_window::WindowSDL2;
use sdl2_mixer as mix;
use event::{
    EventIterator,
    EventSettings,
    WindowSettings,
};

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
    let mut window = WindowSDL2::new(
        shader_version::opengl::OpenGL_3_2,
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
   
    // Loop once. 
    music.play(4).unwrap();

    let event_settings = EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        };
    for _e in EventIterator::new(&mut window, &event_settings) {}
}

