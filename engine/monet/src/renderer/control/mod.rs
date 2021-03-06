
pub use descartes::{N, P3, P2, V3, V4, M4, Iso3, Persp3, ToHomogeneous, Norm, Into2d, Into3d,
                    WithUniqueOrthogonal, Inverse, Rotate};
use kay::{World, External};
use glium::Frame;

use super::{Renderer, RendererID};

impl Renderer {
    /// Critical
    pub fn setup(&mut self, world: &mut World) {
        for (scene_id, scene) in self.scenes.iter().enumerate() {
            for renderable in &scene.renderables {
                renderable.setup_in_scene(self.id, scene_id, world);
            }
        }
    }

    /// Critical
    pub fn render(&mut self, world: &mut World) {
        let self_id = self.id;
        let current_frame = self.current_frame;
        for (scene_id, scene) in self.scenes.iter_mut().enumerate() {
            for renderable in &scene.renderables {
                renderable.render_to_scene(self_id, scene_id, current_frame, world);
            }
        }
        self.current_frame += 1;
    }

    /// Critical
    pub fn submit(
        &mut self,
        given_target: &External<Frame>,
        return_to: TargetProviderID,
        world: &mut World,
    ) {
        let mut target = given_target.steal();
        for scene in &self.scenes {
            self.render_context.submit(scene, &mut *target);
        }

        return_to.submitted(target, world);
    }
}

pub trait TargetProvider {
    fn submitted(&mut self, target: &External<Frame>, world: &mut World);
}

mod kay_auto;
pub use self::kay_auto::*;
