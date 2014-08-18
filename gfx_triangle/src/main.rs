#![crate_name = "triangle"]
#![feature(phase)]

extern crate piston;
extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate sdl2_game_window;
extern crate glfw_game_window;

// use Window = sdl2_game_window::GameWindowSDL2;
use Window = glfw_game_window::GameWindowGLFW;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};
use gfx::{
    Device,
    DeviceHelper,
};

#[vertex_format]
struct Vertex {
    pos: [f32, ..2],
    color: [f32, ..3],
}

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    attribute vec2 pos;
    attribute vec3 color;
    varying vec4 v_Color;
    void main() {
        v_Color = vec4(color, 1.0);
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 pos;
    in vec3 color;
    out vec4 v_Color;
    void main() {
        v_Color = vec4(color, 1.0);
        gl_Position = vec4(pos, 0.0, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    varying vec4 v_Color;
    void main() {
        gl_FragColor = v_Color;
    }
"
GLSL_150: b"
    #version 150 core
    in vec4 v_Color;
    out vec4 o_Color;
    void main() {
        o_Color = v_Color;
    }
"
};

fn main() {
    let ref mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        GameWindowSettings {
            title: "triangle".to_string(),
            size: [300, 300],
            fullscreen: false,
            exit_on_esc: true,
        }
    );
    let (mut device, frame) = window.gfx();

    let state = gfx::DrawState::new();
    let vertex_data = vec![
        Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
        Vertex { pos: [ 0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
        Vertex { pos: [ 0.0, 0.5 ], color: [0.0, 0.0, 1.0] }
    ];
    let mesh = device.create_mesh(vertex_data);
    let program = device.link_program(
            (), 
            VERTEX_SRC.clone(), 
            FRAGMENT_SRC.clone()
        ).unwrap();

    let mut list = device.create_draw_list();
    list.clear(
        gfx::ClearData {
            color: Some(gfx::Color([0.3, 0.3, 0.3, 0.1])),
            depth: None,
            stencil: None,
        },
        &frame
    );
    list.draw(
            &mesh, 
            mesh.get_slice(), 
            &frame,
            &program,
            &state
        ).unwrap();
    
    
    let mut game_iter = GameIterator::new(
        window,
        &GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60,
        }
    );
    for e in game_iter {
        match e {
            Render(_args) => {
                device.submit(list.as_slice());
            },
            _ => {},
        }
    }
}

