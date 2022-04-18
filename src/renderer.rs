use crate::common::Color;
use crate::common::Line;
use crate::common::Triangle;
use crate::common::Vertex;

pub trait Renderer {
    fn set_color(&mut self, color: Color);
    fn point(&mut self, vertex: Vertex);
    fn line(&mut self, line: Line);
    fn triangle(&mut self, triangle: Triangle);
}

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct BufferSize {
    pub height: usize, 
    pub width: usize,
}

pub struct TinyRenderer<'a> {
    color: Color,
    size: BufferSize,
    buffer: &'a mut [u8],
}

impl<'a> TinyRenderer<'a> {
    pub fn make(size: BufferSize, buffer: &'a mut [u8]) -> Self {
        TinyRenderer {
            color: Color { ..Default::default() },
            size, 
            buffer,
        }
    }
}

impl<'a> Renderer for TinyRenderer<'a> {
    fn set_color(&mut self, color: Color) { 
        self.color = color
    }

    fn point(&mut self, vertex: Vertex) {
        let x = vertex.x as usize;
        let mut y = vertex.y as usize;
        y = (self.size.height - 1) - y;

        self.buffer[4 * (x + y * self.size.width)] = self.color.r;
        self.buffer[4 * (x + y * self.size.width) + 1] = self.color.g;
        self.buffer[4 * (x + y * self.size.width) + 2] = self.color.b;
        self.buffer[4 * (x + y * self.size.width) + 3] = self.color.a;
    }

    fn line(&mut self, line: Line) {
        let (mut x0, mut y0) = (line.0.x, line.0.y);
        let (mut x1, mut y1) = (line.1.x, line.1.y);
        let mut steep = false;

        if f32::abs(x0 - x1) < f32::abs(y0 - y1) {
            (x0, y0) = (y0, x0);
            (x1, y1) = (y1, x1);
            steep = true;
        }
        if x0 > x1 {
            (x0, x1) = (x1, x0);
            (y0, y1) = (y1, y0);
        }

        let x0_u = ((x0 + 1.0) * (self.size.width - 1) as f32 / 2.0) as usize;
        let y0_u = (y0 + 1.0) * (self.size.height - 1) as f32 / 2.0;
        let x1_u = ((x1 + 1.0) * (self.size.width - 1) as f32 / 2.0) as usize;
        let y1_u = (y1 + 1.0) * (self.size.height - 1) as f32 / 2.0;
        for x in x0_u..x1_u {
            let t = (x - x0_u) as f32 / (x1_u - x0_u) as f32;
            let y = y0_u * (1.0 - t) + y1_u * t;
            let v = Vertex { x: if steep { y } else { x as f32 }, y: if steep { x as f32 } else { y }, z: 0.0 };
            self.point(v)
        }
    }

    fn triangle(&mut self, triangle: Triangle) {
        self.line(Line(triangle.0, triangle.1));
        self.line(Line(triangle.0, triangle.2));
        self.line(Line(triangle.1, triangle.2));
    }
}