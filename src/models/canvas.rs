use std::{
    fs::File,
    io::{Result, Write},
    ops::{Index, IndexMut},
};

use super::colors::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,

    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: Option<Color>) -> Canvas {
        Canvas {
            width,
            height,
            pixels: (0..height)
                .map(|_x| {
                    (0..width)
                        .map(|_x| color.unwrap_or(Color::new(0.0, 0.0, 0.0)))
                        .collect()
                })
                .collect(),
        }
    }

    pub fn write(&mut self, x: usize, y: usize, value: &Color) {
        self.pixels[y][x] = *value;
    }

    pub fn save(&self) -> Result<()> {
        let mut output = File::create("image.ppm")?;
        let mut content = format!("P3\n{} {}\n255\n", self.width, self.height);
        content += self
            .pixels
            .iter()
            .map(|row| {
                row.iter()
                    .map(|pixel| pixel.format())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n")
            .as_str();
        output.write(content.as_bytes())?;
        Ok(())
    }
}
