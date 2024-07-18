use crate::{core::InputResult, scene::Spawner};

use super::{Figure, Name, Transform};

pub struct ComponentsCtx<'a> {
    pub names: &'a [Option<Name>],
    pub transforms: &'a mut [Option<Transform>],
    pub figures: &'a [Option<Figure>],
}

pub trait Behavior {
    fn start(&mut self, components: ComponentsCtx);
    fn update(
        &mut self,
        delta_time: f64,
        input_result: &InputResult,
        spawner: &mut Spawner,
        components: ComponentsCtx,
    );
    fn destroy(&mut self);

    fn id_by_name(&self, name_components: &[Option<Name>], name: &str) -> Option<usize> {
        name_components
            .iter()
            .position(|name_component| match name_component {
                None => false,
                Some(name_component) => name_component.value() == name,
            })
    }
}
