use std::thread::{Thread, self};
use std::time;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::libc::{sleep, __timeval};
use sdl2::{self, pixels};
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
mod circle;
use  circle::Circle;
fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video = sdl_ctx.video().unwrap();
    let window = video
        .window("rust-physics", 800, 600)
        .position_centered()
        .opengl()
        .build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut events = sdl_ctx.event_pump().unwrap();
    let mut lastx = 0;
    let mut lasty=0;
    'main: loop {
        let color = pixels::Color::RGB(0, 0, 255);
                    let mut new_circ = Circle::new();
                    new_circ.x_pos=400 as u128;
                    new_circ.y_pos=0 as u128;
                    new_circ.draw(&mut canvas, color);
                    canvas.present();

        thread::sleep(time::Duration::from_secs(1));
        for event in events.poll_iter(){
            match event{
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    
                }
                _ => {}
            }
        }
        new_circ.calculate_displacement_and_redraw();

    }
}
