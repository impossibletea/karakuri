use karakuri::{
    utils::{Color, Resolution},
    Engine,
};

mod prefabs;
pub mod utils;

fn main() {
    let mut engine = Engine::new(
        String::from("Breakout"),
        Resolution::new(800, 600),
        Color::BLACK,
        60,
        30,
    );

    let resolution = engine.resolution();

    engine.set_scene(vec![
        prefabs::paddle(&resolution),
        prefabs::ball(&resolution),
        prefabs::left_wall(&resolution),
        prefabs::top_wall(&resolution),
        prefabs::right_wall(&resolution),
    ]);

    engine.start();
}
