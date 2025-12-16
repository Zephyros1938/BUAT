pub struct MouseHandler {
    last_x: f32,
    last_y: f32,
    first_mouse: bool,
    pub locked: bool,
    sy: f32,
}

impl MouseHandler {
    pub fn new(width: f32, height: f32) -> MouseHandler {
        Self {
            last_x: width as f32 / 2.0,
            last_y: height as f32 / 2.0,
            first_mouse: true,
            locked: true,
            sy: 0.,
        }
    }

    pub fn handle_mouse(&mut self, xpos: f32, ypos: f32) -> (f32, f32) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }

        let xoffset = xpos - self.last_x;
        let yoffset = self.last_y - ypos;

        self.last_x = xpos;
        self.last_y = ypos;

        (xoffset, yoffset)
    }
}
