
mod utils;
use utils::js::log_1;


pub type Mat4 = [[f32; 4]; 4];
pub type Mat3 = [[f32; 3]; 3];
pub type Mat2 = [[f32; 2]; 2];


pub fn mat4_zero() -> Mat4 {
    [
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ]
}

pub fn mat4_one() -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub trait Matrix{
    fn get(&self, x: usize, y: usize) -> f32;
    fn mul(&mut self, value: f32);
}

impl Matrix for Mat4 {

    fn get(&self, x: usize, y: usize) -> f32 {
        self[x][y]
    }

    fn mul(&mut self, value: f32) {
        for x in 0..4 {
            for y in 0..4 {
                self[x][y] *= value;
            }
        }
    }
}


fn print<T: Debug + JsValue>(v: T) {
    log_1(T)
}


