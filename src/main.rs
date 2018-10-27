use std::{thread, time, f64};
extern crate termsize;


fn home() -> String {
    let esc = "\x1b[";
    let result = format!("{}0;0H",esc);
    return result;
}

fn rgbpixel(r: u8, g: u8, b: u8) -> String {
    let esc = "\x1b[";
    let block = String::from("█");
    let result = format!("{}38;2;{};{};{}m{}",esc,r,g,b,block);
    return result;
}

fn pixel(i: u8) -> String {
    let esc = "\x1b[";
    let block = String::from("█");
    let result = format!("{}38;5;{}m{}",esc,i,block);
    return result;
}

pub fn main() {

    let mut rows = 20;
    let mut cols = 20;

    let size = termsize::get();
    match size {
        Some(x) => {
            rows = x.rows;
            cols = x.cols;
        },
        None => println!("wat"),
    }

    let mv = home();
    let modulus = 255;

    let mut i = 0;
    loop {
        print!("{}", mv);
        i += 1;
        let mut vec = Vec::new();
        for row in 0..rows {
            for col in 0..cols {

                let phase = i as f64 / 20.0;

                let u = col as f64 / cols as f64;
                let v = row as f64 / rows as f64;

                let w0 = ((phase + 1.0 * 8.0 * f64::consts::PI * u).sin() + 1.0) / 2.0;
                let w1 = ((phase + 1.0 * 12.0 * f64::consts::PI * v).sin() + 1.0) / 2.0;

                let w2 = (((w0 + w1)+2.0*phase).sin() + 1.0) / 2.0;
                //thx https://www.bidouille.org/prog/plasma
                let cx = 0.5 * (u + ((phase/5.0).sin()+1.0)/2.0);
                let cy = 0.5 * (v + ((phase/3.0).sin() + 1.0)/2.0);
                let w3 = (200.0*(cx.powf(2.0) + cy.powf(2.0)+1.0)).sqrt() + 2.0*phase;

                let wf = w3 + w2 + w1 + w0;
                // let wf = w2;
                // let wf = w1;
                // let wf = w0;

                // let r = 1.0 * 255.0 * wf;
                // let g = 1.0 * 255.0 * wf;
                // let b = 1.0 * 255.0 * wf;
                let r = 1.0 * 255.0 * ((wf * 1.0).sin() + 1.0)/2.0;
                let g = 1.0 * 255.0 * ((wf * 5.0).sin() + 1.0)/2.0;
                let b = 1.0 * 255.0 * ((wf * 10.0).sin() + 1.0)/2.0;
                let p = rgbpixel(r as u8, g as u8, b as u8);

                // print!("{}", p);
                vec.push(p);
            }
        }
        print!("{}", vec.join(""));
        let delay = time::Duration::from_millis(5);
        thread::sleep(delay);

    }
    //let result = format!("{}{}{}{}{}{}",blue,block,hello,p,"WAT",rst);
    // println!([blue, "Hello World!"].join("\n"));
}

