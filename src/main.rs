extern crate image;

mod canvas;
mod model;
mod vec3;

use std::time::{Instant};
use canvas::Canvas;
use model::Model;

const WIDTH: usize = 800;
const HEIGHT: usize = 800;

fn main() {
    let start = Instant::now();

    let mut canvas = Canvas::new(WIDTH as u32, HEIGHT as u32);

    // canvas.line(100, 100, 200, 200);
    // canvas.line(100, 100, 200, 0);
    // canvas.line(100, 100, 0, 0);

    // let head = Model::load("./models/head.obj");

    // for face in head.faces {
    //     for i in 0..3 {
    //         let v0 = match head.vertexes.get(face[i] as usize) {
    //             Some(value) => { value },
    //             None => {
    //                 println!("panic at {}", face[i]);
    //                 &[0.0, 0.0, 0.0]
    //             }
    //         };
    //         let v1 = match head.vertexes.get(face[(i + 1) % 3] as usize) {
    //             Some(value) => { value },
    //             None => {
    //                 println!("panic at {}", face[i]);
    //                 &[0.0, 0.0, 0.0]
    //             }
    //         };

    //         let x0 = ((v0[0] + 1.0) * WIDTH as f32 / 2.0) as u32;
    //         let y0 = ((v0[1] + 1.0) * HEIGHT as f32 / 2.0) as u32;
    //         let x1 = ((v1[0] + 1.0) * WIDTH as f32 / 2.0) as u32;
    //         let y1 = ((v1[1] + 1.0) * HEIGHT as f32 / 2.0) as u32;

    //         canvas.line(x0, y0, x1, y1);
    //     }
    // }

    canvas.triangle((10, 70), (50, 160), (70, 80));
    canvas.triangle((180, 50), (150, 1), (70, 180));
    canvas.triangle((180, 150), (120, 160), (130, 180));

    let duration = start.elapsed();
    
    println!("Render time: {}us", duration.as_micros());
    canvas.save_picture("test.png");
}
