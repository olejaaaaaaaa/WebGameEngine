#![allow(warnings)]

use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, Request, RequestInit, RequestMode, Response};

use web_sys::js_sys::Date;

#[path="physics.rs"]
mod physics;
use physics::*;

extern crate hecs;
use hecs::*;

use std::{rc::Rc, sync::mpsc::{channel, Receiver, Sender}, time::Duration};

use log::{debug, info, warn};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{js_sys::JsString, MessageEvent, WebSocket};
use winit::{dpi::PhysicalSize, event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::Window};

extern crate winit;
extern crate wgpu;
use wgpu::*;

#[path="setup.rs"]
mod setup;
use setup::*;

#[path="render.rs"]
mod render;
use render::*;


const url: &str = "ws://193.124.66.129:443";


const FPS_30: u32 = 33; // milliseconds
const FPS_60: u32 = 16;



pub async fn game_loop(event_loop: EventLoop<()>, mut window: Window) {


    // работает!
    wasm_bindgen_futures::spawn_local(async move {
        let a = reqwest::get("http://oleja.ru/music/file.txt").await;
        warn!("{:?}", a);
        if a.is_ok() {
            warn!("{:?}", a.unwrap().text().await.unwrap());
        }
    });


    info!("game loop is run");
    setup_canvas(&window);

    window.request_inner_size(PhysicalSize::new(640, 640));
   
    let websocket = WebSocket::new(url).unwrap();
    let rx = setup_onmessage(websocket.clone());
    websocket.onopen();
    websocket.send_with_str("I am ready!");

    let mut surface_configured = false;
    
    let mut gpu_config = ConfigWebGPU::new(&window).await;
    let mut gpu = RenderWebGpu::new(gpu_config);


    let mut phys = Physics::new();

    let win = &window;
    let ws = websocket.clone();


    let mut t = vec![];

    let r = 0.7;
    let angle = 0.017 / 2.0;

    let mut c = false;

    for i in 0..371 * 2  {
        t.push(Vertex::new(r*(i as f64 * angle).cos() as f32, r*(i as f64 * angle).sin() as f32, 0.0, [0.0, 0.0, 0.2]));
        t.push(Vertex::new(0.0, 0.0, 0.0, [0.0, 0.0, 0.5]));
        t.push(Vertex::new(r*((i+1) as f64 * angle).cos() as f32, r*((i+1) as f64 * angle).sin() as f32, 0.0, [0.0, 0.0, 1.0]));
    }

    gpu.push_vertex(t);



    let mut t = vec![];
    let r = 0.3;
    for i in 0..371 * 2  {
        t.push(Vertex::new(r*(i as f64 * angle).cos() as f32, r*(i as f64 * angle).sin() as f32, 0.0, [1.0, 0.0, 0.0]));
        t.push(Vertex::new(0.0, 0.0, 0.0, [0.0, 0.0, 0.5]));
        t.push(Vertex::new(r*((i+1) as f64 * angle).cos() as f32, r*((i+1) as f64 * angle).sin() as f32, 0.0, [1.0, 0.0, 0.0]));
    }

    gpu.push_vertex(t);


    let mut t = vec![];
    let r = 0.1;
    for i in 0..371 * 2  {
        t.push(Vertex::new(r*(i as f64 * angle).cos() as f32, r*(i as f64 * angle).sin() as f32, 0.0, [1.0, 0.5, 0.0]));
        t.push(Vertex::new(0.0, 0.0, 0.0, [0.0, 0.0, 0.5]));
        t.push(Vertex::new(r*((i+1) as f64 * angle).cos() as f32, r*((i+1) as f64 * angle).sin() as f32, 0.0, [1.0, 0.5, 0.0]));
    }

    gpu.push_vertex(t);


    let mut time = 0.0f32; 

    let mut time_begin = Date::new_0();
    event_loop.run(move |event, control_flow| 

        match event {
            Event::WindowEvent { window_id, event } => {

                match event {

                    WindowEvent::CursorMoved { device_id, position } => {

                        if let Ok(message) = rx.try_recv() {
                            info!("{:?}", message);
                        }
                       
                    },

                    WindowEvent::RedrawRequested => {
                        if !surface_configured { return; }
                        let dt = Date::new_0().get_milliseconds().overflowing_sub(time_begin.get_milliseconds()).0;

   
                        

                        gpu.draw();
                    },
    
                    WindowEvent::Resized(phys_size) => {
                        gpu.resize(phys_size);
                        surface_configured = true;
                    }

                    _ => ()
                }
            },

            Event::AboutToWait => {
                win.request_redraw();
                time_begin = Date::new_0();
            }

            _ => ()
        }
    )
    .unwrap();

    websocket.close();
}