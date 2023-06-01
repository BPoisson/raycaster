use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh};

const INITIAL_LENGTH: f32 = 10.0;

pub struct Ray {
    pub mesh: Mesh,
    pub start_point: Vec2,
    pub end_point: Vec2,
    pub angle: f32,
    pub original_end_point: Vec2,
    pub intersecting: bool
}

impl Ray {
    pub fn new(ctx: &Context, start_point: &Vec2, end_point: &Vec2, angle: &f32) -> Self {
        let mesh: Mesh = Mesh::new_line(
            ctx,
            &[*start_point, *end_point],
            1.0,
            Color::WHITE
        ).unwrap();

        return Ray {
            mesh,
            start_point: *start_point,
            end_point: *end_point,
            angle: *angle,
            original_end_point: *end_point,
            intersecting: true
        };
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        // if self.intersecting {
            canvas.draw(
                &self.mesh,
                graphics::DrawParam::default()
            );
        // }
    }

    pub fn update_ray(&mut self, ctx: &Context, start_point: Vec2, line_start: Vec2, line_end: Vec2) -> () {
        let difference: Vec2 = self.start_point - start_point;
        self.start_point = start_point;
        self.end_point -= difference;

        Self::check_intersects(self, line_start, line_end);

        self.mesh = Mesh::new_line(
            ctx,
            &[self.start_point, self.end_point],
            1.0,
            Color::WHITE
        ).unwrap();
    }

    // https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection
    pub fn check_intersects(&mut self, wall_start: Vec2, wall_end: Vec2) -> () {
        let wall_x1: f32 = wall_start.x;
        let wall_y1: f32 = wall_start.y;
        let wall_x2: f32 = wall_end.x;
        let wall_y2: f32 = wall_end.y;
        let ray_x1: f32 = self.start_point.x;
        let ray_y1: f32 = self.start_point.y;
        let ray_x2: f32 = self.end_point.x;
        let ray_y2: f32 = self.end_point.y;

        let denominator: f32 = (wall_x1 - wall_x2) * (ray_y1 - ray_y2) - (wall_y1 - wall_y2) * (ray_x1 - ray_x2);

        // Parallel or coincident lines.
        if denominator == 0.0 {
            return;
        }

        let t: f32 = ((wall_x1 - ray_x1) * (ray_y1 - ray_y2) - (wall_y1 - ray_y1) * (ray_x1 - ray_x2)) / denominator;
        let u: f32 = ((wall_x1 - wall_x2) * (wall_y1 - ray_y1) - (wall_y1 - wall_y2) * (wall_x1 - ray_x1)) / denominator;

        if t > 0.0 && t < 1.0 && u < 0.0 {
            let x_intersect: f32 = wall_x1 + t * (wall_x2 - wall_x1);
            let y_intersect: f32 = wall_y1 + t * (wall_y2 - wall_y1);

            // Distance formula: √((x_2-x_1)²+(y_2-y_1)²)
            let curr_dist: f32 = ((self.end_point.x - self.start_point.x).powf(2.0) + (self.end_point.y - self.start_point.y).powf(2.0)).sqrt();
            let new_dist: f32 = ((x_intersect - self.start_point.x).powf(2.0) + (y_intersect - self.start_point.y).powf(2.0)).sqrt();

            if !self.intersecting || new_dist < curr_dist {
                self.end_point.x = x_intersect;
                self.end_point.y = y_intersect;
                self.intersecting = true;
            }
        }
    }

    pub fn reset_end_point(&mut self) -> () {
        self.end_point = self.start_point + Vec2::new(INITIAL_LENGTH * self.angle.cos(), INITIAL_LENGTH * self.angle.sin());
    }
}