use crate::definitions::{MAX_VALUE_F};


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
    pub shoulder: f64
}

impl Default for HdCurveSettings {
    fn default() -> Self {
        Self {
            contrast: 3.0,
            exposure_bias: 0.0,
            toe: 1.0,
            shoulder: 1.0,
        }
    }
}

impl HdCurveSettings {
    pub fn apply(&self, v: u16) -> u16 {
        hd_curve(v, self)
    }
}

pub fn hd_curve(v: u16, settings: &HdCurveSettings) -> u16 {
    let x: f64 = (v as f64 / MAX_VALUE_F) * 2.0 - 1.0;
    // Normal sigmoid
    let s: f64 = 1.0 / (1.0 + (-settings.contrast * (x + settings.exposure_bias)).exp());
    let y_toe = s.powf(settings.toe);
    let y_shoulder = 1.0 - (1.0 -  y_toe).powf(settings.shoulder);

    (y_shoulder * MAX_VALUE_F) as u16
}