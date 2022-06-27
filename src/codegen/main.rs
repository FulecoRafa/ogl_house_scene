extern crate obj;

use std::{env, fs};
use std::fs::DirBuilder;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use obj::{load_obj, Obj, TexturedVertex};
use walkdir::WalkDir;

fn main() {
    let files = fs::read_dir("./models").expect("Could not read models directory");
    for path in files {
        // File data
        let path_buf = path.unwrap()
            .path();

        if path_buf.extension().unwrap() != "obj" {
            continue;
        };

        let name = path_buf
            .file_stem().unwrap()
            .to_str().unwrap();

        let file = BufReader::new(fs::File::open(path_buf.clone()).unwrap());
        let object: Obj<TexturedVertex, u16> = load_obj(file).expect("Failed to load model");

        // Model data
        let (vertices_length, indices_length) = (object.vertices.len(), object.indices.len());
        let vertices = object.vertices.iter()
            .map(|v| {
                format!("Vertex {{ position: [{:.}_f32, {:.}_f32, {:.}_f32], tex_coords: [{:.}_f32, {:.}_f32] }}", v.position[0], v.position[1], v.position[2], v.texture[0], v.texture[1])
            })
            .collect::<Vec<_>>()
            .join(", \n");
        let normals = object.vertices.iter()
            .map(|v| {
                format!("Normal {{ normal: [{:.}_f32, {:.}_f32, {:.}_f32] }}", v.normal[0], v.normal[1], v.normal[2])
            })
            .collect::<Vec<_>>()
            .join(", \n");
        let indices = object.indices.iter()
            .map(|i| {
                format!("{}", i)
            })
            .collect::<Vec<_>>()
            .join(", \n");

        // Modified string to go on file
        let data_template = format!(
            include_str!("data_template.rs"),
            name = name,
            name_lowercase = name.to_lowercase(),
            vertices_len = vertices_length,
            indices_len = indices_length,
            vertices = vertices,
            indices = indices,
            normals_len = vertices_length,
            normals = normals,
        );

        let model_template = format!(
            include_str!("model_template.rs"),
            name = name,
        );

        let name = name.to_lowercase();

        // Write to file
        // create folder
        let folder_path_str = format!("./src/model/{}", name);
        let folder_path = Path::new(&folder_path_str);
        if !folder_path.exists() {
            fs::create_dir(folder_path).unwrap();
        }

        let mod_path_str = format!("./src/model/{name}/mod.rs", name = name);
        let mod_path = Path::new(&mod_path_str);
        if !mod_path.exists() {
            let mut file = fs::File::create(mod_path).expect("Failed to create file");
            file.write_all(model_template.as_bytes()).expect("Failed to write to file");
        }
        let data_path_str = format!("./src/model/{name}/data.rs", name = name);
        let data_path = Path::new(&data_path_str);
        if !data_path.exists() {
            let mut file = fs::File::create(data_path).expect("Failed to create file");
            file.write_all(data_template.as_bytes()).expect("Failed to write to file");
        }
    }
}