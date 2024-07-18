use karakuri::{
    components::{Behavior, ComponentsPayload, Figure, Name, Transform},
    math::Vector2,
    utils::{Color, Resolution},
    Engine, InputResult,
};

fn ball_prefab() -> ComponentsPayload {
    struct Ball {
        id: Option<usize>,
    }

    impl Ball {
        fn new() -> Ball {
            Ball { id: None }
        }
    }

    impl Behavior for Ball {
        fn start(&mut self, components: karakuri::components::ComponentsCtx) {
            self.id = self.id_by_name(components.names, "Ball");
        }

        fn update(
            &mut self,
            delta_time: f64,
            _input_result: &InputResult,
            _spawner: &mut karakuri::scene::Spawner,
            components: karakuri::components::ComponentsCtx,
        ) {
            let transform = components.transforms[self.id.unwrap()].as_mut().unwrap();

            transform
                .position
                .add(&Vector2::new(10., 0.).to_scaled(delta_time));
        }
        fn destroy(&mut self) {}
    }

    ComponentsPayload {
        name: Name::new(String::from("Ball")),
        figure: Some(Figure::new(Color::WHITE, Vector2::new(10., 10.))),
        transform: Transform::from_position(Vector2::new(400., 300.)),
        behavior: Some(Box::new(Ball::new())),
    }
}

fn main() {
    let mut engine = Engine::new(
        String::from("Breakout"),
        Resolution::new(800, 600),
        Color::BLACK,
        60,
        30,
    );

    engine.set_scene(vec![ball_prefab()]);

    engine.start();
}
