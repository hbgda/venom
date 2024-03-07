crate::native_func!(
    crate::utils::scan(crate::patterns::transform::SET_POSITION).unwrap(),
    SET_POSITION(*mut Transform, *const Vector3)
);
crate::native_func!(
    crate::utils::scan(crate::patterns::transform::SET_SCALE).unwrap(),
    SET_SCALE(*mut Transform, *const Vector3)
);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct SpatialData {
    _0x0: [u8; 0x30],
    pos: Vector3
}

#[repr(C)]
pub struct Transform {
    spatial_data: SpatialData
}

impl Transform {
    pub fn spatial_data(&self) -> &SpatialData {
        &self.spatial_data
    }

    pub unsafe fn set_position(&mut self, new_pos: &Vector3) {
        SET_POSITION(self, new_pos);
    }

    pub fn position(&self) -> &Vector3 {
        &self.spatial_data.pos
    }

    pub unsafe fn set_scale(&mut self, new_scale: &Vector3) {
        SET_SCALE(self, new_scale);
    }

    // pub fn get_scale(&self) -> Vector3 {
    //     self.scale
    // }
}