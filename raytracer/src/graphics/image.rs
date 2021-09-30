use super::ColorRGB;

pub struct Image {
    pixels: Vec<Vec<ColorRGB>>,
}

impl Image {
    pub fn from_size(height: usize, width: usize) -> Image {
        if height == 0 || width == 0 {
            panic!("image height and width cannot be zero")
        }
        let mut pixels: Vec<Vec<ColorRGB>> = Vec::with_capacity(height);
        for _ in 0..height {
            let row: Vec<ColorRGB> = vec![ColorRGB(0.0, 0.0, 0.0); width];
            pixels.push(row);
        }
        Image { pixels }
    }

    pub fn height(&self) -> usize {
        self.pixels.len()
    }

    pub fn width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn save_as_ppm(&self, path: String) {
        use std::io::Write;
        let mut file = std::fs::File::create(path).unwrap();

        file.write_all(format!("P6\n{} {}\n255\n", self.width(), self.height()).as_bytes())
            .unwrap();

        let mut data: Vec<u8> = Vec::with_capacity(self.height() * self.width() * 3);
        for i in 0..self.height() {
            for j in 0..self.width() {
                let (r, g, b) = self[(i, j)].quantize();
                data.push(r);
                data.push(g);
                data.push(b);
            }
        }

        file.write_all(&data).unwrap();
    }
}

use std::ops::{Index, IndexMut};
impl Index<(usize, usize)> for Image {
    type Output = ColorRGB;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixels[index.0][index.1]
    }
}
impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[index.0][index.1]
    }
}
