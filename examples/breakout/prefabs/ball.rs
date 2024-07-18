use karakuri::{
    components::{Behavior, ComponentsPayload, Figure, Name, Transform},
    math::Vector2,
    utils::{Color, Resolution},
};

pub fn ball(resolution: &Resolution) -> ComponentsPayload {
    ComponentsPayload {
        name: Name::new(String::from("ball")),
        figure: Some(Figure::new(Color::WHITE, Vector2::new(10., 10.))),
        transform: Transform::from_position(Vector2::new(
            resolution.width as f64 / 2.,
            resolution.height as f64 / 2.,
        )),
        behavior: Some(Box::new(Ball::new())),
    }
}

struct Ball {
    id: Option<usize>,
    velocity: Vector2,
    paddle_id: Option<usize>,
    left_wall_id: Option<usize>,
    top_wall_id: Option<usize>,
    right_wall_id: Option<usize>,
}

impl Ball {
    pub fn new() -> Ball {
        Ball {
            id: None,
            velocity: Vector2::new(500., 500.),
            left_wall_id: None,
            paddle_id: None,
            right_wall_id: None,
            top_wall_id: None,
        }
    }

    fn collide_with_paddle(
        &mut self,
        position: &Vector2,
        paddle_position: &Vector2,
        paddle_size: &Vector2,
    ) {
        let mut diff = paddle_position.x - position.x;
        if diff <= 0. {
            diff = -diff;
        }

        if diff <= paddle_size.x / 2. && position.y >= paddle_position.y && self.velocity.y > 0. {
            self.velocity.y *= -1.;
        }
    }

    fn collide_with_walls(
        &mut self,
        position: &Vector2,
        top_wall_position: &Vector2,
        left_wall_position: &Vector2,
        right_wall_position: &Vector2,
    ) {
        if position.y <= top_wall_position.y {
            self.velocity.y *= -1.;
        }

        if position.x >= right_wall_position.x || position.x <= left_wall_position.x {
            self.velocity.x *= -1.;
        }
    }
}

impl Behavior for Ball {
    fn start(&mut self, components: karakuri::components::ComponentsCtx) {
        self.id = self.id_by_name(components.names, "ball");
        self.paddle_id = self.id_by_name(components.names, "paddle");
        self.left_wall_id = self.id_by_name(components.names, "left wall");
        self.top_wall_id = self.id_by_name(components.names, "top wall");
        self.right_wall_id = self.id_by_name(components.names, "right wall");
    }

    fn update(
        &mut self,
        delta_time: f64,
        _input_result: &karakuri::InputResult,
        _spawner: &mut karakuri::scene::Spawner,
        components: karakuri::components::ComponentsCtx,
    ) {
        let position = &components.transforms[self.id.unwrap()]
            .as_ref()
            .unwrap()
            .position;

        self.collide_with_paddle(
            position,
            &components.transforms[self.paddle_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &components.figures[self.paddle_id.unwrap()]
                .as_ref()
                .unwrap()
                .size,
        );

        self.collide_with_walls(
            position,
            &components.transforms[self.top_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &components.transforms[self.left_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
            &components.transforms[self.right_wall_id.unwrap()]
                .as_ref()
                .unwrap()
                .position,
        );

        components.transforms[self.id.unwrap()]
            .as_mut()
            .unwrap()
            .position
            .add(&self.velocity.to_scaled(delta_time));
    }

    fn destroy(&mut self) {}
}
