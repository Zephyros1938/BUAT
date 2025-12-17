use std::collections::HashMap;
use std::io::BufRead;
use std::num::{ParseFloatError, ParseIntError};
use std::path::Path;

use log::debug;

use crate::graphics::texture::{Texture, TextureLoadOptions, load_texture_from_file};

pub const MESH_VERT: u32 = 0b0000_0001;
pub const MESH_INDICE: u32 = 0b0000_0010;
pub const MESH_COLOR: u32 = 0b0000_0100;
pub const MESH_UV: u32 = 0b0000_1000;
pub const MESH_NORMAL: u32 = 0b0001_0000;
pub const MESH_MATERIAL: u32 = 0b0010_0000;
pub const MESH_GROUP: u32 = 0b0100_0000;
pub const MESH_SMOOTH: u32 = 0b1000_0000;
pub const MESH_ALL: u32 = 0b1111_1111;

#[derive(Debug)]
pub enum MeshLoadError {
    Io(std::io::Error),
    ParseFloat(ParseFloatError),
    ParseInt(ParseIntError),
    InvalidFormat(String),
}

impl From<std::io::Error> for MeshLoadError {
    fn from(err: std::io::Error) -> Self {
        MeshLoadError::Io(err)
    }
}

impl From<ParseFloatError> for MeshLoadError {
    fn from(err: ParseFloatError) -> Self {
        MeshLoadError::ParseFloat(err)
    }
}

impl From<ParseIntError> for MeshLoadError {
    fn from(err: ParseIntError) -> Self {
        MeshLoadError::ParseInt(err)
    }
}

impl From<String> for MeshLoadError {
    fn from(err: String) -> Self {
        MeshLoadError::InvalidFormat(err)
    }
}

pub fn load_vertices_from_obj(
    file_path: &str,
    flags: u32,
) -> Result<
    (
        Option<Vec<f32>>,       // vertices
        Option<Vec<u32>>,       // indices
        Option<Vec<f32>>,       // colors
        Option<Vec<f32>>,       // uvs
        Option<Vec<f32>>,       // normals
        Option<Vec<u32>>,       // material IDs per triangle
        HashMap<u32, Material>, // material table
    ),
    MeshLoadError,
> {
    debug!("Loading OBJ file {} with flags 0b{:08x}", file_path, flags);
    let mut vertices = (flags & MESH_VERT != 0).then(Vec::new);
    let mut indices = (flags & MESH_INDICE != 0).then(Vec::new);
    let mut colors = (flags & MESH_COLOR != 0).then(Vec::new);
    let mut uvs = (flags & MESH_UV != 0).then(Vec::new);
    let mut normals = (flags & MESH_NORMAL != 0).then(Vec::new);
    let mut mats = (flags & MESH_MATERIAL != 0).then(Vec::new);

    let mut material_lib: HashMap<u32, Material> = HashMap::new();
    let mut material_ids: HashMap<String, u32> = HashMap::new();
    let mut current_mat: u32 = 0;

    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let obj_dir = Path::new(file_path).parent().unwrap_or(Path::new(""));

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "mtllib" if flags & MESH_MATERIAL != 0 => {
                let mtl_path = obj_dir.join(parts[1]);
                let mtl = load_mtl_file(&mtl_path)?;

                for (name, mat) in mtl {
                    let id = material_lib.len() as u32;
                    material_ids.insert(name.clone(), id);
                    material_lib.insert(id, mat);
                }
            }

            "usemtl" if flags & MESH_MATERIAL != 0 => {
                if let Some(id) = material_ids.get(parts[1]) {
                    current_mat = *id;
                }
            }

            "v" => {
                if let Some(v) = &mut vertices {
                    v.push(parts[1].parse()?);
                    v.push(parts[2].parse()?);
                    v.push(parts[3].parse()?);
                }
                if let Some(c) = &mut colors {
                    if parts.len() >= 7 {
                        c.push(parts[4].parse()?);
                        c.push(parts[5].parse()?);
                        c.push(parts[6].parse()?);
                    }
                }
            }

            "vt" => {
                if let Some(uv) = &mut uvs {
                    uv.push(parts[1].parse()?);
                    uv.push(parts[2].parse()?);
                }
            }

            "vn" => {
                if let Some(n) = &mut normals {
                    n.push(parts[1].parse()?);
                    n.push(parts[2].parse()?);
                    n.push(parts[3].parse()?);
                }
            }

            "f" => {
                let mut face = Vec::new();
                for part in &parts[1..] {
                    let i = part.split('/').next().unwrap().parse::<i32>()?;
                    let idx = if i < 0 {
                        (vertices.as_ref().unwrap().len() as i32 / 3 + i) as u32
                    } else {
                        (i - 1) as u32
                    };
                    face.push(idx);
                }

                for i in 1..face.len() - 1 {
                    if let Some(ind) = &mut indices {
                        ind.extend_from_slice(&[face[0], face[i], face[i + 1]]);
                    }
                    if let Some(m) = &mut mats {
                        m.push(current_mat);
                    }
                }
            }

            _ => {}
        }
    }

    Ok((vertices, indices, colors, uvs, normals, mats, material_lib))
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: Option<Texture>,
}

pub fn load_mtl_file(mtl_path: &Path) -> Result<HashMap<String, Material>, MeshLoadError> {
    let file = std::fs::File::open(mtl_path)?;
    let reader = std::io::BufReader::new(file);

    let mut materials = HashMap::new();
    let mut current_name: Option<String> = None;

    let base_dir = mtl_path.parent().unwrap_or(Path::new(""));

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "newmtl" => {
                current_name = Some(parts[1].to_string());
                materials.insert(
                    parts[1].to_string(),
                    Material {
                        name: parts[1].to_string(),
                        diffuse_texture: None,
                    },
                );
            }
            "map_Kd" => {
                if let Some(name) = &current_name {
                    let tex_path = base_dir.join(parts[1]);
                    let texture = load_texture_from_file(
                        tex_path.to_str().unwrap(),
                        TextureLoadOptions::default(),
                    );

                    materials.get_mut(name).unwrap().diffuse_texture = Some(texture?);
                }
            }
            _ => {}
        }
    }

    Ok(materials)
}
