use crate::definitions::{Lut3D, HALD_IMAGE_SIZE};
use image::{ImageBuffer, Rgb, RgbImage};

pub fn write_lut(fname: &str, lut: Lut3D) {
    let mut img: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(HALD_IMAGE_SIZE as u32, HALD_IMAGE_SIZE as u32);
    let mut index: usize;
    let mut rgb: Rgb<u8>;

    for i in 0..HALD_IMAGE_SIZE {
        for j in 0..HALD_IMAGE_SIZE {
            index = i + HALD_IMAGE_SIZE * j;
            rgb = Rgb([
                lut.lut_map[index],
                lut.lut_map[index + 1],
                lut.lut_map[index + 2]
            ]);

            img.put_pixel(i as u32, j as u32, rgb);
        }
        img.save(fname).unwrap();
    }
}