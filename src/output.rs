use crate::definitions::{Lut3D, HALD_IMAGE_SIZE, MAX_VALUE_F};
use image::{ImageBuffer, Rgb};


pub fn to_float(v: u16) -> f64 {
    v as f64 / MAX_VALUE_F
}

pub fn to_uint_and_clip(v: f64) -> u16 {
    (v.clamp(0.0, 1.0) * MAX_VALUE_F).round() as u16
}

pub fn apply_srgb_gamma_correction(v: f64) -> f64 {
    if v < 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

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

    let lut_srgb: Vec<u16> = lut.lut_map.into_iter()
        .map(apply_srgb_gamma_correction)
        .map(to_uint_and_clip)
        .collect();

    let img: ImageBuffer<Rgb<u16>, Vec<u16>> = ImageBuffer::from_raw(
        HALD_IMAGE_SIZE as u32,
        HALD_IMAGE_SIZE as u32,
        lut_srgb
    ).expect("Conversion failed");
    img.save(fname).expect("Can't save image");
}