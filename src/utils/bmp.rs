use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::str;
use std::{char, io};

use crate::types::color::ColorRGB;

#[derive(Debug)]
pub struct BMP {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>,
    pub channels: u32,
}

impl BMP {
    pub fn new(width: i32, height: i32, data: Vec<u8>, channels: u32) -> Self {
        Self {
            width,
            height,
            data,
            channels,
        }
    }

    pub fn highlight_bmp(&mut self, color: ColorRGB) {
        for y in (1..self.height).rev() {
            for x in 1..self.width {
                let index = ((y * self.width + x) * self.channels as i32) as usize;

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

        BMP {
            width: new_width,
            height: new_height,
            data: scaled_data,
            channels: self.channels,
        }
    }
}

pub fn read_bmp(path: &str, channels: u32) -> io::Result<BMP> {
    let mut file = File::open(path)?;

    // --- 1. The File Header (14 bytes) ---
    let mut file_header = [0u8; 14];
    file.read_exact(&mut file_header)?;

    // Check Signature 'BM' (0x42, 0x4D)
    if file_header[0] != 0x42 || file_header[1] != 0x4D {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a BMP file"));
    }

    // Offset to where pixel data starts is at bytes 10-13 (Little Endian)
    let pixel_offset = u32::from_le_bytes([
        file_header[10],
        file_header[11],
        file_header[12],
        file_header[13],
    ]) as u64;

    // --- 2. The DIB Header (Info Header) ---
    // We only read the first 40 bytes which is standard for Windows V3 BMPs
    let mut dib_header = [0u8; 40];
    file.read_exact(&mut dib_header)?;

    // Width is at bytes 4-7, Height is at bytes 8-11
    let width = i32::from_le_bytes([dib_header[4], dib_header[5], dib_header[6], dib_header[7]]);
    let height = i32::from_le_bytes([dib_header[8], dib_header[9], dib_header[10], dib_header[11]]);
    let bit_count = u16::from_le_bytes([dib_header[14], dib_header[15]]);

    // Basic validation for this specific parser
    if bit_count != 24 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Only 24-bit BMPs supported",
        ));
    }

    // --- 3. The Pixel Data ---
    // Jump to the pixel data offset we found earlier
    file.seek(SeekFrom::Start(pixel_offset))?;

    let row_size_bytes = (width * 3) as usize;
    // BMP rows are padded to multiples of 4 bytes
    let padding = (4 - (row_size_bytes % 4)) % 4;
    let stride = row_size_bytes + padding;

    let mut raw_data = Vec::new();

    // BMPs are stored Upside-Down (Bottom-to-Top) usually.
    // If height is negative, it's Top-to-Bottom. Here we assume standard Bottom-to-Top.
    let abs_height = height.abs();

    // We'll read the whole raw block first for efficiency
    let mut pixel_buffer = vec![0u8; (stride as i32 * abs_height) as usize];
    file.read_exact(&mut pixel_buffer)?;

    // Process rows to remove padding and reorder RGB
    for y in (0..abs_height).rev() {
        let row_start = (y as usize) * stride;
        let row_end = row_start + row_size_bytes;
        let row_pixels = &pixel_buffer[row_start..row_end];

        for chunk in row_pixels.chunks(3) {
            raw_data.push(chunk[2]); // R
            raw_data.push(chunk[1]); // G
            raw_data.push(chunk[0]); // B
        }
    }

    Ok(BMP {
        width,
        height,
        data: raw_data,
        channels,
    })
}
