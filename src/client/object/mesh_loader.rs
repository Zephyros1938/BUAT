use std::io::BufRead;

pub const MESH_VERT: u32 = 0b0000_0000_0000_0001;
pub const MESH_INDICE: u32 = 0b0000_0000_0000_0010;
pub const MESH_COLOR: u32 = 0b0000_0000_0000_0100;
pub const MESH_UV: u32 = 0b0000_0000_0000_1000;

#[derive(Debug)]
pub enum MeshLoadError {
    Io(std::io::Error),
    Parse(std::num::ParseFloatError),
    InvalidFormat(String),
}

impl From<std::io::Error> for MeshLoadError {
    fn from(err: std::io::Error) -> Self {
        MeshLoadError::Io(err)
    }
}
impl From<std::num::ParseFloatError> for MeshLoadError {
    fn from(err: std::num::ParseFloatError) -> Self {
        MeshLoadError::Parse(err)
    }
}

pub fn load_vertices_from_obj(
    file_path: &str,
    flags: u32,
) -> Result<
    (
        (bool, Vec<f32>),
        (bool, Vec<u32>),
        (bool, Vec<f32>),
        (bool, Vec<f32>),
    ),
    MeshLoadError,
> {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut colors: Vec<f32> = Vec::new();
    let mut uvs: Vec<f32> = Vec::new();

    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" => {
                let x: f32 = parts[1].parse().expect("Invalid vertex x");
                let y: f32 = parts[2].parse().expect("Invalid vertex y");
                let z: f32 = parts[3].parse().expect("Invalid vertex z");

                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
            }
            "f" => {
                // Face Data (indiced)
                let mut face_indices: Vec<u32> = Vec::new();

                for part in &parts[1..] {
                    let vertex_index = part
                        .split('/')
                        .next()
                        .unwrap()
                        .parse::<i32>()
                        .expect("Invalid face index");

                    let index = if vertex_index < 0 {
                        // Negative indices are relative to the end
                        (vertices.len() as i32 / 3 + vertex_index) as u32
                    } else {
                        (vertex_index - 1) as u32
                    };

                    face_indices.push(index);
                }

                // Fan triangulation
                for i in 1..face_indices.len() - 1 {
                    indices.push(face_indices[0]);
                    indices.push(face_indices[i]);
                    indices.push(face_indices[i + 1]);
                }
            }
            _ => {}
        }
    }

    Ok((
        (flags & MESH_VERT != 0, vertices),
        (flags & MESH_INDICE != 0, indices),
        (flags & MESH_COLOR != 0, colors),
        (flags & MESH_UV != 0, uvs),
    ))
}
