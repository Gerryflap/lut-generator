use crate::definitions::{MAX_VALUE, MAX_VALUE_F};


pub fn simple_sigmoid(v: u16, contrast: f64) -> u16 {
    let vf: f64 = (v as f64 / MAX_VALUE_F) * 2.0 - 1.0;
    let of = 1.0 / (1.0 + (-vf * contrast).exp());

    (of * MAX_VALUE_F) as u16
}


pub struct HdCurveSettings {
    // Controls the contrast, 3 is quite low, 6 is quite high
    pub contrast: f64,
    // Offsets exposure (where exposure values run from -1.0 to 1.0)
    pub exposure_bias: f64,
    // Controls the toe (bottom end of the curve).
    // 1.0 is no change, towards 0 is less toe, above 1 is more. Don't use negative numbers.
    pub toe: f64,
    // Controls the shoulder (upper end of the curve).
    // 1.0 is no change, towards 0 is less shoulder, above 1 is more. Don't use negative numbers.
    pub shoulder: f64,
    // When defined, ensures that f(pivot) = pivot.
    // When pivot = 1.0, this keeps the function between 0 and 1 but might affect the perceived
    //  brightness.
    // When set to 0.5 or 0.18, you can more effectively control the mid-tones, but clipping may occur
    pub pivot: Option<f64>
}

impl Default for HdCurveSettings {
    fn default() -> Self {
        Self {
            contrast: 3.0,
            exposure_bias: 0.0,
            toe: 1.0,
            shoulder: 1.0,
            pivot: None
        }
    }
}

impl HdCurveSettings {
    pub fn apply(&self, v: f64) -> f64 {
        hd_curve(v, self)
    }

}

pub fn hd_curve(v: f64, settings: &HdCurveSettings) -> f64 {
    let y: f64 = hd_curve_internal(v, settings);
    let y_corr = match settings.pivot {
        None => {y}
        Some(p) => {y * (p / hd_curve_internal(p, settings))}
    };
    y_corr
}

fn hd_curve_internal(v: f64, settings: &HdCurveSettings) -> f64 {
    // Normal sigmoid
    let x = v * 2.0 - 1.0;
    let s: f64 = 1.0 / (1.0 + (-settings.contrast * (x + settings.exposure_bias)).exp());
    let y_toe = s.powf(settings.toe);
    let y_shoulder = 1.0 - (1.0 -  y_toe).powf(settings.shoulder);
    y_shoulder
}