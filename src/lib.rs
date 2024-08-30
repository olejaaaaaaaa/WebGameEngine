#![allow(warnings)]



mod utils;
use utils::js::*;
use utils::utils::types::*;



fn main() {
    let mut a: Mat4 = mat4_one();
    print(a);
}