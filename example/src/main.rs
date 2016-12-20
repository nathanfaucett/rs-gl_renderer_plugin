extern crate gl;
extern crate glutin;

extern crate camera_components;
extern crate scene_graph;
extern crate scene_renderer;
extern crate gl_renderer_plugin;


use camera_components::Camera3D;
use scene_graph::{Scene, Entity};
use scene_renderer::SceneRenderer;
use gl_renderer_plugin::GLRendererPlugin;


fn main() {
    let window = glutin::Window::new().unwrap();

    let mut scene = Scene::new();
    let mut entity = Entity::new();
    let mut scene_renderer = SceneRenderer::new(scene.clone());
    let mut camera3d = Camera3D::new();

    camera3d.set_background(&[0.25, 0.5, 0.75, 1.0]);
    entity.add_component(camera3d);

    scene.add_entity(&mut entity);
    scene.init();

    unsafe {
        match window.make_current() {
            Ok(_) => {
                gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    scene_renderer.add_plugin(GLRendererPlugin::new());
    scene_renderer.init();

    {
        let gl_renderer_plugin = scene_renderer.get_plugin::<GLRendererPlugin>().unwrap();
        let context = gl_renderer_plugin.get_context();
        println!(
            "OpenGL version: {:?}.{:?}, GLSL version {:?}.{:?}0",
            context.get_major(), context.get_minor(), context.get_glsl_major(), context.get_glsl_minor()
        );
    }

    let mut playing = true;
    while playing {
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => {
                    playing = false;
                },
                glutin::Event::Resized(w, h) => {
                    scene_renderer.get_plugin::<GLRendererPlugin>()
                        .unwrap().get_context_mut().set_viewport(0, 0, w as usize, h as usize);
                },
                _ => (),
            }
        }

        scene.update();
        scene_renderer.render();

        match window.swap_buffers() {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }

    scene.clear();
    scene_renderer.clear();
}
