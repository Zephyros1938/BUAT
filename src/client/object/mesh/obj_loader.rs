use tobj;

// Assuming these are your types
use crate::graphics::shader::VertexArrayObject;
use crate::object::part::RenderData;

#[derive(Debug)]
pub enum MeshLoadError {
    Tobj(tobj::LoadError),
    NoMeshes,
}

impl From<tobj::LoadError> for MeshLoadError {
    fn from(err: tobj::LoadError) -> Self {
        MeshLoadError::Tobj(err)
    }
}

/// Loads an OBJ file and converts it to RenderData
/// Returns a Vec because OBJ files can contain multiple meshes
pub fn load_obj_to_render_data(
    file_path: &str,
    include_normals: bool,
    include_texcoords: bool,
) -> Result<Vec<RenderData>, MeshLoadError> {
    let (models, _materials) = tobj::load_obj(
        file_path,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true, // makes vertices match indices
            ..Default::default()
        },
    )?;

    if models.is_empty() {
        return Err(MeshLoadError::NoMeshes);
    }

    let mut render_data_vec = Vec::new();

    for model in models {

        let mesh = model.mesh;
        
        // Build interleaved vertex buffer
        let mut interleaved_data = Vec::new();
        let vertex_count = mesh.positions.len() / 3;

        for i in 0..vertex_count {
            // Position (always included) - Location 0
            interleaved_data.push(mesh.positions[i * 3]);
            interleaved_data.push(mesh.positions[i * 3 + 1]);
            interleaved_data.push(mesh.positions[i * 3 + 2]);

            // Texture coordinates (if requested and available) - Location 2
            if include_texcoords && !mesh.texcoords.is_empty() {
                interleaved_data.push(mesh.texcoords[i * 2]);
                interleaved_data.push(mesh.texcoords[i * 2 + 1]);
            } else if include_texcoords {
                // Default UV if not available
                interleaved_data.extend_from_slice(&[0.0, 0.0]);
            }

            // Normals (if requested and available) - Location 3
            if include_normals && !mesh.normals.is_empty() {
                interleaved_data.push(mesh.normals[i * 3]);
                interleaved_data.push(mesh.normals[i * 3 + 1]);
                interleaved_data.push(mesh.normals[i * 3 + 2]);
            } else if include_normals {
                // Default normal if not available
                interleaved_data.extend_from_slice(&[0.0, 1.0, 0.0]);
            }
        }

        // Create OpenGL buffers
        let render_data = create_render_data(
            &interleaved_data,
            &mesh.indices,
            include_normals,
            include_texcoords,
        );

        render_data_vec.push(render_data);
    }

    Ok(render_data_vec)
}

/// Creates RenderData from interleaved vertex data and indices
fn create_render_data(
    vertices: &[f32],
    indices: &[u32],
    has_normals: bool,
    has_texcoords: bool,
) -> RenderData {
    let mut vbo = 0;
    let mut ebo = 0;
    let mut vao = VertexArrayObject::new();

    // Calculate vertex stride
    let mut floats_per_vertex = 3; // Position
    if has_texcoords {
        floats_per_vertex += 2;
    }
    if has_normals {
        floats_per_vertex += 3;
    }
    let stride = (floats_per_vertex * std::mem::size_of::<f32>()) as i32;

    unsafe {
        // Create and bind VAO
        gl::GenVertexArrays(1, &mut vao.id);
        gl::BindVertexArray(vao.id);

        // Create and upload VBO
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Create and upload EBO
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as _,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // Set up vertex attributes
        let mut offset: usize = 0;

        // Position (Location 0) - Always present
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(0);
        offset += 3 * std::mem::size_of::<f32>();

        // Texture Coordinates (Location 2)
        if has_texcoords {
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(2);
            offset += 2 * std::mem::size_of::<f32>();
        }

        // Normals (Location 3)
        if has_normals {
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                offset as *const _,
            );
            gl::EnableVertexAttribArray(3);
        }

        // Unbind VAO
        gl::BindVertexArray(0);
    }

    RenderData {
        vao,
        index_count: indices.len() as i32,
        program_id: 0,
    }
}