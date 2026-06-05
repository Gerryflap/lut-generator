use crate::definitions::{Lut3D, MAX_VALUE_F};


pub fn simple_sigmoid_op(v: u16, contrast: f64) -> u16 {
    let vf: f64 = (v as f64 / MAX_VALUE_F) * 2.0 - 1.0;
    let of = 1.0 / (1.0 + (-vf * contrast).exp());

    (of * MAX_VALUE_F) as u16
}
pub fn simple_sigmoid(lut_in: Lut3D, contrast: f64) -> Lut3D {
    let lut_map_out: Vec<u16> = lut_in.lut_map.iter().map(
        |x| simple_sigmoid_op(*x, contrast)
    ).collect();

    Lut3D{
        lut_map: lut_map_out,
    }
}