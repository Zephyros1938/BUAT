use std::io::BufRead;

pub fn load_vertices_from_obj(file_path: &str) -> (Vec<f32>, Vec<u32>) {
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let file = std::fs::File::open(file_path).expect("Failed to open OBJ file");
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
            "f" => { // Face Data (indiced)
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

    (vertices, indices)
}
