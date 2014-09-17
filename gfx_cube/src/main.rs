#![feature(phase)]
#![feature(globs)]
#![crate_name = "cube"]

extern crate gfx;
extern crate piston;
// extern crate glfw_game_window;
extern crate sdl2;
extern crate sdl2_game_window;
#[phase(plugin)]
extern crate gfx_macros;
extern crate native;
extern crate time;

// use glfw_game_window::WindowGLFW;
use sdl2_game_window::WindowSDL2;
use gfx::{Device, DeviceHelper};
use piston::{cam, Window};
//----------------------------------------
// Cube associated data

#[vertex_format]
struct Vertex {
    #[as_float]
    a_pos: [i8, ..3],
    #[as_float]
    a_tex_coord: [u8, ..2],
}

impl Vertex {
    fn new(pos: [i8, ..3], tc: [u8, ..2]) -> Vertex {
        Vertex {
            a_pos: pos,
            a_tex_coord: tc,
        }
    }
}

#[shader_param(CubeBatch)]
struct Params {
    u_model_view_proj: [[f32, ..4], ..4],
    t_color: gfx::shade::TextureParam,
}

static VERTEX_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    attribute vec3 a_pos;
    attribute vec2 a_tex_coord;
    varying vec2 v_TexCoord;
    uniform mat4 u_model_view_proj;
    void main() {
        v_TexCoord = a_tex_coord;
        gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec3 a_pos;
    in vec2 a_tex_coord;
    out vec2 v_TexCoord;
    uniform mat4 u_model_view_proj;
    void main() {
        v_TexCoord = a_tex_coord;
        gl_Position = u_model_view_proj * vec4(a_pos, 1.0);
    }
"
};

static FRAGMENT_SRC: gfx::ShaderSource = shaders! {
GLSL_120: b"
    #version 120
    varying vec2 v_TexCoord;
    uniform sampler2D t_color;
    void main() {
        vec4 tex = texture2D(t_color, v_TexCoord);
        float blend = dot(v_TexCoord-vec2(0.5,0.5), v_TexCoord-vec2(0.5,0.5));
        gl_FragColor = mix(tex, vec4(0.0,0.0,0.0,0.0), blend*1.0);
    }
"
GLSL_150: b"
    #version 150 core
    in vec2 v_TexCoord;
    out vec4 o_Color;
    uniform sampler2D t_color;
    void main() {
        vec4 tex = texture(t_color, v_TexCoord);
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
    let (win_width, win_height) = (640, 480);
    let mut window = WindowSDL2::new(
        piston::shader_version::opengl::OpenGL_3_2,
        piston::WindowSettings {
            title: "cube".to_string(),
            size: [win_width, win_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    window.capture_cursor(true);

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let frame = gfx::Frame::new(win_width as u16, win_height as u16);
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

    let mesh = device.create_mesh(vertex_data.as_slice());

    let slice = {
        let index_data = vec![
            0u8, 1, 2, 2, 3, 0,    //top
            4, 5, 6, 6, 7, 4,       //bottom
            8, 9, 10, 10, 11, 8,    //right
            12, 13, 14, 14, 16, 12, //left
            16, 17, 18, 18, 19, 16, //front
            20, 21, 22, 22, 23, 20, //back
        ];

        let buf = device.create_buffer_static(index_data.as_slice());
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
            vec![0x20u8, 0xA0u8, 0xC0u8, 0x00u8].as_slice()
        ).unwrap();

    let sampler = device.create_sampler(
        gfx::tex::SamplerInfo::new(
            gfx::tex::Bilinear, 
            gfx::tex::Clamp
        )
    );
    
    let program = device.link_program(
            VERTEX_SRC.clone(), 
            FRAGMENT_SRC.clone()
        ).unwrap();

    let mut graphics = gfx::Graphics::new(device);
    let batch: CubeBatch = graphics.make_batch(&program, &mesh, slice, &state).unwrap();

    let mut data = Params {
        u_model_view_proj: piston::vecmath::mat4_id(),
        t_color: (texture, Some(sampler)),
    };

    let model = piston::vecmath::mat4_id();
    let projection = cam::CameraPerspective {
            fov: 90.0f32,
            near_clip: 0.1,
            far_clip: 1000.0,
            aspect_ratio: (win_width as f32) / (win_height as f32)
        }.projection();
    let mut first_person = cam::FirstPerson::new(
        [0.5f32, 0.5, 4.0],
        cam::FirstPersonSettings::keyboard_wasd()
    );

    let mut game_iter = piston::EventIterator::new(
        &mut window,
        &piston::EventSettings {
            updates_per_second: 120,
            max_frames_per_second: 60
        }
    );

    for e in game_iter {
        match e {
            piston::Render(args) => {
                graphics.clear(
                    gfx::ClearData {
                        color: [0.3, 0.3, 0.3, 1.0],
                        depth: 1.0,
                        stencil: 0,
                    },
                    gfx::Color | gfx::Depth,
                    &frame
                );
                data.u_model_view_proj = cam::model_view_projection(
                        model,
                        first_person.camera(args.ext_dt).orthogonal(),
                        projection
                    );
                graphics.draw(&batch, &data, &frame);
                graphics.end_frame();
            },
            piston::Update(args) => first_person.update(args.dt),
            piston::Input(e) => first_person.input(&e),
        }
    }
}


