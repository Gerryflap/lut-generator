use crate::definitions::{generate_identity_lut, Lut3D};
use crate::operations::{simple_sigmoid, HdCurveSettings};
use crate::output::{write_lut};

pub mod definitions;
mod output;
mod operations;

fn main() {
    println!("Generating LUT");
    let mut lut: Lut3D = generate_identity_lut();

    println!("Applying hd curve per-channel");
    let curve = HdCurveSettings{
        contrast: 3.0,
        exposure_bias: 0.0,
        toe: 1.0,
        shoulder: 0.7,
        ..Default::default()
    };

    lut = lut.apply_operation(|x| curve.apply(x));

    println!("Writing LUT");
    write_lut("/run/media/gerben/LinuxSpeedyData/haldclut_film_sim/HaldCLUT/test/lut.png", lut);
    println!("Done :3");
}
