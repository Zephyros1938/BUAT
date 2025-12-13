/*
Note:
This code is so old, if theres any way to make it more efficient, please do so.
Not that its bad, just that i want ALL of my code to be efficient, but i think this is good for now
Most comments are from collabs
*/

use nalgebra_glm::{Mat4, Vec3};

pub struct Camera3d {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub mouse_sensitivity: f32,
    pub move_speed: f32,
    pub zoom: f32, // Field of View (FOV) in degrees
    pub constrain_pitch: bool,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera3d {
    pub fn new(
        position: Vec3,
        world_up: Vec3,
        yaw: f32,
        pitch: f32,
        mouse_sensitivity: f32,
        move_speed: f32,
        zoom: f32,
        constrain_pitch: bool,
        aspect_ratio: f32,
        near: f32,
        far: f32,
    ) -> Camera3d {
        let front = get_front(yaw, pitch);
        let right = Vec3::normalize(&front.cross(&world_up));
        let up = Vec3::normalize(&right.cross(&front));

        Self {
            position,
            front,
            right,
            up,
            world_up,
            yaw,
            pitch,
            mouse_sensitivity,
            move_speed,
            zoom,
            constrain_pitch,
            aspect_ratio,
            near,
            far,
        }
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let eye: Vec3 = Vec3::from(self.position);
        let target = eye + self.front;

        nalgebra_glm::look_at(&eye, &target, &self.up)
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        Mat4::new_perspective(
            self.aspect_ratio,      // aspect ratio FIRST
            self.zoom.to_radians(), // FOV (vertical) SECOND
            self.near,
            self.far,
        )
    }

    pub fn process_mouse(&mut self, xoff: f32, yoff: f32) {
        let xf = xoff * self.mouse_sensitivity;
        let yf = yoff * self.mouse_sensitivity;

        self.yaw += xf;
        self.pitch += yf;

        if self.constrain_pitch {
            // Clamps pitch between -89 and 89 deg
            self.pitch = self.pitch.max(-89f32).min(89f32);
        }

        self.update_vectors();
    }

    pub fn update_vectors(&mut self) {
        self.front = get_front(self.yaw, self.pitch);

        let mut right = self.front.cross(&self.world_up);

        if right.magnitude_squared() < 1e-8 {
            right = self.front.cross(&Vec3::new(0.0, 0.0, -1.0));
        }

        self.right = right.normalize();
        self.up = self.right.cross(&self.front).normalize();
    }

    pub fn set_aspect_ratio(&mut self, w: f32, h: f32) {
        self.aspect_ratio = w / h;
    }
}

fn get_front(yaw: f32, pitch: f32) -> Vec3 {
    let yaw_rad = yaw.to_radians();
    let pitch_rad = pitch.to_radians();
    Vec3::new(
        yaw_rad.sin() * pitch_rad.cos(),  // x
        pitch_rad.sin(),                  // y
        -yaw_rad.cos() * pitch_rad.cos(), // z
    )
    .normalize()
}
