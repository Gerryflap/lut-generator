use crate::definitions::{generate_identity_lut, Lut3D};
use crate::output::write_lut;

pub mod definitions;
mod output;

fn main() {
    println!("Generating LUT");
    let lut: Lut3D = generate_identity_lut();
    println!("Writing LUT");
    write_lut("lut.png", lut);
    println!("Done :3");
}
