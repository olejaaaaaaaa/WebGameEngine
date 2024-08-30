


use utils::opengl::*;
use web_sys::{WebGlBuffer, WebGlShader};


struct ShaderProgram {
    program: WebGlProgram
}

struct VertexShader {
    shader: WebGlShader
}
struct FragmentShader {
    shader: WebGlShader
}

struct Location {
    location: Vec<u32>
}

struct Uniform {
    uniform: WebGlUniformLocation
}

struct VertexDynamicBuffer {
    buffer: WebGlBuffer
}

struct VertexStaticBuffer {
    buffer: WebGlBuffer
}
struct IndexBuffer {
    buffer: WebGlBuffer
}
struct Texture {
    tex: WebGlBuffer
}
struct TextureCoordinates {
    coord: Vec<u8>
}

struct FrameBuffer {
    buffer: WebGlBuffer
}

struct Normal {
    normal: Vec<u32>
}

struct Color {
    color: [f32; 4]
}

