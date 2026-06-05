use crate::definitions::{Lut3D, HALD_IMAGE_SIZE};
use image::{ImageBuffer, Rgb, RgbImage};

/// 
///  Writes the LUT to a file with the given name and path
/// # Arguments 
/// 
/// * `fname`: the file path and name
/// * `lut`: the lut to write
/// 
/// returns: () 
///
/// 
pub fn write_lut(fname: &str, lut: Lut3D) {
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::from_raw(
        HALD_IMAGE_SIZE as u32,
        HALD_IMAGE_SIZE as u32,
        (&*lut.lut_map).to_vec()
    ).expect("Conversion failed");
    img.save(fname).expect("Can't save image");
}