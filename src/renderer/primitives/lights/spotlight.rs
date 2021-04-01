pub struct SpotLight {
    position: glm::Vec3,
    direction: glm::Mat4,
    color: wgpu::Color,
    constant: f32,
    linear: f32,
    quadratic: f32,
    /// Requires cos
    inner_cutoff: f32,
    /// Requires cos
    outer_cutoff: f32,
}
pub struct SpotLightRaw {
    pos: [f32; 4],
    dir: [[f32; 4]; 4],
    color: [f32; 4],
    attenuation_values: [f32; 4],
    cutoffs: [f32; 4],
}

impl SpotLight {
    pub fn new(
        position: glm::Vec3,
        direction: glm::Mat4,
        color: wgpu::Color,
        constant: f32,
        linear: f32,
        quadratic: f32,
        inner_cutoff: f32,
        outer_cutoff: f32,
    ) -> Self {
        Self {
            position,
            color,
            direction,
            constant,
            linear,
            quadratic,
            inner_cutoff,
            outer_cutoff,
        }
    }
    pub fn to_raw(&self) -> SpotLightRaw {
        SpotLightRaw {
            attenuation_values: [self.constant, self.linear, self.quadratic, 1.0],
            color: [
                self.color.r as f32,
                self.color.g as f32,
                self.color.b as f32,
                self.color.a as f32,
            ],
            pos: [self.position.x, self.position.y, self.position.z, 1.0],
            dir: self.direction.into(),
            cutoffs: [self.inner_cutoff, self.outer_cutoff, 1.0, 1.0],
        }
    }
}