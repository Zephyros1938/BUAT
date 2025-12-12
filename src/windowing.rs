use glfw::Context;

pub const KEY_COUNT: usize = 1024;

pub struct GameWindow {
    pub key_states: [bool; KEY_COUNT],
    pub fullscreen: bool,
    pub position: (i32, i32),
    pub size: (i32, i32),
    pub glfw: glfw::Glfw,
    pub win: glfw::PWindow,
    pub ev: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    
    dt: f32,
    last_frame: f32
}

impl GameWindow {
    pub fn new(hints: GameWindowHints) -> Result<Self, String> {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(
            hints.gl_context.0,
            hints.gl_context.1,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(hints.profile));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut p, ev) = glfw
            .create_window(
                hints.size.0 as u32,
                hints.size.1 as u32,
                hints.title,
                glfw::WindowMode::Windowed,
            )
            .unwrap();

        p.make_current();
        p.set_all_polling(true);

        Ok(Self {
            key_states: [false; KEY_COUNT],
            position: (0, 0),
            fullscreen: hints.fullscreen,
            size: hints.size,
            glfw: glfw,
            win: p,
            ev: ev,
            dt: 0.,
            last_frame: 0.
        })
    }

    pub fn tick(&mut self) -> f32
    {
        let current = self.glfw.get_time() as f32;
        self.dt = current - self.last_frame;
        self.last_frame = current;
        self.glfw.poll_events();
        self.dt
    }

}

pub enum WindowMode {
    FULLSCREEN,
    WINDOWED,
}

pub struct GameWindowHints<'a> {
    pub gl_context: (u32, u32),
    pub profile: glfw::OpenGlProfileHint,
    pub title: &'a str,
    pub fullscreen: bool,
    pub size: (i32, i32),
}
