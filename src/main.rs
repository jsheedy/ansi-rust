extern crate termsize;
extern crate image;
extern crate num_complex;

use std::{thread, time, f64};

mod renderers;
mod effects;
mod color;
mod math;
mod utils;

fn home() -> String {
    let esc = "\x1b[";
    let result = format!("{}0;0H",esc);
    return result;
}

fn ansi_pixel(color: &color::Color) -> String {
    let esc = "\x1b[";
    let block = String::from("█");
    let result = format!("{}38;2;{};{};{}m{}",esc, color.r, color.g, color.b, block);
    return result;
}

fn size() -> (u16, u16) {
    let size = termsize::get();
    let mut rows = 0;
    let mut cols = 0;
    match size {
        Some(x) => {
            rows = x.rows;
            cols = x.cols;
        },
        None => println!("wat"),
    }
    return (rows, cols);
}

pub fn main() {

    renderers::sdl::init();

    // let mv = home();
    // let (rows, cols) = size();
    // let velotron = load_img("img/bike.jpg");
    // loop {
    //     print!("{}", mv);
    //     let mut vec = Vec::new();
    //     let phase = t() / 4.0;
    //     for row in 0..rows {
    //         for col in 0..cols {

    //             let u = col as f64 / cols as f64;
    //             let v = row as f64 / rows as f64;

    //             let color = julia(u, v, phase);
    //             // let color = plasma(u, v, phase);
    //             // let color = img(u, v, phase, &velotron);

    //             let p = ansi_pixel(&color);
    //             vec.push(p);
    //         }
    //     }
    //     print!("{}", vec.join(""));
    // }
}

