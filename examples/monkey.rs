use piston_window::*;
use turbine::scene3d::*;
use turbine::scene3d::Command::*;
use turbine_scene3d_wgpu::{utils, State};
use vecmath::*;
use camera_controllers::*;
use texture::TextureSettings;

fn main() {
    let mut capture_cursor = false;
    let (mut window, mut scene, vertex_shader, fragment_shader) = {
        let settings = WindowSettings::new("monkey", [512; 2])
            .samples(4)
            .exit_on_esc(true);
        let mut window: PistonWindow = settings.build().unwrap();
        window.set_capture_cursor(capture_cursor);

        let depth_texture_view = utils::create_depth_texture_view(
            &window.device,
            &window.surface_config,
            1,
            "depth_texture",
        );

        let mut scene: Scene<State> = Scene::new(SceneSettings::new(), State::new(
            window.device.clone(),
            window.queue.clone(),
            window.surface_config.clone(),
            depth_texture_view,
        ));
        let vertex_shader = scene.vertex_shader(include_str!("../assets/basic_shading.wgsl"))
            .unwrap();
        let fragment_shader = scene.fragment_shader(include_str!("../assets/basic_shading.wgsl"))
            .unwrap();
        (window, scene, vertex_shader, fragment_shader)
    };

    let mut events = Events::new(EventSettings::new());
    let mut frame_graph = FrameGraph::new();

    let mut first_person = FirstPerson::new(
        [0.5, 0.5, 4.0],
        FirstPersonSettings::keyboard_wasd()
    );

    let (monkey, program, light_position_id, ambient_light_id) = {
        let obj_mesh = ObjMesh::load("assets/monkey.obj").unwrap();
        let vertex_array = scene.vertex_array();
        let vertex_buffer = scene.vertex_buffer3(vertex_array, 0, &obj_mesh.vertices);
        let _ = scene.uv_buffer(vertex_array, 1, &obj_mesh.uvs);
        let _ = scene.normal_buffer(vertex_array, 2, &obj_mesh.normals);
        let texture_settings = TextureSettings::new();
        let texture = scene.load_texture("assets/monkey.png", &texture_settings).unwrap();

        let program = scene.program_from_vertex_fragment(vertex_shader, fragment_shader);

        let matrix_id = scene.matrix4_uniform(program, "mvp").unwrap();
        let model_matrix_id = scene.matrix4_uniform(program, "m").unwrap();
        let view_matrix_id = scene.matrix4_uniform(program, "v").unwrap();
        let light_position_id = scene.vector3_uniform(program, "LightPosition_worldspace").unwrap();
        let ambient_light_id = scene.f32_uniform(program, "ambientLight").unwrap();

        (frame_graph.command_list(vec![
            SetModelViewProjection(matrix_id),
            SetView(view_matrix_id),
            SetModel(model_matrix_id),
            SetTexture(texture),
            DrawTriangles(vertex_array, vertex_buffer.len()),
        ]), program, light_position_id, ambient_light_id)
    };

    let monkeys = frame_graph.command_list(vec![
        Translate([0.0, -1.0, 0.0]),
        Draw(monkey),

        PushTransform,
        Translate([1.0, 2.0, 0.0]),
        RotateYDeg(20.0),
        Draw(monkey),
        PopTransform,

        Translate([0.0, 4.0, 0.0]),
        RotateYDeg(-20.0),
        Draw(monkey),
    ]);

    let mut time: f32 = 0.0;
    while let Some(e) = events.next(&mut window) {
        if capture_cursor {first_person.event(&e)};

        if let Some(args) = e.render_args() {
            scene.state.start_render(&window.surface);

            let proj = get_projection(&window);
            scene.projection(proj);
            scene.camera(first_person.camera(args.ext_dt).orthogonal());
            scene.model(mat4_id());
            scene.clear([0.0, 0.0, 0.0, 1.0]);

            scene.use_program(program);
            scene.set_vector3(light_position_id, [time.cos() * 4.0, 5.0, time.sin() * 4.0]);
            scene.set_f32(ambient_light_id, 0.1);

            scene.draw(monkeys, &frame_graph);

            scene.state.end_render();
        }

        if let Some(button) = e.press_args() {
            if let Button::Keyboard(Key::C) = button {
                capture_cursor = !capture_cursor;
                window.set_capture_cursor(capture_cursor);
            }
        }

        if let Some(args) = e.update_args() {
            time += args.dt as f32;
        }
    }
}

fn get_projection<W: Window>(w: &W) -> Matrix4<f32> {
    let draw_size = w.draw_size();
    CameraPerspective {
        fov: 90.0, near_clip: 0.1, far_clip: 1000.0,
        aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32)
    }.projection()
}
