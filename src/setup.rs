#![allow(warnings)]
use std::sync::mpsc::{channel, Receiver};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{MessageEvent, WebSocket};
use winit::window::Window;

//
// create canvas and append to body
//
pub fn setup_canvas(window: &Window) {

    #[cfg(target_arch = "wasm32")] {
        use winit::platform::web::WindowExtWebSys;
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let body = doc.get_element_by_id("main-body").unwrap();
        let canvas = web_sys::Element::from(window.canvas().unwrap());
        body.append_child(&canvas);
    }
}


//
//  function for get message with channel
//
pub fn setup_onmessage(ws: WebSocket) -> Receiver<String> {

    
    let (sx, rx) = channel::<String>();

    let closure = Closure::<dyn FnMut(MessageEvent)>::new(move |e: MessageEvent| {
        sx.send(e.data().as_string().unwrap());
    });
    ws.set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    rx
}

pub const default_shader: &str = include_str!("shaders/default.wgsl");
pub const test_shader: &str = include_str!("shaders/test.wgsl");

pub enum Shaders {
    Default,
    Test,
}