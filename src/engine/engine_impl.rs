use sdl2::Sdl;

use crate::{
    components::ComponentsPayload,
    core::{FpsController, InputController, Renderer},
    scene::Scene,
    utils::{Color, Resolution},
};

pub struct Engine {
    fps_controller: FpsController,
    renderer: Renderer,
    input_controller: InputController,
    scene: Option<Scene>,
}

impl Engine {
    pub fn new(
        title: String,
        resolution: Resolution,
        clear_color: Color,
        target_fps: u32,
        min_update_fps: u32,
    ) -> Engine {
        let sdl = Engine::init_sdl();

        Engine {
            scene: None,
            renderer: Renderer::new(&sdl, &title, resolution, clear_color),
            fps_controller: FpsController::new(
                sdl.timer()
                    .unwrap_or_else(|e| panic!("Failed to get SDL2 timer: {}", e)),
                target_fps,
                min_update_fps,
            ),
            input_controller: InputController::new(
                sdl.event_pump()
                    .unwrap_or_else(|e| panic!("Failed to get SDL2 event pump: {}", e)),
            ),
        }
    }

    pub fn start(&mut self) {
        self.scene.as_mut().expect("Scene wasn't set").play(
            &mut self.fps_controller,
            &mut self.input_controller,
            &mut self.renderer,
        );
    }

    pub fn resolution(&self) -> Resolution {
        self.renderer.resolution()
    }

    // TODO: This must be 'load level' or 'load scene'
    pub fn set_scene(&mut self, initial_entities: Vec<ComponentsPayload>) {
        self.scene = Some(Scene::new());

        self.scene
            .as_mut()
            .unwrap()
            .add_initial_entities(initial_entities);
    }

    fn init_sdl() -> Sdl {
        sdl2::init().unwrap_or_else(|e| {
            panic!("Failed to initialize SDL2: {}", e);
        })
    }
}
