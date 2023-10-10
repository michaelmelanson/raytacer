use serde::{Deserialize, Serialize};

use crate::vec::Vec3;

#[derive(Serialize, Deserialize)]
pub enum CameraLens {
    Orthogonal {
        pixel0_loc: Vec3,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
        defocus_disc_uv: Option<(Vec3, Vec3)>,
    },
}
