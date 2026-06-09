pub const HALD_LVL: usize = 12;
pub const HALD_CUBE_SIZE: usize = HALD_LVL * HALD_LVL;
pub const HALD_IMAGE_SIZE: usize = HALD_LVL * HALD_LVL * HALD_LVL;

pub const HALD_ARRAY_SIZE: usize = HALD_IMAGE_SIZE * HALD_IMAGE_SIZE * 3;

pub const MAX_VALUE: u16 = u16::MAX;
pub const MAX_VALUE_F: f64 = MAX_VALUE as f64;

pub struct Lut3D {
    pub(crate) lut_map: Vec<f64>
}

impl Lut3D {
    pub fn apply_operation<F>(&self, operation: F) -> Self
    where
        F: Fn(f64) -> f64,
    {
        let lut_map_out: Vec<f64> = self.lut_map.iter().copied().map(operation).collect();

        Lut3D{
            lut_map: lut_map_out,
        }
    }

    pub fn apply_rgb_operation<F>(&self, operation: F) -> Self
    where
        F: Fn(f64, f64, f64) -> (f64, f64, f64),
    {
        let mut lut_map_out: Vec<f64> = vec![0.0; HALD_ARRAY_SIZE];

        for i in (0..HALD_ARRAY_SIZE).step_by(3) {
            let r = self.lut_map[i];
            let g = self.lut_map[i + 1];
            let b = self.lut_map[i + 2];
            let (ro, go, bo) = operation(r, g, b);
            lut_map_out[i + 0] = ro;
            lut_map_out[i + 1] = go;
            lut_map_out[i + 2] = bo;
        }

        Lut3D{
            lut_map: lut_map_out,
        }
    }

}

///
/// Generates an Identity Lut3D
pub fn generate_identity_lut() -> Lut3D {
    println!("Allocating...");
    let mut lut_map: Vec<f64> = vec![0.0; HALD_ARRAY_SIZE];
    println!("Allocated...");


    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let mut index: usize;

    for i in 0..HALD_CUBE_SIZE {
        for j in 0..HALD_CUBE_SIZE {
            for k in 0..HALD_CUBE_SIZE {
                r = i as f64 / (HALD_CUBE_SIZE - 1) as f64;
                g = j as f64 / (HALD_CUBE_SIZE - 1) as f64;
                b = k as f64 / (HALD_CUBE_SIZE - 1) as f64;

                index = i * 3 + j * 3 * HALD_CUBE_SIZE + k * 3 * HALD_CUBE_SIZE * HALD_CUBE_SIZE;

                lut_map[index] = r;
                lut_map[index + 1] = g;
                lut_map[index + 2] = b;
            }
        }
    }

    Lut3D {
        lut_map
    }
}

#[cfg(test)]
mod tests {
    use crate::definitions::generate_identity_lut;

    #[test]
    fn test_lut_apply_rgb() {
        let lut_in = generate_identity_lut();
        let lut_out = lut_in.apply_rgb_operation(|r, g, b| (r, g, b));
        assert_eq!(lut_out.lut_map, lut_in.lut_map);
    }
}