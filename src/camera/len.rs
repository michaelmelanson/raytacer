use crate::vec::Vec3;

pub enum CameraLens {
    Orthogonal {
        pixel0_loc: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    },
}
