use super::*;

impl GridTerrain {
    pub fn as_gray(&self) -> GrayImage {
        let (w, h) = self.get_size();
        let mut out = GrayImage::new(w as u32, h as u32);
        for (x, y, pixel) in out.enumerate_pixels_mut() {
            let value = self.get_normed(self[(x, y)]) * 255.0;
            *pixel = image::Luma([value as u8]);
        }
        out
    }
}
