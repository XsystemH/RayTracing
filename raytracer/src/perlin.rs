use crate::vec3::Point3;
use rand::{thread_rng, Rng};

pub struct Perlin {
    rand_float: Vec<f64>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut rand_float: Vec<f64> = vec![];
        for _i in 0..Self::POINT_COUNT {
            rand_float.push(thread_rng().gen_range(0.0..1.0));
        }

        Self {
            rand_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2usize {
            for dj in 0..2usize {
                for dk in 0..2usize {
                    c[di][dj][dk] = self.rand_float[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
    }
    fn perlin_generate_perm() -> Vec<u32> {
        let mut p: Vec<u32> = vec![];
        for i in 0..Self::POINT_COUNT {
            p.push(i as u32);
        }
        Self::permute(&mut p, Self::POINT_COUNT);
        p
    }
    fn permute(p: &mut [u32], n: usize) {
        for i in (1..n - 1).rev() {
            let target = thread_rng().gen_range(0..i);
            p.swap(i, target);
        }
    }
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i as usize][j as usize][k as usize];
                }
            }
        }
        accum
    }
}
