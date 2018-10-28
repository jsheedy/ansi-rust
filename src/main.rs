use std::{thread, time, f64};
use std::time::{SystemTime, UNIX_EPOCH};

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

                let w0 = (phase + 1.0 * 8.0 * f64::consts::PI * u).sin();
                let w1 = (phase + 1.0 * 12.0 * f64::consts::PI * (u+v)).sin();
                let w2 = ((w0 + w1)+2.0*phase).sin();

                //thx https://www.bidouille.org/prog/plasma
                let cx = 1.0 * u + (phase/3.0 + 1.0).sin();
                let cy = 1.0 * v + (phase/1.0).sin();
                let w3 = ((1000.0*(cx.powf(2.0) + cy.powf(2.0)+1.0)).sqrt()).sin();

                let wf = w3 + w2 + w1 + w0;
                //let wf = w3;
                // let wf = w2;
                // let wf = w1;
                // let wf = w0;

                //let r = 1.0 * 255.0 * wf;
                //let g = 1.0 * 255.0 * wf;
                //let b = 1.0 * 255.0 * wf;
                let r = 0.8 * 255.0 * ((wf).sin() + 1.0)/2.0;
                let g = 0.9 * 255.0 * ((wf+phase).sin() + 1.0)/2.0;
                let b = 0.7 * 255.0 * ((2.0*wf).sin() + 1.0)/2.0;
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

