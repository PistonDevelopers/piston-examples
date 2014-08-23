#![feature(phase)]
#![feature(globs)]
#![crate_name = "cube"]

extern crate piston;
// extern crate glfw_game_window;
extern crate sdl2_game_window;
extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate native;
extern crate time;

// use glfw_game_window::GameWindowGLFW as Window;
use sdl2_game_window::GameWindowSDL2 as Window;
use gfx::{Device, DeviceHelper};
use piston::{cam, GameWindow};
//----------------------------------------
// Cube associated data

#[vertex_format]
struct Vertex {
    #[as_float]
    a_Pos: [i8, ..3],
    #[as_float]
    a_TexCoord: [u8, ..2],
}

impl Vertex {
    fn new(pos: [i8, ..3], tc: [u8, ..2]) -> Vertex {
        Vertex {
            a_Pos: pos,
            a_TexCoord: tc,
        }
    }
}

#[shader_param(Program)]
struct Params {
    u_ModelViewProj: [[f32, ..4], ..4],
    t_Color: gfx::shade::TextureParam,
}

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    attribute vec3 a_Pos;
    attribute vec2 a_TexCoord;
    varying vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec3 a_Pos;
    in vec2 a_TexCoord;
    out vec2 v_TexCoord;
    uniform mat4 u_ModelViewProj;
    void main() {
        v_TexCoord = a_TexCoord;
        gl_Position = u_ModelViewProj * vec4(a_Pos, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    varying vec2 v_TexCoord;
    uniform sampler2D t_Color;
    void main() {
        vec4 tex = texture2D(t_Color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;
    out vec4 o_Color;
    uniform sampler2D t_Color;
    void main() {
        vec4 tex = texture(t_Color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        o_Color = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
};

//----------------------------------------

// We need to run on the main thread, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
     native::start(argc, argv, main)
}

fn main() {
    let mut window = Window::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::GameWindowSettings {
            title: "cube".to_string(),
            size: [640, 480],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    window.capture_cursor(true);
    
    let (mut device, frame) = window.gfx();
    let mut renderer = device.create_renderer();
    let state = gfx::DrawState::new().depth(gfx::state::LessEqual, true);

    let vertex_data = vec![
        //top (0, 0, 1)
        Vertex::new([-1, -1,  1], [0, 0]),
        Vertex::new([ 1, -1,  1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //bottom (0, 0, -1)
        Vertex::new([ 1,  1, -1], [0, 0]),
        Vertex::new([-1,  1, -1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
        //right (1, 0, 0)
        Vertex::new([ 1, -1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([ 1, -1,  1], [0, 1]),
        //left (-1, 0, 0)
        Vertex::new([-1,  1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([-1,  1, -1], [0, 1]),
        //front (0, 1, 0)
        Vertex::new([-1,  1, -1], [0, 0]),
        Vertex::new([ 1,  1, -1], [1, 0]),
        Vertex::new([ 1,  1,  1], [1, 1]),
        Vertex::new([-1,  1,  1], [0, 1]),
        //back (0, -1, 0)
        Vertex::new([ 1, -1,  1], [0, 0]),
        Vertex::new([-1, -1,  1], [1, 0]),
        Vertex::new([-1, -1, -1], [1, 1]),
        Vertex::new([ 1, -1, -1], [0, 1]),
    ];

    let mesh = device.create_mesh(vertex_data);

    let slice = {
        let index_data = vec![
            0u8, 1, 2, 2, 3, 0,    //top
            4, 5, 6, 6, 7, 4,       //bottom
            8, 9, 10, 10, 11, 8,    //right
            12, 13, 14, 14, 16, 12, //left
            16, 17, 18, 18, 19, 16, //front
            20, 21, 22, 22, 23, 20, //back
        ];

        let buf = device.create_buffer_static(&index_data);
        gfx::IndexSlice8(gfx::TriangleList, buf, 0, 36)
    };

    let tinfo = gfx::tex::TextureInfo {
        width: 1,
        height: 1,
        depth: 1,
        levels: 1,
        kind: gfx::tex::Texture2D,
        format: gfx::tex::RGBA8,
    };
    let img_info = tinfo.to_image_info();
    let texture = device.create_texture(tinfo).unwrap();
    device.update_texture(
            &texture, 
            &img_info,
            &vec![0x20u8, 0xA0u8, 0xC0u8, 0x00u8]
        ).unwrap();

    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(
            gfx::tex::Bilinear, 
            gfx::tex::Clamp
        )
    );

    let mut data = Params {
        u_ModelViewProj: piston::vecmath::mat4_id(),
        t_Color: (texture, Some(sampler)),
    };
    let prog: Program = device.link_program(
            VERTEX_SRC.clone(), 
            FRAGMENT_SRC.clone()
        ).unwrap();

    let model = piston::vecmath::mat4_id();
    let projection = cam::CameraPerspective {
            fov: 90.0f32,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: 1.0
        }.projection();
    let mut camera = cam::Camera::new(0.5f32, 0.5, 4.0);
    let mut fps_controller = cam::FPSController::new(
        cam::FPSControllerSettings::default()
    );
    camera.set_yaw_pitch(fps_controller.yaw, fps_controller.pitch);

    let mut game_iter = piston::GameIterator::new(
        &mut window,
        &piston::GameIteratorSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );

    for e in game_iter {
        match e {
            piston::Render(_args) => {
                renderer.reset();
                renderer.clear(
                    gfx::ClearData {
                        color: Some([0.3, 0.3, 0.3, 1.0]),
                        depth: Some(1.0),
                        stencil: None,
                    },
                    &frame
                );
                data.u_ModelViewProj = cam::model_view_projection(
                        model,
                        camera.orthogonal(),
                        projection
                    );
                renderer.draw(&mesh, slice, &frame, (&prog, &data), &state).unwrap();
                device.submit(renderer.as_buffer());
            },
            piston::Update(args) => fps_controller.update(args.dt, &mut camera),
            piston::Input(e) => fps_controller.input(&e, &mut camera),
        }
    }
}

