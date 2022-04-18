mod common;
mod model;
mod renderer;

use common::*;
use model::*;
use renderer::*;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event},
    event_loop::{EventLoop},
    window::WindowBuilder,
};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

fn main() {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let scaled_size = LogicalSize::new(WIDTH as f64 * 3.0, HEIGHT as f64 * 3.0);
        WindowBuilder::new()
            .with_title("Tiny Renderer")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels =  Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();
    
    let color = Color { r: 0xff, g: 0xff, b: 0xff, a: 0xff };
    let buffer_size = BufferSize { width: WIDTH, height: HEIGHT };

    event_loop.run(move |event, _, __| {
        if let Event::RedrawRequested(_) = event {
            let mut renderer = TinyRenderer::make(buffer_size, pixels.get_frame());
            renderer.set_color(color);
            let model = WavefrontObj::from_file("data/model.obj");
            model.render(&mut renderer);
            pixels.render();
        }
    });
}