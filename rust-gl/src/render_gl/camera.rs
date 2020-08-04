use glm;

pub struct Camera {
    pos: glm::Vec3,
    target: glm::Vec3,
    up: glm::Vec3,
    perspective: glm::Mat4
}

impl Camera {
    pub fn new(pos: glm::Vec3, fov: f32, aspect: f32, near: f32, far: f32) -> Camera {
        Camera {
            pos,
            target: glm::vec3(0f32, 0f32, -1f32),
            up: glm::vec3(0f32, 1f32, 0f32),
            perspective: glm::perspective(fov.to_radians(), aspect, near, far),
        }
    }

    pub fn get_view(&self) -> glm::Mat4 {
        glm::look_at(
            &self.pos,
            &self.target,
            &self.up
        )
    }

    pub fn get_proj(&self) -> glm::Mat4 {
        self.perspective
    }

    pub fn set_position(&mut self, pos: glm::Vec3) {
        self.pos = pos;
    }

    pub fn get_position(&self) -> &glm::Vec3 {
        &self.pos
    }

    pub fn set_target(&mut self, target: glm::Vec3) {
        self.target = target;
    }
}