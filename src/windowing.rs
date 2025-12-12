use glfw::Context;
use log::{debug, error, info, trace, warn};
use log4rs;

pub const KEY_COUNT: usize = 1024;

pub struct GameWindow {
    pub key_states: [bool; KEY_COUNT],
    FULLSCREEN: bool,
    POSITION: (i32, i32),
    SIZE: (i32, i32),

    pub glfw: glfw::Glfw,
    pub win: glfw::PWindow,
    pub ev: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    dt: f32,
    last_frame: f32,
}

impl GameWindow {
    pub fn new(hints: GameWindowHints) -> Result<Self, String> {
        debug!(
            "Creating GameWindow with hints:\n\tContext Version: {}.{}\n\tProfile: {}\n\tTitle: <{}>\n\tFullscreen?: {}\n\tSize: {}x{}",
            hints.gl_context.0,
            hints.gl_context.1,
            match hints.profile {
                glfw::OpenGlProfileHint::Any => "ANY",
                glfw::OpenGlProfileHint::Compat => "COMPAT",
                glfw::OpenGlProfileHint::Core => "CORE",
            },
            hints.title,
            hints.fullscreen,
            hints.size.0,
            hints.size.1
        );
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        debug!("Initialized GLFW");
        glfw.window_hint(glfw::WindowHint::ContextVersion(
            hints.gl_context.0,
            hints.gl_context.1,
        ));
        debug!(
            "Set GLFW context to {}.{}",
            hints.gl_context.0, hints.gl_context.1
        );
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(hints.profile));
        debug!(
            "Set GLFW profile to {}",
            match hints.profile {
                glfw::OpenGlProfileHint::Any => "ANY",
                glfw::OpenGlProfileHint::Compat => "COMPAT",
                glfw::OpenGlProfileHint::Core => "CORE",
            }
        );
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut p, ev) = glfw
            .create_window(
                hints.size.0 as u32,
                hints.size.1 as u32,
                hints.title,
                glfw::WindowMode::Windowed,
            )
            .unwrap();
        debug!("Created PWindow");

        p.make_current();
        debug!("Made PWindow current");
        p.set_all_polling(true);
        debug!("Set polling to all");
        debug!("GameWindow created");

        Ok(Self {
            key_states: [false; KEY_COUNT],
            POSITION: (0, 0),
            FULLSCREEN: hints.fullscreen,
            SIZE: hints.size,
            glfw: glfw,
            win: p,
            ev: ev,
            dt: 0.,
            last_frame: 0.,
        })
    }

    pub fn tick(&mut self) -> f32 {
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
