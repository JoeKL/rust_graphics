use crate::{
    types::color::ColorRGB,
    utils::bmp::{read_bmp, BMP},
};

use super::FrameBuffer;

pub struct FontProvider {
    pub font_file: BMP,
    pub letter_width: u32,
    pub letter_height: u32,
}

impl FontProvider {
    pub fn new(font_file_path: &str, channels: u32, letter_width: u32, letter_height: u32) -> Self {
        let font_file = read_bmp(font_file_path, channels).unwrap();
        Self {
            font_file,
            letter_width,
            letter_height,
        }
    }

    pub fn draw_font_file(&mut self, framebuffer: &mut FrameBuffer, x_pos: i32, y_pos: i32) {
        for y in (1..self.font_file.height).rev() {
            for x in 1..self.font_file.width {
                let index =
                    ((y * self.font_file.width + x) * self.font_file.channels as i32) as usize;

                let r = self.font_file.data[index];
                let g = self.font_file.data[index + 1];
                let b = self.font_file.data[index + 2];

                framebuffer.set_pixel(x + x_pos, y + y_pos, ColorRGB::from_rgb(r, g, b));
            }
        }
    }

    pub fn draw_bmp(&mut self, bmp: &BMP, framebuffer: &mut FrameBuffer, x_pos: i32, y_pos: i32) {
        for y in (1..bmp.height).rev() {
            for x in 1..bmp.width {
                let index = ((y * bmp.width + x) * bmp.channels as i32) as usize;

                let r = bmp.data[index];
                let g = bmp.data[index + 1];
                let b = bmp.data[index + 2];

                framebuffer.set_pixel(x + x_pos, y + y_pos, ColorRGB::from_rgb(r, g, b));
            }
        }
    }

    pub fn draw_as_character(
        &mut self,
        bmp: &BMP,
        framebuffer: &mut FrameBuffer,
        x_pos: i32,
        y_pos: i32,
        color: ColorRGB,
    ) {
        for y in (1..bmp.height).rev() {
            for x in 1..bmp.width {
                let index = ((y * bmp.width + x) * bmp.channels as i32) as usize;

                if bmp.data[index] == 255
                    && bmp.data[index + 1] == 255
                    && bmp.data[index + 2] == 255
                {
                    framebuffer.set_pixel(x + x_pos, y + y_pos, color);
                }
            }
        }
    }

    pub fn get_character(&mut self, x_idx: u32, y_idx: u32) -> BMP {
        let lw = self.letter_width as usize;
        let lh = self.letter_height as usize;
        let atlas_w = self.font_file.width as usize;

        let start_x = x_idx as usize * lw;
        let start_y = y_idx as usize * lh;

        let mut letter_data: Vec<u8> = Vec::with_capacity(lw * lh * 3);

        for y in start_y..(start_y + lh) {
            for x in start_x..(start_x + lw) {
                let index = (y * atlas_w + x) * 3;
                if index + 2 < self.font_file.data.len() {
                    letter_data.push(self.font_file.data[index]); // R
                    letter_data.push(self.font_file.data[index + 1]); // G
                    letter_data.push(self.font_file.data[index + 2]); // B
                } else {
                    // Safety fallback (push black) if we go out of bounds
                    letter_data.push(0);
                    letter_data.push(0);
                    letter_data.push(0);
                }
            }
        }

        BMP {
            width: self.letter_width as i32,
            height: self.letter_height as i32,
            data: letter_data,
            channels: self.font_file.channels,
        }
    }

    pub(crate) fn get_glyph_grid_pos(&self, c: char) -> (u32, u32) {
        let ascii_val = c as u32;

        // If we get a control code (<32), map it to Space (0,0) or a "missing" symbol.
        if ascii_val < 32 {
            return (0, 0);
        }

        // 2. Flatten the index (0 = Space, 1 = '!', etc.)
        let index = ascii_val - 32;

        // 3. Calculate Grid Position (16 columns per row)
        let grid_x = index % 16;
        let grid_y = index / 16;

        (grid_x, grid_y)
    }
}
