use camera_components::{Camera3DManager, Camera2DManager};

use uuid::Uuid;

use gl_context::{Context, Program};
use gl_geometry::GLGeometry;

use geometry::Geometry;
use material::Material;

use scene_graph::Id;
use scene_renderer::{SceneRenderer, Plugin};

use hash_map::HashMap;
use insert::Insert;
use map::Map;

use shared::Shared;



struct GLMaterialData {
    program: Program,
    material: Material,
}

#[derive(Clone)]
pub struct GLMaterial {
    data: Shared<GLMaterialData>,
}

impl GLMaterial {
    pub fn new(program: Program, material: Material) -> Self {
        GLMaterial {
            data: Shared::new(GLMaterialData {
                program: program,
                material: material,
            })
        }
    }

    pub fn get_program(&self) -> &Program {&self.data.program}
    pub fn get_program_mut(&mut self) -> &mut Program {&mut self.data.program}

    pub fn get_material(&self) -> &Material {&self.data.material}
    pub fn get_material_mut(&mut self) -> &mut Material {&mut self.data.material}
}


struct GLRendererPluginData {
    uuid: Uuid,

    context: Context,
    scene_renderer: Option<SceneRenderer>,

    geometries: HashMap<Uuid, GLGeometry>,
    materials: HashMap<Uuid, GLMaterial>,
}

#[derive(Clone)]
pub struct GLRendererPlugin {
    data: Shared<GLRendererPluginData>,
}

impl GLRendererPlugin {
    pub fn new() -> Self {
        GLRendererPlugin {
            data: Shared::new(GLRendererPluginData {
                uuid: Uuid::new_v4(),

                context: Context::new(),
                scene_renderer: None,

                geometries: HashMap::new(),
                materials: HashMap::new(),
            })
        }
    }

    pub fn get_uuid(&self) -> &Uuid {&self.data.uuid}

    pub fn get_context(&self) -> &Context {&self.data.context}
    pub fn get_context_mut(&mut self) -> &mut Context {&mut self.data.context}

    pub fn has_material(&mut self, uuid: &Uuid) -> bool {
        self.data.materials.contains_key(uuid)
    }
    pub fn get_material(&mut self, material: &Material) -> GLMaterial {
        if !self.has_material(material.get_uuid()) {
            let mut program = self.data.context.new_program();

            let shader = material.get_shader().unwrap();
            program.set(shader.get_vertex(), shader.get_fragment());

            self.data.materials.insert(material.get_uuid().clone(), GLMaterial::new(program, material.clone()));
        }
        self.data.materials.get_mut(material.get_uuid()).unwrap().clone()
    }

    pub fn has_geometry(&mut self, geometry: &Geometry) -> bool {
        self.data.geometries.contains_key(geometry.get_uuid())
    }
    pub fn get_geometry(&mut self, geometry: &Geometry) -> GLGeometry {
        let uuid = geometry.get_uuid();

        if self.data.geometries.contains_key(uuid) {
            self.data.geometries.get(uuid).unwrap().clone()
        } else {
            let gl_geometry = GLGeometry::new(self.get_context_mut(), geometry.clone());
            self.data.geometries.insert(uuid.clone(), gl_geometry.clone());
            gl_geometry
        }
    }

    pub fn bind_material(&mut self, gl_material: &GLMaterial) {
        let mut context = self.get_context_mut();
        let material = gl_material.get_material();

        context.set_cull_face(material.get_cull_face());
        context.set_blending(material.get_blending());

        if material.get_wireframe() {
            context.set_line_width(material.get_wireframe_line_width());
        }
    }

    pub fn bind_uniforms(
        &mut self, program: &mut Program,
        projection: &[f32; 16], model_view: &[f32; 16], view: &[f32; 16], normal: &[f32; 9], force: bool
    ) {
        let mut context = self.get_context_mut();

        if program.has_uniform("projection") {
            program.set_uniform("projection", &mut context, projection, force);
        }
        if program.has_uniform("model_view") {
            program.set_uniform("model_view", &mut context, model_view, force);
        }
        if program.has_uniform("view") {
            program.set_uniform("view", &mut context, view, force);
        }
        if program.has_uniform("normal") {
            program.set_uniform("normal", &mut context, normal, force);
        }
    }

    pub fn bind_attributes(
        &mut self, gl_geometry: &mut GLGeometry, program: &mut Program, force: bool
    ) {
        let mut context = self.get_context_mut();

        let mut tmp_gl_geometry = gl_geometry.clone();
        let vertex_buffer = tmp_gl_geometry.get_vertex_buffer(&mut context, force);

        for (name, attribute) in program.get_attributes_mut() {
            attribute.set(&mut context, &vertex_buffer, gl_geometry.get_offset(name), force);
        }
    }
}

impl Plugin for GLRendererPlugin {

    fn get_id(&self) -> Id { Id::of::<GLRendererPlugin>() }

    fn get_scene_renderer(&self) -> Option<SceneRenderer> {
        self.data.scene_renderer.clone()
    }
    fn set_scene_renderer(&mut self, scene_renderer: Option<SceneRenderer>) {
        self.data.scene_renderer = scene_renderer;
    }

    fn get_order(&self) -> usize {0}

    fn clear(&mut self) {
        self.get_context_mut().reset();
    }
    fn init(&mut self) {
        self.get_context_mut().init();
    }
    fn before_render(&mut self) {
        if let Some(camera3d) = {
            let scene = self.get_scene_renderer().unwrap().get_scene();

            if let Some(camera3d_manager) = scene.get_component_manager::<Camera3DManager>() {
                camera3d_manager.get_active_camera()
            } else {
                None
            }
        } {
            let mut context = self.get_context_mut();
            context.set_clear_color(camera3d.get_background());
            context.clear(true, true, true);
        } else if let Some(camera2d) = {
            let scene = self.get_scene_renderer().unwrap().get_scene();

            if let Some(camera2d_manager) = scene.get_component_manager::<Camera2DManager>() {
                camera2d_manager.get_active_camera()
            } else {
                None
            }
        } {
            let mut context = self.get_context_mut();
            context.set_clear_color(camera2d.get_background());
            context.clear(true, true, true);
        } else {
            let mut context = self.get_context_mut();
            context.set_clear_color(&[0.0, 0.0, 0.0, 1.0]);
            context.clear(true, true, true);
        }
    }
    fn after_render(&mut self) {}
}
