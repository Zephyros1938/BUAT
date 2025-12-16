use gl::types::{GLint, GLuint};
use image;
use log::debug;

pub struct Texture {
    pub id: GLuint,
    pub width: i32,
    pub height: i32,
}

impl Texture {
    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Texture {
            id: self.id,
            width: self.width,
            height: self.height,
        }
    }
}

impl Copy for Texture {}

pub struct TextureLoadOptions {
    pub generate_mipmaps: bool,
    pub wrap_s: GLint,
    pub wrap_t: GLint,
    pub min_filter: GLint,
    pub mag_filter: GLint,
}

impl Default for TextureLoadOptions {
    fn default() -> Self {
        TextureLoadOptions {
            generate_mipmaps: true,
            wrap_s: gl::REPEAT as GLint,
            wrap_t: gl::REPEAT as GLint,
            min_filter: gl::LINEAR_MIPMAP_LINEAR as GLint,
            mag_filter: gl::LINEAR as GLint,
        }
    }
}

pub fn load_texture_from_file(
    path: &str,
    load_options: TextureLoadOptions,
) -> Result<Texture, String> {
    let img = image::open(path).map_err(|e| format!("Failed to load texture: {}", e))?;
    let img = img.flipv().into_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();

    let mut texture_id: GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA8 as GLint,
            width as GLint,
            height as GLint,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
        );
        if load_options.generate_mipmaps {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            load_options.wrap_s as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            load_options.wrap_t as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            load_options.min_filter as GLint,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MAG_FILTER,
            load_options.mag_filter as GLint,
        );

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }

    if texture_id == 0 {
        return Err("Failed to generate texture ID".to_string());
    }

    debug!(
        "Loaded texture from {} ({}x{}), with ID #{}",
        path, width, height, texture_id
    );

    Ok(Texture {
        id: texture_id,
        width: width as i32,
        height: height as i32,
    })
}
