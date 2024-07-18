use karakuri::{
    components::{Behavior, ComponentsPayload, Figure, Name, Transform},
    math::Vector2,
    utils::{Color, Resolution},
};

use crate::utils;

pub fn paddle(resolution: &Resolution) -> ComponentsPayload {
    ComponentsPayload {
        name: Name::new(String::from("paddle")),
        figure: Some(Figure::new(Color::WHITE, Vector2::new(100., 10.))),
        transform: Transform::from_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 - utils::WALL_THICKNESS,
        )),
        behavior: Some(Box::new(Paddle::new())),
    }
}

pub struct Paddle {
    id: Option<usize>,
    speed: f64,
}

impl Paddle {
    pub fn new() -> Paddle {
        Paddle {
            id: None,
            speed: 500.,
        }
    }
}

impl Behavior for Paddle {
    fn start(&mut self, components: karakuri::components::ComponentsCtx) {
        self.id = self.id_by_name(components.names, "paddle");
    }

    fn update(
        &mut self,
        delta_time: f64,
        input_result: &karakuri::InputResult,
        _spawner: &mut karakuri::scene::Spawner,
        components: karakuri::components::ComponentsCtx,
    ) {
        let mut velocity = Vector2::new(0., 0.);

        if input_result.a {
            velocity.x = -self.speed;
        }

        if input_result.d {
            velocity.x = self.speed;
        }

        components.transforms[self.id.unwrap()]
            .as_mut()
            .unwrap()
            .position
            .add(&velocity.to_scaled(delta_time));
    }

    fn destroy(&mut self) {}
}
