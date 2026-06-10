use crate::renderer::color::ColorRGB;
use std::io;

#[derive(Debug, Clone)]
pub struct Bmp {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
}

impl Bmp {
    #[allow(dead_code)]
    pub fn new(width: i32, height: i32, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn highlight_bmp(&mut self, color: ColorRGB) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = ((y * self.width + x) * 3) as usize;

                let r = self.data[index];
                let g = self.data[index + 1];
                let b = self.data[index + 2];

                if !(r == color.r && g == color.g && b == color.b) {
                    self.data[index] = 255;
                    self.data[index + 1] = 255;
                    self.data[index + 2] = 255;
                } else {
                    self.data[index] = 0;
                    self.data[index + 1] = 0;
                    self.data[index + 2] = 0;
                }
            }
        }
    }

    pub fn scale_up(self, scale: u32) -> Self {
        let scale_i32 = scale as i32;
        let new_width = self.width * scale_i32;
        let new_height = self.height * scale_i32;

        let mut scaled_data: Vec<u8> =
            Vec::with_capacity(new_width as usize * new_height as usize * 3);

        for y in 0..new_height {
            for x in 0..new_width {
                let src_x = x / scale_i32;
                let src_y = y / scale_i32;

                let src_index = ((src_y * self.width + src_x) * 3) as usize;

                scaled_data.push(self.data[src_index]); // R
                scaled_data.push(self.data[src_index + 1]); // G
                scaled_data.push(self.data[src_index + 2]); // B
            }
        }

        Bmp {
            width: new_width,
            height: new_height,
            data: scaled_data,
        }
    }
}

pub fn read_bmp(path: &str) -> io::Result<Bmp> {
    let img = bmp::open(path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to open BMP file: {:?}", e),
        )
    })?;

    let width = img.get_width() as i32;
    let height = img.get_height() as i32;
    let mut data = Vec::with_capacity((width * height * 3) as usize);

    for y in 0..img.get_height() {
        for x in 0..img.get_width() {
            let pixel = img.get_pixel(x, y);
            data.push(pixel.r);
            data.push(pixel.g);
            data.push(pixel.b);
        }
    }

    Ok(Bmp {
        width,
        height,
        data,
    })
}
