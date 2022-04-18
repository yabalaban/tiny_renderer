#[derive(Clone, Copy, Default)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,    
}

#[derive(Debug)]
pub struct Line(pub Vertex, pub Vertex);
#[derive(Debug)]
pub struct Triangle(pub Vertex, pub Vertex, pub Vertex);