pub const HALD_LVL: usize = 16;
pub const HALD_CUBE_SIZE: usize = HALD_LVL * HALD_LVL;
pub const HALD_IMAGE_SIZE: usize = HALD_LVL * HALD_LVL * HALD_LVL;

pub const HALD_ARRAY_SIZE: usize = HALD_IMAGE_SIZE * HALD_IMAGE_SIZE * 3;

pub const MAX_VALUE: u16 = u16::MAX;
pub const MAX_VALUE_F: f64 = MAX_VALUE as f64;

pub struct Lut3D {
    pub(crate) lut_map: Vec<u16>
}

impl Lut3D {
    pub fn apply_operation<F>(&self, operation: F) -> Self
    where
        F: Fn(u16) -> u16,
    {
        let lut_map_out: Vec<u16> = self.lut_map.iter().copied().map(operation).collect();

        Lut3D{
            lut_map: lut_map_out,
        }
    }
}

///
/// Generates an Identity Lut3D
pub fn generate_identity_lut() -> Lut3D {
    println!("Allocating...");
    let mut lut_map: Vec<u16> = vec![0u16; HALD_ARRAY_SIZE];
    println!("Allocated...");


    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let mut index: usize;

    for i in 0..HALD_CUBE_SIZE {
        for j in 0..HALD_CUBE_SIZE {
            for k in 0..HALD_CUBE_SIZE {
                r = (i as f64 / (HALD_CUBE_SIZE - 1) as f64) * MAX_VALUE_F;
                g = (j as f64 / (HALD_CUBE_SIZE - 1) as f64) * MAX_VALUE_F;
                b = (k as f64 / (HALD_CUBE_SIZE - 1) as f64) * MAX_VALUE_F;

                index = i * 3 + j * 3 * HALD_CUBE_SIZE + k * 3 * HALD_CUBE_SIZE * HALD_CUBE_SIZE;

                lut_map[index] = r as u16;
                lut_map[index + 1] = g as u16;
                lut_map[index + 2] = b as u16;
            }
        }
    }

    Lut3D {
        lut_map
    }
}