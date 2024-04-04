use bevy::prelude::*;

pub struct Simulation {
    params: Params,
}

pub struct Params {
    pub node_size: f32,
    pub node_num_x: usize,
    pub node_num_y: usize,

    pub dt: f32,
    pub m: f32,
    pub g: f32,

    pub mouse_force: Vec3,
    pub r: Vec3,
    pub k: Vec3,

    pub dampen_factor: f32,
    pub enable_wind: bool,

    pub side_panel_width: f32,
}

impl Params {
    fn get_rest_lengths(&mut self, structural_rest_length: f32) {
        self.r[0] = structural_rest_length;
        self.r[1] = self.r[0] * (2_f32).sqrt();
        self.r[2] = self.r[1] * 2.0;
    }
}

impl Simulation {
    pub fn new(mut params: Params) -> Self {
        params.get_rest_lengths(params.r[0]);
        Simulation { params }
    }
}
