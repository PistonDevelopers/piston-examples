#![crate_name = "triangle"]
#![feature(phase)]

extern crate piston;
#[phase(plugin)]
extern crate gfx_macros;
extern crate sdl2_game_window;
extern crate glfw_game_window;

// use sdl2_game_window::GameWindowSDL2 as Window;
use glfw_game_window::GameWindowGLFW as Window;
use piston::{
    GameIterator,
    GameIteratorSettings,
    GameWindowSettings,
    Render,
};
use piston::gfx;
use piston::gfx::{
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
    let program: gfx::shade::EmptyProgram = device.link_program( 
            VERTEX_SRC.clone(), 
            FRAGMENT_SRC.clone()
        ).unwrap();

    let mut renderer = device.create_renderer();
    renderer.clear(
        gfx::ClearData {
            color: Some([0.3, 0.3, 0.3, 0.1]),
            depth: None,
            stencil: None,
        },
        &frame
    );
    renderer.draw(
            &mesh, 
            mesh.get_slice(gfx::TriangleList), 
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
                device.submit(renderer.as_buffer());
            },
            _ => {},
        }
    }
}

