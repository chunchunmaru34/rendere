use image::{ImageBuffer, Rgb};

pub struct Canvas {
    buffer: ImageBuffer<Rgb<u8>, Vec<u8>>,
    background_color: Rgb<u8>,
    current_color: Rgb<u8>
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            buffer: ImageBuffer::new(width + 1, height + 1),
            background_color: Rgb([0, 0, 0]),
            current_color: Rgb([255, 255, 255]),
        }
    }

    pub fn save_picture(&self, path: &str) {
        self.buffer.save(path).unwrap();
    }

    pub fn set_color(&mut self, color: Rgb<u8>) {
        self.current_color = color;
    }

    pub fn line(&mut self, mut x0: u32, mut y0: u32, mut x1: u32, mut y1: u32) {
        let mut steep = false;
        if (x0 as i32 - x1 as i32).abs() < (y0 as i32 - y1 as i32).abs() {
            steep = true;
            let (x0_temp, x1_temp) = (x0, x1);
            x0 = y0;
            x1 = y1;
            y0 = x0_temp;
            y1 = x1_temp;
        }

        if x0 > x1 {
            let (x0_temp, y0_temp) = (x0, y0);
            x0 = x1;
            y0 = y1;
            x1 = x0_temp;
            y1 = y0_temp;
        }

        for x in x0..x1 {
            let t = (x - x0) as f32 / (x1 - x0) as f32;
            let y = y0 as f32 * (1.0 - t) + y1 as f32 * t;
            
            if steep {
                self.buffer.put_pixel(y as u32, x as u32, self.current_color);
            } else {
                self.buffer.put_pixel(x as u32, y as u32, self.current_color);
            }
        }
    }
}