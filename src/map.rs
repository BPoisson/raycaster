use ggez::{Context};
use ggez::glam::Vec2;
use ggez::graphics::Canvas;
use crate::constants::SCREEN_SIZE;
use crate::wall::Wall;

pub struct Map {
    pub walls: Vec<Wall>
}

impl Map {
    pub fn new(ctx: &Context) -> Self {
        let mut walls: Vec<Wall> = Vec::new();
        let start_point1: Vec2 = Vec2::new(100.0, 20.0);
        let end_point1: Vec2 = Vec2::new(SCREEN_SIZE.x - 100.0, 150.0);

        let start_point2: Vec2 = Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y - 100.0);
        let end_point2: Vec2 = Vec2::new(SCREEN_SIZE.x - 100.0, SCREEN_SIZE.y / 2.0);

        walls.push(Wall::new(ctx, &start_point1, &end_point1));
        walls.push(Wall::new(ctx, &start_point2, &end_point2));

        return Map {
            walls
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        for wall in &self.walls {
            wall.draw(canvas);
        }
    }
}