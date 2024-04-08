extern crate sdl2;

use core::ops::Add;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use rand::{thread_rng, Rng};
use sdl2::video::Window;
use std::time::Duration;

const GRID_X_SIZE: u32 = 101;
const GRID_Y_SIZE: u32 = 100;
const DOT_SIZE_IN_PXS: u32 = 5;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Game of life",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;
    let mut context = SimContext::new();

    let mut frame_counter = 0;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Space => context.toggle_pause(),
                    Keycode::Escape => context.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 65536));

        frame_counter += 1;
        if frame_counter % 10 == 0 {
            context.next_tick();
            frame_counter = 0;
        }
        renderer.draw(&context)?;
    }

    Ok(())
}

pub enum SimulationState {
    Playing,
    Paused,
}

#[derive(Copy, Clone, Debug)]
pub struct Point(pub i32, pub i32);

pub struct SimContext {
    pub points: [[bool; GRID_Y_SIZE as usize]; GRID_X_SIZE as usize],
    pub state: SimulationState,
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl SimContext {
    pub fn new() -> SimContext {
        let mut rng = thread_rng();
        let mut cells: [[bool; GRID_Y_SIZE as usize]; GRID_X_SIZE as usize] =
            [[false; GRID_Y_SIZE as usize]; GRID_X_SIZE as usize];

        for x in 0..GRID_X_SIZE {
            for y in 0..GRID_Y_SIZE {
                cells[x as usize][y as usize] = rng.gen_bool(0.5)
            }
        }

        SimContext {
            points: cells,
            state: SimulationState::Playing,
        }
    }
    pub fn next_tick(&mut self) {
        if let SimulationState::Paused = self.state {
            return
        }

        self.calculate_new_points()

    }
    pub fn get_value_at_point(&self, point_x: usize, point_y: usize) -> bool {


        if point_x > GRID_X_SIZE as usize || point_y > GRID_Y_SIZE as usize {
            return false
        }

        return self.points[point_x][point_y]
    }
    pub fn get_neighbours_at_point(&self, point_x: usize, point_y: usize) -> i32 {
        let mut sum = 0;
        if point_x > 0 && point_y > 0 {
            if self.get_value_at_point(point_x - 1, point_y - 1) { sum += 1 };
        }
        if point_x > 0 {
            if self.get_value_at_point(point_x - 1, point_y) { sum += 1 };
        }

        if point_x > 0 && point_y < (GRID_Y_SIZE - 1) as usize {
             if self.get_value_at_point(point_x - 1, point_y + 1) { sum += 1 };
        }

        if point_y < (GRID_Y_SIZE - 1) as usize {
            if self.get_value_at_point(point_x, point_y + 1) { sum += 1 };
        } 

        if point_x < (GRID_X_SIZE - 1 ) as usize && point_y > 0  {
           if self.get_value_at_point(point_x + 1, point_y - 1) { sum += 1 };
        }

        if point_x < (GRID_X_SIZE - 1 ) as usize {
           if self.get_value_at_point(point_x + 1, point_y) { sum += 1 };
        }

        if point_x < (GRID_X_SIZE - 1) as usize && point_y < (GRID_Y_SIZE - 1) as usize {
           if self.get_value_at_point(point_x + 1, point_y + 1) { sum += 1 };
        }

        if point_y > 0 {
            if self.get_value_at_point(point_x, point_y - 1) { sum += 1 };
        }
        
        return sum;
    }
    pub fn calculate_new_points(&mut self) -> () {
        let mut new_cells: [[bool; GRID_Y_SIZE as usize]; GRID_X_SIZE as usize] =
            [[false; GRID_Y_SIZE as usize]; GRID_X_SIZE as usize];

        for (x, col) in self.points.iter().enumerate() {
            for (y, _row) in col.iter().enumerate() {
                let count = self.get_neighbours_at_point(x, y);
                new_cells[x][y] = count == 3 || (self.points[x][y] && count == 2);  
            }
        }

        self.points = new_cells;
    }
    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            SimulationState::Playing => SimulationState::Paused,
            SimulationState::Paused => SimulationState::Playing,
        }
    }
}

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }
    fn draw_dot(&mut self, point: &Point) -> Result<(), String> {
        let Point(x, y) = point;
        self.canvas.fill_rect(Rect::new(
            x * DOT_SIZE_IN_PXS as i32,
            y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }
    pub fn draw(&mut self, context: &SimContext) -> Result<(), String> {
        self.draw_background(context);
        self.draw_sim(context)?;
        self.canvas.present();

        Ok(())
    }

    fn draw_background(&mut self, context: &SimContext) {
        let color = match context.state {
            SimulationState::Playing => Color::RGB(0, 0, 0),
            SimulationState::Paused => Color::RGB(30, 30, 30),
        };
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    fn draw_sim(&mut self, context: &SimContext) -> Result<(), String> {
        for (x, col) in context.points.iter().enumerate() {
            for (y, _row) in col.iter().enumerate() {
                let current_point = context.points[x][y];
                match current_point {
                    false => {},
                    true => {
                        self.canvas.set_draw_color(Color::WHITE);
                        self.draw_dot(&Point(x as i32, y as i32))?;
                    }
                };
            }
        }
        Ok(())
    }
}
