extern crate sdl2;

use renderers::sdl::sdl2::pixels::Color;
use renderers::sdl::sdl2::event::Event;
use renderers::sdl::sdl2::rect::Rect;
use renderers::sdl::sdl2::rect::Point;

use renderers::sdl::sdl2::keyboard::Keycode;

use std::time::Duration;

use utils;
use math;

pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let width = 800;
    let height = 600;
    let window = video_subsystem.window("l33t", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_pump.pump_events();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();

    'mainloop: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }

        }

        let t = utils::t();
        let r = (math::sin(t) * 255.0) as u8;
        let g = (math::cos(t*3.0) * 255.0) as u8;
        let b = 100;
        canvas.set_draw_color(Color::RGB(r,g,b));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255,0,0));
        let x = ((width / 2) + ((math::sin(t)*100.0) as u32)) as u32;
        let y = (height / 2) as u32;
        let size = 50 as u32;
        canvas.fill_rects(&[Rect::new(x as i32,y as i32,x+size,y+size)]).unwrap();
        // canvas.draw_line(Point::new(0,0), Point::new(500,500)).expect("WAT");
        canvas.present();
        // let ten_millis = Duration::from_millis(10);
        // ::std::thread::sleep(ten_millis);
    }
}
