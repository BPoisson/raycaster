use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh};

pub struct Wall {
    pub mesh: Mesh,
    pub start_point: Vec2,
    pub end_point: Vec2
}

impl Wall {
    pub fn new(ctx: &Context, start_point: &Vec2, end_point: &Vec2) -> Self {
        let mesh: Mesh = Mesh::new_line(
            ctx,
            &[*start_point, *end_point],
            1.0,
            Color::WHITE
        ).unwrap();

        return Wall {
            mesh,
            start_point: *start_point,
            end_point: *end_point
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &self.mesh,
            graphics::DrawParam::default()
        );
    }
}