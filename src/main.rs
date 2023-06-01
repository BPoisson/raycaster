mod raycaster;
mod constants;
mod camera;
mod ray;
mod map;
mod wall;

use std::error::Error;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{ContextBuilder, event};
use crate::constants::SCREEN_SIZE;
use crate::raycaster::Raycaster;

const GAME_ID: &str = "Raycaster";
const AUTHOR: &str = "BPoisson";

fn main() -> Result<(), Box<dyn Error>> {
    let (ctx, event_loop) = ContextBuilder::new(GAME_ID, AUTHOR)
        .window_setup(WindowSetup::default().title(GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.x, SCREEN_SIZE.y))
        .add_resource_path("resources")
        .build()?;

    let raycaster: Raycaster = Raycaster::new(&ctx);

    event::run(ctx, event_loop, raycaster)
}
