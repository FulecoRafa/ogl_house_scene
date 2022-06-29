extern crate obj;

use std::{env, fs};
use std::fs::DirBuilder;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use obj::{load_obj, Obj, TexturedVertex};

use crate::assets::vertex::{Normal, Vertex};

pub(crate) fn parse_model(path: &str) -> (Vec<Vertex>, Vec<u32>, Vec<Normal>) {
    let file = BufReader::new(fs::File::open(path).unwrap());
    println!("Loading model: {}", path);
    let object: Obj<TexturedVertex, u16> = load_obj(file).expect("Failed to load model");

    let vertices: Vec<Vertex> = object.vertices.iter()
        .map(|v| {
            Vertex {
                position: [v.position[0] / 200., v.position[1] / 200., v.position[2] / 200.],
                tex_coords: [v.texture[0], v.texture[1]],
            }
        })
        .collect();

    let normals: Vec<Normal> = object.vertices.iter()
        .map(|v| {
            Normal {
                normal: [v.normal[0], v.normal[1], v.normal[2]],
            }
        })
        .collect();

    let indices: Vec<u32> = object.indices.iter()
        .map(|i| {
            *i as u32
        })
        .collect();

    println!("Model loaded: {}", path);
    (vertices, indices, normals)
}