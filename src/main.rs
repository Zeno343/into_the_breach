use std::{
    collections::HashMap,
    ops::{
        Index,
        IndexMut,
    },
};

use sdl2::pixels::Color;
use sdl2::rect::{
    Point,
    Rect,
};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
 
pub struct Grid {
    data: Vec<Option<Box<Material>>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let mut data = Vec::new();
        for _ in 0 .. width * height {
            data.push(None);
        }

        Grid {
            data,
            width,
            height
        }
    }

    pub fn update(&mut self) {
        let mut new_grid = Grid::new(self.width, self.height);

        for (idx, cell) in self.data.iter().enumerate() {
            if let Some(material) = cell {
                let x = (idx % self.width) as i32;
                let y = (idx / self.width) as i32;

                let position = Point::new(x, y);
                let new_position = material.update(self, position);
                new_grid[new_position] = self[position].clone();
            }
        }
        *self = new_grid;
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        for (idx, cell) in self.data.iter().enumerate() {
            if let Some(material) = cell {
                let x = (idx % self.width) as i32;
                let y = (idx / self.width) as i32;

                canvas.set_draw_color(material.color());
                canvas.draw_point(Point::new(x, y));
            }
        }
    }
}

impl Index<Point> for Grid {
    type Output = Option<Box<Material>>;

    fn index(&self, point: Point) -> &Self::Output {
        let idx = point.y as usize * self.width + point.x as usize; 

        &self.data[idx]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, point: Point) -> &mut Self::Output {
        let idx = point.y as usize * self.width + point.x as usize; 

        &mut self.data[idx]
    }
}

#[derive(Clone, Copy)]
pub struct Sand;

impl Material for Sand {
    fn update(&self, grid: &Grid, position: Point) -> Point {
        position
    }
    
    fn color(&self) -> Color {
        Color::RGB(198, 178, 128)
    }
}

pub trait Material: MaterialClone {
    fn update(&self, grid: &Grid, position: Point) -> Point;

    fn color(&self) -> Color;
}

pub trait MaterialClone {
    fn clone_box(&self) -> Box<Material>;
}

impl<T: 'static + Material + Clone> MaterialClone for T {
    fn clone_box(&self) -> Box<Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<Material> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let mut grid = Grid::new(WIDTH, HEIGHT);

    let window = video_subsystem.window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        let mouse = event_pump.mouse_state();
        if(mouse.left()) {
            let cursor = Point::new(mouse.x(), mouse.y());
            grid[cursor] = Some(Box::new(Sand));
        }
        // The rest of the game loop goes here...

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        grid.draw(&mut canvas);
        canvas.present();
    }
}
