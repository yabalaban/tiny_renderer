use crate::common::*;
use crate::renderer::*;

use std::fs::File;
use std::io::{BufReader, BufRead};

pub trait Model {
    fn render<T>(self, renderer: &mut T) where T: Renderer;
}

pub struct WavefrontObj {
    v: Vec<Vertex>,
    f: Vec<Triangle>,
}

impl Model for WavefrontObj {
    fn render<T>(self, renderer: &mut T)  where T: Renderer {
        for face in self.f {
            renderer.triangle(face);
        }
    }
}

impl WavefrontObj {
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut obj = WavefrontObj { v: Vec::new(), f: Vec::new() };

        for line in reader.lines() {
            let u = line.unwrap();
            let mut spl = u.split_whitespace();
            match spl.next() {
                Some("v") => {
                    let p = Vertex {
                        x: spl.next().unwrap().parse::<f32>().unwrap(),
                        y: spl.next().unwrap().parse::<f32>().unwrap(),
                        z: spl.next().unwrap().parse::<f32>().unwrap(),
                    };
                    obj.v.push(p)
                },
                Some("f") => {
                    let v1: Vec<usize> = spl.next().unwrap().split("/").map(|x| x.parse::<usize>().unwrap() ).collect();
                    let v2: Vec<usize>= spl.next().unwrap().split("/").map(|x| x.parse::<usize>().unwrap() ).collect();
                    let v3: Vec<usize> = spl.next().unwrap().split("/").map(|x| x.parse::<usize>().unwrap() ).collect();
                    obj.f.push(Triangle(
                        obj.v[v1[0] - 1],
                        obj.v[v2[0] - 1],
                        obj.v[v3[0] - 1],
                    ))
                },
                _ => (),
            }
        };
        obj
    }
}