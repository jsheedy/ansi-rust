extern crate sdl2;

use sdl::sdl2::pixels::Color;
use sdl::sdl2::event::Event;
use sdl::sdl2::rect::Rect;
use sdl::sdl2::rect::Point;

use sdl::sdl2::keyboard::Keycode;

use std::time::Duration;

pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("l33t", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    event_pump.pump_events();
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.clear();
    canvas.present();
    let mut i = 0;

    'mainloop: loop {

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => break 'mainloop,
                _ => {}
            }

        }
        let i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i,255,255));
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255,0,0));
        canvas.fill_rects(&[Rect::new(0, 0, 100, 100)]).unwrap();
        canvas.set_draw_color(Color::RGB(0,0,255));
        canvas.draw_rect(Rect::new(200, 200, 100, 100)).unwrap();
        canvas.draw_line(Point::new(0,0), Point::new(500,500)).expect("WAT");

                  canvas.set_draw_color(Color::RGB(0,0,255));
        canvas.draw_rect(Rect::new(200, 200, 10, 10)).unwrap();
        canvas.present();

        let ten_millis = Duration::from_millis(10);
        ::std::thread::sleep(ten_millis);
    }
}
