
use num_traits::float::FloatConst;
use crate::definitions::Lut3D;
use crate::operations::tone_mapping::{to_float, to_uint_and_clip, HdCurveSettings};

pub struct SensitiveLayer<'a> {
    // Lowest hue at which there's some response
    pub(crate) hue_min: f64,
    // Highest hue at which there's some response
    pub(crate) hue_max: f64,
    // Sensitivity across the spectrum, divided in bins
    // You can add however many entries you'd like, the bins will be divided over the range
    pub(crate) sensitivity: Vec<f64>,

    // Dye colour, inverse of the final output colour
    pub(crate) dye_colour: [f64; 3],

    // Scale the final response with this value
    pub(crate) density_multiplier: f64,

    // The response curve for this layer
    pub(crate) response_curve: &'a HdCurveSettings
}

impl SensitiveLayer<'_> {

    fn compute_rgb_output(&self, r: f64, g: f64, b: f64) -> [f64; 3] {
        let (hue, purity) = compute_hue_and_purity(r, g, b);
        let raw_out = self.compute_raw_activation(hue, purity);
        let response = self.response_curve.apply(raw_out);
        self.dye_colour.map(|v| v * response * self.density_multiplier)
    }

    fn compute_raw_activation(&self, hue: f64, purity: f64) -> f64 {
        let step_size = (self.hue_max - self.hue_min) / self.sensitivity.len() as f64;

        let mut activation: f64 = 0.0;
        for bin in 0..self.sensitivity.len() {
            let sensitivity = self.sensitivity[bin];
            let h_min = self.hue_min * bin as f64 * step_size;
            let h_max = self.hue_min * (bin + 1) as f64 * step_size;
            activation += sensitivity * compute_response_between(h_min, h_max, hue, purity)
        }
        println!("{}", activation);
        activation
    }
}

pub fn apply_to_lut(layers: &Vec<SensitiveLayer>, lut: Lut3D) -> Lut3D {
    lut.apply_rgb_operation(|r, g, b| {
        let mut ro: f64 = 0.0;
        let mut go: f64 = 0.0;
        let mut bo: f64 = 0.0;

        let rf: f64 = to_float(r);
        let gf: f64 = to_float(g);
        let bf: f64 = to_float(b);

        for layer in layers {
            let layer_rgb_out = layer.compute_rgb_output(rf, gf, bf);
            ro += layer_rgb_out[0];
            go += layer_rgb_out[1];
            bo += layer_rgb_out[2];
        }
        (to_uint_and_clip(ro), to_uint_and_clip(go), to_uint_and_clip(bo))
    })
}

///
///
/// # Arguments
///
/// * `angle`: Angle in degrees
///
/// returns: the angle, normalized to the range -180 to 180
///
fn normalize(angle: f64) -> f64 {
    (angle + 360.0 + 180.0) % 360.0 - 180.0
}

fn compute_hue_and_purity(r: f64, g: f64, b: f64) -> (f64, f64) {
    let yc: f64 = f64::sin(2.0 * f64::PI() / 3.0);

    let xv: f64 = r - 0.5 * (g + b);
    let yv: f64 = yc * (g - b);

    // Compute the hue angle in degrees, defaulting to 0.0 if there is no hue
    let angle = if xv == 0.0 && yv == 0.0 {
        0.0
    }   else {
        f64::atan2(yv, xv) * 360.0 / f64::TAU()
    };

    let purity:f64 = if r == 0.0 && g == 0.0 && b == 0.0 {
        // Catch this case, otherwise we'll get a division by 0
        0.0
    } else {
        let mut order: [f64; 3] = [r, g, b];
        // Sort values to compute purity
        order.sort_by(f64::total_cmp);
        // Purity increases as lowest of the 3 channels gets lower proportional to the avg of the
        //  other 2.
        // This way, we can have pure orange, cyan, or purple
        // It's only the 3rd channel that reduces the purity
        1.0 - (order[0] / (0.5 * (order[1] + order[2])))
    };

    (angle, purity)
}

fn width_at_purity(purity: f64) -> f64 {
    if purity > 0.5 {
        1.0 + 359.0 * 2.0 * (1.0 - purity)
    } else {
        360.0
    }
}

fn base_height_at_purity(purity: f64) -> f64 {
    if purity < 0.5 {
        ((0.5 - purity) / 0.5) * purity
    } else {
        0.0
    }
}

fn response_at(hue: f64, h_center: f64, purity: f64) -> f64 {
    let width = width_at_purity(purity);
    let height = 1.0 / width;
    // "base" height, height at the lowest point of the response curve
    let base = base_height_at_purity(purity);
    // Height of the slope above the base height
    let slope_height = height - base;
    // Distance in hue angle
    let d = normalize(hue - h_center).abs();
    let x = if d < width {
        1.0 - (d/width)
    } else {
        0.0
    };

    slope_height * x + base
}

fn slope_width(h_min: f64, h_max: f64, hue: f64, purity: f64, width: f64) -> f64 {
    let pwidth = width_at_purity(purity);

    let h_upper_n = normalize(hue + pwidth - h_min);
    let h_lower_n = normalize(hue - pwidth - h_min);
    let h_max_n = normalize(h_max - h_min);

    if 0.0 < h_upper_n && h_upper_n < h_max_n {
        // Right point of the purity activation triangle is within bound
        h_upper_n
    } else if 0.0 < h_lower_n && h_lower_n < h_max_n {
        h_max_n - h_lower_n
    } else {
        width
    }
}

fn compute_response_between_inner(h_min: f64, h_max: f64, hue: f64, purity: f64) -> f64 {
    let mut r_min = response_at(h_min, hue, purity);
    let mut r_max = response_at(h_max, hue, purity);

    if r_min > r_max {
        (r_min, r_max) = (r_max, r_min);
    }

    let width = normalize(h_max - h_min).abs();
    let swidth = slope_width(h_min, h_max, hue, purity, width);

    // Compute and return the area as sum of base area and slope area
    r_min * width + 0.5 * (r_max - r_min).abs() * swidth
}

fn compute_response_between(h_min: f64, h_max: f64, hue: f64, purity: f64) -> f64 {
    let h_n = normalize(hue - h_min);
    let h_max_n = normalize(h_max - h_min);
    if 0.0 < h_n && h_n < h_max_n {
        // Edge case, if the top of the hue triangle is in our bin, do left and right slope
        compute_response_between_inner(h_min, hue, hue, purity) +
            compute_response_between_inner(hue, h_max, hue, purity)
    } else {
        // Else, just compute one slope
        compute_response_between_inner(h_min, h_max, hue, purity)
    }
}

