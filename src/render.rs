#![allow(warnings)]


#[path="physics.rs"]
mod physics;

mod setup;
use setup::Shaders;

extern crate hecs;
use hecs::*;
use util::{BufferInitDescriptor, DeviceExt};

use std::{
    iter, 
    mem::transmute, 
    rc::Rc, 
    sync::{
        mpsc::{channel, Receiver, Sender}, 
        Arc, Mutex
    }
};

use log::{debug, info};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{js_sys::JsString, MessageEvent, WebSocket};

use winit::{
    dpi::PhysicalSize, 
    event::{ElementState, Event, KeyEvent, WindowEvent}, 
    event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, 
    window::Window
};

extern crate winit;
extern crate log;
extern crate wgpu;
use wgpu::*;

extern crate bytemuck;
use bytemuck::*;


use super::{default_shader, test_shader};


#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    fn layout() -> VertexBufferLayout<'static> {

        VertexBufferLayout {
            
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0
            },

            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1
            }

            ]
        }

    }

    pub fn new(x: f32, y: f32, z: f32, color: [f32; 3]) -> Self {
        Vertex {
            pos: [x, y, z],
            color
        }
    }
}


#[derive(Default)]
pub struct RenderWebGpu<'s> {
    pub webgpu_config: ConfigWebGPU<'s>,
    pub vertex: Vec<Vec<Vertex>>,
    pub pipeline: Vec<RenderPipeline>,
    pub buffer: Vec<Buffer>,
    pub shader: Option<ShaderModule>,
}


impl<'s> RenderWebGpu<'s> {

    pub fn new(webgpu: ConfigWebGPU<'s>) -> Self {

        let mut r = RenderWebGpu { 
            webgpu_config: webgpu,
            ..Default::default() 
        };

        r.create_shader();
        r.create_pipeline();

        r
    }

    pub fn push_vertex(&mut self, data: Vec<Vertex>) {
        self.vertex.push(data.clone());
        let buf = self.webgpu_config.device().create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&data),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST
        });

        self.buffer.push(buf);
    }

    pub fn create_shader(&mut self) {
        let shader = self.webgpu_config.device().create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(std::borrow::Cow::Borrowed(test_shader))
        });

        self.shader = Some(shader);
    }

    pub fn create_pipeline(&mut self) {
        let pipeline_layout = self.webgpu_config.device().create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = self.webgpu_config.device().create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: self.shader.as_ref().unwrap(),
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::layout()],
            },

            fragment: Some(wgpu::FragmentState {
                module: self.shader.as_ref().unwrap(),
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: TextureFormat::Rgba8UnormSrgb,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            operation: wgpu::BlendOperation::Add,
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        },
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),

            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                topology: PrimitiveTopology::PointList,
                ..Default::default()
            },

            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        
        self.pipeline.push(pipeline);

    }


    pub fn update_vertex(&mut self, index: usize, data: Vec<Vertex>) {
        self.vertex[index] = data.clone();
        self.webgpu_config.queue.as_mut().unwrap().write_buffer(&self.buffer[index], 0, bytemuck::cast_slice(&data));
    }

    pub fn draw(&mut self) {

        let mut encoder = self.webgpu_config.device().create_command_encoder(&CommandEncoderDescriptor { label: None });
        let mut output = self.webgpu_config.surface.as_ref().unwrap().get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            rpass.set_pipeline(&self.pipeline.last().unwrap());

            for i in &self.buffer {
                for j in &self.vertex {
                    let l = j.len() as u32;
                    rpass.set_vertex_buffer(0, i.slice(..));
                    rpass.draw(0..l, 0..1);
                }
            }

        }    
      
        self.webgpu_config.queue.as_ref().unwrap().submit(iter::once(encoder.finish()));
        output.present();

    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.webgpu_config.resize(size);
    }

}


#[derive(Default)]
pub struct ConfigWebGPU<'surface> {
    window: Option<&'surface Window>,
    instance: Option<Instance>,
    device: Option<Device>,
    queue: Option<Queue>,
    surface: Option<Surface<'surface>>,
    surface_config: Option<SurfaceConfiguration>,
    surface_caps: Option<SurfaceCapabilities>,
    surface_format: Option<TextureFormat>,
    adapter: Option<Adapter>,
}

impl<'s> ConfigWebGPU<'s> {

    pub async fn new(window: &'s Window) -> ConfigWebGPU<'s> {

        let mut webgpu_config = ConfigWebGPU {
            ..Default::default()
        };

        webgpu_config.window = Some(window);
        //
        //  window
        //

        webgpu_config.setup_instance();
        //
        //  instance
        //

        webgpu_config.setup_surface(&window).await;
        //
        //  surface
        //
    
        webgpu_config.setup_adapter().await;
        //
        //  adapter
        //

        webgpu_config.setup_device_and_queue().await;
        //
        //  device
        //  queue
        //

        webgpu_config.setup_surface_config();
        //
        //  surface_format
        //  surface_capabilities
        //  surface_configuration
        //

        webgpu_config
    }


    // 1
    fn setup_instance(&mut self) {
        let inst = Instance::default();
        self.instance = Some(inst);
    }

    // 2
    async fn setup_surface(&mut self, win: &'s Window) {
        let surface = self.instance
            .as_ref()
            .unwrap()
            .create_surface(win)
            .unwrap();

        self.surface = Some(surface);
    }

    // 3
    async fn setup_adapter(&mut self) {
        let adapter = self.instance.as_mut().unwrap().request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(self.surface.as_ref().unwrap())
        }).await.unwrap();

        self.adapter = Some(adapter);
    }

    // 4
    async fn setup_device_and_queue(&mut self) {
        let (device, queue) = self.adapter.as_ref().unwrap().request_device(&DeviceDescriptor { 
            label: Some("Default Device"), 
            required_features: Features::empty(), 
            required_limits: Limits::downlevel_webgl2_defaults(), 
            memory_hints: MemoryHints::Performance 
        }, None).await.unwrap();

        self.device = Some(device);
        self.queue = Some(queue);
    }

    // 5
    fn setup_surface_config(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        let surface_caps = surface.get_capabilities(self.adapter.as_ref().unwrap());
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.window().inner_size().width,
            height: self.window().inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.surface_format = Some(surface_format);
        self.surface_caps = Some(surface_caps);
        self.surface_config = Some(surface_config);
    }    


    pub fn resize(&mut self, phys_size: PhysicalSize<u32>) {

        if  self.surface.is_some() && 
            self.surface_config.is_some() &&
            self.device.is_some() {

            unsafe {
                let surf = self.surface_config.as_mut().unwrap_unchecked();

                surf.width = phys_size.width;
                surf.height = phys_size.height;

                let dev = self.device.as_mut().unwrap_unchecked();
                self.surface.as_mut().unwrap_unchecked().configure(dev, surf);
            }
        }
    }

    fn window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    fn device(&self) -> &Device {
        self.device.as_ref().unwrap()
    }

}


