extern crate piston;
extern crate piston_window;
extern crate vecmath;
extern crate camera_controllers;
#[macro_use]
extern crate gfx;
extern crate gfx_device_gl;
extern crate glfw_window;

use std::cell::RefCell;
use std::rc::Rc;
use piston_window::*;
use piston::event::*;
use piston::window::{ AdvancedWindow, WindowSettings };
use camera_controllers::{
    FirstPersonSettings,
    FirstPerson,
    CameraPerspective,
    model_view_projection
};
use gfx::attrib::Floater;
use gfx::traits::*;
use glfw_window::{ GlfwWindow, OpenGL };

//----------------------------------------
// Cube associated data

gfx_vertex!( Vertex {
    a_pos@ a_pos: [Floater<i8>; 3],
    a_tex_coord@ a_tex_coord: [Floater<u8>; 2],
});

impl Vertex {
    fn new(pos: [i8; 3], tc: [u8; 2]) -> Vertex {
        Vertex {
            a_pos: Floater::cast3(pos),
            a_tex_coord: Floater::cast2(tc),
        }
    }
}

gfx_parameters!( Params/ParamsLink {
    u_model_view_proj@ u_model_view_proj: [[f32; 4]; 4],
    t_color@ t_color: gfx::shade::TextureParam<R>,
});

//----------------------------------------

fn main() {
    let (win_width, win_height) = (640, 480);
    let window = Rc::new(RefCell::new(GlfwWindow::new(
        OpenGL::_3_2,
        WindowSettings::new("piston-example-gfx_cube", [win_width, win_height])
        .exit_on_esc(true)
        .samples(4)
    ).capture_cursor(true)));

    let events = PistonWindow::new(window, empty_app());

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

    let mesh = events.canvas.borrow_mut().factory.create_mesh(&vertex_data);

    let index_data: &[u8] = &[
         0,  1,  2,  2,  3,  0, // top
         4,  6,  5,  6,  4,  7, // bottom
         8,  9, 10, 10, 11,  8, // right
        12, 14, 13, 14, 12, 16, // left
        16, 18, 17, 18, 16, 19, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let slice = events.canvas.borrow_mut().factory.create_buffer_index(index_data)
                       .to_slice(gfx::PrimitiveType::TriangleList);

    let tinfo = gfx::tex::TextureInfo {
        width: 1, height: 1, depth: 1, levels: 1,
        kind: gfx::tex::TextureKind::Texture2D,
        format: gfx::tex::RGBA8,
    };
    let img_info = tinfo.to_image_info();
    let texture = events.canvas.borrow_mut().factory.create_texture(tinfo).unwrap();
    events.canvas.borrow_mut().factory.update_texture(
        &texture,
        &img_info,
        &[0x20u8, 0xA0, 0xC0, 0x00],
        Some(gfx::tex::TextureKind::Texture2D)
    ).unwrap();

    let sampler = events.canvas.borrow_mut().factory.create_sampler(
        gfx::tex::SamplerInfo::new(gfx::tex::FilterMethod::Bilinear,
            gfx::tex::WrapMode::Clamp));

    let program = {
        let canvas = &mut *events.canvas.borrow_mut();
        let vertex = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("cube_120.glslv")),
            glsl_150: Some(include_bytes!("cube_150.glslv")),
            .. gfx::ShaderSource::empty()
        };
        let fragment = gfx::ShaderSource {
            glsl_120: Some(include_bytes!("cube_120.glslf")),
            glsl_150: Some(include_bytes!("cube_150.glslf")),
            .. gfx::ShaderSource::empty()
        };
        canvas.factory.link_program_source(vertex, fragment,
            &canvas.device.get_capabilities()).unwrap()
    };

    let mut data = Params {
        u_model_view_proj: vecmath::mat4_id(),
        t_color: (texture, Some(sampler)),
        _r: std::marker::PhantomData,
    };

    let get_projection = |w: &PistonWindow<GlfwWindow>| {
        use piston::window::Window;

        let draw_size = w.window.borrow().draw_size();
        CameraPerspective {
            fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
            aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
        }.projection()
    };

    let model = vecmath::mat4_id();
    let mut projection = get_projection(&events);
    let mut first_person = FirstPerson::new([0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd());
    let state = gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true);

    for e in events {
        first_person.event(&e);

        e.draw_3d(|canvas| {
            let args = e.render_args().unwrap();
            canvas.renderer.clear(
                gfx::ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                },
                gfx::COLOR | gfx::DEPTH,
                &canvas.output
            );
            data.u_model_view_proj = model_view_projection(
                model,
                first_person.camera(args.ext_dt).orthogonal(),
                projection
            );
            canvas.renderer.draw(&(&mesh, slice.clone(), &program, &data, &state),
                &canvas.output).unwrap();
        });

        if let Some(_) = e.resize_args() {
            projection = get_projection(&e);
        }
    }
}
