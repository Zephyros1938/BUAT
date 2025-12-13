use glfw::Context;
use log::debug;

pub const KEY_COUNT: usize = 1024;

fn check_hyprland() -> bool {
    use regex::Regex;
    use std::process::{Command, Stdio};
    let out = Command::new("sh")
        .arg("-c") // The '-c' flag tells the shell to read the command from the next string
        .arg("ps aux | grep hyprland | grep -v grep") // The actual shell command
        .output()
        .expect("Failed to run hyprland check command via shell");
    let out_p: String = String::from_utf8(out.stdout).unwrap();
    debug!("{}", out_p.as_str());
    let re = Regex::new(r"hyprland").unwrap();
    if re.is_match(out_p.as_str()) {
        return true;
    }
    return false;
}

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
        debug!("{}", check_hyprland());
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

    pub fn fullscreen(&mut self) -> Result<(), String> {
        if check_hyprland() {
            return Err("Hyprland dissallows full screen".to_owned());
        }
        self.FULLSCREEN = !self.FULLSCREEN;
        if self.FULLSCREEN {
            self.POSITION = self.win.get_pos();
            self.SIZE = (self.win.get_size().0 as i32, self.win.get_size().1 as i32);

            self.glfw.with_primary_monitor(|_, monitor| {
                if let Some(monitor) = monitor {
                    let video_mode = monitor.get_video_mode().expect("Failed to get video mode");
                    self.win.set_monitor(
                        glfw::WindowMode::FullScreen(monitor),
                        0,
                        0,
                        video_mode.width,
                        video_mode.height,
                        Some(video_mode.refresh_rate),
                    );
                }
            })
        } else {
            self.win.set_monitor(
                glfw::WindowMode::Windowed,
                self.POSITION.0,
                self.POSITION.1,
                self.SIZE.0 as u32,
                self.SIZE.1 as u32,
                None,
            );
        }
        Ok(())
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
