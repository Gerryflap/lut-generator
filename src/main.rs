use crate::definitions::{generate_identity_lut, Lut3D};
use crate::operations::colour::{apply_to_lut, SensitiveLayer};
use crate::operations::tone_mapping::{simple_sigmoid, HdCurveSettings};
use crate::output::{write_lut};

pub mod definitions;
mod output;
mod operations;

fn main() {
    println!("Generating LUT");
    let mut lut: Lut3D = generate_identity_lut();

    println!("Applying layers and curves");
    // let curve = HdCurveSettings{
    //     contrast: 4.0,
    //     exposure_bias: 0.0,
    //     toe: 1.0,
    //     shoulder: 0.5,
    //     pivot: Some(1.0),
    // };

    let curve = HdCurveSettings{
        contrast: 2.38,
        exposure_bias: 0.38,
        toe: 3.0,
        shoulder: 0.72,
        pivot: Some(1.0),
    };

    let mut layers: Vec<SensitiveLayer> = Vec::new();

    // layers.push(SensitiveLayer{
    //         hue_min: 45.0,
    //         hue_max: 195.0,
    //         sensitivity: vec![1.0],
    //         output_colour: [1.0, 1.0, 1.0],
    //         density_multiplier: 1.0,
    //         response_curve: &curve,
    //     });


    // Red
    layers.push(SensitiveLayer{
        hue_min: -80.0,
        hue_max: 80.0,
        sensitivity: vec![0.3, 0.7, 1.0, 1.0, 1.0, 1.0, 0.7, 0.3],
        output_colour: [1.0, 0.0, 0.0],
        density_multiplier: 1.0,
        response_curve: &curve,
    });

    // Green
    layers.push(SensitiveLayer{
        hue_min: 40.0,
        hue_max: 200.0,
        sensitivity: vec![0.3, 0.7, 1.0, 1.0, 1.0, 1.0, 0.7, 0.3],
        output_colour: [0.0, 1.0, 0.0],
        density_multiplier: 1.0,
        response_curve: &curve,
    });

    // Blue
    layers.push(SensitiveLayer{
        hue_min: 160.0,
        hue_max: 320.0,
        sensitivity: vec![0.3, 0.7, 1.0, 1.0, 1.0, 1.0, 0.7, 0.3],
        output_colour: [0.0, 0.0, 1.0],
        density_multiplier: 1.0,
        response_curve: &curve,
    });

    lut = apply_to_lut(&layers, lut);

    println!("Writing LUT");
    write_lut("/run/media/gerben/LinuxSpeedyData/haldclut_film_sim/HaldCLUT/test/lut.png", lut);
    println!("Done :3");
}
