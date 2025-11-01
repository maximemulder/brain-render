use serde::{Deserialize, Serialize};


pub struct Nifti {
    pub volume: ndarray::Array3<f32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiProperies {
    pub dimensions: VoxelDimensions,
    pub maximum: f32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct VoxelDimensions {
    pub rows:    usize,
    pub columns: usize,
    pub slices:  usize,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AnatomicalAxis {
    Axial    = 0, // XY plane (constant Z)
    Coronal  = 1, // XZ plane (constant Y)
    Sagittal = 2, // YZ plane (constant X)
}

impl Nifti {
    pub fn get_properties(&self) -> NiftiProperies {
        let maximum = self.get_max_intensity();
        let dimensions = self.volume.dim();

        NiftiProperies {
            dimensions: VoxelDimensions {
                rows:    dimensions.0,
                columns: dimensions.1,
                slices:  dimensions.2,
            },
            maximum,
        }
    }

    pub fn get_max_intensity(&self) -> f32 {
        self.volume.fold(0.0, |max, &x| max.max(x))
    }
}
