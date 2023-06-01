use std::collections::HashSet;
use std::time::Instant;
use ggez::{Context, event, GameError, GameResult};
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use crate::camera::{Camera, Direction};
use crate::map::Map;

pub struct Raycaster {
    camera: Camera,
    map: Map,
    input_set: HashSet<KeyCode>,
    last_update: Instant,
    paused: bool,
    game_over: bool
}

impl Raycaster {
    pub fn new(ctx: &Context) -> Self {
        return Raycaster {
            camera: Camera::new(ctx),
            map: Map::new(ctx),
            input_set: HashSet::new(),
            last_update: Instant::now(),
            paused: false,
            game_over: false
        }
    }

    fn handle_input(&mut self, ctx: &Context, dt: &f32) -> () {
        for key in &self.input_set {
            match key {
                KeyCode::Up => self.camera.move_camera(ctx, Direction::UP, dt),
                KeyCode::Down => self.camera.move_camera(ctx, Direction::DOWN, dt),
                KeyCode::Left => self.camera.move_camera(ctx, Direction::LEFT, dt),
                KeyCode::Right => self.camera.move_camera(ctx, Direction::RIGHT, dt),
                _ => ()
            }
        }
    }

    fn update_rays(&mut self, ctx: &Context) -> () {
        for ray in &mut self.camera.rays {
            // Reset each ray at the start of the frame.
            ray.intersecting = false;
            ray.reset_end_point();
            for wall in &mut self.map.walls {
                ray.update_ray(ctx, self.camera.position, wall.start_point, wall.end_point);
            }
        }
    }
}

impl event::EventHandler<GameError> for Raycaster {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let now: Instant = Instant::now();
        let dt: f32 = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.paused || self.game_over {
            return Ok(());
        }

        self.handle_input(ctx, &dt);

        self.update_rays(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.camera.draw(&mut canvas);

        self.map.draw(&mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeated: bool) -> Result<(), GameError> {
        let register_actions: bool = !self.paused && !self.game_over;

        if let Some(key) = input.keycode {
            if register_actions && (key == KeyCode::Up || key == KeyCode::Down || key == KeyCode::Left || key == KeyCode::Right) {
                self.input_set.insert(key);
            }
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, input: KeyInput) -> Result<(), GameError> {
        if let Some(key) = input.keycode {
            self.input_set.remove(&key);
        }
        Ok(())
    }
}