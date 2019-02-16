extern crate termsize;
extern crate image;
extern crate num_complex;

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

    renderers::terminal::init();
    // renderers::sdl::init();

}

