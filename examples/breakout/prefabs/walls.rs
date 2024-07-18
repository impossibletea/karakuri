use karakuri::{
    components::{ComponentsPayload, Figure, Name, Transform},
    math::Vector2,
    utils::{Color, Resolution},
};

use crate::utils::WALL_THICKNESS;

pub fn left_wall(resolution: &Resolution) -> ComponentsPayload {
    ComponentsPayload {
        name: Name::new(String::from("left wall")),
        figure: Some(Figure::new(
            Color::WHITE,
            Vector2::new(WALL_THICKNESS, resolution.height as f64),
        )),
        transform: Transform::from_position(Vector2::new(
            WALL_THICKNESS / 2.,
            resolution.height as f64 / 2.,
        )),
        behavior: None,
    }
}

pub fn top_wall(resolution: &Resolution) -> ComponentsPayload {
    ComponentsPayload {
        name: Name::new(String::from("top wall")),
        figure: Some(Figure::new(
            Color::WHITE,
            Vector2::new(resolution.width as f64, WALL_THICKNESS),
        )),
        transform: Transform::from_position(Vector2::new(
            resolution.width as f64 / 2.,
            WALL_THICKNESS / 2.,
        )),
        behavior: None,
    }
}

pub fn right_wall(resolution: &Resolution) -> ComponentsPayload {
    ComponentsPayload {
        name: Name::new(String::from("right wall")),
        figure: Some(Figure::new(
            Color::WHITE,
            Vector2::new(WALL_THICKNESS, resolution.height as f64),
        )),
        transform: Transform::from_position(Vector2::new(
            resolution.width as f64 - WALL_THICKNESS / 2.,
            resolution.height as f64 / 2.,
        )),
        behavior: None,
    }
}
