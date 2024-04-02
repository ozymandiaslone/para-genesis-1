pub struct ZCamera {
    pub xpos: f32,
    pub ypos: f32, 
    pub zoom: f64,
}

impl ZCamera {
    pub fn new_origin() -> ZCamera {
        ZCamera {
            xpos: 0., 
            ypos: 0.,
            zoom: 0.1,
        }
    }
    pub fn add_zoom(&mut self, dz: f32) {
        self.zoom += dz as f64 / 2.;
    }
}
