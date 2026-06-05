

// Hald 32

pub const HALD_LVL: usize = 8;
pub const HALD_CUBE_SIZE: usize = HALD_LVL * HALD_LVL;
pub const HALD_IMAGE_SIZE: usize = HALD_LVL * HALD_LVL * HALD_LVL;

pub const HALD_ARRAY_SIZE: usize = HALD_IMAGE_SIZE * HALD_IMAGE_SIZE * 3;

pub struct Lut3D {
    pub(crate) lut_map: Box<[u8; HALD_ARRAY_SIZE]>
}


pub fn generate_identity_lut() -> Lut3D {
    println!("Allocating...");
    let mut lut_map: Box<[u8; HALD_ARRAY_SIZE]> = Box::new([0; HALD_ARRAY_SIZE]);
    println!("Allocated...");


    let mut r: f64;
    let mut g: f64;
    let mut b: f64;
    let mut index: usize;

    for i in 0..HALD_CUBE_SIZE {
        for j in 0..HALD_CUBE_SIZE {
            for k in 0..HALD_CUBE_SIZE {
                r = (i as f64 / (HALD_CUBE_SIZE - 1) as f64) * 255.0;
                g = (j as f64 / (HALD_CUBE_SIZE - 1) as f64) * 255.0;
                b = (k as f64 / (HALD_CUBE_SIZE - 1) as f64) * 255.0;

                index = i * 3 + j * 3 * HALD_CUBE_SIZE + k * 3 * HALD_CUBE_SIZE * HALD_CUBE_SIZE;

                lut_map[index + 0] = r as u8;
                lut_map[index + 1] = g as u8;
                lut_map[index + 2] = b as u8;
            }
        }
    }

    Lut3D {
        lut_map: lut_map
    }
}

/*
Example:

```C
    cube_size = level * level;
    image_size = level * level * level;
    data = p = malloc((sizeof *data) * image_size * image_size * 3);
    for(blue = 0; blue < cube_size; blue++)
    {
         for(green = 0; green < cube_size; green++)
         {
             for(red = 0; red < cube_size; red++)
             {
                 *p++ = (float)red / (float)(cube_size - 1);
                 *p++ = (float)green / (float)(cube_size - 1);
                 *p++ = (float)blue / (float)(cube_size - 1);
             }
         }
    }
```
*/