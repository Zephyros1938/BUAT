use gl::types::{GLint, GLuint};
use log::debug;
use nalgebra_glm as glm;

const ERROR_ON_NO_UNIFORM_FOUND: bool = false;
pub struct Shader {
    pub program: GLuint,
}

fn compile_shader(source: &str, kind: GLuint) -> GLuint {
    debug!(
        "Compiling {} shader",
        match kind {
            gl::VERTEX_SHADER => "VERT",
            gl::FRAGMENT_SHADER => "FRAG",
            gl::COMPUTE_SHADER => "COMP",
            gl::GEOMETRY_SHADER => "GEOM",
            _ => "UNKNOWN",
        }
    );
    let id = unsafe { gl::CreateShader(kind) };
    let c_source = std::ffi::CString::new(source.as_bytes()).unwrap();

    unsafe {
        gl::ShaderSource(id, 1, &c_source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: GLint = 1;
    unsafe { gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success) };

    if success == 0 {
        let mut len = 0;
        unsafe { gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len) };

        let error_buffer = vec![0u8; (len as usize).saturating_sub(1)];
        let error_log = std::ffi::CString::new(error_buffer).unwrap();

        unsafe {
            gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error_log.as_ptr() as *mut _);
        }

        panic!(
            "Shader compilation failed (type {}): {}",
            kind,
            error_log.to_string_lossy()
        );
    }

    debug!("Created shader with id {}", id);

    id
}
fn get_uniform_location(program: GLuint, name: &str) -> i32 {
    let c_name = std::ffi::CString::new(name)
        .map_err(|_| format!("Failed to create CString for uniform: {}", name))
        .unwrap();

    unsafe { gl::GetUniformLocation(program, c_name.as_ptr()) }
}

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Self {
        let vert_shader = compile_shader(vertex_src, gl::VERTEX_SHADER);
        let frag_shader = compile_shader(fragment_src, gl::FRAGMENT_SHADER);

        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vert_shader);
            gl::AttachShader(program, frag_shader);
            gl::LinkProgram(program);

            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);

            program
        };
        debug!("Created program with id {}", program);

        Shader { program }
    }

    pub fn from_files(
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<Shader, Box<dyn std::error::Error>> {
        let vertex_source = crate::util::file::load_file_as_cstr(vertex_path)?;
        let fragment_source = crate::util::file::load_file_as_cstr(fragment_path)?;

        Ok(Shader::new(
            &vertex_source.to_string_lossy(),
            &fragment_source.to_string_lossy(),
        ))
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_mat4(&self, name: &str, mat: &glm::Mat4) -> Result<(), String> {
        let location = get_uniform_location(self.program, name);
        if location == -1 {
            if ERROR_ON_NO_UNIFORM_FOUND {
                return Err(format!("Uniform '{}' not found in shader program", name));
            } else {
                return Ok(());
            }
        }

        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, mat.as_ptr());
        }
        Ok(())
    }

    pub fn set_vec3(&self, name: &str, vec: &glm::Vec3) -> Result<(), String> {
        let location = get_uniform_location(self.program, name);
        if location == -1 {
            if ERROR_ON_NO_UNIFORM_FOUND {
                return Err(format!("Uniform '{}' not found in shader program", name));
            } else {
                return Ok(());
            }
        }

        unsafe {
            gl::Uniform3fv(location, 1, vec.as_ptr());
        }
        Ok(())
    }

    pub fn set_int(&self, name: &str, v: i32) -> Result<(), String> {
        let location = get_uniform_location(self.program, name);
        if location == -1 {
            if ERROR_ON_NO_UNIFORM_FOUND {
                return Err(format!("Uniform '{}' not found in shader program", name));
            } else {
                return Ok(());
            }
        }

        unsafe {
            gl::Uniform1i(location, v);
        }
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

pub struct VertexArrayObject {
    pub id: GLuint,
}

impl VertexArrayObject {
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        debug!("Created VAO #{}", id);
        VertexArrayObject { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}
