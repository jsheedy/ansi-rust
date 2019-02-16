use std::{f64};
use std::path::Path;

use image::DynamicImage;
use image::GenericImageView;
use num_complex::Complex;

use color::{Color};
use math::{sin};

extern crate sdl2;


pub fn julia(u: f64, v: f64, phase: f64) -> Color {

    let phase = phase * 0.5;
    // convert u,v 0,1 to -2,2
    let scale = 0.2 + 0.4 * (((3.0*phase).sin()*(2.0*phase).cos())+1.0);
    let cy = 2.0 * scale * v - scale + 0.5*phase.sin();
    let cx = 2.0 * scale * u - scale + 0.5*phase.cos();

    let max_iterations = 127;

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
    let r = (0.9 * ((0.2*i+2.0).sin()+1.0)*127.0) as u8;
    let g = (0.7 * ((0.3*i+3.0).sin()+1.0)*127.0) as u8;
    let b = (1.0 * ((0.5*i+1.0).sin()+1.0)*127.0) as u8;
    Color{r,g,b}
}

fn load_img(path: &str) -> DynamicImage {
    image::open(&Path::new(path)).ok().expect("Opening image failed")
}

fn img(u: f64, v: f64, phase: f64, img: &DynamicImage) -> Color {
    let (w, h) = img.dimensions();

    let u = 0.7*sin(sin(-v*u*2.2+ 0.1 * phase)+1.5 * phase) + 0.3*sin(sin(u+v+phase)+1.3 * phase);
    let v = sin(sin(v*u+1.9 * phase)+1.1 * phase);

    let x = (u * w as f64) as u32;
    let y = (v * h as f64) as u32;

    let pixel = img.get_pixel(x, y);
    let r = pixel.data[0];
    let g = pixel.data[1];
    let b = pixel.data[2];
    Color{r,g,b}
}

pub fn plasma(u: f64, v: f64, phase: f64) -> Color {
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
