use std::time::{self, SystemTime};

use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window, libc::NEW_TIME};
pub struct Circle {
    pub x_pos: f64,
    pub y_pos: f64,
    pub radius: i32,
    mass: i32,
    velocity: i32,
    last_time: SystemTime,
}

impl Circle {
    pub fn new() -> Self {
          Circle {
            x_pos: 0,
            y_pos: 0,
            radius: 100,
            mass: 5,
            velocity: 0,
            last_time: time::SystemTime::now(),
        }
        
    }
    pub fn draw(&mut self,canvas: &mut Canvas<Window>,color: Color){
        let _ = canvas.filled_circle(
            self.x_pos.into().unwrap(),
            self.y_pos.try_into().unwrap(),
            self.radius.try_into().unwrap(),
            color,
        );
        canvas.present();
    }
    pub fn calculate_displacement_and_redraw(&mut self){
        let new_time = time::SystemTime::now();
        let d_time = new_time.duration_since(self.last_time).unwrap();
        println!("{:?}", d_time);
        // self.last_time=new_time;

        self.y_pos+=dbg!(1/2)*9.8*d_time.as_secs() as u128* (d_time.as_secs() as u128);
        println!("self.y = {}",self.y_pos);
    }
}
