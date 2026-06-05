use crate::definitions::{generate_identity_lut, Lut3D};
use crate::operations::simple_sigmoid;
use crate::output::{write_lut};

pub mod definitions;
mod output;
mod operations;

fn main() {
    println!("Generating LUT");
    let mut lut: Lut3D = generate_identity_lut();

    println!("Applying sigmoid per-channel");
    lut = simple_sigmoid(lut, 3.0);

    println!("Writing LUT");
    write_lut("/run/media/gerben/LinuxSpeedyData/haldclut_film_sim/HaldCLUT/test/lut.png", lut);
    println!("Done :3");
}
