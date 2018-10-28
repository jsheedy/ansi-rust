extern crate termsize;
extern crate image;
extern crate num_complex;

use num_complex::Complex;
use std::{thread, time, f64};
use std::time::{SystemTime, UNIX_EPOCH};
use std::path::Path;


struct Color {
    r: u8,
    g: u8,
    b: u8
}

fn load_img(path: &str) {
    image::open(&Path::new(path)).ok().expect("Opening image failed");
    // let filtered = img.fliph();
}

fn julia(u: f64, v: f64, phase: f64) -> Color {

    // convert u,v 0,1 to -2,2
    let scale = 0.1 + 1.0 * (((3.0*phase).sin()*(2.0*phase).cos())+1.0);
    let cy = 2.0 * scale * v - scale + 0.5*phase.sin();
    let cx = 2.0 * scale * u - scale + 0.5*phase.cos();

    let max_iterations = 256;

    let mut z = Complex::new(cx, cy);
    // 0.7885e^(ia)
    let a = phase % (2.0 * f64::consts::PI);
    let c = 0.7885 * Complex::new(a.cos(), a.sin());

    let mut i = 0;

    for t in 0..max_iterations {
        if z.norm() > 2.0 {
            break
        }
        z = z * z + c;
        i = t;
    }

    let i = i as f64;
    let r = (0.9 * ((2.0*i+2.0).sin()+1.0)*127.0) as u8;
    let g = (0.7 * ((3.0*i+3.0).sin()+1.0)*127.0) as u8;
    let b = (1.0 * ((5.0*i+1.0).sin()+1.0)*127.0) as u8;
    Color{r,g,b}
}

fn home() -> String {
    let esc = "\x1b[";
    let result = format!("{}0;0H",esc);
    return result;
}

fn ansi_pixel(color: &Color) -> String {
    let esc = "\x1b[";
    let block = String::from("█");
    let result = format!("{}38;2;{};{};{}m{}",esc, color.r, color.g, color.b, block);
    return result;
}

fn t() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_secs() as f64 +
            since_the_epoch.subsec_nanos() as f64 / 1_000_000_000.0;
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

fn plasma(u: f64, v: f64, phase: f64) -> Color {
    let w0 = (phase + 1.0 * 8.0 * f64::consts::PI * u).sin();
    let w1 = (phase + 1.0 * 12.0 * f64::consts::PI * (u+v)).sin();
    let w2 = ((w0 + w1)+2.0*phase).sin();

    //thx https://www.bidouille.org/prog/plasma
    let cx = 1.0 * u + (phase/3.0 + 1.0).sin();
    let cy = 1.0 * v + (phase/1.0).sin();
    let w3 = ((1000.0*(cx.powf(2.0) + cy.powf(2.0)+1.0)).sqrt()).sin();

    let wf = w3 + w2 + w1 + w0;

    let r = 0.8 * 255.0 * ((wf).sin() + 1.0)/2.0;
    let g = 0.9 * 255.0 * ((wf+phase).sin() + 1.0)/2.0;
    let b = 0.7 * 255.0 * ((2.0*wf).sin() + 1.0)/2.0;

    Color{r: r as u8,g: g as u8,b: b as u8}
}

pub fn main() {
    let mv = home();
    let (rows, cols) = size();

    loop {
        print!("{}", mv);
        let mut vec = Vec::new();
        let phase = t() / 4.0;
        for row in 0..rows {
            for col in 0..cols {

                let u = col as f64 / cols as f64;
                let v = row as f64 / rows as f64;

                let color = julia(u, v, phase);
                // let color = plasma(u, v, phase);

                let p = ansi_pixel(&color);

                vec.push(p);
            }
        }
        print!("{}", vec.join(""));
        let delay = time::Duration::from_millis(5);
        thread::sleep(delay);
    }
}

