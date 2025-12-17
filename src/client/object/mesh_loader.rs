use std::io::BufRead;
use std::num::{ParseFloatError, ParseIntError};

use log::debug;

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

pub fn load_vertices_from_obj(
    file_path: &str,
    flags: u32,
) -> Result<
    (
        (bool, Option<Vec<f32>>),    // vertices
        (bool, Option<Vec<u32>>),    // indices
        (bool, Option<Vec<f32>>),    // colors
        (bool, Option<Vec<f32>>),    // uvs
        (bool, Option<Vec<f32>>),    // normals
        (bool, Option<Vec<String>>), // materials
        (bool, Option<Vec<String>>), // groups
        (bool, Option<Vec<bool>>),   // smoothing
    ),
    MeshLoadError,
> {
    let start_time = std::time::Instant::now();
    let mut vertices = if flags & MESH_VERT != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut indices = if flags & MESH_INDICE != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut colors = if flags & MESH_COLOR != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut uvs = if flags & MESH_UV != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut normals = if flags & MESH_NORMAL != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut materials = if flags & MESH_MATERIAL != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut groups = if flags & MESH_GROUP != 0 {
        Some(Vec::new())
    } else {
        None
    };
    let mut smoothing = if flags & MESH_SMOOTH != 0 {
        Some(Vec::new())
    } else {
        None
    };

    let mut current_material = String::new();
    let mut current_group = String::new();
    let mut current_smooth = false;

    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                if let Some(v) = &mut vertices {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    v.extend_from_slice(&[x, y, z]);
                }
                if let Some(c) = &mut colors {
                    if parts.len() >= 7 {
                        let r: f32 = parts[4].parse()?;
                        let g: f32 = parts[5].parse()?;
                        let b: f32 = parts[6].parse()?;
                        c.extend_from_slice(&[r, g, b]);
                    }
                }
            }
            "vt" => {
                if let Some(uv) = &mut uvs {
                    let u: f32 = parts[1].parse()?;
                    let v: f32 = parts[2].parse()?;
                    uv.extend_from_slice(&[u, v]);
                }
            }
            "vn" => {
                if let Some(n) = &mut normals {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    n.extend_from_slice(&[x, y, z]);
                }
            }
            "usemtl" => {
                current_material = parts[1].to_string();
            }
            "g" => {
                current_group = parts[1].to_string();
            }
            "s" => {
                current_smooth = match parts[1] {
                    "off" | "0" => false,
                    _ => true,
                };
            }
            "f" => {
                let mut face_indices: Vec<u32> = Vec::new();
                for part in &parts[1..] {
                    let vertex_index = part.split('/').next().unwrap().parse::<i32>()?;
                    let index = if let Some(v) = &vertices {
                        if vertex_index < 0 {
                            (v.len() as i32 / 3 + vertex_index) as u32
                        } else {
                            (vertex_index - 1) as u32
                        }
                    } else {
                        (vertex_index - 1) as u32
                    };
                    face_indices.push(index);
                }
                for i in 1..face_indices.len() - 1 {
                    if let Some(idx) = &mut indices {
                        idx.push(face_indices[0]);
                        idx.push(face_indices[i]);
                        idx.push(face_indices[i + 1]);
                    }
                    if let Some(m) = &mut materials {
                        m.push(current_material.clone());
                    }
                    if let Some(g) = &mut groups {
                        g.push(current_group.clone());
                    }
                    if let Some(s) = &mut smoothing {
                        s.push(current_smooth);
                    }
                }
            }
            _ => {}
        }
    }

    debug!(
        "Loaded mesh from {} in {:?}",
        file_path,
        start_time.elapsed()
    );

    Ok((
        (flags & MESH_VERT != 0, vertices),
        (flags & MESH_INDICE != 0, indices),
        (flags & MESH_COLOR != 0, colors),
        (flags & MESH_UV != 0, uvs),
        (flags & MESH_NORMAL != 0, normals),
        (flags & MESH_MATERIAL != 0, materials),
        (flags & MESH_GROUP != 0, groups),
        (flags & MESH_SMOOTH != 0, smoothing),
    ))
}
