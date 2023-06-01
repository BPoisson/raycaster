use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, Mesh};
use crate::constants::SCREEN_SIZE;
use crate::ray::Ray;

const CAMERA_RADIUS: f32 = 5.0;
const CAMERA_SPEED: f32 = 250.0;
const CAMERA_FIELD_OF_VIEW: f32 = 360.0;

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

pub struct Camera {
    circle_mesh: Mesh,
    pub position: Vec2,
    pub rays: Vec<Ray>
}

impl Camera {
    pub fn new(ctx: &Context) -> Self {
        let position: Vec2 = Vec2::new(SCREEN_SIZE.x / 2.0, SCREEN_SIZE.y / 2.0);

        let circle_mesh: Mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            position,
            CAMERA_RADIUS,
            0.1,
            Color::WHITE
        ).unwrap();

        let rays: Vec<Ray> = Camera::spawn_rays(&position, ctx);

        return Camera {
            circle_mesh,
            position,
            rays
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.draw(
            &self.circle_mesh,
            graphics::DrawParam::default()
        );

        for ray in &self.rays {
            ray.draw(canvas);
        }
    }

    pub fn move_camera(&mut self, ctx: &Context, direction: Direction, dt: &f32) -> () {
        match direction {
            Direction::UP => self.position.y -= CAMERA_SPEED * dt,
            Direction::DOWN => self.position.y += CAMERA_SPEED * dt,
            Direction::LEFT => self.position.x -= CAMERA_SPEED * dt,
            Direction::RIGHT => self.position.x += CAMERA_SPEED * dt,
        }

        self.circle_mesh = Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position,
            CAMERA_RADIUS,
            0.1,
            Color::WHITE
        ).unwrap();
    }

    fn spawn_rays(position: &Vec2, ctx: &Context) -> Vec<Ray> {
        let mut rays: Vec<Ray> = Vec::new();
        let mut angle: f32 = 0.0;
        let angle_increment: f32 = 10.0;
        let ray_length: f32 = 50.0;

        while angle < CAMERA_FIELD_OF_VIEW {
            let rotated_point: Vec2 = Self::rotate_point(ray_length, angle.to_radians());
            let end_point: Vec2 = *position + rotated_point;

            rays.push(Ray::new(ctx, position, &end_point, &angle.to_radians()));

            angle += angle_increment;
        }
        return rays;
    }

    fn rotate_point(line_length: f32, angle: f32) -> Vec2 {
        return Vec2::new(line_length * angle.cos(), line_length * angle.sin());
    }
}