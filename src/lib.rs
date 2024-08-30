#![allow(warnings)]

use wasm_bindgen::prelude::*;

extern crate console_log;
extern crate log;
use log::{debug, error, info, warn};

extern crate console_error_panic_hook;
extern crate pollster;
extern crate winit;

mod game;
use game::*;


#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info);

        info!("main is run");
        let main_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = winit::window::WindowBuilder::new().build(&main_loop).unwrap();

        pollster::block_on(game_loop(main_loop, window));
        
    Ok(())
}