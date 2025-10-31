use std::cell::RefCell;

use ndarray::{Array3, ShapeBuilder};
use nifti::{NiftiObject, ReaderStreamedOptions};
use serde::{Deserialize, Serialize};
use web_sys::File;
use nifti::volume::ndarray::IntoNdArray;

pub struct NiftiWorkerState {
    pub volume: ndarray::Array3<f32>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct NiftiProperies {
    pub dimensions: VoxelDimensions,
    pub maximum: f32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct VoxelDimensions {
    pub rows:    u16,
    pub columns: u16,
    pub slices:  u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum AnatomicalAxis {
    Axial    = 0, // XY plane (constant Z)
    Coronal  = 1, // XZ plane (constant Y)
    Sagittal = 2, // YZ plane (constant X)
}

thread_local! {
    pub static STATE: RefCell<Option<NiftiWorkerState>> = RefCell::new(None);
}

pub async fn read_file(file: File) -> NiftiProperies {
    crate::debug!("[file-reader] reading the nifti file");
    let nifti = ReaderStreamedOptions::new().read_web_file(file).expect("Cannot read NIfTI");
    let mut volume = nifti.into_volume();
    let dimensions = volume.dim();

    let voxel_dimensions = VoxelDimensions {
        rows:    dimensions[0],
        columns: dimensions[1],
        slices:  dimensions[2],
    };

   // Pre-allocate a 3D array for the entire volume
    let mut volume_array = ndarray::Array3::<f32>::zeros((
        dimensions[0] as usize,
        dimensions[1] as usize,
        dimensions[2] as usize,
    ).f());

    let mut slices_counter = 0;
    while volume.slices_left() != 0 {
        let slice = volume.read_slice().expect("Could not read slice");

        // Convert the slice to a 2D array and insert it into the 3D volume
        let slice_array = slice.into_ndarray::<f32>()
            .expect("Could not convert slice to ndarray")
            .into_dimensionality::<ndarray::Ix2>()
            .expect("Could not convert slice to 2D array");

        // Copy the slice data into the appropriate position in the 3D volume
        volume_array.slice_mut(ndarray::s![.., .., slices_counter])
            .assign(&slice_array);

        slices_counter += 1;
    }

    crate::log!("[file-reader] read {} nifti slices", slices_counter);

    let maximum = get_max_data_value(&volume_array);

    STATE.replace(Some(NiftiWorkerState {
        volume: volume_array,
    }));

    NiftiProperies {
        dimensions: voxel_dimensions,
        maximum,
    }
}

fn get_max_data_value(array: &Array3<f32>) -> f32 {
    array.fold(0.0, |max, &x| max.max(x))
}
